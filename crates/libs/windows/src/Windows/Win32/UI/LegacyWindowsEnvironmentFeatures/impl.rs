pub trait IADesktopP2Impl: Sized {
    fn ReReadWallpaper();
    fn GetADObjectFlags();
    fn UpdateAllDesktopSubscriptions();
    fn MakeDynamicChanges();
}
impl ::windows::core::RuntimeName for IADesktopP2 {
    const NAME: &'static str = "Windows.Win32.UI.LegacyWindowsEnvironmentFeatures.IADesktopP2";
}
impl IADesktopP2Vtbl {
    pub const fn new<Impl: IADesktopP2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IADesktopP2Vtbl {
        unsafe extern "system" fn ReReadWallpaper<Impl: IADesktopP2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReReadWallpaper() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetADObjectFlags<Impl: IADesktopP2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32, dwmask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetADObjectFlags(::core::mem::transmute_copy(&pdwflags), dwmask) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateAllDesktopSubscriptions<Impl: IADesktopP2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdateAllDesktopSubscriptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeDynamicChanges<Impl: IADesktopP2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poleobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MakeDynamicChanges(&*(&poleobj as *const <super::super::System::Ole::IOleObject as ::windows::core::Abi>::Abi as *const <super::super::System::Ole::IOleObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IADesktopP2>, base.5, ReReadWallpaper::<Impl, OFFSET>, GetADObjectFlags::<Impl, OFFSET>, UpdateAllDesktopSubscriptions::<Impl, OFFSET>, MakeDynamicChanges::<Impl, OFFSET>)
    }
}
pub trait IActiveDesktopPImpl: Sized {
    fn SetSafeMode();
    fn EnsureUpdateHTML();
    fn SetScheme();
    fn GetScheme();
}
impl ::windows::core::RuntimeName for IActiveDesktopP {
    const NAME: &'static str = "Windows.Win32.UI.LegacyWindowsEnvironmentFeatures.IActiveDesktopP";
}
impl IActiveDesktopPVtbl {
    pub const fn new<Impl: IActiveDesktopPImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IActiveDesktopPVtbl {
        unsafe extern "system" fn SetSafeMode<Impl: IActiveDesktopPImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSafeMode(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnsureUpdateHTML<Impl: IActiveDesktopPImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnsureUpdateHTML() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScheme<Impl: IActiveDesktopPImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszschemename: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetScheme(&*(&pwszschemename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScheme<Impl: IActiveDesktopPImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszschemename: super::super::Foundation::PWSTR, pdwcchbuffer: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetScheme(::core::mem::transmute_copy(&pwszschemename), pdwcchbuffer, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IActiveDesktopP>, base.5, SetSafeMode::<Impl, OFFSET>, EnsureUpdateHTML::<Impl, OFFSET>, SetScheme::<Impl, OFFSET>, GetScheme::<Impl, OFFSET>)
    }
}
pub trait IBriefcaseInitiatorImpl: Sized {
    fn IsMonikerInBriefcase();
}
impl ::windows::core::RuntimeName for IBriefcaseInitiator {
    const NAME: &'static str = "Windows.Win32.UI.LegacyWindowsEnvironmentFeatures.IBriefcaseInitiator";
}
impl IBriefcaseInitiatorVtbl {
    pub const fn new<Impl: IBriefcaseInitiatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IBriefcaseInitiatorVtbl {
        unsafe extern "system" fn IsMonikerInBriefcase<Impl: IBriefcaseInitiatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsMonikerInBriefcase(&*(&pmk as *const <super::super::System::Com::IMoniker as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IMoniker as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IBriefcaseInitiator>, base.5, IsMonikerInBriefcase::<Impl, OFFSET>)
    }
}
pub trait IEmptyVolumeCacheImpl: Sized {
    fn Initialize();
    fn GetSpaceUsed();
    fn Purge();
    fn ShowProperties();
    fn Deactivate();
}
impl ::windows::core::RuntimeName for IEmptyVolumeCache {
    const NAME: &'static str = "Windows.Win32.UI.LegacyWindowsEnvironmentFeatures.IEmptyVolumeCache";
}
impl IEmptyVolumeCacheVtbl {
    pub const fn new<Impl: IEmptyVolumeCacheImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEmptyVolumeCacheVtbl {
        unsafe extern "system" fn Initialize<Impl: IEmptyVolumeCacheImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: super::super::Foundation::PWSTR, ppwszdisplayname: *mut super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(
                &*(&hkregkey as *const <super::super::System::Registry::HKEY as ::windows::core::Abi>::Abi as *const <super::super::System::Registry::HKEY as ::windows::core::DefaultType>::DefaultType),
                &*(&pcwszvolume as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppwszdisplayname),
                ::core::mem::transmute_copy(&ppwszdescription),
                ::core::mem::transmute_copy(&pdwflags),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpaceUsed<Impl: IEmptyVolumeCacheImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwlspaceused: *mut u64, picb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSpaceUsed(::core::mem::transmute_copy(&pdwlspaceused), &*(&picb as *const <IEmptyVolumeCacheCallBack as ::windows::core::Abi>::Abi as *const <IEmptyVolumeCacheCallBack as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Purge<Impl: IEmptyVolumeCacheImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlspacetofree: u64, picb: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Purge(dwlspacetofree, &*(&picb as *const <IEmptyVolumeCacheCallBack as ::windows::core::Abi>::Abi as *const <IEmptyVolumeCacheCallBack as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShowProperties<Impl: IEmptyVolumeCacheImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ShowProperties(&*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Impl: IEmptyVolumeCacheImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Deactivate(::core::mem::transmute_copy(&pdwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEmptyVolumeCache>, base.5, Initialize::<Impl, OFFSET>, GetSpaceUsed::<Impl, OFFSET>, Purge::<Impl, OFFSET>, ShowProperties::<Impl, OFFSET>, Deactivate::<Impl, OFFSET>)
    }
}
pub trait IEmptyVolumeCache2Impl: Sized + IEmptyVolumeCacheImpl {
    fn InitializeEx();
}
impl ::windows::core::RuntimeName for IEmptyVolumeCache2 {
    const NAME: &'static str = "Windows.Win32.UI.LegacyWindowsEnvironmentFeatures.IEmptyVolumeCache2";
}
impl IEmptyVolumeCache2Vtbl {
    pub const fn new<Impl: IEmptyVolumeCache2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEmptyVolumeCache2Vtbl {
        unsafe extern "system" fn InitializeEx<Impl: IEmptyVolumeCache2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkregkey: super::super::System::Registry::HKEY, pcwszvolume: super::super::Foundation::PWSTR, pcwszkeyname: super::super::Foundation::PWSTR, ppwszdisplayname: *mut super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, ppwszbtntext: *mut super::super::Foundation::PWSTR, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).InitializeEx(
                &*(&hkregkey as *const <super::super::System::Registry::HKEY as ::windows::core::Abi>::Abi as *const <super::super::System::Registry::HKEY as ::windows::core::DefaultType>::DefaultType),
                &*(&pcwszvolume as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pcwszkeyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppwszdisplayname),
                ::core::mem::transmute_copy(&ppwszdescription),
                ::core::mem::transmute_copy(&ppwszbtntext),
                ::core::mem::transmute_copy(&pdwflags),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEmptyVolumeCache2>, base.5, InitializeEx::<Impl, OFFSET>)
    }
}
pub trait IEmptyVolumeCacheCallBackImpl: Sized {
    fn ScanProgress();
    fn PurgeProgress();
}
impl ::windows::core::RuntimeName for IEmptyVolumeCacheCallBack {
    const NAME: &'static str = "Windows.Win32.UI.LegacyWindowsEnvironmentFeatures.IEmptyVolumeCacheCallBack";
}
impl IEmptyVolumeCacheCallBackVtbl {
    pub const fn new<Impl: IEmptyVolumeCacheCallBackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEmptyVolumeCacheCallBackVtbl {
        unsafe extern "system" fn ScanProgress<Impl: IEmptyVolumeCacheCallBackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlspaceused: u64, dwflags: u32, pcwszstatus: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ScanProgress(dwlspaceused, dwflags, &*(&pcwszstatus as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PurgeProgress<Impl: IEmptyVolumeCacheCallBackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlspacefreed: u64, dwlspacetofree: u64, dwflags: u32, pcwszstatus: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PurgeProgress(dwlspacefreed, dwlspacetofree, dwflags, &*(&pcwszstatus as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEmptyVolumeCacheCallBack>, base.5, ScanProgress::<Impl, OFFSET>, PurgeProgress::<Impl, OFFSET>)
    }
}
pub trait IReconcilableObjectImpl: Sized {
    fn Reconcile();
    fn GetProgressFeedbackMaxEstimate();
}
impl ::windows::core::RuntimeName for IReconcilableObject {
    const NAME: &'static str = "Windows.Win32.UI.LegacyWindowsEnvironmentFeatures.IReconcilableObject";
}
impl IReconcilableObjectVtbl {
    pub const fn new<Impl: IReconcilableObjectImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IReconcilableObjectVtbl {
        unsafe extern "system" fn Reconcile<Impl: IReconcilableObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinitiator: ::windows::core::RawPtr, dwflags: u32, hwndowner: super::super::Foundation::HWND, hwndprogressfeedback: super::super::Foundation::HWND, ulcinput: u32, rgpmkotherinput: *mut ::windows::core::RawPtr, ploutindex: *mut i32, pstgnewresidues: ::windows::core::RawPtr, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reconcile(
                &*(&pinitiator as *const <IReconcileInitiator as ::windows::core::Abi>::Abi as *const <IReconcileInitiator as ::windows::core::DefaultType>::DefaultType),
                dwflags,
                &*(&hwndowner as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&hwndprogressfeedback as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                ulcinput,
                &*(&rgpmkotherinput as *const <super::super::System::Com::IMoniker as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IMoniker as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ploutindex),
                &*(&pstgnewresidues as *const <super::super::System::Com::StructuredStorage::IStorage as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::IStorage as ::windows::core::DefaultType>::DefaultType),
                &*(&pvreserved as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProgressFeedbackMaxEstimate<Impl: IReconcilableObjectImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulprogressmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetProgressFeedbackMaxEstimate(::core::mem::transmute_copy(&pulprogressmax)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IReconcilableObject>, base.5, Reconcile::<Impl, OFFSET>, GetProgressFeedbackMaxEstimate::<Impl, OFFSET>)
    }
}
pub trait IReconcileInitiatorImpl: Sized {
    fn SetAbortCallback();
    fn SetProgressFeedback();
}
impl ::windows::core::RuntimeName for IReconcileInitiator {
    const NAME: &'static str = "Windows.Win32.UI.LegacyWindowsEnvironmentFeatures.IReconcileInitiator";
}
impl IReconcileInitiatorVtbl {
    pub const fn new<Impl: IReconcileInitiatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IReconcileInitiatorVtbl {
        unsafe extern "system" fn SetAbortCallback<Impl: IReconcileInitiatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkforabort: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAbortCallback(&*(&punkforabort as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProgressFeedback<Impl: IReconcileInitiatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulprogress: u32, ulprogressmax: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProgressFeedback(ulprogress, ulprogressmax) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IReconcileInitiator>, base.5, SetAbortCallback::<Impl, OFFSET>, SetProgressFeedback::<Impl, OFFSET>)
    }
}
