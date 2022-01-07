pub trait IDesktopWindowXamlSourceNativeImpl: Sized {
    fn AttachToWindow();
    fn WindowHandle();
}
impl ::windows::core::RuntimeName for IDesktopWindowXamlSourceNative {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Xaml.IDesktopWindowXamlSourceNative";
}
impl IDesktopWindowXamlSourceNativeVtbl {
    pub const fn new<Impl: IDesktopWindowXamlSourceNativeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDesktopWindowXamlSourceNativeVtbl {
        unsafe extern "system" fn AttachToWindow<Impl: IDesktopWindowXamlSourceNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parentwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AttachToWindow(&*(&parentwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WindowHandle<Impl: IDesktopWindowXamlSourceNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).WindowHandle(::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDesktopWindowXamlSourceNative>, base.5, AttachToWindow::<Impl, OFFSET>, WindowHandle::<Impl, OFFSET>)
    }
}
pub trait IDesktopWindowXamlSourceNative2Impl: Sized + IDesktopWindowXamlSourceNativeImpl {
    fn PreTranslateMessage();
}
impl ::windows::core::RuntimeName for IDesktopWindowXamlSourceNative2 {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Xaml.IDesktopWindowXamlSourceNative2";
}
impl IDesktopWindowXamlSourceNative2Vtbl {
    pub const fn new<Impl: IDesktopWindowXamlSourceNative2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDesktopWindowXamlSourceNative2Vtbl {
        unsafe extern "system" fn PreTranslateMessage<Impl: IDesktopWindowXamlSourceNative2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: *const super::super::super::UI::WindowsAndMessaging::MSG, result: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PreTranslateMessage(&*(&message as *const <super::super::super::UI::WindowsAndMessaging::MSG as ::windows::core::Abi>::Abi as *const <super::super::super::UI::WindowsAndMessaging::MSG as ::windows::core::DefaultType>::DefaultType), &*(&result as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDesktopWindowXamlSourceNative2>, base.5, PreTranslateMessage::<Impl, OFFSET>)
    }
}
pub trait IFindReferenceTargetsCallbackImpl: Sized {
    fn FoundTrackerTarget();
}
impl ::windows::core::RuntimeName for IFindReferenceTargetsCallback {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Xaml.IFindReferenceTargetsCallback";
}
impl IFindReferenceTargetsCallbackVtbl {
    pub const fn new<Impl: IFindReferenceTargetsCallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IFindReferenceTargetsCallbackVtbl {
        unsafe extern "system" fn FoundTrackerTarget<Impl: IFindReferenceTargetsCallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FoundTrackerTarget(&*(&target as *const <IReferenceTrackerTarget as ::windows::core::Abi>::Abi as *const <IReferenceTrackerTarget as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IFindReferenceTargetsCallback>, base.5, FoundTrackerTarget::<Impl, OFFSET>)
    }
}
pub trait IReferenceTrackerImpl: Sized {
    fn ConnectFromTrackerSource();
    fn DisconnectFromTrackerSource();
    fn FindTrackerTargets();
    fn GetReferenceTrackerManager();
    fn AddRefFromTrackerSource();
    fn ReleaseFromTrackerSource();
    fn PegFromTrackerSource();
}
impl ::windows::core::RuntimeName for IReferenceTracker {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Xaml.IReferenceTracker";
}
impl IReferenceTrackerVtbl {
    pub const fn new<Impl: IReferenceTrackerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IReferenceTrackerVtbl {
        unsafe extern "system" fn ConnectFromTrackerSource<Impl: IReferenceTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConnectFromTrackerSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisconnectFromTrackerSource<Impl: IReferenceTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisconnectFromTrackerSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindTrackerTargets<Impl: IReferenceTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindTrackerTargets(&*(&callback as *const <IFindReferenceTargetsCallback as ::windows::core::Abi>::Abi as *const <IFindReferenceTargetsCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetReferenceTrackerManager<Impl: IReferenceTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetReferenceTrackerManager(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRefFromTrackerSource<Impl: IReferenceTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddRefFromTrackerSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseFromTrackerSource<Impl: IReferenceTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseFromTrackerSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PegFromTrackerSource<Impl: IReferenceTrackerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).PegFromTrackerSource() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IReferenceTracker>, base.5, ConnectFromTrackerSource::<Impl, OFFSET>, DisconnectFromTrackerSource::<Impl, OFFSET>, FindTrackerTargets::<Impl, OFFSET>, GetReferenceTrackerManager::<Impl, OFFSET>, AddRefFromTrackerSource::<Impl, OFFSET>, ReleaseFromTrackerSource::<Impl, OFFSET>, PegFromTrackerSource::<Impl, OFFSET>)
    }
}
pub trait IReferenceTrackerExtensionImpl: Sized {}
impl ::windows::core::RuntimeName for IReferenceTrackerExtension {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Xaml.IReferenceTrackerExtension";
}
impl IReferenceTrackerExtensionVtbl {
    pub const fn new<Impl: IReferenceTrackerExtensionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IReferenceTrackerExtensionVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IReferenceTrackerExtension>, base.5)
    }
}
pub trait IReferenceTrackerHostImpl: Sized {
    fn DisconnectUnusedReferenceSources();
    fn ReleaseDisconnectedReferenceSources();
    fn NotifyEndOfReferenceTrackingOnThread();
    fn GetTrackerTarget();
    fn AddMemoryPressure();
    fn RemoveMemoryPressure();
}
impl ::windows::core::RuntimeName for IReferenceTrackerHost {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Xaml.IReferenceTrackerHost";
}
impl IReferenceTrackerHostVtbl {
    pub const fn new<Impl: IReferenceTrackerHostImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IReferenceTrackerHostVtbl {
        unsafe extern "system" fn DisconnectUnusedReferenceSources<Impl: IReferenceTrackerHostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: XAML_REFERENCETRACKER_DISCONNECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DisconnectUnusedReferenceSources(options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDisconnectedReferenceSources<Impl: IReferenceTrackerHostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseDisconnectedReferenceSources() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyEndOfReferenceTrackingOnThread<Impl: IReferenceTrackerHostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).NotifyEndOfReferenceTrackingOnThread() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTrackerTarget<Impl: IReferenceTrackerHostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unknown: *mut ::core::ffi::c_void, newreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTrackerTarget(&*(&unknown as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&newreference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMemoryPressure<Impl: IReferenceTrackerHostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bytesallocated: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddMemoryPressure(bytesallocated) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMemoryPressure<Impl: IReferenceTrackerHostImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bytesallocated: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveMemoryPressure(bytesallocated) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IReferenceTrackerHost>, base.5, DisconnectUnusedReferenceSources::<Impl, OFFSET>, ReleaseDisconnectedReferenceSources::<Impl, OFFSET>, NotifyEndOfReferenceTrackingOnThread::<Impl, OFFSET>, GetTrackerTarget::<Impl, OFFSET>, AddMemoryPressure::<Impl, OFFSET>, RemoveMemoryPressure::<Impl, OFFSET>)
    }
}
pub trait IReferenceTrackerManagerImpl: Sized {
    fn ReferenceTrackingStarted();
    fn FindTrackerTargetsCompleted();
    fn ReferenceTrackingCompleted();
    fn SetReferenceTrackerHost();
}
impl ::windows::core::RuntimeName for IReferenceTrackerManager {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Xaml.IReferenceTrackerManager";
}
impl IReferenceTrackerManagerVtbl {
    pub const fn new<Impl: IReferenceTrackerManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IReferenceTrackerManagerVtbl {
        unsafe extern "system" fn ReferenceTrackingStarted<Impl: IReferenceTrackerManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReferenceTrackingStarted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindTrackerTargetsCompleted<Impl: IReferenceTrackerManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, findfailed: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FindTrackerTargetsCompleted(findfailed) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReferenceTrackingCompleted<Impl: IReferenceTrackerManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReferenceTrackingCompleted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReferenceTrackerHost<Impl: IReferenceTrackerManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetReferenceTrackerHost(&*(&value as *const <IReferenceTrackerHost as ::windows::core::Abi>::Abi as *const <IReferenceTrackerHost as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IReferenceTrackerManager>, base.5, ReferenceTrackingStarted::<Impl, OFFSET>, FindTrackerTargetsCompleted::<Impl, OFFSET>, ReferenceTrackingCompleted::<Impl, OFFSET>, SetReferenceTrackerHost::<Impl, OFFSET>)
    }
}
pub trait IReferenceTrackerTargetImpl: Sized {
    fn AddRefFromReferenceTracker();
    fn ReleaseFromReferenceTracker();
    fn Peg();
    fn Unpeg();
}
impl ::windows::core::RuntimeName for IReferenceTrackerTarget {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Xaml.IReferenceTrackerTarget";
}
impl IReferenceTrackerTargetVtbl {
    pub const fn new<Impl: IReferenceTrackerTargetImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IReferenceTrackerTargetVtbl {
        unsafe extern "system" fn AddRefFromReferenceTracker<Impl: IReferenceTrackerTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddRefFromReferenceTracker() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseFromReferenceTracker<Impl: IReferenceTrackerTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseFromReferenceTracker() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Peg<Impl: IReferenceTrackerTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Peg() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unpeg<Impl: IReferenceTrackerTargetImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unpeg() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IReferenceTrackerTarget>, base.5, AddRefFromReferenceTracker::<Impl, OFFSET>, ReleaseFromReferenceTracker::<Impl, OFFSET>, Peg::<Impl, OFFSET>, Unpeg::<Impl, OFFSET>)
    }
}
pub trait ISurfaceImageSourceManagerNativeImpl: Sized {
    fn FlushAllSurfacesWithDevice();
}
impl ::windows::core::RuntimeName for ISurfaceImageSourceManagerNative {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Xaml.ISurfaceImageSourceManagerNative";
}
impl ISurfaceImageSourceManagerNativeVtbl {
    pub const fn new<Impl: ISurfaceImageSourceManagerNativeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISurfaceImageSourceManagerNativeVtbl {
        unsafe extern "system" fn FlushAllSurfacesWithDevice<Impl: ISurfaceImageSourceManagerNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FlushAllSurfacesWithDevice(&*(&device as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISurfaceImageSourceManagerNative>, base.5, FlushAllSurfacesWithDevice::<Impl, OFFSET>)
    }
}
pub trait ISurfaceImageSourceNativeImpl: Sized {
    fn SetDevice();
    fn BeginDraw();
    fn EndDraw();
}
impl ::windows::core::RuntimeName for ISurfaceImageSourceNative {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Xaml.ISurfaceImageSourceNative";
}
impl ISurfaceImageSourceNativeVtbl {
    pub const fn new<Impl: ISurfaceImageSourceNativeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISurfaceImageSourceNativeVtbl {
        unsafe extern "system" fn SetDevice<Impl: ISurfaceImageSourceNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDevice(&*(&device as *const <super::super::super::Graphics::Dxgi::IDXGIDevice as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Dxgi::IDXGIDevice as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginDraw<Impl: ISurfaceImageSourceNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updaterect: super::super::super::Foundation::RECT, surface: *mut ::windows::core::RawPtr, offset: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginDraw(&*(&updaterect as *const <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&surface), ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDraw<Impl: ISurfaceImageSourceNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndDraw() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISurfaceImageSourceNative>, base.5, SetDevice::<Impl, OFFSET>, BeginDraw::<Impl, OFFSET>, EndDraw::<Impl, OFFSET>)
    }
}
pub trait ISurfaceImageSourceNativeWithD2DImpl: Sized {
    fn SetDevice();
    fn BeginDraw();
    fn EndDraw();
    fn SuspendDraw();
    fn ResumeDraw();
}
impl ::windows::core::RuntimeName for ISurfaceImageSourceNativeWithD2D {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Xaml.ISurfaceImageSourceNativeWithD2D";
}
impl ISurfaceImageSourceNativeWithD2DVtbl {
    pub const fn new<Impl: ISurfaceImageSourceNativeWithD2DImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISurfaceImageSourceNativeWithD2DVtbl {
        unsafe extern "system" fn SetDevice<Impl: ISurfaceImageSourceNativeWithD2DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDevice(&*(&device as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginDraw<Impl: ISurfaceImageSourceNativeWithD2DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, offset: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BeginDraw(&*(&updaterect as *const <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType), &*(&iid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&updateobject), ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndDraw<Impl: ISurfaceImageSourceNativeWithD2DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EndDraw() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SuspendDraw<Impl: ISurfaceImageSourceNativeWithD2DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SuspendDraw() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResumeDraw<Impl: ISurfaceImageSourceNativeWithD2DImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResumeDraw() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISurfaceImageSourceNativeWithD2D>, base.5, SetDevice::<Impl, OFFSET>, BeginDraw::<Impl, OFFSET>, EndDraw::<Impl, OFFSET>, SuspendDraw::<Impl, OFFSET>, ResumeDraw::<Impl, OFFSET>)
    }
}
pub trait ISwapChainBackgroundPanelNativeImpl: Sized {
    fn SetSwapChain();
}
impl ::windows::core::RuntimeName for ISwapChainBackgroundPanelNative {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Xaml.ISwapChainBackgroundPanelNative";
}
impl ISwapChainBackgroundPanelNativeVtbl {
    pub const fn new<Impl: ISwapChainBackgroundPanelNativeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISwapChainBackgroundPanelNativeVtbl {
        unsafe extern "system" fn SetSwapChain<Impl: ISwapChainBackgroundPanelNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, swapchain: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSwapChain(&*(&swapchain as *const <super::super::super::Graphics::Dxgi::IDXGISwapChain as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Dxgi::IDXGISwapChain as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISwapChainBackgroundPanelNative>, base.5, SetSwapChain::<Impl, OFFSET>)
    }
}
pub trait ISwapChainPanelNativeImpl: Sized {
    fn SetSwapChain();
}
impl ::windows::core::RuntimeName for ISwapChainPanelNative {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Xaml.ISwapChainPanelNative";
}
impl ISwapChainPanelNativeVtbl {
    pub const fn new<Impl: ISwapChainPanelNativeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISwapChainPanelNativeVtbl {
        unsafe extern "system" fn SetSwapChain<Impl: ISwapChainPanelNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, swapchain: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSwapChain(&*(&swapchain as *const <super::super::super::Graphics::Dxgi::IDXGISwapChain as ::windows::core::Abi>::Abi as *const <super::super::super::Graphics::Dxgi::IDXGISwapChain as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISwapChainPanelNative>, base.5, SetSwapChain::<Impl, OFFSET>)
    }
}
pub trait ISwapChainPanelNative2Impl: Sized + ISwapChainPanelNativeImpl {
    fn SetSwapChainHandle();
}
impl ::windows::core::RuntimeName for ISwapChainPanelNative2 {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Xaml.ISwapChainPanelNative2";
}
impl ISwapChainPanelNative2Vtbl {
    pub const fn new<Impl: ISwapChainPanelNative2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISwapChainPanelNative2Vtbl {
        unsafe extern "system" fn SetSwapChainHandle<Impl: ISwapChainPanelNative2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, swapchainhandle: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSwapChainHandle(&*(&swapchainhandle as *const <super::super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISwapChainPanelNative2>, base.5, SetSwapChainHandle::<Impl, OFFSET>)
    }
}
pub trait ITrackerOwnerImpl: Sized {
    fn CreateTrackerHandle();
    fn DeleteTrackerHandle();
    fn SetTrackerValue();
    fn TryGetSafeTrackerValue();
}
impl ::windows::core::RuntimeName for ITrackerOwner {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Xaml.ITrackerOwner";
}
impl ITrackerOwnerVtbl {
    pub const fn new<Impl: ITrackerOwnerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ITrackerOwnerVtbl {
        unsafe extern "system" fn CreateTrackerHandle<Impl: ITrackerOwnerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, returnvalue: *mut *mut TrackerHandle__) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateTrackerHandle(::core::mem::transmute_copy(&returnvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTrackerHandle<Impl: ITrackerOwnerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handle: *const TrackerHandle__) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeleteTrackerHandle(&*(&handle as *const <TrackerHandle__ as ::windows::core::Abi>::Abi as *const <TrackerHandle__ as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTrackerValue<Impl: ITrackerOwnerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handle: *const TrackerHandle__, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTrackerValue(&*(&handle as *const <TrackerHandle__ as ::windows::core::Abi>::Abi as *const <TrackerHandle__ as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryGetSafeTrackerValue<Impl: ITrackerOwnerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handle: *const TrackerHandle__, returnvalue: *mut *mut ::core::ffi::c_void) -> u8 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).TryGetSafeTrackerValue(&*(&handle as *const <TrackerHandle__ as ::windows::core::Abi>::Abi as *const <TrackerHandle__ as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&returnvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ITrackerOwner>, base.5, CreateTrackerHandle::<Impl, OFFSET>, DeleteTrackerHandle::<Impl, OFFSET>, SetTrackerValue::<Impl, OFFSET>, TryGetSafeTrackerValue::<Impl, OFFSET>)
    }
}
pub trait IVirtualSurfaceImageSourceNativeImpl: Sized + ISurfaceImageSourceNativeImpl {
    fn Invalidate();
    fn GetUpdateRectCount();
    fn GetUpdateRects();
    fn GetVisibleBounds();
    fn RegisterForUpdatesNeeded();
    fn Resize();
}
impl ::windows::core::RuntimeName for IVirtualSurfaceImageSourceNative {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Xaml.IVirtualSurfaceImageSourceNative";
}
impl IVirtualSurfaceImageSourceNativeVtbl {
    pub const fn new<Impl: IVirtualSurfaceImageSourceNativeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVirtualSurfaceImageSourceNativeVtbl {
        unsafe extern "system" fn Invalidate<Impl: IVirtualSurfaceImageSourceNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updaterect: super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Invalidate(&*(&updaterect as *const <super::super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateRectCount<Impl: IVirtualSurfaceImageSourceNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUpdateRectCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateRects<Impl: IVirtualSurfaceImageSourceNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updates: *mut super::super::super::Foundation::RECT, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUpdateRects(::core::mem::transmute_copy(&updates), count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVisibleBounds<Impl: IVirtualSurfaceImageSourceNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bounds: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVisibleBounds(::core::mem::transmute_copy(&bounds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterForUpdatesNeeded<Impl: IVirtualSurfaceImageSourceNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterForUpdatesNeeded(&*(&callback as *const <IVirtualSurfaceUpdatesCallbackNative as ::windows::core::Abi>::Abi as *const <IVirtualSurfaceUpdatesCallbackNative as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Resize<Impl: IVirtualSurfaceImageSourceNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newwidth: i32, newheight: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Resize(newwidth, newheight) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVirtualSurfaceImageSourceNative>, base.5, Invalidate::<Impl, OFFSET>, GetUpdateRectCount::<Impl, OFFSET>, GetUpdateRects::<Impl, OFFSET>, GetVisibleBounds::<Impl, OFFSET>, RegisterForUpdatesNeeded::<Impl, OFFSET>, Resize::<Impl, OFFSET>)
    }
}
pub trait IVirtualSurfaceUpdatesCallbackNativeImpl: Sized {
    fn UpdatesNeeded();
}
impl ::windows::core::RuntimeName for IVirtualSurfaceUpdatesCallbackNative {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Xaml.IVirtualSurfaceUpdatesCallbackNative";
}
impl IVirtualSurfaceUpdatesCallbackNativeVtbl {
    pub const fn new<Impl: IVirtualSurfaceUpdatesCallbackNativeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVirtualSurfaceUpdatesCallbackNativeVtbl {
        unsafe extern "system" fn UpdatesNeeded<Impl: IVirtualSurfaceUpdatesCallbackNativeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UpdatesNeeded() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVirtualSurfaceUpdatesCallbackNative>, base.5, UpdatesNeeded::<Impl, OFFSET>)
    }
}
