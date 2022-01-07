pub trait IDirectManipulationAutoScrollBehaviorImpl: Sized {
    fn SetConfiguration();
}
impl ::windows::core::RuntimeName for IDirectManipulationAutoScrollBehavior {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectManipulation.IDirectManipulationAutoScrollBehavior";
}
impl IDirectManipulationAutoScrollBehaviorVtbl {
    pub const fn new<Impl: IDirectManipulationAutoScrollBehaviorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectManipulationAutoScrollBehaviorVtbl {
        unsafe extern "system" fn SetConfiguration<Impl: IDirectManipulationAutoScrollBehaviorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetConfiguration(motiontypes, scrollmotion) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectManipulationAutoScrollBehavior>, base.5, SetConfiguration::<Impl, OFFSET>)
    }
}
pub trait IDirectManipulationCompositorImpl: Sized {
    fn AddContent();
    fn RemoveContent();
    fn SetUpdateManager();
    fn Flush();
}
impl ::windows::core::RuntimeName for IDirectManipulationCompositor {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectManipulation.IDirectManipulationCompositor";
}
impl IDirectManipulationCompositorVtbl {
    pub const fn new<Impl: IDirectManipulationCompositorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectManipulationCompositorVtbl {
        unsafe extern "system" fn AddContent<Impl: IDirectManipulationCompositorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddContent(
                &*(&content as *const <IDirectManipulationContent as ::windows::core::Abi>::Abi as *const <IDirectManipulationContent as ::windows::core::DefaultType>::DefaultType),
                &*(&device as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&parentvisual as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&childvisual as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContent<Impl: IDirectManipulationCompositorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveContent(&*(&content as *const <IDirectManipulationContent as ::windows::core::Abi>::Abi as *const <IDirectManipulationContent as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdateManager<Impl: IDirectManipulationCompositorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updatemanager: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUpdateManager(&*(&updatemanager as *const <IDirectManipulationUpdateManager as ::windows::core::Abi>::Abi as *const <IDirectManipulationUpdateManager as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Flush<Impl: IDirectManipulationCompositorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Flush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectManipulationCompositor>, base.5, AddContent::<Impl, OFFSET>, RemoveContent::<Impl, OFFSET>, SetUpdateManager::<Impl, OFFSET>, Flush::<Impl, OFFSET>)
    }
}
pub trait IDirectManipulationCompositor2Impl: Sized + IDirectManipulationCompositorImpl {
    fn AddContentWithCrossProcessChaining();
}
impl ::windows::core::RuntimeName for IDirectManipulationCompositor2 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectManipulation.IDirectManipulationCompositor2";
}
impl IDirectManipulationCompositor2Vtbl {
    pub const fn new<Impl: IDirectManipulationCompositor2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectManipulationCompositor2Vtbl {
        unsafe extern "system" fn AddContentWithCrossProcessChaining<Impl: IDirectManipulationCompositor2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddContentWithCrossProcessChaining(
                &*(&content as *const <IDirectManipulationPrimaryContent as ::windows::core::Abi>::Abi as *const <IDirectManipulationPrimaryContent as ::windows::core::DefaultType>::DefaultType),
                &*(&device as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&parentvisual as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&childvisual as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectManipulationCompositor2>, base.5, AddContentWithCrossProcessChaining::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDirectManipulationContent {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectManipulation.IDirectManipulationContent";
}
impl IDirectManipulationContentVtbl {
    pub const fn new<Impl: IDirectManipulationContentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectManipulationContentVtbl {
        unsafe extern "system" fn GetContentRect<Impl: IDirectManipulationContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contentsize: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetContentRect(::core::mem::transmute_copy(&contentsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContentRect<Impl: IDirectManipulationContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contentsize: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetContentRect(&*(&contentsize as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewport<Impl: IDirectManipulationContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetViewport(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&object)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTag<Impl: IDirectManipulationContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTag(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTag<Impl: IDirectManipulationContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTag(&*(&object as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), id) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputTransform<Impl: IDirectManipulationContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputTransform(::core::mem::transmute_copy(&matrix), pointcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentTransform<Impl: IDirectManipulationContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetContentTransform(::core::mem::transmute_copy(&matrix), pointcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncContentTransform<Impl: IDirectManipulationContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SyncContentTransform(matrix, pointcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectManipulationContent>, base.5, GetContentRect::<Impl, OFFSET>, SetContentRect::<Impl, OFFSET>, GetViewport::<Impl, OFFSET>, GetTag::<Impl, OFFSET>, SetTag::<Impl, OFFSET>, GetOutputTransform::<Impl, OFFSET>, GetContentTransform::<Impl, OFFSET>, SyncContentTransform::<Impl, OFFSET>)
    }
}
pub trait IDirectManipulationDeferContactServiceImpl: Sized {
    fn DeferContact();
    fn CancelContact();
    fn CancelDeferral();
}
impl ::windows::core::RuntimeName for IDirectManipulationDeferContactService {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectManipulation.IDirectManipulationDeferContactService";
}
impl IDirectManipulationDeferContactServiceVtbl {
    pub const fn new<Impl: IDirectManipulationDeferContactServiceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectManipulationDeferContactServiceVtbl {
        unsafe extern "system" fn DeferContact<Impl: IDirectManipulationDeferContactServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerid: u32, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DeferContact(pointerid, timeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelContact<Impl: IDirectManipulationDeferContactServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelContact(pointerid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelDeferral<Impl: IDirectManipulationDeferContactServiceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelDeferral(pointerid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectManipulationDeferContactService>, base.5, DeferContact::<Impl, OFFSET>, CancelContact::<Impl, OFFSET>, CancelDeferral::<Impl, OFFSET>)
    }
}
pub trait IDirectManipulationDragDropBehaviorImpl: Sized {
    fn SetConfiguration();
    fn GetStatus();
}
impl ::windows::core::RuntimeName for IDirectManipulationDragDropBehavior {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectManipulation.IDirectManipulationDragDropBehavior";
}
impl IDirectManipulationDragDropBehaviorVtbl {
    pub const fn new<Impl: IDirectManipulationDragDropBehaviorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectManipulationDragDropBehaviorVtbl {
        unsafe extern "system" fn SetConfiguration<Impl: IDirectManipulationDragDropBehaviorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetConfiguration(configuration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IDirectManipulationDragDropBehaviorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&status)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectManipulationDragDropBehavior>, base.5, SetConfiguration::<Impl, OFFSET>, GetStatus::<Impl, OFFSET>)
    }
}
pub trait IDirectManipulationDragDropEventHandlerImpl: Sized {
    fn OnDragDropStatusChange();
}
impl ::windows::core::RuntimeName for IDirectManipulationDragDropEventHandler {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectManipulation.IDirectManipulationDragDropEventHandler";
}
impl IDirectManipulationDragDropEventHandlerVtbl {
    pub const fn new<Impl: IDirectManipulationDragDropEventHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectManipulationDragDropEventHandlerVtbl {
        unsafe extern "system" fn OnDragDropStatusChange<Impl: IDirectManipulationDragDropEventHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnDragDropStatusChange(&*(&viewport as *const <IDirectManipulationViewport2 as ::windows::core::Abi>::Abi as *const <IDirectManipulationViewport2 as ::windows::core::DefaultType>::DefaultType), current, previous) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectManipulationDragDropEventHandler>, base.5, OnDragDropStatusChange::<Impl, OFFSET>)
    }
}
pub trait IDirectManipulationFrameInfoProviderImpl: Sized {
    fn GetNextFrameInfo();
}
impl ::windows::core::RuntimeName for IDirectManipulationFrameInfoProvider {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectManipulation.IDirectManipulationFrameInfoProvider";
}
impl IDirectManipulationFrameInfoProviderVtbl {
    pub const fn new<Impl: IDirectManipulationFrameInfoProviderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectManipulationFrameInfoProviderVtbl {
        unsafe extern "system" fn GetNextFrameInfo<Impl: IDirectManipulationFrameInfoProviderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNextFrameInfo(::core::mem::transmute_copy(&time), ::core::mem::transmute_copy(&processtime), ::core::mem::transmute_copy(&compositiontime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectManipulationFrameInfoProvider>, base.5, GetNextFrameInfo::<Impl, OFFSET>)
    }
}
pub trait IDirectManipulationInteractionEventHandlerImpl: Sized {
    fn OnInteraction();
}
impl ::windows::core::RuntimeName for IDirectManipulationInteractionEventHandler {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectManipulation.IDirectManipulationInteractionEventHandler";
}
impl IDirectManipulationInteractionEventHandlerVtbl {
    pub const fn new<Impl: IDirectManipulationInteractionEventHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectManipulationInteractionEventHandlerVtbl {
        unsafe extern "system" fn OnInteraction<Impl: IDirectManipulationInteractionEventHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnInteraction(&*(&viewport as *const <IDirectManipulationViewport2 as ::windows::core::Abi>::Abi as *const <IDirectManipulationViewport2 as ::windows::core::DefaultType>::DefaultType), interaction) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectManipulationInteractionEventHandler>, base.5, OnInteraction::<Impl, OFFSET>)
    }
}
pub trait IDirectManipulationManagerImpl: Sized {
    fn Activate();
    fn Deactivate();
    fn RegisterHitTestTarget();
    fn ProcessInput();
    fn GetUpdateManager();
    fn CreateViewport();
    fn CreateContent();
}
impl ::windows::core::RuntimeName for IDirectManipulationManager {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectManipulation.IDirectManipulationManager";
}
impl IDirectManipulationManagerVtbl {
    pub const fn new<Impl: IDirectManipulationManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectManipulationManagerVtbl {
        unsafe extern "system" fn Activate<Impl: IDirectManipulationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Activate(&*(&window as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Impl: IDirectManipulationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Deactivate(&*(&window as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterHitTestTarget<Impl: IDirectManipulationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterHitTestTarget(&*(&window as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&hittestwindow as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessInput<Impl: IDirectManipulationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: *const super::super::UI::WindowsAndMessaging::MSG, handled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProcessInput(&*(&message as *const <super::super::UI::WindowsAndMessaging::MSG as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowsAndMessaging::MSG as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&handled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpdateManager<Impl: IDirectManipulationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetUpdateManager(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&object)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateViewport<Impl: IDirectManipulationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr, window: super::super::Foundation::HWND, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateViewport(
                &*(&frameinfo as *const <IDirectManipulationFrameInfoProvider as ::windows::core::Abi>::Abi as *const <IDirectManipulationFrameInfoProvider as ::windows::core::DefaultType>::DefaultType),
                &*(&window as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&object),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateContent<Impl: IDirectManipulationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateContent(
                &*(&frameinfo as *const <IDirectManipulationFrameInfoProvider as ::windows::core::Abi>::Abi as *const <IDirectManipulationFrameInfoProvider as ::windows::core::DefaultType>::DefaultType),
                &*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&object),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectManipulationManager>, base.5, Activate::<Impl, OFFSET>, Deactivate::<Impl, OFFSET>, RegisterHitTestTarget::<Impl, OFFSET>, ProcessInput::<Impl, OFFSET>, GetUpdateManager::<Impl, OFFSET>, CreateViewport::<Impl, OFFSET>, CreateContent::<Impl, OFFSET>)
    }
}
pub trait IDirectManipulationManager2Impl: Sized + IDirectManipulationManagerImpl {
    fn CreateBehavior();
}
impl ::windows::core::RuntimeName for IDirectManipulationManager2 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectManipulation.IDirectManipulationManager2";
}
impl IDirectManipulationManager2Vtbl {
    pub const fn new<Impl: IDirectManipulationManager2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectManipulationManager2Vtbl {
        unsafe extern "system" fn CreateBehavior<Impl: IDirectManipulationManager2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateBehavior(&*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&object)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectManipulationManager2>, base.5, CreateBehavior::<Impl, OFFSET>)
    }
}
pub trait IDirectManipulationManager3Impl: Sized + IDirectManipulationManager2Impl + IDirectManipulationManagerImpl {
    fn GetService();
}
impl ::windows::core::RuntimeName for IDirectManipulationManager3 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectManipulation.IDirectManipulationManager3";
}
impl IDirectManipulationManager3Vtbl {
    pub const fn new<Impl: IDirectManipulationManager3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectManipulationManager3Vtbl {
        unsafe extern "system" fn GetService<Impl: IDirectManipulationManager3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetService(&*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&object)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectManipulationManager3>, base.5, GetService::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IDirectManipulationPrimaryContent {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectManipulation.IDirectManipulationPrimaryContent";
}
impl IDirectManipulationPrimaryContentVtbl {
    pub const fn new<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectManipulationPrimaryContentVtbl {
        unsafe extern "system" fn SetSnapInterval<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSnapInterval(motion, interval, offset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapPoints<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSnapPoints(motion, points, pointcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapType<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSnapType(motion, r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSnapCoordinate<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSnapCoordinate(motion, coordinate, origin) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetZoomBoundaries<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, zoomminimum: f32, zoommaximum: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetZoomBoundaries(zoomminimum, zoommaximum) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHorizontalAlignment<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetHorizontalAlignment(alignment) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVerticalAlignment<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVerticalAlignment(alignment) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInertiaEndTransform<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetInertiaEndTransform(::core::mem::transmute_copy(&matrix), pointcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCenterPoint<Impl: IDirectManipulationPrimaryContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, centerx: *mut f32, centery: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCenterPoint(::core::mem::transmute_copy(&centerx), ::core::mem::transmute_copy(&centery)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectManipulationPrimaryContent>, base.5, SetSnapInterval::<Impl, OFFSET>, SetSnapPoints::<Impl, OFFSET>, SetSnapType::<Impl, OFFSET>, SetSnapCoordinate::<Impl, OFFSET>, SetZoomBoundaries::<Impl, OFFSET>, SetHorizontalAlignment::<Impl, OFFSET>, SetVerticalAlignment::<Impl, OFFSET>, GetInertiaEndTransform::<Impl, OFFSET>, GetCenterPoint::<Impl, OFFSET>)
    }
}
pub trait IDirectManipulationUpdateHandlerImpl: Sized {
    fn Update();
}
impl ::windows::core::RuntimeName for IDirectManipulationUpdateHandler {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectManipulation.IDirectManipulationUpdateHandler";
}
impl IDirectManipulationUpdateHandlerVtbl {
    pub const fn new<Impl: IDirectManipulationUpdateHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectManipulationUpdateHandlerVtbl {
        unsafe extern "system" fn Update<Impl: IDirectManipulationUpdateHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Update() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectManipulationUpdateHandler>, base.5, Update::<Impl, OFFSET>)
    }
}
pub trait IDirectManipulationUpdateManagerImpl: Sized {
    fn RegisterWaitHandleCallback();
    fn UnregisterWaitHandleCallback();
    fn Update();
}
impl ::windows::core::RuntimeName for IDirectManipulationUpdateManager {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectManipulation.IDirectManipulationUpdateManager";
}
impl IDirectManipulationUpdateManagerVtbl {
    pub const fn new<Impl: IDirectManipulationUpdateManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectManipulationUpdateManagerVtbl {
        unsafe extern "system" fn RegisterWaitHandleCallback<Impl: IDirectManipulationUpdateManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, eventhandler: ::windows::core::RawPtr, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RegisterWaitHandleCallback(&*(&handle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), &*(&eventhandler as *const <IDirectManipulationUpdateHandler as ::windows::core::Abi>::Abi as *const <IDirectManipulationUpdateHandler as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&cookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterWaitHandleCallback<Impl: IDirectManipulationUpdateManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UnregisterWaitHandleCallback(cookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Update<Impl: IDirectManipulationUpdateManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frameinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Update(&*(&frameinfo as *const <IDirectManipulationFrameInfoProvider as ::windows::core::Abi>::Abi as *const <IDirectManipulationFrameInfoProvider as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectManipulationUpdateManager>, base.5, RegisterWaitHandleCallback::<Impl, OFFSET>, UnregisterWaitHandleCallback::<Impl, OFFSET>, Update::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDirectManipulationViewport {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectManipulation.IDirectManipulationViewport";
}
impl IDirectManipulationViewportVtbl {
    pub const fn new<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectManipulationViewportVtbl {
        unsafe extern "system" fn Enable<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disable<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Disable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContact<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetContact(pointerid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseContact<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseContact(pointerid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseAllContacts<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReleaseAllContacts() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut DIRECTMANIPULATION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&status)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTag<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetTag(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTag<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTag(&*(&object as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), id) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetViewportRect<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewport: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetViewportRect(::core::mem::transmute_copy(&viewport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewportRect<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewport: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetViewportRect(&*(&viewport as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ZoomToRect<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ZoomToRect(left, top, right, bottom, &*(&animate as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewportTransform<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetViewportTransform(matrix, pointcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SyncDisplayTransform<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SyncDisplayTransform(matrix, pointcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrimaryContent<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPrimaryContent(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&object)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddContent<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddContent(&*(&content as *const <IDirectManipulationContent as ::windows::core::Abi>::Abi as *const <IDirectManipulationContent as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveContent<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveContent(&*(&content as *const <IDirectManipulationContent as ::windows::core::Abi>::Abi as *const <IDirectManipulationContent as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetViewportOptions<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetViewportOptions(options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddConfiguration<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddConfiguration(configuration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveConfiguration<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveConfiguration(configuration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateConfiguration<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ActivateConfiguration(configuration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManualGesture<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetManualGesture(configuration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChaining<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetChaining(enabledtypes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddEventHandler<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, eventhandler: ::windows::core::RawPtr, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddEventHandler(&*(&window as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&eventhandler as *const <IDirectManipulationViewportEventHandler as ::windows::core::Abi>::Abi as *const <IDirectManipulationViewportEventHandler as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&cookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEventHandler<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveEventHandler(cookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputMode<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetInputMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUpdateMode<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUpdateMode(mode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abandon<Impl: IDirectManipulationViewportImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Abandon() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDirectManipulationViewport>,
            base.5,
            Enable::<Impl, OFFSET>,
            Disable::<Impl, OFFSET>,
            SetContact::<Impl, OFFSET>,
            ReleaseContact::<Impl, OFFSET>,
            ReleaseAllContacts::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            GetTag::<Impl, OFFSET>,
            SetTag::<Impl, OFFSET>,
            GetViewportRect::<Impl, OFFSET>,
            SetViewportRect::<Impl, OFFSET>,
            ZoomToRect::<Impl, OFFSET>,
            SetViewportTransform::<Impl, OFFSET>,
            SyncDisplayTransform::<Impl, OFFSET>,
            GetPrimaryContent::<Impl, OFFSET>,
            AddContent::<Impl, OFFSET>,
            RemoveContent::<Impl, OFFSET>,
            SetViewportOptions::<Impl, OFFSET>,
            AddConfiguration::<Impl, OFFSET>,
            RemoveConfiguration::<Impl, OFFSET>,
            ActivateConfiguration::<Impl, OFFSET>,
            SetManualGesture::<Impl, OFFSET>,
            SetChaining::<Impl, OFFSET>,
            AddEventHandler::<Impl, OFFSET>,
            RemoveEventHandler::<Impl, OFFSET>,
            SetInputMode::<Impl, OFFSET>,
            SetUpdateMode::<Impl, OFFSET>,
            Stop::<Impl, OFFSET>,
            Abandon::<Impl, OFFSET>,
        )
    }
}
pub trait IDirectManipulationViewport2Impl: Sized + IDirectManipulationViewportImpl {
    fn AddBehavior();
    fn RemoveBehavior();
    fn RemoveAllBehaviors();
}
impl ::windows::core::RuntimeName for IDirectManipulationViewport2 {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectManipulation.IDirectManipulationViewport2";
}
impl IDirectManipulationViewport2Vtbl {
    pub const fn new<Impl: IDirectManipulationViewport2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectManipulationViewport2Vtbl {
        unsafe extern "system" fn AddBehavior<Impl: IDirectManipulationViewport2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, behavior: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddBehavior(&*(&behavior as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&cookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBehavior<Impl: IDirectManipulationViewport2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveBehavior(cookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAllBehaviors<Impl: IDirectManipulationViewport2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveAllBehaviors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectManipulationViewport2>, base.5, AddBehavior::<Impl, OFFSET>, RemoveBehavior::<Impl, OFFSET>, RemoveAllBehaviors::<Impl, OFFSET>)
    }
}
pub trait IDirectManipulationViewportEventHandlerImpl: Sized {
    fn OnViewportStatusChanged();
    fn OnViewportUpdated();
    fn OnContentUpdated();
}
impl ::windows::core::RuntimeName for IDirectManipulationViewportEventHandler {
    const NAME: &'static str = "Windows.Win32.Graphics.DirectManipulation.IDirectManipulationViewportEventHandler";
}
impl IDirectManipulationViewportEventHandlerVtbl {
    pub const fn new<Impl: IDirectManipulationViewportEventHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectManipulationViewportEventHandlerVtbl {
        unsafe extern "system" fn OnViewportStatusChanged<Impl: IDirectManipulationViewportEventHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnViewportStatusChanged(&*(&viewport as *const <IDirectManipulationViewport as ::windows::core::Abi>::Abi as *const <IDirectManipulationViewport as ::windows::core::DefaultType>::DefaultType), current, previous) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnViewportUpdated<Impl: IDirectManipulationViewportEventHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnViewportUpdated(&*(&viewport as *const <IDirectManipulationViewport as ::windows::core::Abi>::Abi as *const <IDirectManipulationViewport as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnContentUpdated<Impl: IDirectManipulationViewportEventHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewport: ::windows::core::RawPtr, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnContentUpdated(&*(&viewport as *const <IDirectManipulationViewport as ::windows::core::Abi>::Abi as *const <IDirectManipulationViewport as ::windows::core::DefaultType>::DefaultType), &*(&content as *const <IDirectManipulationContent as ::windows::core::Abi>::Abi as *const <IDirectManipulationContent as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectManipulationViewportEventHandler>, base.5, OnViewportStatusChanged::<Impl, OFFSET>, OnViewportUpdated::<Impl, OFFSET>, OnContentUpdated::<Impl, OFFSET>)
    }
}
