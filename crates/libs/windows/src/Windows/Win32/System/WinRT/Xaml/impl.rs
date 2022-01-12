#[cfg(feature = "Win32_Foundation")]
pub trait IDesktopWindowXamlSourceNativeImpl: Sized {
    fn AttachToWindow();
    fn WindowHandle();
}
#[cfg(feature = "Win32_Foundation")]
impl IDesktopWindowXamlSourceNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDesktopWindowXamlSourceNativeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDesktopWindowXamlSourceNativeVtbl {
        unsafe extern "system" fn AttachToWindow<Impl: IDesktopWindowXamlSourceNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parentwnd: super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WindowHandle<Impl: IDesktopWindowXamlSourceNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AttachToWindow: AttachToWindow::<Impl, IMPL_OFFSET>,
            WindowHandle: WindowHandle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDesktopWindowXamlSourceNative as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDesktopWindowXamlSourceNative2Impl: Sized + IDesktopWindowXamlSourceNativeImpl {
    fn PreTranslateMessage();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDesktopWindowXamlSourceNative2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDesktopWindowXamlSourceNative2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDesktopWindowXamlSourceNative2Vtbl {
        unsafe extern "system" fn PreTranslateMessage<Impl: IDesktopWindowXamlSourceNative2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *const super::super::super::UI::WindowsAndMessaging::MSG, result: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDesktopWindowXamlSourceNativeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PreTranslateMessage: PreTranslateMessage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDesktopWindowXamlSourceNative2 as ::windows::core::Interface>::IID
    }
}
pub trait IFindReferenceTargetsCallbackImpl: Sized {
    fn FoundTrackerTarget();
}
impl IFindReferenceTargetsCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFindReferenceTargetsCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFindReferenceTargetsCallbackVtbl {
        unsafe extern "system" fn FoundTrackerTarget<Impl: IFindReferenceTargetsCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), FoundTrackerTarget: FoundTrackerTarget::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFindReferenceTargetsCallback as ::windows::core::Interface>::IID
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
impl IReferenceTrackerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReferenceTrackerVtbl {
        unsafe extern "system" fn ConnectFromTrackerSource<Impl: IReferenceTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisconnectFromTrackerSource<Impl: IReferenceTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindTrackerTargets<Impl: IReferenceTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReferenceTrackerManager<Impl: IReferenceTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddRefFromTrackerSource<Impl: IReferenceTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseFromTrackerSource<Impl: IReferenceTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PegFromTrackerSource<Impl: IReferenceTrackerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ConnectFromTrackerSource: ConnectFromTrackerSource::<Impl, IMPL_OFFSET>,
            DisconnectFromTrackerSource: DisconnectFromTrackerSource::<Impl, IMPL_OFFSET>,
            FindTrackerTargets: FindTrackerTargets::<Impl, IMPL_OFFSET>,
            GetReferenceTrackerManager: GetReferenceTrackerManager::<Impl, IMPL_OFFSET>,
            AddRefFromTrackerSource: AddRefFromTrackerSource::<Impl, IMPL_OFFSET>,
            ReleaseFromTrackerSource: ReleaseFromTrackerSource::<Impl, IMPL_OFFSET>,
            PegFromTrackerSource: PegFromTrackerSource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceTracker as ::windows::core::Interface>::IID
    }
}
pub trait IReferenceTrackerExtensionImpl: Sized {}
impl IReferenceTrackerExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerExtensionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReferenceTrackerExtensionVtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceTrackerExtension as ::windows::core::Interface>::IID
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
impl IReferenceTrackerHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerHostImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReferenceTrackerHostVtbl {
        unsafe extern "system" fn DisconnectUnusedReferenceSources<Impl: IReferenceTrackerHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: XAML_REFERENCETRACKER_DISCONNECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseDisconnectedReferenceSources<Impl: IReferenceTrackerHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyEndOfReferenceTrackingOnThread<Impl: IReferenceTrackerHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTrackerTarget<Impl: IReferenceTrackerHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, unknown: *mut ::core::ffi::c_void, newreference: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddMemoryPressure<Impl: IReferenceTrackerHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bytesallocated: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveMemoryPressure<Impl: IReferenceTrackerHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bytesallocated: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            DisconnectUnusedReferenceSources: DisconnectUnusedReferenceSources::<Impl, IMPL_OFFSET>,
            ReleaseDisconnectedReferenceSources: ReleaseDisconnectedReferenceSources::<Impl, IMPL_OFFSET>,
            NotifyEndOfReferenceTrackingOnThread: NotifyEndOfReferenceTrackingOnThread::<Impl, IMPL_OFFSET>,
            GetTrackerTarget: GetTrackerTarget::<Impl, IMPL_OFFSET>,
            AddMemoryPressure: AddMemoryPressure::<Impl, IMPL_OFFSET>,
            RemoveMemoryPressure: RemoveMemoryPressure::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceTrackerHost as ::windows::core::Interface>::IID
    }
}
pub trait IReferenceTrackerManagerImpl: Sized {
    fn ReferenceTrackingStarted();
    fn FindTrackerTargetsCompleted();
    fn ReferenceTrackingCompleted();
    fn SetReferenceTrackerHost();
}
impl IReferenceTrackerManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReferenceTrackerManagerVtbl {
        unsafe extern "system" fn ReferenceTrackingStarted<Impl: IReferenceTrackerManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindTrackerTargetsCompleted<Impl: IReferenceTrackerManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, findfailed: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReferenceTrackingCompleted<Impl: IReferenceTrackerManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReferenceTrackerHost<Impl: IReferenceTrackerManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ReferenceTrackingStarted: ReferenceTrackingStarted::<Impl, IMPL_OFFSET>,
            FindTrackerTargetsCompleted: FindTrackerTargetsCompleted::<Impl, IMPL_OFFSET>,
            ReferenceTrackingCompleted: ReferenceTrackingCompleted::<Impl, IMPL_OFFSET>,
            SetReferenceTrackerHost: SetReferenceTrackerHost::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceTrackerManager as ::windows::core::Interface>::IID
    }
}
pub trait IReferenceTrackerTargetImpl: Sized {
    fn AddRefFromReferenceTracker();
    fn ReleaseFromReferenceTracker();
    fn Peg();
    fn Unpeg();
}
impl IReferenceTrackerTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReferenceTrackerTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReferenceTrackerTargetVtbl {
        unsafe extern "system" fn AddRefFromReferenceTracker<Impl: IReferenceTrackerTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseFromReferenceTracker<Impl: IReferenceTrackerTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Peg<Impl: IReferenceTrackerTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unpeg<Impl: IReferenceTrackerTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddRefFromReferenceTracker: AddRefFromReferenceTracker::<Impl, IMPL_OFFSET>,
            ReleaseFromReferenceTracker: ReleaseFromReferenceTracker::<Impl, IMPL_OFFSET>,
            Peg: Peg::<Impl, IMPL_OFFSET>,
            Unpeg: Unpeg::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReferenceTrackerTarget as ::windows::core::Interface>::IID
    }
}
pub trait ISurfaceImageSourceManagerNativeImpl: Sized {
    fn FlushAllSurfacesWithDevice();
}
impl ISurfaceImageSourceManagerNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSourceManagerNativeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISurfaceImageSourceManagerNativeVtbl {
        unsafe extern "system" fn FlushAllSurfacesWithDevice<Impl: ISurfaceImageSourceManagerNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), FlushAllSurfacesWithDevice: FlushAllSurfacesWithDevice::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISurfaceImageSourceManagerNative as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
