pub trait IDirectManipulationAutoScrollBehaviorImpl: Sized {
    fn SetConfiguration(&mut self, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows::core::Result<()>;
}
impl IDirectManipulationAutoScrollBehaviorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationAutoScrollBehaviorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationAutoScrollBehaviorVtbl {
        unsafe extern "system" fn SetConfiguration<Impl: IDirectManipulationAutoScrollBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConfiguration(::core::mem::transmute_copy(&motiontypes), ::core::mem::transmute_copy(&scrollmotion)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetConfiguration: SetConfiguration::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationAutoScrollBehavior as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationCompositorImpl: Sized {
    fn AddContent(&mut self, content: ::core::option::Option<IDirectManipulationContent>, device: ::core::option::Option<::windows::core::IUnknown>, parentvisual: ::core::option::Option<::windows::core::IUnknown>, childvisual: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn RemoveContent(&mut self, content: ::core::option::Option<IDirectManipulationContent>) -> ::windows::core::Result<()>;
    fn SetUpdateManager(&mut self, updatemanager: ::core::option::Option<IDirectManipulationUpdateManager>) -> ::windows::core::Result<()>;
    fn Flush(&mut self) -> ::windows::core::Result<()>;
}
impl IDirectManipulationCompositorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationCompositorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationCompositorVtbl {
        unsafe extern "system" fn AddContent<Impl: IDirectManipulationCompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddContent(::core::mem::transmute(&content), ::core::mem::transmute(&device), ::core::mem::transmute(&parentvisual), ::core::mem::transmute(&childvisual)).into()
        }
        unsafe extern "system" fn RemoveContent<Impl: IDirectManipulationCompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveContent(::core::mem::transmute(&content)).into()
        }
        unsafe extern "system" fn SetUpdateManager<Impl: IDirectManipulationCompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatemanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUpdateManager(::core::mem::transmute(&updatemanager)).into()
        }
        unsafe extern "system" fn Flush<Impl: IDirectManipulationCompositorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flush().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddContent: AddContent::<Impl, IMPL_OFFSET>,
            RemoveContent: RemoveContent::<Impl, IMPL_OFFSET>,
            SetUpdateManager: SetUpdateManager::<Impl, IMPL_OFFSET>,
            Flush: Flush::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationCompositor as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationCompositor2Impl: Sized + IDirectManipulationCompositorImpl {
    fn AddContentWithCrossProcessChaining(&mut self, content: ::core::option::Option<IDirectManipulationPrimaryContent>, device: ::core::option::Option<::windows::core::IUnknown>, parentvisual: ::core::option::Option<::windows::core::IUnknown>, childvisual: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IDirectManipulationCompositor2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationCompositor2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationCompositor2Vtbl {
        unsafe extern "system" fn AddContentWithCrossProcessChaining<Impl: IDirectManipulationCompositor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddContentWithCrossProcessChaining(::core::mem::transmute(&content), ::core::mem::transmute(&device), ::core::mem::transmute(&parentvisual), ::core::mem::transmute(&childvisual)).into()
        }
        Self {
            base: IDirectManipulationCompositorVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddContentWithCrossProcessChaining: AddContentWithCrossProcessChaining::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationCompositor2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectManipulationContentImpl: Sized {
    fn GetContentRect(&mut self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn SetContentRect(&mut self, contentsize: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn GetViewport(&mut self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetTag(&mut self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::Result<()>;
    fn SetTag(&mut self, object: ::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<()>;
    fn GetOutputTransform(&mut self, matrix: *mut f32, pointcount: u32) -> ::windows::core::Result<()>;
    fn GetContentTransform(&mut self, matrix: *mut f32, pointcount: u32) -> ::windows::core::Result<()>;
    fn SyncContentTransform(&mut self, matrix: *const f32, pointcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectManipulationContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationContentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationContentVtbl {
        unsafe extern "system" fn GetContentRect<Impl: IDirectManipulationContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentsize: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContentRect() {
                ::core::result::Result::Ok(ok__) => {
                    *contentsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentRect<Impl: IDirectManipulationContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentsize: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContentRect(::core::mem::transmute_copy(&contentsize)).into()
        }
        unsafe extern "system" fn GetViewport<Impl: IDirectManipulationContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetViewport(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        unsafe extern "system" fn GetTag<Impl: IDirectManipulationContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTag(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SetTag<Impl: IDirectManipulationContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetOutputTransform<Impl: IDirectManipulationContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutputTransform(::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn GetContentTransform<Impl: IDirectManipulationContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetContentTransform(::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn SyncContentTransform<Impl: IDirectManipulationContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SyncContentTransform(::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetContentRect: GetContentRect::<Impl, IMPL_OFFSET>,
            SetContentRect: SetContentRect::<Impl, IMPL_OFFSET>,
            GetViewport: GetViewport::<Impl, IMPL_OFFSET>,
            GetTag: GetTag::<Impl, IMPL_OFFSET>,
            SetTag: SetTag::<Impl, IMPL_OFFSET>,
            GetOutputTransform: GetOutputTransform::<Impl, IMPL_OFFSET>,
            GetContentTransform: GetContentTransform::<Impl, IMPL_OFFSET>,
            SyncContentTransform: SyncContentTransform::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationContent as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationDeferContactServiceImpl: Sized {
    fn DeferContact(&mut self, pointerid: u32, timeout: u32) -> ::windows::core::Result<()>;
    fn CancelContact(&mut self, pointerid: u32) -> ::windows::core::Result<()>;
    fn CancelDeferral(&mut self, pointerid: u32) -> ::windows::core::Result<()>;
}
impl IDirectManipulationDeferContactServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationDeferContactServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationDeferContactServiceVtbl {
        unsafe extern "system" fn DeferContact<Impl: IDirectManipulationDeferContactServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeferContact(::core::mem::transmute_copy(&pointerid), ::core::mem::transmute_copy(&timeout)).into()
        }
        unsafe extern "system" fn CancelContact<Impl: IDirectManipulationDeferContactServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelContact(::core::mem::transmute_copy(&pointerid)).into()
        }
        unsafe extern "system" fn CancelDeferral<Impl: IDirectManipulationDeferContactServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelDeferral(::core::mem::transmute_copy(&pointerid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            DeferContact: DeferContact::<Impl, IMPL_OFFSET>,
            CancelContact: CancelContact::<Impl, IMPL_OFFSET>,
            CancelDeferral: CancelDeferral::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationDeferContactService as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationDragDropBehaviorImpl: Sized {
    fn SetConfiguration(&mut self, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows::core::Result<()>;
    fn GetStatus(&mut self) -> ::windows::core::Result<DIRECTMANIPULATION_DRAG_DROP_STATUS>;
}
impl IDirectManipulationDragDropBehaviorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationDragDropBehaviorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationDragDropBehaviorVtbl {
        unsafe extern "system" fn SetConfiguration<Impl: IDirectManipulationDragDropBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConfiguration(::core::mem::transmute_copy(&configuration)).into()
        }
        unsafe extern "system" fn GetStatus<Impl: IDirectManipulationDragDropBehaviorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetConfiguration: SetConfiguration::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationDragDropBehavior as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationDragDropEventHandlerImpl: Sized {
    fn OnDragDropStatusChange(&mut self, viewport: ::core::option::Option<IDirectManipulationViewport2>, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::Result<()>;
}
impl IDirectManipulationDragDropEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationDragDropEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationDragDropEventHandlerVtbl {
        unsafe extern "system" fn OnDragDropStatusChange<Impl: IDirectManipulationDragDropEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDragDropStatusChange(::core::mem::transmute(&viewport), ::core::mem::transmute_copy(&current), ::core::mem::transmute_copy(&previous)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnDragDropStatusChange: OnDragDropStatusChange::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationDragDropEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationFrameInfoProviderImpl: Sized {
    fn GetNextFrameInfo(&mut self, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows::core::Result<()>;
}
impl IDirectManipulationFrameInfoProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationFrameInfoProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationFrameInfoProviderVtbl {
        unsafe extern "system" fn GetNextFrameInfo<Impl: IDirectManipulationFrameInfoProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNextFrameInfo(::core::mem::transmute_copy(&time), ::core::mem::transmute_copy(&processtime), ::core::mem::transmute_copy(&compositiontime)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetNextFrameInfo: GetNextFrameInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationFrameInfoProvider as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationInteractionEventHandlerImpl: Sized {
    fn OnInteraction(&mut self, viewport: ::core::option::Option<IDirectManipulationViewport2>, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows::core::Result<()>;
}
impl IDirectManipulationInteractionEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationInteractionEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationInteractionEventHandlerVtbl {
        unsafe extern "system" fn OnInteraction<Impl: IDirectManipulationInteractionEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnInteraction(::core::mem::transmute(&viewport), ::core::mem::transmute_copy(&interaction)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnInteraction: OnInteraction::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationInteractionEventHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDirectManipulationManagerImpl: Sized {
    fn Activate(&mut self, window: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn Deactivate(&mut self, window: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn RegisterHitTestTarget(&mut self, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::Result<()>;
    fn ProcessInput(&mut self, message: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetUpdateManager(&mut self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateViewport(&mut self, frameinfo: ::core::option::Option<IDirectManipulationFrameInfoProvider>, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateContent(&mut self, frameinfo: ::core::option::Option<IDirectManipulationFrameInfoProvider>, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDirectManipulationManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationManagerVtbl {
        unsafe extern "system" fn Activate<Impl: IDirectManipulationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Activate(::core::mem::transmute_copy(&window)).into()
        }
        unsafe extern "system" fn Deactivate<Impl: IDirectManipulationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Deactivate(::core::mem::transmute_copy(&window)).into()
        }
        unsafe extern "system" fn RegisterHitTestTarget<Impl: IDirectManipulationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterHitTestTarget(::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&hittestwindow), ::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn ProcessInput<Impl: IDirectManipulationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *const super::super::UI::WindowsAndMessaging::MSG, handled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessInput(::core::mem::transmute_copy(&message)) {
                ::core::result::Result::Ok(ok__) => {
                    *handled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateManager<Impl: IDirectManipulationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUpdateManager(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        unsafe extern "system" fn CreateViewport<Impl: IDirectManipulationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateViewport(::core::mem::transmute(&frameinfo), ::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        unsafe extern "system" fn CreateContent<Impl: IDirectManipulationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateContent(::core::mem::transmute(&frameinfo), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Activate: Activate::<Impl, IMPL_OFFSET>,
            Deactivate: Deactivate::<Impl, IMPL_OFFSET>,
            RegisterHitTestTarget: RegisterHitTestTarget::<Impl, IMPL_OFFSET>,
            ProcessInput: ProcessInput::<Impl, IMPL_OFFSET>,
            GetUpdateManager: GetUpdateManager::<Impl, IMPL_OFFSET>,
            CreateViewport: CreateViewport::<Impl, IMPL_OFFSET>,
            CreateContent: CreateContent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDirectManipulationManager2Impl: Sized + IDirectManipulationManagerImpl {
    fn CreateBehavior(&mut self, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDirectManipulationManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationManager2Vtbl {
        unsafe extern "system" fn CreateBehavior<Impl: IDirectManipulationManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateBehavior(::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        Self { base: IDirectManipulationManagerVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CreateBehavior: CreateBehavior::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationManager2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDirectManipulationManager3Impl: Sized + IDirectManipulationManagerImpl + IDirectManipulationManager2Impl {
    fn GetService(&mut self, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDirectManipulationManager3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationManager3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationManager3Vtbl {
        unsafe extern "system" fn GetService<Impl: IDirectManipulationManager3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetService(::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        Self { base: IDirectManipulationManager2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetService: GetService::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationManager3 as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationPrimaryContentImpl: Sized {
    fn SetSnapInterval(&mut self, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> ::windows::core::Result<()>;
    fn SetSnapPoints(&mut self, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> ::windows::core::Result<()>;
    fn SetSnapType(&mut self, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> ::windows::core::Result<()>;
    fn SetSnapCoordinate(&mut self, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> ::windows::core::Result<()>;
    fn SetZoomBoundaries(&mut self, zoomminimum: f32, zoommaximum: f32) -> ::windows::core::Result<()>;
    fn SetHorizontalAlignment(&mut self, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> ::windows::core::Result<()>;
    fn SetVerticalAlignment(&mut self, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> ::windows::core::Result<()>;
    fn GetInertiaEndTransform(&mut self, matrix: *mut f32, pointcount: u32) -> ::windows::core::Result<()>;
    fn GetCenterPoint(&mut self, centerx: *mut f32, centery: *mut f32) -> ::windows::core::Result<()>;
}
impl IDirectManipulationPrimaryContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationPrimaryContentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationPrimaryContentVtbl {
        unsafe extern "system" fn SetSnapInterval<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSnapInterval(::core::mem::transmute_copy(&motion), ::core::mem::transmute_copy(&interval), ::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn SetSnapPoints<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSnapPoints(::core::mem::transmute_copy(&motion), ::core::mem::transmute_copy(&points), ::core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn SetSnapType<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSnapType(::core::mem::transmute_copy(&motion), ::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn SetSnapCoordinate<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSnapCoordinate(::core::mem::transmute_copy(&motion), ::core::mem::transmute_copy(&coordinate), ::core::mem::transmute_copy(&origin)).into()
        }
        unsafe extern "system" fn SetZoomBoundaries<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomminimum: f32, zoommaximum: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetZoomBoundaries(::core::mem::transmute_copy(&zoomminimum), ::core::mem::transmute_copy(&zoommaximum)).into()
        }
        unsafe extern "system" fn SetHorizontalAlignment<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHorizontalAlignment(::core::mem::transmute_copy(&alignment)).into()
        }
        unsafe extern "system" fn SetVerticalAlignment<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVerticalAlignment(::core::mem::transmute_copy(&alignment)).into()
        }
        unsafe extern "system" fn GetInertiaEndTransform<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInertiaEndTransform(::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn GetCenterPoint<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: *mut f32, centery: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCenterPoint(::core::mem::transmute_copy(&centerx), ::core::mem::transmute_copy(&centery)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetSnapInterval: SetSnapInterval::<Impl, IMPL_OFFSET>,
            SetSnapPoints: SetSnapPoints::<Impl, IMPL_OFFSET>,
            SetSnapType: SetSnapType::<Impl, IMPL_OFFSET>,
            SetSnapCoordinate: SetSnapCoordinate::<Impl, IMPL_OFFSET>,
            SetZoomBoundaries: SetZoomBoundaries::<Impl, IMPL_OFFSET>,
            SetHorizontalAlignment: SetHorizontalAlignment::<Impl, IMPL_OFFSET>,
            SetVerticalAlignment: SetVerticalAlignment::<Impl, IMPL_OFFSET>,
            GetInertiaEndTransform: GetInertiaEndTransform::<Impl, IMPL_OFFSET>,
            GetCenterPoint: GetCenterPoint::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationPrimaryContent as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationUpdateHandlerImpl: Sized {
    fn Update(&mut self) -> ::windows::core::Result<()>;
}
impl IDirectManipulationUpdateHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationUpdateHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationUpdateHandlerVtbl {
        unsafe extern "system" fn Update<Impl: IDirectManipulationUpdateHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Update: Update::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationUpdateHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectManipulationUpdateManagerImpl: Sized {
    fn RegisterWaitHandleCallback(&mut self, handle: super::super::Foundation::HANDLE, eventhandler: ::core::option::Option<IDirectManipulationUpdateHandler>) -> ::windows::core::Result<u32>;
    fn UnregisterWaitHandleCallback(&mut self, cookie: u32) -> ::windows::core::Result<()>;
    fn Update(&mut self, frameinfo: ::core::option::Option<IDirectManipulationFrameInfoProvider>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectManipulationUpdateManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationUpdateManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationUpdateManagerVtbl {
        unsafe extern "system" fn RegisterWaitHandleCallback<Impl: IDirectManipulationUpdateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, eventhandler: ::windows::core::RawPtr, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterWaitHandleCallback(::core::mem::transmute_copy(&handle), ::core::mem::transmute(&eventhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *cookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterWaitHandleCallback<Impl: IDirectManipulationUpdateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterWaitHandleCallback(::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn Update<Impl: IDirectManipulationUpdateManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update(::core::mem::transmute(&frameinfo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterWaitHandleCallback: RegisterWaitHandleCallback::<Impl, IMPL_OFFSET>,
            UnregisterWaitHandleCallback: UnregisterWaitHandleCallback::<Impl, IMPL_OFFSET>,
            Update: Update::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationUpdateManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectManipulationViewportImpl: Sized {
    fn Enable(&mut self) -> ::windows::core::Result<()>;
    fn Disable(&mut self) -> ::windows::core::Result<()>;
    fn SetContact(&mut self, pointerid: u32) -> ::windows::core::Result<()>;
    fn ReleaseContact(&mut self, pointerid: u32) -> ::windows::core::Result<()>;
    fn ReleaseAllContacts(&mut self) -> ::windows::core::Result<()>;
    fn GetStatus(&mut self) -> ::windows::core::Result<DIRECTMANIPULATION_STATUS>;
    fn GetTag(&mut self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::Result<()>;
    fn SetTag(&mut self, object: ::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<()>;
    fn GetViewportRect(&mut self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn SetViewportRect(&mut self, viewport: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn ZoomToRect(&mut self, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetViewportTransform(&mut self, matrix: *const f32, pointcount: u32) -> ::windows::core::Result<()>;
    fn SyncDisplayTransform(&mut self, matrix: *const f32, pointcount: u32) -> ::windows::core::Result<()>;
    fn GetPrimaryContent(&mut self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn AddContent(&mut self, content: ::core::option::Option<IDirectManipulationContent>) -> ::windows::core::Result<()>;
    fn RemoveContent(&mut self, content: ::core::option::Option<IDirectManipulationContent>) -> ::windows::core::Result<()>;
    fn SetViewportOptions(&mut self, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::Result<()>;
    fn AddConfiguration(&mut self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()>;
    fn RemoveConfiguration(&mut self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()>;
    fn ActivateConfiguration(&mut self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()>;
    fn SetManualGesture(&mut self, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::Result<()>;
    fn SetChaining(&mut self, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::Result<()>;
    fn AddEventHandler(&mut self, window: super::super::Foundation::HWND, eventhandler: ::core::option::Option<IDirectManipulationViewportEventHandler>) -> ::windows::core::Result<u32>;
    fn RemoveEventHandler(&mut self, cookie: u32) -> ::windows::core::Result<()>;
    fn SetInputMode(&mut self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()>;
    fn SetUpdateMode(&mut self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Abandon(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectManipulationViewportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationViewportVtbl {
        unsafe extern "system" fn Enable<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enable().into()
        }
        unsafe extern "system" fn Disable<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disable().into()
        }
        unsafe extern "system" fn SetContact<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContact(::core::mem::transmute_copy(&pointerid)).into()
        }
        unsafe extern "system" fn ReleaseContact<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseContact(::core::mem::transmute_copy(&pointerid)).into()
        }
        unsafe extern "system" fn ReleaseAllContacts<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseAllContacts().into()
        }
        unsafe extern "system" fn GetStatus<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut DIRECTMANIPULATION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTag<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTag(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SetTag<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetViewportRect<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetViewportRect() {
                ::core::result::Result::Ok(ok__) => {
                    *viewport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewportRect<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewportRect(::core::mem::transmute_copy(&viewport)).into()
        }
        unsafe extern "system" fn ZoomToRect<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ZoomToRect(::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&top), ::core::mem::transmute_copy(&right), ::core::mem::transmute_copy(&bottom), ::core::mem::transmute_copy(&animate)).into()
        }
        unsafe extern "system" fn SetViewportTransform<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewportTransform(::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn SyncDisplayTransform<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SyncDisplayTransform(::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn GetPrimaryContent<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPrimaryContent(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        unsafe extern "system" fn AddContent<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddContent(::core::mem::transmute(&content)).into()
        }
        unsafe extern "system" fn RemoveContent<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveContent(::core::mem::transmute(&content)).into()
        }
        unsafe extern "system" fn SetViewportOptions<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetViewportOptions(::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn AddConfiguration<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddConfiguration(::core::mem::transmute_copy(&configuration)).into()
        }
        unsafe extern "system" fn RemoveConfiguration<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveConfiguration(::core::mem::transmute_copy(&configuration)).into()
        }
        unsafe extern "system" fn ActivateConfiguration<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ActivateConfiguration(::core::mem::transmute_copy(&configuration)).into()
        }
        unsafe extern "system" fn SetManualGesture<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetManualGesture(::core::mem::transmute_copy(&configuration)).into()
        }
        unsafe extern "system" fn SetChaining<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChaining(::core::mem::transmute_copy(&enabledtypes)).into()
        }
        unsafe extern "system" fn AddEventHandler<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, eventhandler: ::windows::core::RawPtr, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddEventHandler(::core::mem::transmute_copy(&window), ::core::mem::transmute(&eventhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *cookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEventHandler<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveEventHandler(::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn SetInputMode<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInputMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetUpdateMode<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUpdateMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn Stop<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Abandon<Impl: IDirectManipulationViewportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Abandon().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Enable: Enable::<Impl, IMPL_OFFSET>,
            Disable: Disable::<Impl, IMPL_OFFSET>,
            SetContact: SetContact::<Impl, IMPL_OFFSET>,
            ReleaseContact: ReleaseContact::<Impl, IMPL_OFFSET>,
            ReleaseAllContacts: ReleaseAllContacts::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            GetTag: GetTag::<Impl, IMPL_OFFSET>,
            SetTag: SetTag::<Impl, IMPL_OFFSET>,
            GetViewportRect: GetViewportRect::<Impl, IMPL_OFFSET>,
            SetViewportRect: SetViewportRect::<Impl, IMPL_OFFSET>,
            ZoomToRect: ZoomToRect::<Impl, IMPL_OFFSET>,
            SetViewportTransform: SetViewportTransform::<Impl, IMPL_OFFSET>,
            SyncDisplayTransform: SyncDisplayTransform::<Impl, IMPL_OFFSET>,
            GetPrimaryContent: GetPrimaryContent::<Impl, IMPL_OFFSET>,
            AddContent: AddContent::<Impl, IMPL_OFFSET>,
            RemoveContent: RemoveContent::<Impl, IMPL_OFFSET>,
            SetViewportOptions: SetViewportOptions::<Impl, IMPL_OFFSET>,
            AddConfiguration: AddConfiguration::<Impl, IMPL_OFFSET>,
            RemoveConfiguration: RemoveConfiguration::<Impl, IMPL_OFFSET>,
            ActivateConfiguration: ActivateConfiguration::<Impl, IMPL_OFFSET>,
            SetManualGesture: SetManualGesture::<Impl, IMPL_OFFSET>,
            SetChaining: SetChaining::<Impl, IMPL_OFFSET>,
            AddEventHandler: AddEventHandler::<Impl, IMPL_OFFSET>,
            RemoveEventHandler: RemoveEventHandler::<Impl, IMPL_OFFSET>,
            SetInputMode: SetInputMode::<Impl, IMPL_OFFSET>,
            SetUpdateMode: SetUpdateMode::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Abandon: Abandon::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationViewport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectManipulationViewport2Impl: Sized + IDirectManipulationViewportImpl {
    fn AddBehavior(&mut self, behavior: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<u32>;
    fn RemoveBehavior(&mut self, cookie: u32) -> ::windows::core::Result<()>;
    fn RemoveAllBehaviors(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectManipulationViewport2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationViewport2Vtbl {
        unsafe extern "system" fn AddBehavior<Impl: IDirectManipulationViewport2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, behavior: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddBehavior(::core::mem::transmute(&behavior)) {
                ::core::result::Result::Ok(ok__) => {
                    *cookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBehavior<Impl: IDirectManipulationViewport2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBehavior(::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn RemoveAllBehaviors<Impl: IDirectManipulationViewport2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAllBehaviors().into()
        }
        Self {
            base: IDirectManipulationViewportVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddBehavior: AddBehavior::<Impl, IMPL_OFFSET>,
            RemoveBehavior: RemoveBehavior::<Impl, IMPL_OFFSET>,
            RemoveAllBehaviors: RemoveAllBehaviors::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationViewport2 as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationViewportEventHandlerImpl: Sized {
    fn OnViewportStatusChanged(&mut self, viewport: ::core::option::Option<IDirectManipulationViewport>, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows::core::Result<()>;
    fn OnViewportUpdated(&mut self, viewport: ::core::option::Option<IDirectManipulationViewport>) -> ::windows::core::Result<()>;
    fn OnContentUpdated(&mut self, viewport: ::core::option::Option<IDirectManipulationViewport>, content: ::core::option::Option<IDirectManipulationContent>) -> ::windows::core::Result<()>;
}
impl IDirectManipulationViewportEventHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewportEventHandlerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectManipulationViewportEventHandlerVtbl {
        unsafe extern "system" fn OnViewportStatusChanged<Impl: IDirectManipulationViewportEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnViewportStatusChanged(::core::mem::transmute(&viewport), ::core::mem::transmute_copy(&current), ::core::mem::transmute_copy(&previous)).into()
        }
        unsafe extern "system" fn OnViewportUpdated<Impl: IDirectManipulationViewportEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnViewportUpdated(::core::mem::transmute(&viewport)).into()
        }
        unsafe extern "system" fn OnContentUpdated<Impl: IDirectManipulationViewportEventHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnContentUpdated(::core::mem::transmute(&viewport), ::core::mem::transmute(&content)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnViewportStatusChanged: OnViewportStatusChanged::<Impl, IMPL_OFFSET>,
            OnViewportUpdated: OnViewportUpdated::<Impl, IMPL_OFFSET>,
            OnContentUpdated: OnContentUpdated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationViewportEventHandler as ::windows::core::Interface>::IID
    }
}
