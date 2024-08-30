pub trait IDirectManipulationAutoScrollBehavior_Impl: Sized + windows_core::IUnknownImpl {
    fn SetConfiguration(&self, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectManipulationAutoScrollBehavior {}
impl IDirectManipulationAutoScrollBehavior_Vtbl {
    pub const fn new<Identity: IDirectManipulationAutoScrollBehavior_Impl, const OFFSET: isize>() -> IDirectManipulationAutoScrollBehavior_Vtbl {
        unsafe extern "system" fn SetConfiguration<Identity: IDirectManipulationAutoScrollBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationAutoScrollBehavior_Impl::SetConfiguration(this, core::mem::transmute_copy(&motiontypes), core::mem::transmute_copy(&scrollmotion)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetConfiguration: SetConfiguration::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationAutoScrollBehavior as windows_core::Interface>::IID
    }
}
pub trait IDirectManipulationCompositor_Impl: Sized + windows_core::IUnknownImpl {
    fn AddContent(&self, content: Option<&IDirectManipulationContent>, device: Option<&windows_core::IUnknown>, parentvisual: Option<&windows_core::IUnknown>, childvisual: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RemoveContent(&self, content: Option<&IDirectManipulationContent>) -> windows_core::Result<()>;
    fn SetUpdateManager(&self, updatemanager: Option<&IDirectManipulationUpdateManager>) -> windows_core::Result<()>;
    fn Flush(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectManipulationCompositor {}
impl IDirectManipulationCompositor_Vtbl {
    pub const fn new<Identity: IDirectManipulationCompositor_Impl, const OFFSET: isize>() -> IDirectManipulationCompositor_Vtbl {
        unsafe extern "system" fn AddContent<Identity: IDirectManipulationCompositor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, content: *mut core::ffi::c_void, device: *mut core::ffi::c_void, parentvisual: *mut core::ffi::c_void, childvisual: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationCompositor_Impl::AddContent(this, windows_core::from_raw_borrowed(&content), windows_core::from_raw_borrowed(&device), windows_core::from_raw_borrowed(&parentvisual), windows_core::from_raw_borrowed(&childvisual)).into()
        }
        unsafe extern "system" fn RemoveContent<Identity: IDirectManipulationCompositor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, content: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationCompositor_Impl::RemoveContent(this, windows_core::from_raw_borrowed(&content)).into()
        }
        unsafe extern "system" fn SetUpdateManager<Identity: IDirectManipulationCompositor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updatemanager: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationCompositor_Impl::SetUpdateManager(this, windows_core::from_raw_borrowed(&updatemanager)).into()
        }
        unsafe extern "system" fn Flush<Identity: IDirectManipulationCompositor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationCompositor_Impl::Flush(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddContent: AddContent::<Identity, OFFSET>,
            RemoveContent: RemoveContent::<Identity, OFFSET>,
            SetUpdateManager: SetUpdateManager::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationCompositor as windows_core::Interface>::IID
    }
}
pub trait IDirectManipulationCompositor2_Impl: Sized + IDirectManipulationCompositor_Impl {
    fn AddContentWithCrossProcessChaining(&self, content: Option<&IDirectManipulationPrimaryContent>, device: Option<&windows_core::IUnknown>, parentvisual: Option<&windows_core::IUnknown>, childvisual: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectManipulationCompositor2 {}
impl IDirectManipulationCompositor2_Vtbl {
    pub const fn new<Identity: IDirectManipulationCompositor2_Impl, const OFFSET: isize>() -> IDirectManipulationCompositor2_Vtbl {
        unsafe extern "system" fn AddContentWithCrossProcessChaining<Identity: IDirectManipulationCompositor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, content: *mut core::ffi::c_void, device: *mut core::ffi::c_void, parentvisual: *mut core::ffi::c_void, childvisual: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationCompositor2_Impl::AddContentWithCrossProcessChaining(this, windows_core::from_raw_borrowed(&content), windows_core::from_raw_borrowed(&device), windows_core::from_raw_borrowed(&parentvisual), windows_core::from_raw_borrowed(&childvisual)).into()
        }
        Self {
            base__: IDirectManipulationCompositor_Vtbl::new::<Identity, OFFSET>(),
            AddContentWithCrossProcessChaining: AddContentWithCrossProcessChaining::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationCompositor2 as windows_core::Interface>::IID || iid == &<IDirectManipulationCompositor as windows_core::Interface>::IID
    }
}
pub trait IDirectManipulationContent_Impl: Sized + windows_core::IUnknownImpl {
    fn GetContentRect(&self) -> windows_core::Result<super::super::Foundation::RECT>;
    fn SetContentRect(&self, contentsize: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn GetViewport(&self, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetTag(&self, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void, id: *mut u32) -> windows_core::Result<()>;
    fn SetTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<()>;
    fn GetOutputTransform(&self, matrix: *mut f32, pointcount: u32) -> windows_core::Result<()>;
    fn GetContentTransform(&self, matrix: *mut f32, pointcount: u32) -> windows_core::Result<()>;
    fn SyncContentTransform(&self, matrix: *const f32, pointcount: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectManipulationContent {}
impl IDirectManipulationContent_Vtbl {
    pub const fn new<Identity: IDirectManipulationContent_Impl, const OFFSET: isize>() -> IDirectManipulationContent_Vtbl {
        unsafe extern "system" fn GetContentRect<Identity: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentsize: *mut super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectManipulationContent_Impl::GetContentRect(this) {
                Ok(ok__) => {
                    contentsize.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentRect<Identity: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentsize: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationContent_Impl::SetContentRect(this, core::mem::transmute_copy(&contentsize)).into()
        }
        unsafe extern "system" fn GetViewport<Identity: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationContent_Impl::GetViewport(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&object)).into()
        }
        unsafe extern "system" fn GetTag<Identity: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void, id: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationContent_Impl::GetTag(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SetTag<Identity: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationContent_Impl::SetTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetOutputTransform<Identity: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationContent_Impl::GetOutputTransform(this, core::mem::transmute_copy(&matrix), core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn GetContentTransform<Identity: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationContent_Impl::GetContentTransform(this, core::mem::transmute_copy(&matrix), core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn SyncContentTransform<Identity: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const f32, pointcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationContent_Impl::SyncContentTransform(this, core::mem::transmute_copy(&matrix), core::mem::transmute_copy(&pointcount)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetContentRect: GetContentRect::<Identity, OFFSET>,
            SetContentRect: SetContentRect::<Identity, OFFSET>,
            GetViewport: GetViewport::<Identity, OFFSET>,
            GetTag: GetTag::<Identity, OFFSET>,
            SetTag: SetTag::<Identity, OFFSET>,
            GetOutputTransform: GetOutputTransform::<Identity, OFFSET>,
            GetContentTransform: GetContentTransform::<Identity, OFFSET>,
            SyncContentTransform: SyncContentTransform::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationContent as windows_core::Interface>::IID
    }
}
pub trait IDirectManipulationDeferContactService_Impl: Sized + windows_core::IUnknownImpl {
    fn DeferContact(&self, pointerid: u32, timeout: u32) -> windows_core::Result<()>;
    fn CancelContact(&self, pointerid: u32) -> windows_core::Result<()>;
    fn CancelDeferral(&self, pointerid: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectManipulationDeferContactService {}
impl IDirectManipulationDeferContactService_Vtbl {
    pub const fn new<Identity: IDirectManipulationDeferContactService_Impl, const OFFSET: isize>() -> IDirectManipulationDeferContactService_Vtbl {
        unsafe extern "system" fn DeferContact<Identity: IDirectManipulationDeferContactService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointerid: u32, timeout: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationDeferContactService_Impl::DeferContact(this, core::mem::transmute_copy(&pointerid), core::mem::transmute_copy(&timeout)).into()
        }
        unsafe extern "system" fn CancelContact<Identity: IDirectManipulationDeferContactService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointerid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationDeferContactService_Impl::CancelContact(this, core::mem::transmute_copy(&pointerid)).into()
        }
        unsafe extern "system" fn CancelDeferral<Identity: IDirectManipulationDeferContactService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointerid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationDeferContactService_Impl::CancelDeferral(this, core::mem::transmute_copy(&pointerid)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DeferContact: DeferContact::<Identity, OFFSET>,
            CancelContact: CancelContact::<Identity, OFFSET>,
            CancelDeferral: CancelDeferral::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationDeferContactService as windows_core::Interface>::IID
    }
}
pub trait IDirectManipulationDragDropBehavior_Impl: Sized + windows_core::IUnknownImpl {
    fn SetConfiguration(&self, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<DIRECTMANIPULATION_DRAG_DROP_STATUS>;
}
impl windows_core::RuntimeName for IDirectManipulationDragDropBehavior {}
impl IDirectManipulationDragDropBehavior_Vtbl {
    pub const fn new<Identity: IDirectManipulationDragDropBehavior_Impl, const OFFSET: isize>() -> IDirectManipulationDragDropBehavior_Vtbl {
        unsafe extern "system" fn SetConfiguration<Identity: IDirectManipulationDragDropBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationDragDropBehavior_Impl::SetConfiguration(this, core::mem::transmute_copy(&configuration)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: IDirectManipulationDragDropBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut DIRECTMANIPULATION_DRAG_DROP_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectManipulationDragDropBehavior_Impl::GetStatus(this) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetConfiguration: SetConfiguration::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationDragDropBehavior as windows_core::Interface>::IID
    }
}
pub trait IDirectManipulationDragDropEventHandler_Impl: Sized + windows_core::IUnknownImpl {
    fn OnDragDropStatusChange(&self, viewport: Option<&IDirectManipulationViewport2>, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectManipulationDragDropEventHandler {}
impl IDirectManipulationDragDropEventHandler_Vtbl {
    pub const fn new<Identity: IDirectManipulationDragDropEventHandler_Impl, const OFFSET: isize>() -> IDirectManipulationDragDropEventHandler_Vtbl {
        unsafe extern "system" fn OnDragDropStatusChange<Identity: IDirectManipulationDragDropEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewport: *mut core::ffi::c_void, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationDragDropEventHandler_Impl::OnDragDropStatusChange(this, windows_core::from_raw_borrowed(&viewport), core::mem::transmute_copy(&current), core::mem::transmute_copy(&previous)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnDragDropStatusChange: OnDragDropStatusChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationDragDropEventHandler as windows_core::Interface>::IID
    }
}
pub trait IDirectManipulationFrameInfoProvider_Impl: Sized + windows_core::IUnknownImpl {
    fn GetNextFrameInfo(&self, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectManipulationFrameInfoProvider {}
impl IDirectManipulationFrameInfoProvider_Vtbl {
    pub const fn new<Identity: IDirectManipulationFrameInfoProvider_Impl, const OFFSET: isize>() -> IDirectManipulationFrameInfoProvider_Vtbl {
        unsafe extern "system" fn GetNextFrameInfo<Identity: IDirectManipulationFrameInfoProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationFrameInfoProvider_Impl::GetNextFrameInfo(this, core::mem::transmute_copy(&time), core::mem::transmute_copy(&processtime), core::mem::transmute_copy(&compositiontime)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetNextFrameInfo: GetNextFrameInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationFrameInfoProvider as windows_core::Interface>::IID
    }
}
pub trait IDirectManipulationInteractionEventHandler_Impl: Sized + windows_core::IUnknownImpl {
    fn OnInteraction(&self, viewport: Option<&IDirectManipulationViewport2>, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectManipulationInteractionEventHandler {}
impl IDirectManipulationInteractionEventHandler_Vtbl {
    pub const fn new<Identity: IDirectManipulationInteractionEventHandler_Impl, const OFFSET: isize>() -> IDirectManipulationInteractionEventHandler_Vtbl {
        unsafe extern "system" fn OnInteraction<Identity: IDirectManipulationInteractionEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewport: *mut core::ffi::c_void, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationInteractionEventHandler_Impl::OnInteraction(this, windows_core::from_raw_borrowed(&viewport), core::mem::transmute_copy(&interaction)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnInteraction: OnInteraction::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationInteractionEventHandler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IDirectManipulationManager_Impl: Sized + windows_core::IUnknownImpl {
    fn Activate(&self, window: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn Deactivate(&self, window: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn RegisterHitTestTarget(&self, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> windows_core::Result<()>;
    fn ProcessInput(&self, message: *const super::super::UI::WindowsAndMessaging::MSG) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetUpdateManager(&self, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateViewport(&self, frameinfo: Option<&IDirectManipulationFrameInfoProvider>, window: super::super::Foundation::HWND, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateContent(&self, frameinfo: Option<&IDirectManipulationFrameInfoProvider>, clsid: *const windows_core::GUID, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IDirectManipulationManager {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IDirectManipulationManager_Vtbl {
    pub const fn new<Identity: IDirectManipulationManager_Impl, const OFFSET: isize>() -> IDirectManipulationManager_Vtbl {
        unsafe extern "system" fn Activate<Identity: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationManager_Impl::Activate(this, core::mem::transmute_copy(&window)).into()
        }
        unsafe extern "system" fn Deactivate<Identity: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::super::Foundation::HWND) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationManager_Impl::Deactivate(this, core::mem::transmute_copy(&window)).into()
        }
        unsafe extern "system" fn RegisterHitTestTarget<Identity: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationManager_Impl::RegisterHitTestTarget(this, core::mem::transmute_copy(&window), core::mem::transmute_copy(&hittestwindow), core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn ProcessInput<Identity: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: *const super::super::UI::WindowsAndMessaging::MSG, handled: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectManipulationManager_Impl::ProcessInput(this, core::mem::transmute_copy(&message)) {
                Ok(ok__) => {
                    handled.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateManager<Identity: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationManager_Impl::GetUpdateManager(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&object)).into()
        }
        unsafe extern "system" fn CreateViewport<Identity: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frameinfo: *mut core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationManager_Impl::CreateViewport(this, windows_core::from_raw_borrowed(&frameinfo), core::mem::transmute_copy(&window), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&object)).into()
        }
        unsafe extern "system" fn CreateContent<Identity: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frameinfo: *mut core::ffi::c_void, clsid: *const windows_core::GUID, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationManager_Impl::CreateContent(this, windows_core::from_raw_borrowed(&frameinfo), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&object)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, OFFSET>,
            Deactivate: Deactivate::<Identity, OFFSET>,
            RegisterHitTestTarget: RegisterHitTestTarget::<Identity, OFFSET>,
            ProcessInput: ProcessInput::<Identity, OFFSET>,
            GetUpdateManager: GetUpdateManager::<Identity, OFFSET>,
            CreateViewport: CreateViewport::<Identity, OFFSET>,
            CreateContent: CreateContent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IDirectManipulationManager2_Impl: Sized + IDirectManipulationManager_Impl {
    fn CreateBehavior(&self, clsid: *const windows_core::GUID, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IDirectManipulationManager2 {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IDirectManipulationManager2_Vtbl {
    pub const fn new<Identity: IDirectManipulationManager2_Impl, const OFFSET: isize>() -> IDirectManipulationManager2_Vtbl {
        unsafe extern "system" fn CreateBehavior<Identity: IDirectManipulationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationManager2_Impl::CreateBehavior(this, core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&object)).into()
        }
        Self { base__: IDirectManipulationManager_Vtbl::new::<Identity, OFFSET>(), CreateBehavior: CreateBehavior::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationManager2 as windows_core::Interface>::IID || iid == &<IDirectManipulationManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IDirectManipulationManager3_Impl: Sized + IDirectManipulationManager2_Impl {
    fn GetService(&self, clsid: *const windows_core::GUID, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for IDirectManipulationManager3 {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl IDirectManipulationManager3_Vtbl {
    pub const fn new<Identity: IDirectManipulationManager3_Impl, const OFFSET: isize>() -> IDirectManipulationManager3_Vtbl {
        unsafe extern "system" fn GetService<Identity: IDirectManipulationManager3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationManager3_Impl::GetService(this, core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&object)).into()
        }
        Self { base__: IDirectManipulationManager2_Vtbl::new::<Identity, OFFSET>(), GetService: GetService::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationManager3 as windows_core::Interface>::IID || iid == &<IDirectManipulationManager as windows_core::Interface>::IID || iid == &<IDirectManipulationManager2 as windows_core::Interface>::IID
    }
}
pub trait IDirectManipulationPrimaryContent_Impl: Sized + windows_core::IUnknownImpl {
    fn SetSnapInterval(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> windows_core::Result<()>;
    fn SetSnapPoints(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> windows_core::Result<()>;
    fn SetSnapType(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> windows_core::Result<()>;
    fn SetSnapCoordinate(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> windows_core::Result<()>;
    fn SetZoomBoundaries(&self, zoomminimum: f32, zoommaximum: f32) -> windows_core::Result<()>;
    fn SetHorizontalAlignment(&self, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> windows_core::Result<()>;
    fn SetVerticalAlignment(&self, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> windows_core::Result<()>;
    fn GetInertiaEndTransform(&self, matrix: *mut f32, pointcount: u32) -> windows_core::Result<()>;
    fn GetCenterPoint(&self, centerx: *mut f32, centery: *mut f32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectManipulationPrimaryContent {}
impl IDirectManipulationPrimaryContent_Vtbl {
    pub const fn new<Identity: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>() -> IDirectManipulationPrimaryContent_Vtbl {
        unsafe extern "system" fn SetSnapInterval<Identity: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationPrimaryContent_Impl::SetSnapInterval(this, core::mem::transmute_copy(&motion), core::mem::transmute_copy(&interval), core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn SetSnapPoints<Identity: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationPrimaryContent_Impl::SetSnapPoints(this, core::mem::transmute_copy(&motion), core::mem::transmute_copy(&points), core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn SetSnapType<Identity: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationPrimaryContent_Impl::SetSnapType(this, core::mem::transmute_copy(&motion), core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn SetSnapCoordinate<Identity: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationPrimaryContent_Impl::SetSnapCoordinate(this, core::mem::transmute_copy(&motion), core::mem::transmute_copy(&coordinate), core::mem::transmute_copy(&origin)).into()
        }
        unsafe extern "system" fn SetZoomBoundaries<Identity: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, zoomminimum: f32, zoommaximum: f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationPrimaryContent_Impl::SetZoomBoundaries(this, core::mem::transmute_copy(&zoomminimum), core::mem::transmute_copy(&zoommaximum)).into()
        }
        unsafe extern "system" fn SetHorizontalAlignment<Identity: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationPrimaryContent_Impl::SetHorizontalAlignment(this, core::mem::transmute_copy(&alignment)).into()
        }
        unsafe extern "system" fn SetVerticalAlignment<Identity: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationPrimaryContent_Impl::SetVerticalAlignment(this, core::mem::transmute_copy(&alignment)).into()
        }
        unsafe extern "system" fn GetInertiaEndTransform<Identity: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationPrimaryContent_Impl::GetInertiaEndTransform(this, core::mem::transmute_copy(&matrix), core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn GetCenterPoint<Identity: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, centerx: *mut f32, centery: *mut f32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationPrimaryContent_Impl::GetCenterPoint(this, core::mem::transmute_copy(&centerx), core::mem::transmute_copy(&centery)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetSnapInterval: SetSnapInterval::<Identity, OFFSET>,
            SetSnapPoints: SetSnapPoints::<Identity, OFFSET>,
            SetSnapType: SetSnapType::<Identity, OFFSET>,
            SetSnapCoordinate: SetSnapCoordinate::<Identity, OFFSET>,
            SetZoomBoundaries: SetZoomBoundaries::<Identity, OFFSET>,
            SetHorizontalAlignment: SetHorizontalAlignment::<Identity, OFFSET>,
            SetVerticalAlignment: SetVerticalAlignment::<Identity, OFFSET>,
            GetInertiaEndTransform: GetInertiaEndTransform::<Identity, OFFSET>,
            GetCenterPoint: GetCenterPoint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationPrimaryContent as windows_core::Interface>::IID
    }
}
pub trait IDirectManipulationUpdateHandler_Impl: Sized + windows_core::IUnknownImpl {
    fn Update(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectManipulationUpdateHandler {}
impl IDirectManipulationUpdateHandler_Vtbl {
    pub const fn new<Identity: IDirectManipulationUpdateHandler_Impl, const OFFSET: isize>() -> IDirectManipulationUpdateHandler_Vtbl {
        unsafe extern "system" fn Update<Identity: IDirectManipulationUpdateHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationUpdateHandler_Impl::Update(this).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationUpdateHandler as windows_core::Interface>::IID
    }
}
pub trait IDirectManipulationUpdateManager_Impl: Sized + windows_core::IUnknownImpl {
    fn RegisterWaitHandleCallback(&self, handle: super::super::Foundation::HANDLE, eventhandler: Option<&IDirectManipulationUpdateHandler>) -> windows_core::Result<u32>;
    fn UnregisterWaitHandleCallback(&self, cookie: u32) -> windows_core::Result<()>;
    fn Update(&self, frameinfo: Option<&IDirectManipulationFrameInfoProvider>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectManipulationUpdateManager {}
impl IDirectManipulationUpdateManager_Vtbl {
    pub const fn new<Identity: IDirectManipulationUpdateManager_Impl, const OFFSET: isize>() -> IDirectManipulationUpdateManager_Vtbl {
        unsafe extern "system" fn RegisterWaitHandleCallback<Identity: IDirectManipulationUpdateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: super::super::Foundation::HANDLE, eventhandler: *mut core::ffi::c_void, cookie: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectManipulationUpdateManager_Impl::RegisterWaitHandleCallback(this, core::mem::transmute_copy(&handle), windows_core::from_raw_borrowed(&eventhandler)) {
                Ok(ok__) => {
                    cookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterWaitHandleCallback<Identity: IDirectManipulationUpdateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationUpdateManager_Impl::UnregisterWaitHandleCallback(this, core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn Update<Identity: IDirectManipulationUpdateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frameinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationUpdateManager_Impl::Update(this, windows_core::from_raw_borrowed(&frameinfo)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterWaitHandleCallback: RegisterWaitHandleCallback::<Identity, OFFSET>,
            UnregisterWaitHandleCallback: UnregisterWaitHandleCallback::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationUpdateManager as windows_core::Interface>::IID
    }
}
pub trait IDirectManipulationViewport_Impl: Sized + windows_core::IUnknownImpl {
    fn Enable(&self) -> windows_core::Result<()>;
    fn Disable(&self) -> windows_core::Result<()>;
    fn SetContact(&self, pointerid: u32) -> windows_core::Result<()>;
    fn ReleaseContact(&self, pointerid: u32) -> windows_core::Result<()>;
    fn ReleaseAllContacts(&self) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<DIRECTMANIPULATION_STATUS>;
    fn GetTag(&self, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void, id: *mut u32) -> windows_core::Result<()>;
    fn SetTag(&self, object: Option<&windows_core::IUnknown>, id: u32) -> windows_core::Result<()>;
    fn GetViewportRect(&self) -> windows_core::Result<super::super::Foundation::RECT>;
    fn SetViewportRect(&self, viewport: *const super::super::Foundation::RECT) -> windows_core::Result<()>;
    fn ZoomToRect(&self, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SetViewportTransform(&self, matrix: *const f32, pointcount: u32) -> windows_core::Result<()>;
    fn SyncDisplayTransform(&self, matrix: *const f32, pointcount: u32) -> windows_core::Result<()>;
    fn GetPrimaryContent(&self, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddContent(&self, content: Option<&IDirectManipulationContent>) -> windows_core::Result<()>;
    fn RemoveContent(&self, content: Option<&IDirectManipulationContent>) -> windows_core::Result<()>;
    fn SetViewportOptions(&self, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> windows_core::Result<()>;
    fn AddConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> windows_core::Result<()>;
    fn RemoveConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> windows_core::Result<()>;
    fn ActivateConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> windows_core::Result<()>;
    fn SetManualGesture(&self, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> windows_core::Result<()>;
    fn SetChaining(&self, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> windows_core::Result<()>;
    fn AddEventHandler(&self, window: super::super::Foundation::HWND, eventhandler: Option<&IDirectManipulationViewportEventHandler>) -> windows_core::Result<u32>;
    fn RemoveEventHandler(&self, cookie: u32) -> windows_core::Result<()>;
    fn SetInputMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> windows_core::Result<()>;
    fn SetUpdateMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Abandon(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectManipulationViewport {}
impl IDirectManipulationViewport_Vtbl {
    pub const fn new<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>() -> IDirectManipulationViewport_Vtbl {
        unsafe extern "system" fn Enable<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::Enable(this).into()
        }
        unsafe extern "system" fn Disable<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::Disable(this).into()
        }
        unsafe extern "system" fn SetContact<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointerid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::SetContact(this, core::mem::transmute_copy(&pointerid)).into()
        }
        unsafe extern "system" fn ReleaseContact<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointerid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::ReleaseContact(this, core::mem::transmute_copy(&pointerid)).into()
        }
        unsafe extern "system" fn ReleaseAllContacts<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::ReleaseAllContacts(this).into()
        }
        unsafe extern "system" fn GetStatus<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut DIRECTMANIPULATION_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectManipulationViewport_Impl::GetStatus(this) {
                Ok(ok__) => {
                    status.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTag<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void, id: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::GetTag(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SetTag<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::SetTag(this, windows_core::from_raw_borrowed(&object), core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetViewportRect<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewport: *mut super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectManipulationViewport_Impl::GetViewportRect(this) {
                Ok(ok__) => {
                    viewport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewportRect<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewport: *const super::super::Foundation::RECT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::SetViewportRect(this, core::mem::transmute_copy(&viewport)).into()
        }
        unsafe extern "system" fn ZoomToRect<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::ZoomToRect(this, core::mem::transmute_copy(&left), core::mem::transmute_copy(&top), core::mem::transmute_copy(&right), core::mem::transmute_copy(&bottom), core::mem::transmute_copy(&animate)).into()
        }
        unsafe extern "system" fn SetViewportTransform<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const f32, pointcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::SetViewportTransform(this, core::mem::transmute_copy(&matrix), core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn SyncDisplayTransform<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const f32, pointcount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::SyncDisplayTransform(this, core::mem::transmute_copy(&matrix), core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn GetPrimaryContent<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::GetPrimaryContent(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&object)).into()
        }
        unsafe extern "system" fn AddContent<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, content: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::AddContent(this, windows_core::from_raw_borrowed(&content)).into()
        }
        unsafe extern "system" fn RemoveContent<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, content: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::RemoveContent(this, windows_core::from_raw_borrowed(&content)).into()
        }
        unsafe extern "system" fn SetViewportOptions<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::SetViewportOptions(this, core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn AddConfiguration<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::AddConfiguration(this, core::mem::transmute_copy(&configuration)).into()
        }
        unsafe extern "system" fn RemoveConfiguration<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::RemoveConfiguration(this, core::mem::transmute_copy(&configuration)).into()
        }
        unsafe extern "system" fn ActivateConfiguration<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::ActivateConfiguration(this, core::mem::transmute_copy(&configuration)).into()
        }
        unsafe extern "system" fn SetManualGesture<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::SetManualGesture(this, core::mem::transmute_copy(&configuration)).into()
        }
        unsafe extern "system" fn SetChaining<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::SetChaining(this, core::mem::transmute_copy(&enabledtypes)).into()
        }
        unsafe extern "system" fn AddEventHandler<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::super::Foundation::HWND, eventhandler: *mut core::ffi::c_void, cookie: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectManipulationViewport_Impl::AddEventHandler(this, core::mem::transmute_copy(&window), windows_core::from_raw_borrowed(&eventhandler)) {
                Ok(ok__) => {
                    cookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEventHandler<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::RemoveEventHandler(this, core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn SetInputMode<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::SetInputMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetUpdateMode<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::SetUpdateMode(this, core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn Stop<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::Stop(this).into()
        }
        unsafe extern "system" fn Abandon<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport_Impl::Abandon(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Enable: Enable::<Identity, OFFSET>,
            Disable: Disable::<Identity, OFFSET>,
            SetContact: SetContact::<Identity, OFFSET>,
            ReleaseContact: ReleaseContact::<Identity, OFFSET>,
            ReleaseAllContacts: ReleaseAllContacts::<Identity, OFFSET>,
            GetStatus: GetStatus::<Identity, OFFSET>,
            GetTag: GetTag::<Identity, OFFSET>,
            SetTag: SetTag::<Identity, OFFSET>,
            GetViewportRect: GetViewportRect::<Identity, OFFSET>,
            SetViewportRect: SetViewportRect::<Identity, OFFSET>,
            ZoomToRect: ZoomToRect::<Identity, OFFSET>,
            SetViewportTransform: SetViewportTransform::<Identity, OFFSET>,
            SyncDisplayTransform: SyncDisplayTransform::<Identity, OFFSET>,
            GetPrimaryContent: GetPrimaryContent::<Identity, OFFSET>,
            AddContent: AddContent::<Identity, OFFSET>,
            RemoveContent: RemoveContent::<Identity, OFFSET>,
            SetViewportOptions: SetViewportOptions::<Identity, OFFSET>,
            AddConfiguration: AddConfiguration::<Identity, OFFSET>,
            RemoveConfiguration: RemoveConfiguration::<Identity, OFFSET>,
            ActivateConfiguration: ActivateConfiguration::<Identity, OFFSET>,
            SetManualGesture: SetManualGesture::<Identity, OFFSET>,
            SetChaining: SetChaining::<Identity, OFFSET>,
            AddEventHandler: AddEventHandler::<Identity, OFFSET>,
            RemoveEventHandler: RemoveEventHandler::<Identity, OFFSET>,
            SetInputMode: SetInputMode::<Identity, OFFSET>,
            SetUpdateMode: SetUpdateMode::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Abandon: Abandon::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationViewport as windows_core::Interface>::IID
    }
}
pub trait IDirectManipulationViewport2_Impl: Sized + IDirectManipulationViewport_Impl {
    fn AddBehavior(&self, behavior: Option<&windows_core::IUnknown>) -> windows_core::Result<u32>;
    fn RemoveBehavior(&self, cookie: u32) -> windows_core::Result<()>;
    fn RemoveAllBehaviors(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectManipulationViewport2 {}
impl IDirectManipulationViewport2_Vtbl {
    pub const fn new<Identity: IDirectManipulationViewport2_Impl, const OFFSET: isize>() -> IDirectManipulationViewport2_Vtbl {
        unsafe extern "system" fn AddBehavior<Identity: IDirectManipulationViewport2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, behavior: *mut core::ffi::c_void, cookie: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDirectManipulationViewport2_Impl::AddBehavior(this, windows_core::from_raw_borrowed(&behavior)) {
                Ok(ok__) => {
                    cookie.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBehavior<Identity: IDirectManipulationViewport2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport2_Impl::RemoveBehavior(this, core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn RemoveAllBehaviors<Identity: IDirectManipulationViewport2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewport2_Impl::RemoveAllBehaviors(this).into()
        }
        Self {
            base__: IDirectManipulationViewport_Vtbl::new::<Identity, OFFSET>(),
            AddBehavior: AddBehavior::<Identity, OFFSET>,
            RemoveBehavior: RemoveBehavior::<Identity, OFFSET>,
            RemoveAllBehaviors: RemoveAllBehaviors::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationViewport2 as windows_core::Interface>::IID || iid == &<IDirectManipulationViewport as windows_core::Interface>::IID
    }
}
pub trait IDirectManipulationViewportEventHandler_Impl: Sized + windows_core::IUnknownImpl {
    fn OnViewportStatusChanged(&self, viewport: Option<&IDirectManipulationViewport>, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> windows_core::Result<()>;
    fn OnViewportUpdated(&self, viewport: Option<&IDirectManipulationViewport>) -> windows_core::Result<()>;
    fn OnContentUpdated(&self, viewport: Option<&IDirectManipulationViewport>, content: Option<&IDirectManipulationContent>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDirectManipulationViewportEventHandler {}
impl IDirectManipulationViewportEventHandler_Vtbl {
    pub const fn new<Identity: IDirectManipulationViewportEventHandler_Impl, const OFFSET: isize>() -> IDirectManipulationViewportEventHandler_Vtbl {
        unsafe extern "system" fn OnViewportStatusChanged<Identity: IDirectManipulationViewportEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewport: *mut core::ffi::c_void, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewportEventHandler_Impl::OnViewportStatusChanged(this, windows_core::from_raw_borrowed(&viewport), core::mem::transmute_copy(&current), core::mem::transmute_copy(&previous)).into()
        }
        unsafe extern "system" fn OnViewportUpdated<Identity: IDirectManipulationViewportEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewport: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewportEventHandler_Impl::OnViewportUpdated(this, windows_core::from_raw_borrowed(&viewport)).into()
        }
        unsafe extern "system" fn OnContentUpdated<Identity: IDirectManipulationViewportEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewport: *mut core::ffi::c_void, content: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDirectManipulationViewportEventHandler_Impl::OnContentUpdated(this, windows_core::from_raw_borrowed(&viewport), windows_core::from_raw_borrowed(&content)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnViewportStatusChanged: OnViewportStatusChanged::<Identity, OFFSET>,
            OnViewportUpdated: OnViewportUpdated::<Identity, OFFSET>,
            OnContentUpdated: OnContentUpdated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationViewportEventHandler as windows_core::Interface>::IID
    }
}
