#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Ole")]
pub trait IADesktopP2_Impl: Sized {
    fn ReReadWallpaper(&self) -> ::windows::core::Result<()>;
    fn GetADObjectFlags(&self, pdwflags: *mut u32, dwmask: u32) -> ::windows::core::Result<()>;
    fn UpdateAllDesktopSubscriptions(&self) -> ::windows::core::Result<()>;
    fn MakeDynamicChanges(&self, poleobj: ::core::option::Option<&super::super::System::Ole::IOleObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::windows::core::RuntimeName for IADesktopP2 {}
#[cfg(feature = "Win32_System_Ole")]
impl IADesktopP2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IADesktopP2_Impl, const OFFSET: isize>() -> IADesktopP2_Vtbl {
        unsafe extern "system" fn ReReadWallpaper<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IADesktopP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReReadWallpaper().into()
        }
        unsafe extern "system" fn GetADObjectFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IADesktopP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetADObjectFlags(::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&dwmask)).into()
        }
        unsafe extern "system" fn UpdateAllDesktopSubscriptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IADesktopP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateAllDesktopSubscriptions().into()
        }
        unsafe extern "system" fn MakeDynamicChanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IADesktopP2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poleobj: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MakeDynamicChanges(::windows::core::from_raw_borrowed(&poleobj)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReReadWallpaper: ReReadWallpaper::<Identity, Impl, OFFSET>,
            GetADObjectFlags: GetADObjectFlags::<Identity, Impl, OFFSET>,
            UpdateAllDesktopSubscriptions: UpdateAllDesktopSubscriptions::<Identity, Impl, OFFSET>,
            MakeDynamicChanges: MakeDynamicChanges::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IADesktopP2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`, `\"implement\"`*"]