pub trait ISurfaceImageSourceNativeImpl: Sized {
    fn SetDevice();
    fn BeginDraw();
    fn EndDraw();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
impl ISurfaceImageSourceNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSourceNativeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISurfaceImageSourceNativeVtbl {
        unsafe extern "system" fn SetDevice<Impl: ISurfaceImageSourceNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginDraw<Impl: ISurfaceImageSourceNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updaterect: super::super::super::Foundation::RECT, surface: *mut ::windows::core::RawPtr, offset: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndDraw<Impl: ISurfaceImageSourceNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetDevice: SetDevice::<Impl, IMPL_OFFSET>,
            BeginDraw: BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw: EndDraw::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISurfaceImageSourceNative as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISurfaceImageSourceNativeWithD2DImpl: Sized {
    fn SetDevice();
    fn BeginDraw();
    fn EndDraw();
    fn SuspendDraw();
    fn ResumeDraw();
}
#[cfg(feature = "Win32_Foundation")]
impl ISurfaceImageSourceNativeWithD2DVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISurfaceImageSourceNativeWithD2DImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISurfaceImageSourceNativeWithD2DVtbl {
        unsafe extern "system" fn SetDevice<Impl: ISurfaceImageSourceNativeWithD2DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginDraw<Impl: ISurfaceImageSourceNativeWithD2DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, offset: *mut super::super::super::Foundation::POINT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndDraw<Impl: ISurfaceImageSourceNativeWithD2DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SuspendDraw<Impl: ISurfaceImageSourceNativeWithD2DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResumeDraw<Impl: ISurfaceImageSourceNativeWithD2DImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetDevice: SetDevice::<Impl, IMPL_OFFSET>,
            BeginDraw: BeginDraw::<Impl, IMPL_OFFSET>,
            EndDraw: EndDraw::<Impl, IMPL_OFFSET>,
            SuspendDraw: SuspendDraw::<Impl, IMPL_OFFSET>,
            ResumeDraw: ResumeDraw::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISurfaceImageSourceNativeWithD2D as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait ISwapChainBackgroundPanelNativeImpl: Sized {
    fn SetSwapChain();
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ISwapChainBackgroundPanelNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISwapChainBackgroundPanelNativeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISwapChainBackgroundPanelNativeVtbl {
        unsafe extern "system" fn SetSwapChain<Impl: ISwapChainBackgroundPanelNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, swapchain: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetSwapChain: SetSwapChain::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISwapChainBackgroundPanelNative as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub trait ISwapChainPanelNativeImpl: Sized {
    fn SetSwapChain();
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ISwapChainPanelNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISwapChainPanelNativeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISwapChainPanelNativeVtbl {
        unsafe extern "system" fn SetSwapChain<Impl: ISwapChainPanelNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, swapchain: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetSwapChain: SetSwapChain::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISwapChainPanelNative as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
pub trait ISwapChainPanelNative2Impl: Sized + ISwapChainPanelNativeImpl {
    fn SetSwapChainHandle();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
impl ISwapChainPanelNative2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISwapChainPanelNative2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISwapChainPanelNative2Vtbl {
        unsafe extern "system" fn SetSwapChainHandle<Impl: ISwapChainPanelNative2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, swapchainhandle: super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ISwapChainPanelNativeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetSwapChainHandle: SetSwapChainHandle::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISwapChainPanelNative2 as ::windows::core::Interface>::IID
    }
}
pub trait ITrackerOwnerImpl: Sized {
    fn CreateTrackerHandle();
    fn DeleteTrackerHandle();
    fn SetTrackerValue();
    fn TryGetSafeTrackerValue();
}
impl ITrackerOwnerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITrackerOwnerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITrackerOwnerVtbl {
        unsafe extern "system" fn CreateTrackerHandle<Impl: ITrackerOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, returnvalue: *mut *mut TrackerHandle__) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteTrackerHandle<Impl: ITrackerOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: *const TrackerHandle__) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTrackerValue<Impl: ITrackerOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: *const TrackerHandle__, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TryGetSafeTrackerValue<Impl: ITrackerOwnerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: *const TrackerHandle__, returnvalue: *mut *mut ::core::ffi::c_void) -> u8 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateTrackerHandle: CreateTrackerHandle::<Impl, IMPL_OFFSET>,
            DeleteTrackerHandle: DeleteTrackerHandle::<Impl, IMPL_OFFSET>,
            SetTrackerValue: SetTrackerValue::<Impl, IMPL_OFFSET>,
            TryGetSafeTrackerValue: TryGetSafeTrackerValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITrackerOwner as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
pub trait IVirtualSurfaceImageSourceNativeImpl: Sized + ISurfaceImageSourceNativeImpl {
    fn Invalidate();
    fn GetUpdateRectCount();
    fn GetUpdateRects();
    fn GetVisibleBounds();
    fn RegisterForUpdatesNeeded();
    fn Resize();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
impl IVirtualSurfaceImageSourceNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualSurfaceImageSourceNativeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVirtualSurfaceImageSourceNativeVtbl {
        unsafe extern "system" fn Invalidate<Impl: IVirtualSurfaceImageSourceNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updaterect: super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUpdateRectCount<Impl: IVirtualSurfaceImageSourceNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUpdateRects<Impl: IVirtualSurfaceImageSourceNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updates: *mut super::super::super::Foundation::RECT, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVisibleBounds<Impl: IVirtualSurfaceImageSourceNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bounds: *mut super::super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterForUpdatesNeeded<Impl: IVirtualSurfaceImageSourceNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Resize<Impl: IVirtualSurfaceImageSourceNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newwidth: i32, newheight: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ISurfaceImageSourceNativeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Invalidate: Invalidate::<Impl, IMPL_OFFSET>,
            GetUpdateRectCount: GetUpdateRectCount::<Impl, IMPL_OFFSET>,
            GetUpdateRects: GetUpdateRects::<Impl, IMPL_OFFSET>,
            GetVisibleBounds: GetVisibleBounds::<Impl, IMPL_OFFSET>,
            RegisterForUpdatesNeeded: RegisterForUpdatesNeeded::<Impl, IMPL_OFFSET>,
            Resize: Resize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVirtualSurfaceImageSourceNative as ::windows::core::Interface>::IID
    }
}
pub trait IVirtualSurfaceUpdatesCallbackNativeImpl: Sized {
    fn UpdatesNeeded();
}
impl IVirtualSurfaceUpdatesCallbackNativeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVirtualSurfaceUpdatesCallbackNativeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVirtualSurfaceUpdatesCallbackNativeVtbl {
        unsafe extern "system" fn UpdatesNeeded<Impl: IVirtualSurfaceUpdatesCallbackNativeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), UpdatesNeeded: UpdatesNeeded::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVirtualSurfaceUpdatesCallbackNative as ::windows::core::Interface>::IID
    }
}
