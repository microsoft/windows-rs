#[cfg(feature = "Win32_Foundation")]
pub trait IDesktopWindowXamlSourceNative_Impl: Sized {
    fn AttachToWindow(&self, parentwnd: super::super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn WindowHandle(&self) -> ::windows::core::Result<super::super::super::Foundation::HWND>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDesktopWindowXamlSourceNative_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDesktopWindowXamlSourceNative_Impl, const OFFSET: isize>() -> IDesktopWindowXamlSourceNative_Vtbl {
        unsafe extern "system" fn AttachToWindow<Identity: ::windows::core::IUnknownImpl, Impl: IDesktopWindowXamlSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AttachToWindow(::core::mem::transmute_copy(&parentwnd)).into()
        }
        unsafe extern "system" fn WindowHandle<Identity: ::windows::core::IUnknownImpl, Impl: IDesktopWindowXamlSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WindowHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *hwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AttachToWindow: AttachToWindow::<Identity, Impl, OFFSET>,
            WindowHandle: WindowHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDesktopWindowXamlSourceNative as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDesktopWindowXamlSourceNative2_Impl: Sized + IDesktopWindowXamlSourceNative_Impl {
    fn PreTranslateMessage(&self, message: *const super::super::super::UI::WindowsAndMessaging::MSG, result: *mut super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDesktopWindowXamlSourceNative2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDesktopWindowXamlSourceNative2_Impl, const OFFSET: isize>() -> IDesktopWindowXamlSourceNative2_Vtbl {
        unsafe extern "system" fn PreTranslateMessage<Identity: ::windows::core::IUnknownImpl, Impl: IDesktopWindowXamlSourceNative2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *const super::super::super::UI::WindowsAndMessaging::MSG, result: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PreTranslateMessage(::core::mem::transmute_copy(&message), ::core::mem::transmute_copy(&result)).into()
        }
        Self { base: IDesktopWindowXamlSourceNative_Vtbl::new::<Identity, Impl, OFFSET>(), PreTranslateMessage: PreTranslateMessage::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDesktopWindowXamlSourceNative2 as ::windows::core::Interface>::IID || iid == &<IDesktopWindowXamlSourceNative as ::windows::core::Interface>::IID
    }
}
pub trait IFindReferenceTargetsCallback_Impl: Sized {
    fn FoundTrackerTarget(&self, target: &::core::option::Option<IReferenceTrackerTarget>) -> ::windows::core::Result<()>;
}
impl IFindReferenceTargetsCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFindReferenceTargetsCallback_Impl, const OFFSET: isize>() -> IFindReferenceTargetsCallback_Vtbl {
        unsafe extern "system" fn FoundTrackerTarget<Identity: ::windows::core::IUnknownImpl, Impl: IFindReferenceTargetsCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FoundTrackerTarget(::core::mem::transmute(&target)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), FoundTrackerTarget: FoundTrackerTarget::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFindReferenceTargetsCallback as ::windows::core::Interface>::IID
    }
}
pub trait IReferenceTracker_Impl: Sized {
    fn ConnectFromTrackerSource(&self) -> ::windows::core::Result<()>;
    fn DisconnectFromTrackerSource(&self) -> ::windows::core::Result<()>;
    fn FindTrackerTargets(&self, callback: &::core::option::Option<IFindReferenceTargetsCallback>) -> ::windows::core::Result<()>;
    fn GetReferenceTrackerManager(&self) -> ::windows::core::Result<IReferenceTrackerManager>;
    fn AddRefFromTrackerSource(&self) -> ::windows::core::Result<()>;
    fn ReleaseFromTrackerSource(&self) -> ::windows::core::Result<()>;
    fn PegFromTrackerSource(&self) -> ::windows::core::Result<()>;
}
impl IReferenceTracker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTracker_Impl, const OFFSET: isize>() -> IReferenceTracker_Vtbl {
        unsafe extern "system" fn ConnectFromTrackerSource<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ConnectFromTrackerSource().into()
        }
        unsafe extern "system" fn DisconnectFromTrackerSource<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisconnectFromTrackerSource().into()
        }
        unsafe extern "system" fn FindTrackerTargets<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindTrackerTargets(::core::mem::transmute(&callback)).into()
        }
        unsafe extern "system" fn GetReferenceTrackerManager<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetReferenceTrackerManager() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRefFromTrackerSource<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRefFromTrackerSource().into()
        }
        unsafe extern "system" fn ReleaseFromTrackerSource<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseFromTrackerSource().into()
        }
        unsafe extern "system" fn PegFromTrackerSource<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTracker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PegFromTrackerSource().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ConnectFromTrackerSource: ConnectFromTrackerSource::<Identity, Impl, OFFSET>,
            DisconnectFromTrackerSource: DisconnectFromTrackerSource::<Identity, Impl, OFFSET>,
            FindTrackerTargets: FindTrackerTargets::<Identity, Impl, OFFSET>,
            GetReferenceTrackerManager: GetReferenceTrackerManager::<Identity, Impl, OFFSET>,
            AddRefFromTrackerSource: AddRefFromTrackerSource::<Identity, Impl, OFFSET>,
            ReleaseFromTrackerSource: ReleaseFromTrackerSource::<Identity, Impl, OFFSET>,
            PegFromTrackerSource: PegFromTrackerSource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceTracker as ::windows::core::Interface>::IID
    }
}
pub trait IReferenceTrackerExtension_Impl: Sized {}
impl IReferenceTrackerExtension_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerExtension_Impl, const OFFSET: isize>() -> IReferenceTrackerExtension_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceTrackerExtension as ::windows::core::Interface>::IID
    }
}
pub trait IReferenceTrackerHost_Impl: Sized {
    fn DisconnectUnusedReferenceSources(&self, options: XAML_REFERENCETRACKER_DISCONNECT) -> ::windows::core::Result<()>;
    fn ReleaseDisconnectedReferenceSources(&self) -> ::windows::core::Result<()>;
    fn NotifyEndOfReferenceTrackingOnThread(&self) -> ::windows::core::Result<()>;
    fn GetTrackerTarget(&self, unknown: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<IReferenceTrackerTarget>;
    fn AddMemoryPressure(&self, bytesallocated: u64) -> ::windows::core::Result<()>;
    fn RemoveMemoryPressure(&self, bytesallocated: u64) -> ::windows::core::Result<()>;
}
impl IReferenceTrackerHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerHost_Impl, const OFFSET: isize>() -> IReferenceTrackerHost_Vtbl {
        unsafe extern "system" fn DisconnectUnusedReferenceSources<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: XAML_REFERENCETRACKER_DISCONNECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisconnectUnusedReferenceSources(::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn ReleaseDisconnectedReferenceSources<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseDisconnectedReferenceSources().into()
        }
        unsafe extern "system" fn NotifyEndOfReferenceTrackingOnThread<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyEndOfReferenceTrackingOnThread().into()
        }
        unsafe extern "system" fn GetTrackerTarget<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unknown: *mut ::core::ffi::c_void, newreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTrackerTarget(::core::mem::transmute(&unknown)) {
                ::core::result::Result::Ok(ok__) => {
                    *newreference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMemoryPressure<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bytesallocated: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddMemoryPressure(::core::mem::transmute_copy(&bytesallocated)).into()
        }
        unsafe extern "system" fn RemoveMemoryPressure<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bytesallocated: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveMemoryPressure(::core::mem::transmute_copy(&bytesallocated)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            DisconnectUnusedReferenceSources: DisconnectUnusedReferenceSources::<Identity, Impl, OFFSET>,
            ReleaseDisconnectedReferenceSources: ReleaseDisconnectedReferenceSources::<Identity, Impl, OFFSET>,
            NotifyEndOfReferenceTrackingOnThread: NotifyEndOfReferenceTrackingOnThread::<Identity, Impl, OFFSET>,
            GetTrackerTarget: GetTrackerTarget::<Identity, Impl, OFFSET>,
            AddMemoryPressure: AddMemoryPressure::<Identity, Impl, OFFSET>,
            RemoveMemoryPressure: RemoveMemoryPressure::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceTrackerHost as ::windows::core::Interface>::IID
    }
}
pub trait IReferenceTrackerManager_Impl: Sized {
    fn ReferenceTrackingStarted(&self) -> ::windows::core::Result<()>;
    fn FindTrackerTargetsCompleted(&self, findfailed: u8) -> ::windows::core::Result<()>;
    fn ReferenceTrackingCompleted(&self) -> ::windows::core::Result<()>;
    fn SetReferenceTrackerHost(&self, value: &::core::option::Option<IReferenceTrackerHost>) -> ::windows::core::Result<()>;
}
impl IReferenceTrackerManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerManager_Impl, const OFFSET: isize>() -> IReferenceTrackerManager_Vtbl {
        unsafe extern "system" fn ReferenceTrackingStarted<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReferenceTrackingStarted().into()
        }
        unsafe extern "system" fn FindTrackerTargetsCompleted<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findfailed: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindTrackerTargetsCompleted(::core::mem::transmute_copy(&findfailed)).into()
        }
        unsafe extern "system" fn ReferenceTrackingCompleted<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReferenceTrackingCompleted().into()
        }
        unsafe extern "system" fn SetReferenceTrackerHost<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetReferenceTrackerHost(::core::mem::transmute(&value)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ReferenceTrackingStarted: ReferenceTrackingStarted::<Identity, Impl, OFFSET>,
            FindTrackerTargetsCompleted: FindTrackerTargetsCompleted::<Identity, Impl, OFFSET>,
            ReferenceTrackingCompleted: ReferenceTrackingCompleted::<Identity, Impl, OFFSET>,
            SetReferenceTrackerHost: SetReferenceTrackerHost::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceTrackerManager as ::windows::core::Interface>::IID
    }
}
pub trait IReferenceTrackerTarget_Impl: Sized {
    fn AddRefFromReferenceTracker(&self) -> u32;
    fn ReleaseFromReferenceTracker(&self) -> u32;
    fn Peg(&self) -> ::windows::core::Result<()>;
    fn Unpeg(&self) -> ::windows::core::Result<()>;
}
impl IReferenceTrackerTarget_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerTarget_Impl, const OFFSET: isize>() -> IReferenceTrackerTarget_Vtbl {
        unsafe extern "system" fn AddRefFromReferenceTracker<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRefFromReferenceTracker()
        }
        unsafe extern "system" fn ReleaseFromReferenceTracker<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseFromReferenceTracker()
        }
        unsafe extern "system" fn Peg<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Peg().into()
        }
        unsafe extern "system" fn Unpeg<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerTarget_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unpeg().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddRefFromReferenceTracker: AddRefFromReferenceTracker::<Identity, Impl, OFFSET>,
            ReleaseFromReferenceTracker: ReleaseFromReferenceTracker::<Identity, Impl, OFFSET>,
            Peg: Peg::<Identity, Impl, OFFSET>,
            Unpeg: Unpeg::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceTrackerTarget as ::windows::core::Interface>::IID
    }
}
pub trait ISurfaceImageSourceManagerNative_Impl: Sized {
    fn FlushAllSurfacesWithDevice(&self, device: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ISurfaceImageSourceManagerNative_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSourceManagerNative_Impl, const OFFSET: isize>() -> ISurfaceImageSourceManagerNative_Vtbl {
        unsafe extern "system" fn FlushAllSurfacesWithDevice<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSourceManagerNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FlushAllSurfacesWithDevice(::core::mem::transmute(&device)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), FlushAllSurfacesWithDevice: FlushAllSurfacesWithDevice::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISurfaceImageSourceManagerNative as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
pub trait ISurfaceImageSourceNative_Impl: Sized {
    fn SetDevice(&self, device: &::core::option::Option<super::super::super::Graphics::Dxgi::IDXGIDevice>) -> ::windows::core::Result<()>;
    fn BeginDraw(&self, updaterect: &super::super::super::Foundation::RECT, surface: *mut ::core::option::Option<super::super::super::Graphics::Dxgi::IDXGISurface>, offset: *mut super::super::super::Foundation::POINT) -> ::windows::core::Result<()>;
    fn EndDraw(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
impl ISurfaceImageSourceNative_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSourceNative_Impl, const OFFSET: isize>() -> ISurfaceImageSourceNative_Vtbl {
        unsafe extern "system" fn SetDevice<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDevice(::core::mem::transmute(&device)).into()
        }
        unsafe extern "system" fn BeginDraw<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updaterect: super::super::super::Foundation::RECT, surface: *mut ::windows::core::RawPtr, offset: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginDraw(::core::mem::transmute(&updaterect), ::core::mem::transmute_copy(&surface), ::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn EndDraw<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndDraw().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetDevice: SetDevice::<Identity, Impl, OFFSET>,
            BeginDraw: BeginDraw::<Identity, Impl, OFFSET>,
            EndDraw: EndDraw::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISurfaceImageSourceNative as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISurfaceImageSourceNativeWithD2D_Impl: Sized {
    fn SetDevice(&self, device: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn BeginDraw(&self, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, offset: *mut super::super::super::Foundation::POINT) -> ::windows::core::Result<()>;
    fn EndDraw(&self) -> ::windows::core::Result<()>;
    fn SuspendDraw(&self) -> ::windows::core::Result<()>;
    fn ResumeDraw(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISurfaceImageSourceNativeWithD2D_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSourceNativeWithD2D_Impl, const OFFSET: isize>() -> ISurfaceImageSourceNativeWithD2D_Vtbl {
        unsafe extern "system" fn SetDevice<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSourceNativeWithD2D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDevice(::core::mem::transmute(&device)).into()
        }
        unsafe extern "system" fn BeginDraw<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSourceNativeWithD2D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, offset: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginDraw(::core::mem::transmute_copy(&updaterect), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&updateobject), ::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn EndDraw<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSourceNativeWithD2D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndDraw().into()
        }
        unsafe extern "system" fn SuspendDraw<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSourceNativeWithD2D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SuspendDraw().into()
        }
        unsafe extern "system" fn ResumeDraw<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSourceNativeWithD2D_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResumeDraw().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetDevice: SetDevice::<Identity, Impl, OFFSET>,
            BeginDraw: BeginDraw::<Identity, Impl, OFFSET>,
            EndDraw: EndDraw::<Identity, Impl, OFFSET>,
            SuspendDraw: SuspendDraw::<Identity, Impl, OFFSET>,
            ResumeDraw: ResumeDraw::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISurfaceImageSourceNativeWithD2D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait ISwapChainBackgroundPanelNative_Impl: Sized {
    fn SetSwapChain(&self, swapchain: &::core::option::Option<super::super::super::Graphics::Dxgi::IDXGISwapChain>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ISwapChainBackgroundPanelNative_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISwapChainBackgroundPanelNative_Impl, const OFFSET: isize>() -> ISwapChainBackgroundPanelNative_Vtbl {
        unsafe extern "system" fn SetSwapChain<Identity: ::windows::core::IUnknownImpl, Impl: ISwapChainBackgroundPanelNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, swapchain: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSwapChain(::core::mem::transmute(&swapchain)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetSwapChain: SetSwapChain::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISwapChainBackgroundPanelNative as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait ISwapChainPanelNative_Impl: Sized {
    fn SetSwapChain(&self, swapchain: &::core::option::Option<super::super::super::Graphics::Dxgi::IDXGISwapChain>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ISwapChainPanelNative_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISwapChainPanelNative_Impl, const OFFSET: isize>() -> ISwapChainPanelNative_Vtbl {
        unsafe extern "system" fn SetSwapChain<Identity: ::windows::core::IUnknownImpl, Impl: ISwapChainPanelNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, swapchain: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSwapChain(::core::mem::transmute(&swapchain)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetSwapChain: SetSwapChain::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISwapChainPanelNative as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
pub trait ISwapChainPanelNative2_Impl: Sized + ISwapChainPanelNative_Impl {
    fn SetSwapChainHandle(&self, swapchainhandle: super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
impl ISwapChainPanelNative2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISwapChainPanelNative2_Impl, const OFFSET: isize>() -> ISwapChainPanelNative2_Vtbl {
        unsafe extern "system" fn SetSwapChainHandle<Identity: ::windows::core::IUnknownImpl, Impl: ISwapChainPanelNative2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, swapchainhandle: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSwapChainHandle(::core::mem::transmute_copy(&swapchainhandle)).into()
        }
        Self { base: ISwapChainPanelNative_Vtbl::new::<Identity, Impl, OFFSET>(), SetSwapChainHandle: SetSwapChainHandle::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISwapChainPanelNative2 as ::windows::core::Interface>::IID || iid == &<ISwapChainPanelNative as ::windows::core::Interface>::IID
    }
}
pub trait ITrackerOwner_Impl: Sized {
    fn CreateTrackerHandle(&self) -> ::windows::core::Result<*mut TrackerHandle__>;
    fn DeleteTrackerHandle(&self, handle: *const TrackerHandle__) -> ::windows::core::Result<()>;
    fn SetTrackerValue(&self, handle: *const TrackerHandle__, value: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn TryGetSafeTrackerValue(&self, handle: *const TrackerHandle__, returnvalue: *mut ::core::option::Option<::windows::core::IUnknown>) -> u8;
}
impl ITrackerOwner_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITrackerOwner_Impl, const OFFSET: isize>() -> ITrackerOwner_Vtbl {
        unsafe extern "system" fn CreateTrackerHandle<Identity: ::windows::core::IUnknownImpl, Impl: ITrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, returnvalue: *mut *mut TrackerHandle__) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTrackerHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *returnvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTrackerHandle<Identity: ::windows::core::IUnknownImpl, Impl: ITrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: *const TrackerHandle__) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteTrackerHandle(::core::mem::transmute_copy(&handle)).into()
        }
        unsafe extern "system" fn SetTrackerValue<Identity: ::windows::core::IUnknownImpl, Impl: ITrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: *const TrackerHandle__, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTrackerValue(::core::mem::transmute_copy(&handle), ::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn TryGetSafeTrackerValue<Identity: ::windows::core::IUnknownImpl, Impl: ITrackerOwner_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: *const TrackerHandle__, returnvalue: *mut *mut ::core::ffi::c_void) -> u8 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TryGetSafeTrackerValue(::core::mem::transmute_copy(&handle), ::core::mem::transmute_copy(&returnvalue))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateTrackerHandle: CreateTrackerHandle::<Identity, Impl, OFFSET>,
            DeleteTrackerHandle: DeleteTrackerHandle::<Identity, Impl, OFFSET>,
            SetTrackerValue: SetTrackerValue::<Identity, Impl, OFFSET>,
            TryGetSafeTrackerValue: TryGetSafeTrackerValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITrackerOwner as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
pub trait IVirtualSurfaceImageSourceNative_Impl: Sized + ISurfaceImageSourceNative_Impl {
    fn Invalidate(&self, updaterect: &super::super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn GetUpdateRectCount(&self) -> ::windows::core::Result<u32>;
    fn GetUpdateRects(&self, updates: *mut super::super::super::Foundation::RECT, count: u32) -> ::windows::core::Result<()>;
    fn GetVisibleBounds(&self) -> ::windows::core::Result<super::super::super::Foundation::RECT>;
    fn RegisterForUpdatesNeeded(&self, callback: &::core::option::Option<IVirtualSurfaceUpdatesCallbackNative>) -> ::windows::core::Result<()>;
    fn Resize(&self, newwidth: i32, newheight: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
impl IVirtualSurfaceImageSourceNative_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>() -> IVirtualSurfaceImageSourceNative_Vtbl {
        unsafe extern "system" fn Invalidate<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updaterect: super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Invalidate(::core::mem::transmute(&updaterect)).into()
        }
        unsafe extern "system" fn GetUpdateRectCount<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUpdateRectCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateRects<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updates: *mut super::super::super::Foundation::RECT, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetUpdateRects(::core::mem::transmute_copy(&updates), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetVisibleBounds<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bounds: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVisibleBounds() {
                ::core::result::Result::Ok(ok__) => {
                    *bounds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterForUpdatesNeeded<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterForUpdatesNeeded(::core::mem::transmute(&callback)).into()
        }
        unsafe extern "system" fn Resize<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualSurfaceImageSourceNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newwidth: i32, newheight: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resize(::core::mem::transmute_copy(&newwidth), ::core::mem::transmute_copy(&newheight)).into()
        }
        Self {
            base: ISurfaceImageSourceNative_Vtbl::new::<Identity, Impl, OFFSET>(),
            Invalidate: Invalidate::<Identity, Impl, OFFSET>,
            GetUpdateRectCount: GetUpdateRectCount::<Identity, Impl, OFFSET>,
            GetUpdateRects: GetUpdateRects::<Identity, Impl, OFFSET>,
            GetVisibleBounds: GetVisibleBounds::<Identity, Impl, OFFSET>,
            RegisterForUpdatesNeeded: RegisterForUpdatesNeeded::<Identity, Impl, OFFSET>,
            Resize: Resize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVirtualSurfaceImageSourceNative as ::windows::core::Interface>::IID || iid == &<ISurfaceImageSourceNative as ::windows::core::Interface>::IID
    }
}
pub trait IVirtualSurfaceUpdatesCallbackNative_Impl: Sized {
    fn UpdatesNeeded(&self) -> ::windows::core::Result<()>;
}
impl IVirtualSurfaceUpdatesCallbackNative_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualSurfaceUpdatesCallbackNative_Impl, const OFFSET: isize>() -> IVirtualSurfaceUpdatesCallbackNative_Vtbl {
        unsafe extern "system" fn UpdatesNeeded<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualSurfaceUpdatesCallbackNative_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdatesNeeded().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), UpdatesNeeded: UpdatesNeeded::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVirtualSurfaceUpdatesCallbackNative as ::windows::core::Interface>::IID
    }
}