pub trait IActiveDesktopP_Impl: Sized {
    fn SetSafeMode(&self, dwflags: u32) -> ::windows::core::Result<()>;
    fn EnsureUpdateHTML(&self) -> ::windows::core::Result<()>;
    fn SetScheme(&self, pwszschemename: &::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetScheme(&self, pwszschemename: ::windows::core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IActiveDesktopP {}
impl IActiveDesktopP_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActiveDesktopP_Impl, const OFFSET: isize>() -> IActiveDesktopP_Vtbl {
        unsafe extern "system" fn SetSafeMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSafeMode(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn EnsureUpdateHTML<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnsureUpdateHTML().into()
        }
        unsafe extern "system" fn SetScheme<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszschemename: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetScheme(::core::mem::transmute(&pwszschemename), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetScheme<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActiveDesktopP_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszschemename: ::windows::core::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScheme(::core::mem::transmute_copy(&pwszschemename), ::core::mem::transmute_copy(&pdwcchbuffer), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSafeMode: SetSafeMode::<Identity, Impl, OFFSET>,
            EnsureUpdateHTML: EnsureUpdateHTML::<Identity, Impl, OFFSET>,
            SetScheme: SetScheme::<Identity, Impl, OFFSET>,
            GetScheme: GetScheme::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActiveDesktopP as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IBriefcaseInitiator_Impl: Sized {
    fn IsMonikerInBriefcase(&self, pmk: ::core::option::Option<&super::super::System::Com::IMoniker>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IBriefcaseInitiator {}
#[cfg(feature = "Win32_System_Com")]
impl IBriefcaseInitiator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBriefcaseInitiator_Impl, const OFFSET: isize>() -> IBriefcaseInitiator_Vtbl {
        unsafe extern "system" fn IsMonikerInBriefcase<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBriefcaseInitiator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsMonikerInBriefcase(::windows::core::from_raw_borrowed(&pmk)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsMonikerInBriefcase: IsMonikerInBriefcase::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBriefcaseInitiator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IEmptyVolumeCache_Impl: Sized {
    fn Initialize(&self, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: &::windows::core::PCWSTR, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows::core::Result<()>;
    fn GetSpaceUsed(&self, pdwlspaceused: *mut u64, picb: ::core::option::Option<&IEmptyVolumeCacheCallBack>) -> ::windows::core::Result<()>;
    fn Purge(&self, dwlspacetofree: u64, picb: ::core::option::Option<&IEmptyVolumeCacheCallBack>) -> ::windows::core::Result<()>;
    fn ShowProperties(&self, hwnd: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn Deactivate(&self) -> ::windows::core::Result<EMPTY_VOLUME_CACHE_FLAGS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::windows::core::RuntimeName for IEmptyVolumeCache {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl IEmptyVolumeCache_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>() -> IEmptyVolumeCache_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: ::windows::core::PCWSTR, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&hkregkey), ::core::mem::transmute(&pcwszvolume), ::core::mem::transmute_copy(&ppwszdisplayname), ::core::mem::transmute_copy(&ppwszdescription), ::core::mem::transmute_copy(&pdwflags)).into()
        }
        unsafe extern "system" fn GetSpaceUsed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlspaceused: *mut u64, picb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSpaceUsed(::core::mem::transmute_copy(&pdwlspaceused), ::windows::core::from_raw_borrowed(&picb)).into()
        }
        unsafe extern "system" fn Purge<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlspacetofree: u64, picb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Purge(::core::mem::transmute_copy(&dwlspacetofree), ::windows::core::from_raw_borrowed(&picb)).into()
        }
        unsafe extern "system" fn ShowProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShowProperties(::core::mem::transmute_copy(&hwnd)).into()
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCache_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Deactivate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetSpaceUsed: GetSpaceUsed::<Identity, Impl, OFFSET>,
            Purge: Purge::<Identity, Impl, OFFSET>,
            ShowProperties: ShowProperties::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmptyVolumeCache as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait IEmptyVolumeCache2_Impl: Sized + IEmptyVolumeCache_Impl {
    fn InitializeEx(&self, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: &::windows::core::PCWSTR, pcwszkeyname: &::windows::core::PCWSTR, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, ppwszbtntext: *mut ::windows::core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::windows::core::RuntimeName for IEmptyVolumeCache2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl IEmptyVolumeCache2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCache2_Impl, const OFFSET: isize>() -> IEmptyVolumeCache2_Vtbl {
        unsafe extern "system" fn InitializeEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCache2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: ::windows::core::PCWSTR, pcwszkeyname: ::windows::core::PCWSTR, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, ppwszbtntext: *mut ::windows::core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeEx(::core::mem::transmute_copy(&hkregkey), ::core::mem::transmute(&pcwszvolume), ::core::mem::transmute(&pcwszkeyname), ::core::mem::transmute_copy(&ppwszdisplayname), ::core::mem::transmute_copy(&ppwszdescription), ::core::mem::transmute_copy(&ppwszbtntext), ::core::mem::transmute_copy(&pdwflags)).into()
        }
        Self { base__: IEmptyVolumeCache_Vtbl::new::<Identity, Impl, OFFSET>(), InitializeEx: InitializeEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmptyVolumeCache2 as ::windows::core::ComInterface>::IID || iid == &<IEmptyVolumeCache as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`, `\"implement\"`*"]
pub trait IEmptyVolumeCacheCallBack_Impl: Sized {
    fn ScanProgress(&self, dwlspaceused: u64, dwflags: u32, pcwszstatus: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn PurgeProgress(&self, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IEmptyVolumeCacheCallBack {}
impl IEmptyVolumeCacheCallBack_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCacheCallBack_Impl, const OFFSET: isize>() -> IEmptyVolumeCacheCallBack_Vtbl {
        unsafe extern "system" fn ScanProgress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCacheCallBack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlspaceused: u64, dwflags: u32, pcwszstatus: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ScanProgress(::core::mem::transmute_copy(&dwlspaceused), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pcwszstatus)).into()
        }
        unsafe extern "system" fn PurgeProgress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEmptyVolumeCacheCallBack_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PurgeProgress(::core::mem::transmute_copy(&dwlspacefreed), ::core::mem::transmute_copy(&dwlspacetofree), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pcwszstatus)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ScanProgress: ScanProgress::<Identity, Impl, OFFSET>,
            PurgeProgress: PurgeProgress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEmptyVolumeCacheCallBack as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IReconcilableObject_Impl: Sized {
    fn Reconcile(&self, pinitiator: ::core::option::Option<&IReconcileInitiator>, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut ::core::option::Option<super::super::System::Com::IMoniker>, ploutindex: *mut i32, pstgnewresidues: ::core::option::Option<&super::super::System::Com::StructuredStorage::IStorage>, pvreserved: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetProgressFeedbackMaxEstimate(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IReconcilableObject {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IReconcilableObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReconcilableObject_Impl, const OFFSET: isize>() -> IReconcilableObject_Vtbl {
        unsafe extern "system" fn Reconcile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReconcilableObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitiator: *mut ::core::ffi::c_void, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut *mut ::core::ffi::c_void, ploutindex: *mut i32, pstgnewresidues: *mut ::core::ffi::c_void, pvreserved: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reconcile(::windows::core::from_raw_borrowed(&pinitiator), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&hwndowner), ::core::mem::transmute_copy(&hwndprogressfeedback), ::core::mem::transmute_copy(&ulcinput), ::core::mem::transmute_copy(&rgpmkotherinput), ::core::mem::transmute_copy(&ploutindex), ::windows::core::from_raw_borrowed(&pstgnewresidues), ::core::mem::transmute_copy(&pvreserved)).into()
        }
        unsafe extern "system" fn GetProgressFeedbackMaxEstimate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReconcilableObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulprogressmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProgressFeedbackMaxEstimate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulprogressmax, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reconcile: Reconcile::<Identity, Impl, OFFSET>,
            GetProgressFeedbackMaxEstimate: GetProgressFeedbackMaxEstimate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReconcilableObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_UI_LegacyWindowsEnvironmentFeatures\"`, `\"implement\"`*"]
pub trait IReconcileInitiator_Impl: Sized {
    fn SetAbortCallback(&self, punkforabort: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetProgressFeedback(&self, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IReconcileInitiator {}
impl IReconcileInitiator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReconcileInitiator_Impl, const OFFSET: isize>() -> IReconcileInitiator_Vtbl {
        unsafe extern "system" fn SetAbortCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReconcileInitiator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkforabort: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAbortCallback(::windows::core::from_raw_borrowed(&punkforabort)).into()
        }
        unsafe extern "system" fn SetProgressFeedback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReconcileInitiator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProgressFeedback(::core::mem::transmute_copy(&ulprogress), ::core::mem::transmute_copy(&ulprogressmax)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAbortCallback: SetAbortCallback::<Identity, Impl, OFFSET>,
            SetProgressFeedback: SetProgressFeedback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReconcileInitiator as ::windows::core::ComInterface>::IID
    }
}
