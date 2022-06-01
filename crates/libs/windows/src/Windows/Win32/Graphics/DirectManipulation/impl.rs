pub trait IDirectManipulationAutoScrollBehavior_Impl: Sized {
    fn SetConfiguration(&self, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDirectManipulationAutoScrollBehavior {}
impl IDirectManipulationAutoScrollBehavior_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationAutoScrollBehavior_Impl, const OFFSET: isize>() -> IDirectManipulationAutoScrollBehavior_Vtbl {
        unsafe extern "system" fn SetConfiguration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationAutoScrollBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetConfiguration(::core::mem::transmute_copy(&motiontypes), ::core::mem::transmute_copy(&scrollmotion)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetConfiguration: SetConfiguration::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationAutoScrollBehavior as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationCompositor_Impl: Sized {
    fn AddContent(&self, content: &::core::option::Option<IDirectManipulationContent>, device: &::core::option::Option<::windows::core::IUnknown>, parentvisual: &::core::option::Option<::windows::core::IUnknown>, childvisual: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn RemoveContent(&self, content: &::core::option::Option<IDirectManipulationContent>) -> ::windows::core::Result<()>;
    fn SetUpdateManager(&self, updatemanager: &::core::option::Option<IDirectManipulationUpdateManager>) -> ::windows::core::Result<()>;
    fn Flush(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDirectManipulationCompositor {}
impl IDirectManipulationCompositor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationCompositor_Impl, const OFFSET: isize>() -> IDirectManipulationCompositor_Vtbl {
        unsafe extern "system" fn AddContent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationCompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddContent(::core::mem::transmute(&content), ::core::mem::transmute(&device), ::core::mem::transmute(&parentvisual), ::core::mem::transmute(&childvisual)).into()
        }
        unsafe extern "system" fn RemoveContent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationCompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveContent(::core::mem::transmute(&content)).into()
        }
        unsafe extern "system" fn SetUpdateManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationCompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, updatemanager: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUpdateManager(::core::mem::transmute(&updatemanager)).into()
        }
        unsafe extern "system" fn Flush<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationCompositor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Flush().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn AddContentWithCrossProcessChaining(&self, content: &::core::option::Option<IDirectManipulationPrimaryContent>, device: &::core::option::Option<::windows::core::IUnknown>, parentvisual: &::core::option::Option<::windows::core::IUnknown>, childvisual: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDirectManipulationCompositor2 {}
impl IDirectManipulationCompositor2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationCompositor2_Impl, const OFFSET: isize>() -> IDirectManipulationCompositor2_Vtbl {
        unsafe extern "system" fn AddContentWithCrossProcessChaining<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationCompositor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddContentWithCrossProcessChaining(::core::mem::transmute(&content), ::core::mem::transmute(&device), ::core::mem::transmute(&parentvisual), ::core::mem::transmute(&childvisual)).into()
        }
        Self {
            base__: IDirectManipulationCompositor_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddContentWithCrossProcessChaining: AddContentWithCrossProcessChaining::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationCompositor2 as ::windows::core::Interface>::IID || iid == &<IDirectManipulationCompositor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectManipulationContent_Impl: Sized {
    fn GetContentRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn SetContentRect(&self, contentsize: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn GetViewport(&self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetTag(&self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::Result<()>;
    fn SetTag(&self, object: &::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<()>;
    fn GetOutputTransform(&self, matrix: *mut f32, pointcount: u32) -> ::windows::core::Result<()>;
    fn GetContentTransform(&self, matrix: *mut f32, pointcount: u32) -> ::windows::core::Result<()>;
    fn SyncContentTransform(&self, matrix: *const f32, pointcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectManipulationContent {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectManipulationContent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationContent_Impl, const OFFSET: isize>() -> IDirectManipulationContent_Vtbl {
        unsafe extern "system" fn GetContentRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentsize: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContentRect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contentsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, contentsize: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContentRect(::core::mem::transmute_copy(&contentsize)).into()
        }
        unsafe extern "system" fn GetViewport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetViewport(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        unsafe extern "system" fn GetTag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTag(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SetTag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetOutputTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOutputTransform(::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn GetContentTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetContentTransform(::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn SyncContentTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SyncContentTransform(::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn DeferContact(&self, pointerid: u32, timeout: u32) -> ::windows::core::Result<()>;
    fn CancelContact(&self, pointerid: u32) -> ::windows::core::Result<()>;
    fn CancelDeferral(&self, pointerid: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDirectManipulationDeferContactService {}
impl IDirectManipulationDeferContactService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationDeferContactService_Impl, const OFFSET: isize>() -> IDirectManipulationDeferContactService_Vtbl {
        unsafe extern "system" fn DeferContact<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationDeferContactService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeferContact(::core::mem::transmute_copy(&pointerid), ::core::mem::transmute_copy(&timeout)).into()
        }
        unsafe extern "system" fn CancelContact<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationDeferContactService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelContact(::core::mem::transmute_copy(&pointerid)).into()
        }
        unsafe extern "system" fn CancelDeferral<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationDeferContactService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelDeferral(::core::mem::transmute_copy(&pointerid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn SetConfiguration(&self, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows::core::Result<()>;
    fn GetStatus(&self) -> ::windows::core::Result<DIRECTMANIPULATION_DRAG_DROP_STATUS>;
}
impl ::windows::core::RuntimeName for IDirectManipulationDragDropBehavior {}
impl IDirectManipulationDragDropBehavior_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationDragDropBehavior_Impl, const OFFSET: isize>() -> IDirectManipulationDragDropBehavior_Vtbl {
        unsafe extern "system" fn SetConfiguration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationDragDropBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetConfiguration(::core::mem::transmute_copy(&configuration)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationDragDropBehavior_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetConfiguration: SetConfiguration::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationDragDropBehavior as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationDragDropEventHandler_Impl: Sized {
    fn OnDragDropStatusChange(&self, viewport: &::core::option::Option<IDirectManipulationViewport2>, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDirectManipulationDragDropEventHandler {}
impl IDirectManipulationDragDropEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationDragDropEventHandler_Impl, const OFFSET: isize>() -> IDirectManipulationDragDropEventHandler_Vtbl {
        unsafe extern "system" fn OnDragDropStatusChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationDragDropEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *mut ::core::ffi::c_void, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDragDropStatusChange(::core::mem::transmute(&viewport), ::core::mem::transmute_copy(&current), ::core::mem::transmute_copy(&previous)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnDragDropStatusChange: OnDragDropStatusChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationDragDropEventHandler as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationFrameInfoProvider_Impl: Sized {
    fn GetNextFrameInfo(&self, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDirectManipulationFrameInfoProvider {}
impl IDirectManipulationFrameInfoProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationFrameInfoProvider_Impl, const OFFSET: isize>() -> IDirectManipulationFrameInfoProvider_Vtbl {
        unsafe extern "system" fn GetNextFrameInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationFrameInfoProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNextFrameInfo(::core::mem::transmute_copy(&time), ::core::mem::transmute_copy(&processtime), ::core::mem::transmute_copy(&compositiontime)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetNextFrameInfo: GetNextFrameInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationFrameInfoProvider as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationInteractionEventHandler_Impl: Sized {
    fn OnInteraction(&self, viewport: &::core::option::Option<IDirectManipulationViewport2>, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDirectManipulationInteractionEventHandler {}
impl IDirectManipulationInteractionEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationInteractionEventHandler_Impl, const OFFSET: isize>() -> IDirectManipulationInteractionEventHandler_Vtbl {
        unsafe extern "system" fn OnInteraction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationInteractionEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *mut ::core::ffi::c_void, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnInteraction(::core::mem::transmute(&viewport), ::core::mem::transmute_copy(&interaction)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnInteraction: OnInteraction::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationInteractionEventHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDirectManipulationManager_Impl: Sized {
    fn Activate(&self, window: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn Deactivate(&self, window: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn RegisterHitTestTarget(&self, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::Result<()>;
    fn ProcessInput(&self, message: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetUpdateManager(&self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateViewport(&self, frameinfo: &::core::option::Option<IDirectManipulationFrameInfoProvider>, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateContent(&self, frameinfo: &::core::option::Option<IDirectManipulationFrameInfoProvider>, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows::core::RuntimeName for IDirectManipulationManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDirectManipulationManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationManager_Impl, const OFFSET: isize>() -> IDirectManipulationManager_Vtbl {
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Activate(::core::mem::transmute_copy(&window)).into()
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Deactivate(::core::mem::transmute_copy(&window)).into()
        }
        unsafe extern "system" fn RegisterHitTestTarget<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterHitTestTarget(::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&hittestwindow), ::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn ProcessInput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, message: *const super::super::UI::WindowsAndMessaging::MSG, handled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProcessInput(::core::mem::transmute_copy(&message)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(handled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUpdateManager(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        unsafe extern "system" fn CreateViewport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameinfo: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateViewport(::core::mem::transmute(&frameinfo), ::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        unsafe extern "system" fn CreateContent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameinfo: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateContent(::core::mem::transmute(&frameinfo), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn CreateBehavior(&self, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows::core::RuntimeName for IDirectManipulationManager2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDirectManipulationManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationManager2_Impl, const OFFSET: isize>() -> IDirectManipulationManager2_Vtbl {
        unsafe extern "system" fn CreateBehavior<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateBehavior(::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        Self { base__: IDirectManipulationManager_Vtbl::new::<Identity, Impl, OFFSET>(), CreateBehavior: CreateBehavior::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationManager2 as ::windows::core::Interface>::IID || iid == &<IDirectManipulationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDirectManipulationManager3_Impl: Sized + IDirectManipulationManager_Impl + IDirectManipulationManager2_Impl {
    fn GetService(&self, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows::core::RuntimeName for IDirectManipulationManager3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl IDirectManipulationManager3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationManager3_Impl, const OFFSET: isize>() -> IDirectManipulationManager3_Vtbl {
        unsafe extern "system" fn GetService<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationManager3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetService(::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        Self { base__: IDirectManipulationManager2_Vtbl::new::<Identity, Impl, OFFSET>(), GetService: GetService::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationManager3 as ::windows::core::Interface>::IID || iid == &<IDirectManipulationManager as ::windows::core::Interface>::IID || iid == &<IDirectManipulationManager2 as ::windows::core::Interface>::IID
    }
}
pub trait IDirectManipulationPrimaryContent_Impl: Sized {
    fn SetSnapInterval(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> ::windows::core::Result<()>;
    fn SetSnapPoints(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> ::windows::core::Result<()>;
    fn SetSnapType(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> ::windows::core::Result<()>;
    fn SetSnapCoordinate(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> ::windows::core::Result<()>;
    fn SetZoomBoundaries(&self, zoomminimum: f32, zoommaximum: f32) -> ::windows::core::Result<()>;
    fn SetHorizontalAlignment(&self, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> ::windows::core::Result<()>;
    fn SetVerticalAlignment(&self, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> ::windows::core::Result<()>;
    fn GetInertiaEndTransform(&self, matrix: *mut f32, pointcount: u32) -> ::windows::core::Result<()>;
    fn GetCenterPoint(&self, centerx: *mut f32, centery: *mut f32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDirectManipulationPrimaryContent {}
impl IDirectManipulationPrimaryContent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>() -> IDirectManipulationPrimaryContent_Vtbl {
        unsafe extern "system" fn SetSnapInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSnapInterval(::core::mem::transmute_copy(&motion), ::core::mem::transmute_copy(&interval), ::core::mem::transmute_copy(&offset)).into()
        }
        unsafe extern "system" fn SetSnapPoints<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSnapPoints(::core::mem::transmute_copy(&motion), ::core::mem::transmute_copy(&points), ::core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn SetSnapType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSnapType(::core::mem::transmute_copy(&motion), ::core::mem::transmute_copy(&r#type)).into()
        }
        unsafe extern "system" fn SetSnapCoordinate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSnapCoordinate(::core::mem::transmute_copy(&motion), ::core::mem::transmute_copy(&coordinate), ::core::mem::transmute_copy(&origin)).into()
        }
        unsafe extern "system" fn SetZoomBoundaries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, zoomminimum: f32, zoommaximum: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetZoomBoundaries(::core::mem::transmute_copy(&zoomminimum), ::core::mem::transmute_copy(&zoommaximum)).into()
        }
        unsafe extern "system" fn SetHorizontalAlignment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHorizontalAlignment(::core::mem::transmute_copy(&alignment)).into()
        }
        unsafe extern "system" fn SetVerticalAlignment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVerticalAlignment(::core::mem::transmute_copy(&alignment)).into()
        }
        unsafe extern "system" fn GetInertiaEndTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInertiaEndTransform(::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn GetCenterPoint<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, centerx: *mut f32, centery: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCenterPoint(::core::mem::transmute_copy(&centerx), ::core::mem::transmute_copy(&centery)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn Update(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDirectManipulationUpdateHandler {}
impl IDirectManipulationUpdateHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationUpdateHandler_Impl, const OFFSET: isize>() -> IDirectManipulationUpdateHandler_Vtbl {
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationUpdateHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Update().into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationUpdateHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectManipulationUpdateManager_Impl: Sized {
    fn RegisterWaitHandleCallback(&self, handle: super::super::Foundation::HANDLE, eventhandler: &::core::option::Option<IDirectManipulationUpdateHandler>) -> ::windows::core::Result<u32>;
    fn UnregisterWaitHandleCallback(&self, cookie: u32) -> ::windows::core::Result<()>;
    fn Update(&self, frameinfo: &::core::option::Option<IDirectManipulationFrameInfoProvider>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectManipulationUpdateManager {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectManipulationUpdateManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationUpdateManager_Impl, const OFFSET: isize>() -> IDirectManipulationUpdateManager_Vtbl {
        unsafe extern "system" fn RegisterWaitHandleCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationUpdateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, eventhandler: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterWaitHandleCallback(::core::mem::transmute_copy(&handle), ::core::mem::transmute(&eventhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterWaitHandleCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationUpdateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterWaitHandleCallback(::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn Update<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationUpdateManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Update(::core::mem::transmute(&frameinfo)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn Enable(&self) -> ::windows::core::Result<()>;
    fn Disable(&self) -> ::windows::core::Result<()>;
    fn SetContact(&self, pointerid: u32) -> ::windows::core::Result<()>;
    fn ReleaseContact(&self, pointerid: u32) -> ::windows::core::Result<()>;
    fn ReleaseAllContacts(&self) -> ::windows::core::Result<()>;
    fn GetStatus(&self) -> ::windows::core::Result<DIRECTMANIPULATION_STATUS>;
    fn GetTag(&self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::Result<()>;
    fn SetTag(&self, object: &::core::option::Option<::windows::core::IUnknown>, id: u32) -> ::windows::core::Result<()>;
    fn GetViewportRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn SetViewportRect(&self, viewport: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn ZoomToRect(&self, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetViewportTransform(&self, matrix: *const f32, pointcount: u32) -> ::windows::core::Result<()>;
    fn SyncDisplayTransform(&self, matrix: *const f32, pointcount: u32) -> ::windows::core::Result<()>;
    fn GetPrimaryContent(&self, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn AddContent(&self, content: &::core::option::Option<IDirectManipulationContent>) -> ::windows::core::Result<()>;
    fn RemoveContent(&self, content: &::core::option::Option<IDirectManipulationContent>) -> ::windows::core::Result<()>;
    fn SetViewportOptions(&self, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::Result<()>;
    fn AddConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()>;
    fn RemoveConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()>;
    fn ActivateConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::Result<()>;
    fn SetManualGesture(&self, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::Result<()>;
    fn SetChaining(&self, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::Result<()>;
    fn AddEventHandler(&self, window: super::super::Foundation::HWND, eventhandler: &::core::option::Option<IDirectManipulationViewportEventHandler>) -> ::windows::core::Result<u32>;
    fn RemoveEventHandler(&self, cookie: u32) -> ::windows::core::Result<()>;
    fn SetInputMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()>;
    fn SetUpdateMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Abandon(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectManipulationViewport {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectManipulationViewport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>() -> IDirectManipulationViewport_Vtbl {
        unsafe extern "system" fn Enable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Enable().into()
        }
        unsafe extern "system" fn Disable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Disable().into()
        }
        unsafe extern "system" fn SetContact<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContact(::core::mem::transmute_copy(&pointerid)).into()
        }
        unsafe extern "system" fn ReleaseContact<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseContact(::core::mem::transmute_copy(&pointerid)).into()
        }
        unsafe extern "system" fn ReleaseAllContacts<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseAllContacts().into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut DIRECTMANIPULATION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTag(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn SetTag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTag(::core::mem::transmute(&object), ::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn GetViewportRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetViewportRect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(viewport, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewportRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetViewportRect(::core::mem::transmute_copy(&viewport)).into()
        }
        unsafe extern "system" fn ZoomToRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ZoomToRect(::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&top), ::core::mem::transmute_copy(&right), ::core::mem::transmute_copy(&bottom), ::core::mem::transmute_copy(&animate)).into()
        }
        unsafe extern "system" fn SetViewportTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetViewportTransform(::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn SyncDisplayTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SyncDisplayTransform(::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into()
        }
        unsafe extern "system" fn GetPrimaryContent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPrimaryContent(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into()
        }
        unsafe extern "system" fn AddContent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddContent(::core::mem::transmute(&content)).into()
        }
        unsafe extern "system" fn RemoveContent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveContent(::core::mem::transmute(&content)).into()
        }
        unsafe extern "system" fn SetViewportOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetViewportOptions(::core::mem::transmute_copy(&options)).into()
        }
        unsafe extern "system" fn AddConfiguration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddConfiguration(::core::mem::transmute_copy(&configuration)).into()
        }
        unsafe extern "system" fn RemoveConfiguration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveConfiguration(::core::mem::transmute_copy(&configuration)).into()
        }
        unsafe extern "system" fn ActivateConfiguration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ActivateConfiguration(::core::mem::transmute_copy(&configuration)).into()
        }
        unsafe extern "system" fn SetManualGesture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetManualGesture(::core::mem::transmute_copy(&configuration)).into()
        }
        unsafe extern "system" fn SetChaining<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetChaining(::core::mem::transmute_copy(&enabledtypes)).into()
        }
        unsafe extern "system" fn AddEventHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, eventhandler: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddEventHandler(::core::mem::transmute_copy(&window), ::core::mem::transmute(&eventhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEventHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveEventHandler(::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn SetInputMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInputMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn SetUpdateMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUpdateMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        unsafe extern "system" fn Abandon<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abandon().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn AddBehavior(&self, behavior: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<u32>;
    fn RemoveBehavior(&self, cookie: u32) -> ::windows::core::Result<()>;
    fn RemoveAllBehaviors(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDirectManipulationViewport2 {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectManipulationViewport2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport2_Impl, const OFFSET: isize>() -> IDirectManipulationViewport2_Vtbl {
        unsafe extern "system" fn AddBehavior<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, behavior: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddBehavior(::core::mem::transmute(&behavior)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBehavior<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveBehavior(::core::mem::transmute_copy(&cookie)).into()
        }
        unsafe extern "system" fn RemoveAllBehaviors<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewport2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAllBehaviors().into()
        }
        Self {
            base__: IDirectManipulationViewport_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn OnViewportStatusChanged(&self, viewport: &::core::option::Option<IDirectManipulationViewport>, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows::core::Result<()>;
    fn OnViewportUpdated(&self, viewport: &::core::option::Option<IDirectManipulationViewport>) -> ::windows::core::Result<()>;
    fn OnContentUpdated(&self, viewport: &::core::option::Option<IDirectManipulationViewport>, content: &::core::option::Option<IDirectManipulationContent>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDirectManipulationViewportEventHandler {}
impl IDirectManipulationViewportEventHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewportEventHandler_Impl, const OFFSET: isize>() -> IDirectManipulationViewportEventHandler_Vtbl {
        unsafe extern "system" fn OnViewportStatusChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewportEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *mut ::core::ffi::c_void, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnViewportStatusChanged(::core::mem::transmute(&viewport), ::core::mem::transmute_copy(&current), ::core::mem::transmute_copy(&previous)).into()
        }
        unsafe extern "system" fn OnViewportUpdated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewportEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnViewportUpdated(::core::mem::transmute(&viewport)).into()
        }
        unsafe extern "system" fn OnContentUpdated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDirectManipulationViewportEventHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, viewport: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnContentUpdated(::core::mem::transmute(&viewport), ::core::mem::transmute(&content)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnViewportStatusChanged: OnViewportStatusChanged::<Identity, Impl, OFFSET>,
            OnViewportUpdated: OnViewportUpdated::<Identity, Impl, OFFSET>,
            OnContentUpdated: OnContentUpdated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectManipulationViewportEventHandler as ::windows::core::Interface>::IID
    }
}
