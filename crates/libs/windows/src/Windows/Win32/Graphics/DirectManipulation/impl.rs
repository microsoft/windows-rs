pub trait IDirectManipulationAutoScrollBehavior_Impl: Sized {
    fn SetConfiguration(&mut self, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows::core::Result<()>;
}
impl IDirectManipulationAutoScrollBehavior_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationAutoScrollBehavior_Impl, const OFFSET: isize>() -> IDirectManipulationAutoScrollBehavior_Vtbl {
        unsafe extern "system" fn SetConfiguration<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationAutoScrollBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConfiguration(::core::mem::transmute_copy(&motiontypes), ::core::mem::transmute_copy(&scrollmotion)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetConfiguration: SetConfiguration::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationAutoScrollBehavior as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationCompositor_Impl: Sized {
    fn AddContent(&mut self, content: &::core::option::Option<IDirectManipulationContent>, device: &::core::option::Option<::windows::core::IUnknown>, parentvisual: &::core::option::Option<::windows::core::IUnknown>, childvisual: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn RemoveContent(&mut self, content: &::core::option::Option<IDirectManipulationContent>) -> ::windows::core::Result<()>;
    fn SetUpdateManager(&mut self, updatemanager: &::core::option::Option<IDirectManipulationUpdateManager>) -> ::windows::core::Result<()>;
    fn Flush(&mut self) -> ::windows::core::Result<()>;
}
impl IDirectManipulationCompositor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationCompositor_Impl, const OFFSET: isize>() -> IDirectManipulationCompositor_Vtbl {
        unsafe extern "system" fn AddContent<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationCompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddContent(::core::mem::transmute(&content), ::core::mem::transmute(&device), ::core::mem::transmute(&parentvisual), ::core::mem::transmute(&childvisual)).into()
        }
        unsafe extern "system" fn RemoveContent<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationCompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveContent(::core::mem::transmute(&content)).into()
        }
        unsafe extern "system" fn SetUpdateManager<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationCompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatemanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUpdateManager(::core::mem::transmute(&updatemanager)).into()
        }
        unsafe extern "system" fn Flush<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationCompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Flush().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddContent: AddContent::<Identity, Impl, OFFSET>,
            RemoveContent: RemoveContent::<Identity, Impl, OFFSET>,
            SetUpdateManager: SetUpdateManager::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationCompositor as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationCompositor2_Impl: Sized + IDirectManipulationCompositor_Impl {
    fn AddContentWithCrossProcessChaining(&mut self, content: &::core::option::Option<IDirectManipulationPrimaryContent>, device: &::core::option::Option<::windows::core::IUnknown>, parentvisual: &::core::option::Option<::windows::core::IUnknown>, childvisual: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IDirectManipulationCompositor2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationCompositor2_Impl, const OFFSET: isize>() -> IDirectManipulationCompositor2_Vtbl {
        unsafe extern "system" fn AddContentWithCrossProcessChaining<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationCompositor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddContentWithCrossProcessChaining(::core::mem::transmute(&content), ::core::mem::transmute(&device), ::core::mem::transmute(&parentvisual), ::core::mem::transmute(&childvisual)).into()
        }
        Self {
            base: IDirectManipulationCompositor_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddContentWithCrossProcessChaining: AddContentWithCrossProcessChaining::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationCompositor2 as ::windows::core::Interface>::IID || iid == &<IDirectManipulationCompositor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectManipulationContent_Impl: Sized {
    fn GetContentRect(&mut self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn SetContentRect(&mut self, contentsize: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn GetViewport(&mut self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetTag(&mut self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::Result<()>;
    fn SetTag(&mut self, object: &::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<()>;
    fn GetOutputTransform(&mut self, matrix: *mut f32, pointcount: u32) -> ::windows::core::Result<()>;
    fn GetContentTransform(&mut self, matrix: *mut f32, pointcount: u32) -> ::windows::core::Result<()>;
    fn SyncContentTransform(&mut self, matrix: *const f32, pointcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectManipulationContent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationContent_Impl, const OFFSET: isize>() -> IDirectManipulationContent_Vtbl {
        unsafe extern "system" fn GetContentRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentsize: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContentRect() {
                ::core::result::Result::Ok(ok__) => {
                    *contentsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentsize: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetContentRect(::core::mem::transmute_copy(&contentsize)).into()
        }
        unsafe extern "system" fn GetViewport<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetViewport(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        unsafe extern "system" fn GetTag<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTag(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SetTag<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetOutputTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOutputTransform(::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn GetContentTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetContentTransform(::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn SyncContentTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SyncContentTransform(::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetContentRect: GetContentRect::<Identity, Impl, OFFSET>,
            SetContentRect: SetContentRect::<Identity, Impl, OFFSET>,
            GetViewport: GetViewport::<Identity, Impl, OFFSET>,
            GetTag: GetTag::<Identity, Impl, OFFSET>,
            SetTag: SetTag::<Identity, Impl, OFFSET>,
            GetOutputTransform: GetOutputTransform::<Identity, Impl, OFFSET>,
            GetContentTransform: GetContentTransform::<Identity, Impl, OFFSET>,
            SyncContentTransform: SyncContentTransform::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationContent as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationDeferContactService_Impl: Sized {
    fn DeferContact(&mut self, pointerid: u32, timeout: u32) -> ::windows::core::Result<()>;
    fn CancelContact(&mut self, pointerid: u32) -> ::windows::core::Result<()>;
    fn CancelDeferral(&mut self, pointerid: u32) -> ::windows::core::Result<()>;
}
impl IDirectManipulationDeferContactService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationDeferContactService_Impl, const OFFSET: isize>() -> IDirectManipulationDeferContactService_Vtbl {
        unsafe extern "system" fn DeferContact<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationDeferContactService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeferContact(::core::mem::transmute_copy(&pointerid), ::core::mem::transmute_copy(&timeout)).into()
        }
        unsafe extern "system" fn CancelContact<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationDeferContactService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelContact(::core::mem::transmute_copy(&pointerid)).into()
        }
        unsafe extern "system" fn CancelDeferral<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationDeferContactService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelDeferral(::core::mem::transmute_copy(&pointerid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            DeferContact: DeferContact::<Identity, Impl, OFFSET>,
            CancelContact: CancelContact::<Identity, Impl, OFFSET>,
            CancelDeferral: CancelDeferral::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationDeferContactService as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationDragDropBehavior_Impl: Sized {
    fn SetConfiguration(&mut self, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows::core::Result<()>;
    fn GetStatus(&mut self) -> ::windows::core::Result<DIRECTMANIPULATION_DRAG_DROP_STATUS>;
}
impl IDirectManipulationDragDropBehavior_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationDragDropBehavior_Impl, const OFFSET: isize>() -> IDirectManipulationDragDropBehavior_Vtbl {
        unsafe extern "system" fn SetConfiguration<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationDragDropBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConfiguration(::core::mem::transmute_copy(&configuration)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationDragDropBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetConfiguration: SetConfiguration::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationDragDropBehavior as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationDragDropEventHandler_Impl: Sized {
    fn OnDragDropStatusChange(&mut self, viewport: &::core::option::Option<IDirectManipulationViewport2>, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::Result<()>;
}
impl IDirectManipulationDragDropEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationDragDropEventHandler_Impl, const OFFSET: isize>() -> IDirectManipulationDragDropEventHandler_Vtbl {
        unsafe extern "system" fn OnDragDropStatusChange<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationDragDropEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDragDropStatusChange(::core::mem::transmute(&viewport), ::core::mem::transmute_copy(&current), ::core::mem::transmute_copy(&previous)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnDragDropStatusChange: OnDragDropStatusChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationDragDropEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationFrameInfoProvider_Impl: Sized {
    fn GetNextFrameInfo(&mut self, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows::core::Result<()>;
}
impl IDirectManipulationFrameInfoProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationFrameInfoProvider_Impl, const OFFSET: isize>() -> IDirectManipulationFrameInfoProvider_Vtbl {
        unsafe extern "system" fn GetNextFrameInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationFrameInfoProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNextFrameInfo(::core::mem::transmute_copy(&time), ::core::mem::transmute_copy(&processtime), ::core::mem::transmute_copy(&compositiontime)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetNextFrameInfo: GetNextFrameInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationFrameInfoProvider as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationInteractionEventHandler_Impl: Sized {
    fn OnInteraction(&mut self, viewport: &::core::option::Option<IDirectManipulationViewport2>, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows::core::Result<()>;
}
impl IDirectManipulationInteractionEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationInteractionEventHandler_Impl, const OFFSET: isize>() -> IDirectManipulationInteractionEventHandler_Vtbl {
        unsafe extern "system" fn OnInteraction<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationInteractionEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnInteraction(::core::mem::transmute(&viewport), ::core::mem::transmute_copy(&interaction)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnInteraction: OnInteraction::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationInteractionEventHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDirectManipulationManager_Impl: Sized {
    fn Activate(&mut self, window: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn Deactivate(&mut self, window: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn RegisterHitTestTarget(&mut self, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::Result<()>;
    fn ProcessInput(&mut self, message: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetUpdateManager(&mut self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateViewport(&mut self, frameinfo: &::core::option::Option<IDirectManipulationFrameInfoProvider>, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateContent(&mut self, frameinfo: &::core::option::Option<IDirectManipulationFrameInfoProvider>, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDirectManipulationManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationManager_Impl, const OFFSET: isize>() -> IDirectManipulationManager_Vtbl {
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Activate(::core::mem::transmute_copy(&window)).into()
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Deactivate(::core::mem::transmute_copy(&window)).into()
        }
        unsafe extern "system" fn RegisterHitTestTarget<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterHitTestTarget(::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&hittestwindow), ::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn ProcessInput<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *const super::super::UI::WindowsAndMessaging::MSG, handled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProcessInput(::core::mem::transmute_copy(&message)) {
                ::core::result::Result::Ok(ok__) => {
                    *handled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateManager<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetUpdateManager(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        unsafe extern "system" fn CreateViewport<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateViewport(::core::mem::transmute(&frameinfo), ::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        unsafe extern "system" fn CreateContent<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateContent(::core::mem::transmute(&frameinfo), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
            RegisterHitTestTarget: RegisterHitTestTarget::<Identity, Impl, OFFSET>,
            ProcessInput: ProcessInput::<Identity, Impl, OFFSET>,
            GetUpdateManager: GetUpdateManager::<Identity, Impl, OFFSET>,
            CreateViewport: CreateViewport::<Identity, Impl, OFFSET>,
            CreateContent: CreateContent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDirectManipulationManager2_Impl: Sized + IDirectManipulationManager_Impl {
    fn CreateBehavior(&mut self, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDirectManipulationManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationManager2_Impl, const OFFSET: isize>() -> IDirectManipulationManager2_Vtbl {
        unsafe extern "system" fn CreateBehavior<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateBehavior(::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        Self { base: IDirectManipulationManager_Vtbl::new::<Identity, Impl, OFFSET>(), CreateBehavior: CreateBehavior::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationManager2 as ::windows::core::Interface>::IID || iid == &<IDirectManipulationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDirectManipulationManager3_Impl: Sized + IDirectManipulationManager_Impl + IDirectManipulationManager2_Impl {
    fn GetService(&mut self, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDirectManipulationManager3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationManager3_Impl, const OFFSET: isize>() -> IDirectManipulationManager3_Vtbl {
        unsafe extern "system" fn GetService<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetService(::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        Self { base: IDirectManipulationManager2_Vtbl::new::<Identity, Impl, OFFSET>(), GetService: GetService::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationManager3 as ::windows::core::Interface>::IID || iid == &<IDirectManipulationManager as ::windows::core::Interface>::IID || iid == &<IDirectManipulationManager2 as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationPrimaryContent_Impl: Sized {
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
impl IDirectManipulationPrimaryContent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>() -> IDirectManipulationPrimaryContent_Vtbl {
        unsafe extern "system" fn SetSnapInterval<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSnapInterval(::core::mem::transmute_copy(&motion), ::core::mem::transmute_copy(&interval), ::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn SetSnapPoints<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSnapPoints(::core::mem::transmute_copy(&motion), ::core::mem::transmute_copy(&points), ::core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn SetSnapType<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSnapType(::core::mem::transmute_copy(&motion), ::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn SetSnapCoordinate<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSnapCoordinate(::core::mem::transmute_copy(&motion), ::core::mem::transmute_copy(&coordinate), ::core::mem::transmute_copy(&origin)).into()
        }
        unsafe extern "system" fn SetZoomBoundaries<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomminimum: f32, zoommaximum: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetZoomBoundaries(::core::mem::transmute_copy(&zoomminimum), ::core::mem::transmute_copy(&zoommaximum)).into()
        }
        unsafe extern "system" fn SetHorizontalAlignment<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHorizontalAlignment(::core::mem::transmute_copy(&alignment)).into()
        }
        unsafe extern "system" fn SetVerticalAlignment<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVerticalAlignment(::core::mem::transmute_copy(&alignment)).into()
        }
        unsafe extern "system" fn GetInertiaEndTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInertiaEndTransform(::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn GetCenterPoint<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: *mut f32, centery: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCenterPoint(::core::mem::transmute_copy(&centerx), ::core::mem::transmute_copy(&centery)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetSnapInterval: SetSnapInterval::<Identity, Impl, OFFSET>,
            SetSnapPoints: SetSnapPoints::<Identity, Impl, OFFSET>,
            SetSnapType: SetSnapType::<Identity, Impl, OFFSET>,
            SetSnapCoordinate: SetSnapCoordinate::<Identity, Impl, OFFSET>,
            SetZoomBoundaries: SetZoomBoundaries::<Identity, Impl, OFFSET>,
            SetHorizontalAlignment: SetHorizontalAlignment::<Identity, Impl, OFFSET>,
            SetVerticalAlignment: SetVerticalAlignment::<Identity, Impl, OFFSET>,
            GetInertiaEndTransform: GetInertiaEndTransform::<Identity, Impl, OFFSET>,
            GetCenterPoint: GetCenterPoint::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationPrimaryContent as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationUpdateHandler_Impl: Sized {
    fn Update(&mut self) -> ::windows::core::Result<()>;
}
impl IDirectManipulationUpdateHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationUpdateHandler_Impl, const OFFSET: isize>() -> IDirectManipulationUpdateHandler_Vtbl {
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationUpdateHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Update().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationUpdateHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectManipulationUpdateManager_Impl: Sized {
    fn RegisterWaitHandleCallback(&mut self, handle: super::super::Foundation::HANDLE, eventhandler: &::core::option::Option<IDirectManipulationUpdateHandler>) -> ::windows::core::Result<u32>;
    fn UnregisterWaitHandleCallback(&mut self, cookie: u32) -> ::windows::core::Result<()>;
    fn Update(&mut self, frameinfo: &::core::option::Option<IDirectManipulationFrameInfoProvider>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectManipulationUpdateManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationUpdateManager_Impl, const OFFSET: isize>() -> IDirectManipulationUpdateManager_Vtbl {
        unsafe extern "system" fn RegisterWaitHandleCallback<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationUpdateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, eventhandler: ::windows::core::RawPtr, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegisterWaitHandleCallback(::core::mem::transmute_copy(&handle), ::core::mem::transmute(&eventhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *cookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterWaitHandleCallback<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationUpdateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterWaitHandleCallback(::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationUpdateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Update(::core::mem::transmute(&frameinfo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterWaitHandleCallback: RegisterWaitHandleCallback::<Identity, Impl, OFFSET>,
            UnregisterWaitHandleCallback: UnregisterWaitHandleCallback::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationUpdateManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectManipulationViewport_Impl: Sized {
    fn Enable(&mut self) -> ::windows::core::Result<()>;
    fn Disable(&mut self) -> ::windows::core::Result<()>;
    fn SetContact(&mut self, pointerid: u32) -> ::windows::core::Result<()>;
    fn ReleaseContact(&mut self, pointerid: u32) -> ::windows::core::Result<()>;
    fn ReleaseAllContacts(&mut self) -> ::windows::core::Result<()>;
    fn GetStatus(&mut self) -> ::windows::core::Result<DIRECTMANIPULATION_STATUS>;
    fn GetTag(&mut self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::Result<()>;
    fn SetTag(&mut self, object: &::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<()>;
    fn GetViewportRect(&mut self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn SetViewportRect(&mut self, viewport: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn ZoomToRect(&mut self, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetViewportTransform(&mut self, matrix: *const f32, pointcount: u32) -> ::windows::core::Result<()>;
    fn SyncDisplayTransform(&mut self, matrix: *const f32, pointcount: u32) -> ::windows::core::Result<()>;
    fn GetPrimaryContent(&mut self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn AddContent(&mut self, content: &::core::option::Option<IDirectManipulationContent>) -> ::windows::core::Result<()>;
    fn RemoveContent(&mut self, content: &::core::option::Option<IDirectManipulationContent>) -> ::windows::core::Result<()>;
    fn SetViewportOptions(&mut self, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::Result<()>;
    fn AddConfiguration(&mut self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()>;
    fn RemoveConfiguration(&mut self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()>;
    fn ActivateConfiguration(&mut self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()>;
    fn SetManualGesture(&mut self, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::Result<()>;
    fn SetChaining(&mut self, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::Result<()>;
    fn AddEventHandler(&mut self, window: super::super::Foundation::HWND, eventhandler: &::core::option::Option<IDirectManipulationViewportEventHandler>) -> ::windows::core::Result<u32>;
    fn RemoveEventHandler(&mut self, cookie: u32) -> ::windows::core::Result<()>;
    fn SetInputMode(&mut self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()>;
    fn SetUpdateMode(&mut self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Abandon(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectManipulationViewport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>() -> IDirectManipulationViewport_Vtbl {
        unsafe extern "system" fn Enable<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Enable().into()
        }
        unsafe extern "system" fn Disable<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Disable().into()
        }
        unsafe extern "system" fn SetContact<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetContact(::core::mem::transmute_copy(&pointerid)).into()
        }
        unsafe extern "system" fn ReleaseContact<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseContact(::core::mem::transmute_copy(&pointerid)).into()
        }
        unsafe extern "system" fn ReleaseAllContacts<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseAllContacts().into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut DIRECTMANIPULATION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *status = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTag<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTag(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SetTag<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetViewportRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetViewportRect() {
                ::core::result::Result::Ok(ok__) => {
                    *viewport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewportRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetViewportRect(::core::mem::transmute_copy(&viewport)).into()
        }
        unsafe extern "system" fn ZoomToRect<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ZoomToRect(::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&top), ::core::mem::transmute_copy(&right), ::core::mem::transmute_copy(&bottom), ::core::mem::transmute_copy(&animate)).into()
        }
        unsafe extern "system" fn SetViewportTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetViewportTransform(::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn SyncDisplayTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SyncDisplayTransform(::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn GetPrimaryContent<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPrimaryContent(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        unsafe extern "system" fn AddContent<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddContent(::core::mem::transmute(&content)).into()
        }
        unsafe extern "system" fn RemoveContent<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveContent(::core::mem::transmute(&content)).into()
        }
        unsafe extern "system" fn SetViewportOptions<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetViewportOptions(::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn AddConfiguration<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddConfiguration(::core::mem::transmute_copy(&configuration)).into()
        }
        unsafe extern "system" fn RemoveConfiguration<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveConfiguration(::core::mem::transmute_copy(&configuration)).into()
        }
        unsafe extern "system" fn ActivateConfiguration<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ActivateConfiguration(::core::mem::transmute_copy(&configuration)).into()
        }
        unsafe extern "system" fn SetManualGesture<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetManualGesture(::core::mem::transmute_copy(&configuration)).into()
        }
        unsafe extern "system" fn SetChaining<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetChaining(::core::mem::transmute_copy(&enabledtypes)).into()
        }
        unsafe extern "system" fn AddEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, eventhandler: ::windows::core::RawPtr, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddEventHandler(::core::mem::transmute_copy(&window), ::core::mem::transmute(&eventhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *cookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEventHandler<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveEventHandler(::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn SetInputMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInputMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetUpdateMode<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUpdateMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Abandon<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Abandon().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Enable: Enable::<Identity, Impl, OFFSET>,
            Disable: Disable::<Identity, Impl, OFFSET>,
            SetContact: SetContact::<Identity, Impl, OFFSET>,
            ReleaseContact: ReleaseContact::<Identity, Impl, OFFSET>,
            ReleaseAllContacts: ReleaseAllContacts::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetTag: GetTag::<Identity, Impl, OFFSET>,
            SetTag: SetTag::<Identity, Impl, OFFSET>,
            GetViewportRect: GetViewportRect::<Identity, Impl, OFFSET>,
            SetViewportRect: SetViewportRect::<Identity, Impl, OFFSET>,
            ZoomToRect: ZoomToRect::<Identity, Impl, OFFSET>,
            SetViewportTransform: SetViewportTransform::<Identity, Impl, OFFSET>,
            SyncDisplayTransform: SyncDisplayTransform::<Identity, Impl, OFFSET>,
            GetPrimaryContent: GetPrimaryContent::<Identity, Impl, OFFSET>,
            AddContent: AddContent::<Identity, Impl, OFFSET>,
            RemoveContent: RemoveContent::<Identity, Impl, OFFSET>,
            SetViewportOptions: SetViewportOptions::<Identity, Impl, OFFSET>,
            AddConfiguration: AddConfiguration::<Identity, Impl, OFFSET>,
            RemoveConfiguration: RemoveConfiguration::<Identity, Impl, OFFSET>,
            ActivateConfiguration: ActivateConfiguration::<Identity, Impl, OFFSET>,
            SetManualGesture: SetManualGesture::<Identity, Impl, OFFSET>,
            SetChaining: SetChaining::<Identity, Impl, OFFSET>,
            AddEventHandler: AddEventHandler::<Identity, Impl, OFFSET>,
            RemoveEventHandler: RemoveEventHandler::<Identity, Impl, OFFSET>,
            SetInputMode: SetInputMode::<Identity, Impl, OFFSET>,
            SetUpdateMode: SetUpdateMode::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Abandon: Abandon::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationViewport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectManipulationViewport2_Impl: Sized + IDirectManipulationViewport_Impl {
    fn AddBehavior(&mut self, behavior: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<u32>;
    fn RemoveBehavior(&mut self, cookie: u32) -> ::windows::core::Result<()>;
    fn RemoveAllBehaviors(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectManipulationViewport2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport2_Impl, const OFFSET: isize>() -> IDirectManipulationViewport2_Vtbl {
        unsafe extern "system" fn AddBehavior<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, behavior: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddBehavior(::core::mem::transmute(&behavior)) {
                ::core::result::Result::Ok(ok__) => {
                    *cookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBehavior<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveBehavior(::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn RemoveAllBehaviors<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewport2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAllBehaviors().into()
        }
        Self {
            base: IDirectManipulationViewport_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddBehavior: AddBehavior::<Identity, Impl, OFFSET>,
            RemoveBehavior: RemoveBehavior::<Identity, Impl, OFFSET>,
            RemoveAllBehaviors: RemoveAllBehaviors::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationViewport2 as ::windows::core::Interface>::IID || iid == &<IDirectManipulationViewport as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationViewportEventHandler_Impl: Sized {
    fn OnViewportStatusChanged(&mut self, viewport: &::core::option::Option<IDirectManipulationViewport>, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows::core::Result<()>;
    fn OnViewportUpdated(&mut self, viewport: &::core::option::Option<IDirectManipulationViewport>) -> ::windows::core::Result<()>;
    fn OnContentUpdated(&mut self, viewport: &::core::option::Option<IDirectManipulationViewport>, content: &::core::option::Option<IDirectManipulationContent>) -> ::windows::core::Result<()>;
}
impl IDirectManipulationViewportEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewportEventHandler_Impl, const OFFSET: isize>() -> IDirectManipulationViewportEventHandler_Vtbl {
        unsafe extern "system" fn OnViewportStatusChanged<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewportEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnViewportStatusChanged(::core::mem::transmute(&viewport), ::core::mem::transmute_copy(&current), ::core::mem::transmute_copy(&previous)).into()
        }
        unsafe extern "system" fn OnViewportUpdated<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewportEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnViewportUpdated(::core::mem::transmute(&viewport)).into()
        }
        unsafe extern "system" fn OnContentUpdated<Identity: ::windows::core::IUnknownImpl, Impl: IDirectManipulationViewportEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnContentUpdated(::core::mem::transmute(&viewport), ::core::mem::transmute(&content)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnViewportStatusChanged: OnViewportStatusChanged::<Identity, Impl, OFFSET>,
            OnViewportUpdated: OnViewportUpdated::<Identity, Impl, OFFSET>,
            OnContentUpdated: OnContentUpdated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationViewportEventHandler as ::windows::core::Interface>::IID
    }
}
