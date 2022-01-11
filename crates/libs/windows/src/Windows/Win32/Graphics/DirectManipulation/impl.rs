pub trait IDirectManipulationAutoScrollBehaviorImpl: Sized {
    fn SetConfiguration();
}
impl IDirectManipulationAutoScrollBehaviorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationAutoScrollBehaviorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationAutoScrollBehaviorVtbl {
        unsafe extern "system" fn SetConfiguration<Impl: IDirectManipulationAutoScrollBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetConfiguration::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationAutoScrollBehavior as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationCompositorImpl: Sized {
    fn AddContent();
    fn RemoveContent();
    fn SetUpdateManager();
    fn Flush();
}
impl IDirectManipulationCompositorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationCompositorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationCompositorVtbl {
        unsafe extern "system" fn AddContent<Impl: IDirectManipulationCompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveContent<Impl: IDirectManipulationCompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUpdateManager<Impl: IDirectManipulationCompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatemanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flush<Impl: IDirectManipulationCompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddContent::<Impl, IMPL_OFFSET>, RemoveContent::<Impl, IMPL_OFFSET>, SetUpdateManager::<Impl, IMPL_OFFSET>, Flush::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationCompositor as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationCompositor2Impl: Sized + IDirectManipulationCompositorImpl {
    fn AddContentWithCrossProcessChaining();
}
impl IDirectManipulationCompositor2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationCompositor2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationCompositor2Vtbl {
        unsafe extern "system" fn AddContentWithCrossProcessChaining<Impl: IDirectManipulationCompositor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddContent::<Impl, IMPL_OFFSET>, RemoveContent::<Impl, IMPL_OFFSET>, SetUpdateManager::<Impl, IMPL_OFFSET>, Flush::<Impl, IMPL_OFFSET>, AddContentWithCrossProcessChaining::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationCompositor2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectManipulationContentImpl: Sized {
    fn GetContentRect();
    fn SetContentRect();
    fn GetViewport();
    fn GetTag();
    fn SetTag();
    fn GetOutputTransform();
    fn GetContentTransform();
    fn SyncContentTransform();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectManipulationContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationContentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationContentVtbl {
        unsafe extern "system" fn GetContentRect<Impl: IDirectManipulationContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentsize: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContentRect<Impl: IDirectManipulationContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentsize: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetViewport<Impl: IDirectManipulationContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTag<Impl: IDirectManipulationContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTag<Impl: IDirectManipulationContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputTransform<Impl: IDirectManipulationContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContentTransform<Impl: IDirectManipulationContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SyncContentTransform<Impl: IDirectManipulationContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetContentRect::<Impl, IMPL_OFFSET>, SetContentRect::<Impl, IMPL_OFFSET>, GetViewport::<Impl, IMPL_OFFSET>, GetTag::<Impl, IMPL_OFFSET>, SetTag::<Impl, IMPL_OFFSET>, GetOutputTransform::<Impl, IMPL_OFFSET>, GetContentTransform::<Impl, IMPL_OFFSET>, SyncContentTransform::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationContent as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationDeferContactServiceImpl: Sized {
    fn DeferContact();
    fn CancelContact();
    fn CancelDeferral();
}
impl IDirectManipulationDeferContactServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationDeferContactServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationDeferContactServiceVtbl {
        unsafe extern "system" fn DeferContact<Impl: IDirectManipulationDeferContactServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelContact<Impl: IDirectManipulationDeferContactServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelDeferral<Impl: IDirectManipulationDeferContactServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, DeferContact::<Impl, IMPL_OFFSET>, CancelContact::<Impl, IMPL_OFFSET>, CancelDeferral::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationDeferContactService as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationDragDropBehaviorImpl: Sized {
    fn SetConfiguration();
    fn GetStatus();
}
impl IDirectManipulationDragDropBehaviorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationDragDropBehaviorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationDragDropBehaviorVtbl {
        unsafe extern "system" fn SetConfiguration<Impl: IDirectManipulationDragDropBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatus<Impl: IDirectManipulationDragDropBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetConfiguration::<Impl, IMPL_OFFSET>, GetStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationDragDropBehavior as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationDragDropEventHandlerImpl: Sized {
    fn OnDragDropStatusChange();
}
impl IDirectManipulationDragDropEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationDragDropEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationDragDropEventHandlerVtbl {
        unsafe extern "system" fn OnDragDropStatusChange<Impl: IDirectManipulationDragDropEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnDragDropStatusChange::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationDragDropEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationFrameInfoProviderImpl: Sized {
    fn GetNextFrameInfo();
}
impl IDirectManipulationFrameInfoProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationFrameInfoProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationFrameInfoProviderVtbl {
        unsafe extern "system" fn GetNextFrameInfo<Impl: IDirectManipulationFrameInfoProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetNextFrameInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationFrameInfoProvider as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationInteractionEventHandlerImpl: Sized {
    fn OnInteraction();
}
impl IDirectManipulationInteractionEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationInteractionEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationInteractionEventHandlerVtbl {
        unsafe extern "system" fn OnInteraction<Impl: IDirectManipulationInteractionEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnInteraction::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationInteractionEventHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDirectManipulationManagerImpl: Sized {
    fn Activate();
    fn Deactivate();
    fn RegisterHitTestTarget();
    fn ProcessInput();
    fn GetUpdateManager();
    fn CreateViewport();
    fn CreateContent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDirectManipulationManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationManagerVtbl {
        unsafe extern "system" fn Activate<Impl: IDirectManipulationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Deactivate<Impl: IDirectManipulationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterHitTestTarget<Impl: IDirectManipulationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessInput<Impl: IDirectManipulationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *const super::super::UI::WindowsAndMessaging::MSG, handled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUpdateManager<Impl: IDirectManipulationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateViewport<Impl: IDirectManipulationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateContent<Impl: IDirectManipulationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Activate::<Impl, IMPL_OFFSET>, Deactivate::<Impl, IMPL_OFFSET>, RegisterHitTestTarget::<Impl, IMPL_OFFSET>, ProcessInput::<Impl, IMPL_OFFSET>, GetUpdateManager::<Impl, IMPL_OFFSET>, CreateViewport::<Impl, IMPL_OFFSET>, CreateContent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDirectManipulationManager2Impl: Sized + IDirectManipulationManagerImpl {
    fn CreateBehavior();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDirectManipulationManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationManager2Vtbl {
        unsafe extern "system" fn CreateBehavior<Impl: IDirectManipulationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Activate::<Impl, IMPL_OFFSET>, Deactivate::<Impl, IMPL_OFFSET>, RegisterHitTestTarget::<Impl, IMPL_OFFSET>, ProcessInput::<Impl, IMPL_OFFSET>, GetUpdateManager::<Impl, IMPL_OFFSET>, CreateViewport::<Impl, IMPL_OFFSET>, CreateContent::<Impl, IMPL_OFFSET>, CreateBehavior::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDirectManipulationManager3Impl: Sized + IDirectManipulationManager2Impl + IDirectManipulationManagerImpl {
    fn GetService();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDirectManipulationManager3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationManager3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationManager3Vtbl {
        unsafe extern "system" fn GetService<Impl: IDirectManipulationManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Activate::<Impl, IMPL_OFFSET>,
            Deactivate::<Impl, IMPL_OFFSET>,
            RegisterHitTestTarget::<Impl, IMPL_OFFSET>,
            ProcessInput::<Impl, IMPL_OFFSET>,
            GetUpdateManager::<Impl, IMPL_OFFSET>,
            CreateViewport::<Impl, IMPL_OFFSET>,
            CreateContent::<Impl, IMPL_OFFSET>,
            CreateBehavior::<Impl, IMPL_OFFSET>,
            GetService::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationManager3 as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationPrimaryContentImpl: Sized {
    fn SetSnapInterval();
    fn SetSnapPoints();
    fn SetSnapType();
    fn SetSnapCoordinate();
    fn SetZoomBoundaries();
    fn SetHorizontalAlignment();
    fn SetVerticalAlignment();
    fn GetInertiaEndTransform();
    fn GetCenterPoint();
}
impl IDirectManipulationPrimaryContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationPrimaryContentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationPrimaryContentVtbl {
        unsafe extern "system" fn SetSnapInterval<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSnapPoints<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSnapType<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSnapCoordinate<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetZoomBoundaries<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomminimum: f32, zoommaximum: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHorizontalAlignment<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVerticalAlignment<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInertiaEndTransform<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCenterPoint<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: *mut f32, centery: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetSnapInterval::<Impl, IMPL_OFFSET>,
            SetSnapPoints::<Impl, IMPL_OFFSET>,
            SetSnapType::<Impl, IMPL_OFFSET>,
            SetSnapCoordinate::<Impl, IMPL_OFFSET>,
            SetZoomBoundaries::<Impl, IMPL_OFFSET>,
            SetHorizontalAlignment::<Impl, IMPL_OFFSET>,
            SetVerticalAlignment::<Impl, IMPL_OFFSET>,
            GetInertiaEndTransform::<Impl, IMPL_OFFSET>,
            GetCenterPoint::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationPrimaryContent as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationUpdateHandlerImpl: Sized {
    fn Update();
}
impl IDirectManipulationUpdateHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationUpdateHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationUpdateHandlerVtbl {
        unsafe extern "system" fn Update<Impl: IDirectManipulationUpdateHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Update::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationUpdateHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectManipulationUpdateManagerImpl: Sized {
    fn RegisterWaitHandleCallback();
    fn UnregisterWaitHandleCallback();
    fn Update();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectManipulationUpdateManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationUpdateManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationUpdateManagerVtbl {
        unsafe extern "system" fn RegisterWaitHandleCallback<Impl: IDirectManipulationUpdateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, eventhandler: ::windows::core::RawPtr, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterWaitHandleCallback<Impl: IDirectManipulationUpdateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Update<Impl: IDirectManipulationUpdateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RegisterWaitHandleCallback::<Impl, IMPL_OFFSET>, UnregisterWaitHandleCallback::<Impl, IMPL_OFFSET>, Update::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationUpdateManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectManipulationViewportImpl: Sized {
    fn Enable();
    fn Disable();
    fn SetContact();
    fn ReleaseContact();
    fn ReleaseAllContacts();
    fn GetStatus();
    fn GetTag();
    fn SetTag();
    fn GetViewportRect();
    fn SetViewportRect();
    fn ZoomToRect();
    fn SetViewportTransform();
    fn SyncDisplayTransform();
    fn GetPrimaryContent();
    fn AddContent();
    fn RemoveContent();
    fn SetViewportOptions();
    fn AddConfiguration();
    fn RemoveConfiguration();
    fn ActivateConfiguration();
    fn SetManualGesture();
    fn SetChaining();
    fn AddEventHandler();
    fn RemoveEventHandler();
    fn SetInputMode();
    fn SetUpdateMode();
    fn Stop();
    fn Abandon();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectManipulationViewportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationViewportVtbl {
        unsafe extern "system" fn Enable<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Disable<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContact<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseContact<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseAllContacts<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatus<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut DIRECTMANIPULATION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTag<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTag<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetViewportRect<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetViewportRect<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ZoomToRect<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetViewportTransform<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SyncDisplayTransform<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrimaryContent<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddContent<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveContent<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetViewportOptions<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddConfiguration<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveConfiguration<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ActivateConfiguration<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetManualGesture<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetChaining<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddEventHandler<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, eventhandler: ::windows::core::RawPtr, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveEventHandler<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInputMode<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUpdateMode<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Abandon<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Enable::<Impl, IMPL_OFFSET>,
            Disable::<Impl, IMPL_OFFSET>,
            SetContact::<Impl, IMPL_OFFSET>,
            ReleaseContact::<Impl, IMPL_OFFSET>,
            ReleaseAllContacts::<Impl, IMPL_OFFSET>,
            GetStatus::<Impl, IMPL_OFFSET>,
            GetTag::<Impl, IMPL_OFFSET>,
            SetTag::<Impl, IMPL_OFFSET>,
            GetViewportRect::<Impl, IMPL_OFFSET>,
            SetViewportRect::<Impl, IMPL_OFFSET>,
            ZoomToRect::<Impl, IMPL_OFFSET>,
            SetViewportTransform::<Impl, IMPL_OFFSET>,
            SyncDisplayTransform::<Impl, IMPL_OFFSET>,
            GetPrimaryContent::<Impl, IMPL_OFFSET>,
            AddContent::<Impl, IMPL_OFFSET>,
            RemoveContent::<Impl, IMPL_OFFSET>,
            SetViewportOptions::<Impl, IMPL_OFFSET>,
            AddConfiguration::<Impl, IMPL_OFFSET>,
            RemoveConfiguration::<Impl, IMPL_OFFSET>,
            ActivateConfiguration::<Impl, IMPL_OFFSET>,
            SetManualGesture::<Impl, IMPL_OFFSET>,
            SetChaining::<Impl, IMPL_OFFSET>,
            AddEventHandler::<Impl, IMPL_OFFSET>,
            RemoveEventHandler::<Impl, IMPL_OFFSET>,
            SetInputMode::<Impl, IMPL_OFFSET>,
            SetUpdateMode::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
            Abandon::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationViewport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectManipulationViewport2Impl: Sized + IDirectManipulationViewportImpl {
    fn AddBehavior();
    fn RemoveBehavior();
    fn RemoveAllBehaviors();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectManipulationViewport2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationViewport2Vtbl {
        unsafe extern "system" fn AddBehavior<Impl: IDirectManipulationViewport2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, behavior: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveBehavior<Impl: IDirectManipulationViewport2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAllBehaviors<Impl: IDirectManipulationViewport2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Enable::<Impl, IMPL_OFFSET>,
            Disable::<Impl, IMPL_OFFSET>,
            SetContact::<Impl, IMPL_OFFSET>,
            ReleaseContact::<Impl, IMPL_OFFSET>,
            ReleaseAllContacts::<Impl, IMPL_OFFSET>,
            GetStatus::<Impl, IMPL_OFFSET>,
            GetTag::<Impl, IMPL_OFFSET>,
            SetTag::<Impl, IMPL_OFFSET>,
            GetViewportRect::<Impl, IMPL_OFFSET>,
            SetViewportRect::<Impl, IMPL_OFFSET>,
            ZoomToRect::<Impl, IMPL_OFFSET>,
            SetViewportTransform::<Impl, IMPL_OFFSET>,
            SyncDisplayTransform::<Impl, IMPL_OFFSET>,
            GetPrimaryContent::<Impl, IMPL_OFFSET>,
            AddContent::<Impl, IMPL_OFFSET>,
            RemoveContent::<Impl, IMPL_OFFSET>,
            SetViewportOptions::<Impl, IMPL_OFFSET>,
            AddConfiguration::<Impl, IMPL_OFFSET>,
            RemoveConfiguration::<Impl, IMPL_OFFSET>,
            ActivateConfiguration::<Impl, IMPL_OFFSET>,
            SetManualGesture::<Impl, IMPL_OFFSET>,
            SetChaining::<Impl, IMPL_OFFSET>,
            AddEventHandler::<Impl, IMPL_OFFSET>,
            RemoveEventHandler::<Impl, IMPL_OFFSET>,
            SetInputMode::<Impl, IMPL_OFFSET>,
            SetUpdateMode::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
            Abandon::<Impl, IMPL_OFFSET>,
            AddBehavior::<Impl, IMPL_OFFSET>,
            RemoveBehavior::<Impl, IMPL_OFFSET>,
            RemoveAllBehaviors::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationViewport2 as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationViewportEventHandlerImpl: Sized {
    fn OnViewportStatusChanged();
    fn OnViewportUpdated();
    fn OnContentUpdated();
}
impl IDirectManipulationViewportEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewportEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationViewportEventHandlerVtbl {
        unsafe extern "system" fn OnViewportStatusChanged<Impl: IDirectManipulationViewportEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnViewportUpdated<Impl: IDirectManipulationViewportEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnContentUpdated<Impl: IDirectManipulationViewportEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnViewportStatusChanged::<Impl, IMPL_OFFSET>, OnViewportUpdated::<Impl, IMPL_OFFSET>, OnContentUpdated::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationViewportEventHandler as ::windows::core::Interface>::IID
    }
}
