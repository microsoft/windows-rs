pub const CLSID_AutoScrollBehavior: windows_core::GUID = windows_core::GUID::from_u128(0x26126a51_3c70_4c9a_aec2_948849eeb093);
pub const CLSID_DeferContactService: windows_core::GUID = windows_core::GUID::from_u128(0xd7b67cf4_84bb_434e_86ae_6592bbc9abd9);
pub const CLSID_DragDropConfigurationBehavior: windows_core::GUID = windows_core::GUID::from_u128(0x09b01b3e_ba6c_454d_82e8_95e352329f23);
pub const CLSID_HorizontalIndicatorContent: windows_core::GUID = windows_core::GUID::from_u128(0xe7d18cf5_3ec7_44d5_a76b_3770f3cf903d);
pub const CLSID_VerticalIndicatorContent: windows_core::GUID = windows_core::GUID::from_u128(0xa10b5f17_afe0_4aa2_91e9_3e7001d2e6b4);
pub const CLSID_VirtualViewportContent: windows_core::GUID = windows_core::GUID::from_u128(0x3206a19a_86f0_4cb4_a7f3_16e3b7e2d852);
pub const DCompManipulationCompositor: windows_core::GUID = windows_core::GUID::from_u128(0x79dea627_a08a_43ac_8ef5_6900b9299126);
pub type DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = i32;
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_FORWARD: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = 1;
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_REVERSE: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = 2;
pub const DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION_STOP: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION = 0;
pub const DIRECTMANIPULATION_BUILDING: DIRECTMANIPULATION_STATUS = 0;
pub type DIRECTMANIPULATION_CONFIGURATION = u32;
pub const DIRECTMANIPULATION_CONFIGURATION_INTERACTION: DIRECTMANIPULATION_CONFIGURATION = 1;
pub const DIRECTMANIPULATION_CONFIGURATION_NONE: DIRECTMANIPULATION_CONFIGURATION = 0;
pub const DIRECTMANIPULATION_CONFIGURATION_RAILS_X: DIRECTMANIPULATION_CONFIGURATION = 256;
pub const DIRECTMANIPULATION_CONFIGURATION_RAILS_Y: DIRECTMANIPULATION_CONFIGURATION = 512;
pub const DIRECTMANIPULATION_CONFIGURATION_SCALING: DIRECTMANIPULATION_CONFIGURATION = 16;
pub const DIRECTMANIPULATION_CONFIGURATION_SCALING_INERTIA: DIRECTMANIPULATION_CONFIGURATION = 128;
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_INERTIA: DIRECTMANIPULATION_CONFIGURATION = 32;
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_X: DIRECTMANIPULATION_CONFIGURATION = 2;
pub const DIRECTMANIPULATION_CONFIGURATION_TRANSLATION_Y: DIRECTMANIPULATION_CONFIGURATION = 4;
pub const DIRECTMANIPULATION_COORDINATE_BOUNDARY: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = 0;
pub const DIRECTMANIPULATION_COORDINATE_MIRRORED: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = 16;
pub const DIRECTMANIPULATION_COORDINATE_ORIGIN: DIRECTMANIPULATION_SNAPPOINT_COORDINATE = 1;
pub const DIRECTMANIPULATION_DISABLED: DIRECTMANIPULATION_STATUS = 2;
pub const DIRECTMANIPULATION_DRAG_DROP_CANCELLED: DIRECTMANIPULATION_DRAG_DROP_STATUS = 4;
pub const DIRECTMANIPULATION_DRAG_DROP_COMMITTED: DIRECTMANIPULATION_DRAG_DROP_STATUS = 5;
pub type DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = u32;
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HOLD_DRAG: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 64;
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_HORIZONTAL: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 2;
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_DRAG: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 32;
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_SELECT_ONLY: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 16;
pub const DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION_VERTICAL: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION = 1;
pub const DIRECTMANIPULATION_DRAG_DROP_DRAGGING: DIRECTMANIPULATION_DRAG_DROP_STATUS = 3;
pub const DIRECTMANIPULATION_DRAG_DROP_PRESELECT: DIRECTMANIPULATION_DRAG_DROP_STATUS = 1;
pub const DIRECTMANIPULATION_DRAG_DROP_READY: DIRECTMANIPULATION_DRAG_DROP_STATUS = 0;
pub const DIRECTMANIPULATION_DRAG_DROP_SELECTING: DIRECTMANIPULATION_DRAG_DROP_STATUS = 2;
pub type DIRECTMANIPULATION_DRAG_DROP_STATUS = i32;
pub const DIRECTMANIPULATION_ENABLED: DIRECTMANIPULATION_STATUS = 1;
pub type DIRECTMANIPULATION_GESTURE_CONFIGURATION = u32;
pub const DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_HORIZONTAL: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 16;
pub const DIRECTMANIPULATION_GESTURE_CROSS_SLIDE_VERTICAL: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 8;
pub const DIRECTMANIPULATION_GESTURE_DEFAULT: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 0;
pub const DIRECTMANIPULATION_GESTURE_NONE: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 0;
pub const DIRECTMANIPULATION_GESTURE_PINCH_ZOOM: DIRECTMANIPULATION_GESTURE_CONFIGURATION = 32;
pub type DIRECTMANIPULATION_HITTEST_TYPE = u32;
pub const DIRECTMANIPULATION_HITTEST_TYPE_ASYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = 0;
pub const DIRECTMANIPULATION_HITTEST_TYPE_AUTO_SYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = 2;
pub const DIRECTMANIPULATION_HITTEST_TYPE_SYNCHRONOUS: DIRECTMANIPULATION_HITTEST_TYPE = 1;
pub type DIRECTMANIPULATION_HORIZONTALALIGNMENT = u32;
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_CENTER: DIRECTMANIPULATION_HORIZONTALALIGNMENT = 2;
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_LEFT: DIRECTMANIPULATION_HORIZONTALALIGNMENT = 1;
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_NONE: DIRECTMANIPULATION_HORIZONTALALIGNMENT = 0;
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_RIGHT: DIRECTMANIPULATION_HORIZONTALALIGNMENT = 4;
pub const DIRECTMANIPULATION_HORIZONTALALIGNMENT_UNLOCKCENTER: DIRECTMANIPULATION_HORIZONTALALIGNMENT = 8;
pub const DIRECTMANIPULATION_INERTIA: DIRECTMANIPULATION_STATUS = 4;
pub type DIRECTMANIPULATION_INPUT_MODE = i32;
pub const DIRECTMANIPULATION_INPUT_MODE_AUTOMATIC: DIRECTMANIPULATION_INPUT_MODE = 0;
pub const DIRECTMANIPULATION_INPUT_MODE_MANUAL: DIRECTMANIPULATION_INPUT_MODE = 1;
pub const DIRECTMANIPULATION_INTERACTION_BEGIN: DIRECTMANIPULATION_INTERACTION_TYPE = 0;
pub const DIRECTMANIPULATION_INTERACTION_END: DIRECTMANIPULATION_INTERACTION_TYPE = 100;
pub type DIRECTMANIPULATION_INTERACTION_TYPE = i32;
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_CROSS_SLIDE: DIRECTMANIPULATION_INTERACTION_TYPE = 4;
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_HOLD: DIRECTMANIPULATION_INTERACTION_TYPE = 3;
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_PINCH_ZOOM: DIRECTMANIPULATION_INTERACTION_TYPE = 5;
pub const DIRECTMANIPULATION_INTERACTION_TYPE_GESTURE_TAP: DIRECTMANIPULATION_INTERACTION_TYPE = 2;
pub const DIRECTMANIPULATION_INTERACTION_TYPE_MANIPULATION: DIRECTMANIPULATION_INTERACTION_TYPE = 1;
pub const DIRECTMANIPULATION_KEYBOARDFOCUS: u32 = 4294967294;
pub const DIRECTMANIPULATION_MOTION_ALL: DIRECTMANIPULATION_MOTION_TYPES = 55;
pub const DIRECTMANIPULATION_MOTION_CENTERX: DIRECTMANIPULATION_MOTION_TYPES = 16;
pub const DIRECTMANIPULATION_MOTION_CENTERY: DIRECTMANIPULATION_MOTION_TYPES = 32;
pub const DIRECTMANIPULATION_MOTION_NONE: DIRECTMANIPULATION_MOTION_TYPES = 0;
pub const DIRECTMANIPULATION_MOTION_TRANSLATEX: DIRECTMANIPULATION_MOTION_TYPES = 1;
pub const DIRECTMANIPULATION_MOTION_TRANSLATEY: DIRECTMANIPULATION_MOTION_TYPES = 2;
pub type DIRECTMANIPULATION_MOTION_TYPES = u32;
pub const DIRECTMANIPULATION_MOTION_ZOOM: DIRECTMANIPULATION_MOTION_TYPES = 4;
pub const DIRECTMANIPULATION_MOUSEFOCUS: u32 = 4294967293;
pub const DIRECTMANIPULATION_READY: DIRECTMANIPULATION_STATUS = 5;
pub const DIRECTMANIPULATION_RUNNING: DIRECTMANIPULATION_STATUS = 3;
pub type DIRECTMANIPULATION_SNAPPOINT_COORDINATE = u32;
pub const DIRECTMANIPULATION_SNAPPOINT_MANDATORY: DIRECTMANIPULATION_SNAPPOINT_TYPE = 0;
pub const DIRECTMANIPULATION_SNAPPOINT_MANDATORY_SINGLE: DIRECTMANIPULATION_SNAPPOINT_TYPE = 2;
pub const DIRECTMANIPULATION_SNAPPOINT_OPTIONAL: DIRECTMANIPULATION_SNAPPOINT_TYPE = 1;
pub const DIRECTMANIPULATION_SNAPPOINT_OPTIONAL_SINGLE: DIRECTMANIPULATION_SNAPPOINT_TYPE = 3;
pub type DIRECTMANIPULATION_SNAPPOINT_TYPE = i32;
pub type DIRECTMANIPULATION_STATUS = i32;
pub const DIRECTMANIPULATION_SUSPENDED: DIRECTMANIPULATION_STATUS = 6;
pub type DIRECTMANIPULATION_VERTICALALIGNMENT = u32;
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_BOTTOM: DIRECTMANIPULATION_VERTICALALIGNMENT = 4;
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_CENTER: DIRECTMANIPULATION_VERTICALALIGNMENT = 2;
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_NONE: DIRECTMANIPULATION_VERTICALALIGNMENT = 0;
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_TOP: DIRECTMANIPULATION_VERTICALALIGNMENT = 1;
pub const DIRECTMANIPULATION_VERTICALALIGNMENT_UNLOCKCENTER: DIRECTMANIPULATION_VERTICALALIGNMENT = 8;
pub type DIRECTMANIPULATION_VIEWPORT_OPTIONS = u32;
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_AUTODISABLE: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 1;
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_DEFAULT: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 0;
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_DISABLEPIXELSNAPPING: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 16;
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_EXPLICITHITTEST: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 8;
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_INPUT: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 4;
pub const DIRECTMANIPULATION_VIEWPORT_OPTIONS_MANUALUPDATE: DIRECTMANIPULATION_VIEWPORT_OPTIONS = 2;
pub const DirectManipulationManager: windows_core::GUID = windows_core::GUID::from_u128(0x54e211b6_3650_4f75_8334_fa359598e1c5);
pub const DirectManipulationPrimaryContent: windows_core::GUID = windows_core::GUID::from_u128(0xcaa02661_d59e_41c7_8393_3ba3bacb6b57);
pub const DirectManipulationSharedManager: windows_core::GUID = windows_core::GUID::from_u128(0x99793286_77cc_4b57_96db_3b354f6f9fb5);
pub const DirectManipulationUpdateManager: windows_core::GUID = windows_core::GUID::from_u128(0x9fc1bfd5_1835_441a_b3b1_b6cc74b727d0);
pub const DirectManipulationViewport: windows_core::GUID = windows_core::GUID::from_u128(0x34e211b6_3650_4f75_8334_fa359598e1c5);
windows_core::imp::define_interface!(IDirectManipulationAutoScrollBehavior, IDirectManipulationAutoScrollBehavior_Vtbl, 0x6d5954d4_2003_4356_9b31_d051c9ff0af7);
windows_core::imp::interface_hierarchy!(IDirectManipulationAutoScrollBehavior, windows_core::IUnknown);
impl IDirectManipulationAutoScrollBehavior {
    pub unsafe fn SetConfiguration(&self, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConfiguration)(windows_core::Interface::as_raw(self), motiontypes, scrollmotion) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationAutoScrollBehavior_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_MOTION_TYPES, DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> windows_core::HRESULT,
}
pub trait IDirectManipulationAutoScrollBehavior_Impl: windows_core::IUnknownImpl {
    fn SetConfiguration(&self, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> windows_core::Result<()>;
}
impl IDirectManipulationAutoScrollBehavior_Vtbl {
    pub const fn new<Identity: IDirectManipulationAutoScrollBehavior_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetConfiguration<Identity: IDirectManipulationAutoScrollBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationAutoScrollBehavior_Impl::SetConfiguration(this, core::mem::transmute_copy(&motiontypes), core::mem::transmute_copy(&scrollmotion)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetConfiguration: SetConfiguration::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationAutoScrollBehavior as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectManipulationAutoScrollBehavior {}
windows_core::imp::define_interface!(IDirectManipulationCompositor, IDirectManipulationCompositor_Vtbl, 0x537a0825_0387_4efa_b62f_71eb1f085a7e);
windows_core::imp::interface_hierarchy!(IDirectManipulationCompositor, windows_core::IUnknown);
impl IDirectManipulationCompositor {
    pub unsafe fn AddContent<P0, P1, P2, P3>(&self, content: P0, device: P1, parentvisual: P2, childvisual: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectManipulationContent>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddContent)(windows_core::Interface::as_raw(self), content.param().abi(), device.param().abi(), parentvisual.param().abi(), childvisual.param().abi()) }
    }
    pub unsafe fn RemoveContent<P0>(&self, content: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectManipulationContent>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveContent)(windows_core::Interface::as_raw(self), content.param().abi()) }
    }
    pub unsafe fn SetUpdateManager<P0>(&self, updatemanager: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectManipulationUpdateManager>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetUpdateManager)(windows_core::Interface::as_raw(self), updatemanager.param().abi()) }
    }
    pub unsafe fn Flush(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationCompositor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetUpdateManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDirectManipulationCompositor_Impl: windows_core::IUnknownImpl {
    fn AddContent(&self, content: windows_core::Ref<IDirectManipulationContent>, device: windows_core::Ref<windows_core::IUnknown>, parentvisual: windows_core::Ref<windows_core::IUnknown>, childvisual: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RemoveContent(&self, content: windows_core::Ref<IDirectManipulationContent>) -> windows_core::Result<()>;
    fn SetUpdateManager(&self, updatemanager: windows_core::Ref<IDirectManipulationUpdateManager>) -> windows_core::Result<()>;
    fn Flush(&self) -> windows_core::Result<()>;
}
impl IDirectManipulationCompositor_Vtbl {
    pub const fn new<Identity: IDirectManipulationCompositor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddContent<Identity: IDirectManipulationCompositor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, content: *mut core::ffi::c_void, device: *mut core::ffi::c_void, parentvisual: *mut core::ffi::c_void, childvisual: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationCompositor_Impl::AddContent(this, core::mem::transmute_copy(&content), core::mem::transmute_copy(&device), core::mem::transmute_copy(&parentvisual), core::mem::transmute_copy(&childvisual)).into()
            }
        }
        unsafe extern "system" fn RemoveContent<Identity: IDirectManipulationCompositor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, content: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationCompositor_Impl::RemoveContent(this, core::mem::transmute_copy(&content)).into()
            }
        }
        unsafe extern "system" fn SetUpdateManager<Identity: IDirectManipulationCompositor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, updatemanager: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationCompositor_Impl::SetUpdateManager(this, core::mem::transmute_copy(&updatemanager)).into()
            }
        }
        unsafe extern "system" fn Flush<Identity: IDirectManipulationCompositor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationCompositor_Impl::Flush(this).into()
            }
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
impl windows_core::RuntimeName for IDirectManipulationCompositor {}
windows_core::imp::define_interface!(IDirectManipulationCompositor2, IDirectManipulationCompositor2_Vtbl, 0xd38c7822_f1cb_43cb_b4b9_ac0c767a412e);
impl core::ops::Deref for IDirectManipulationCompositor2 {
    type Target = IDirectManipulationCompositor;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationCompositor2, windows_core::IUnknown, IDirectManipulationCompositor);
impl IDirectManipulationCompositor2 {
    pub unsafe fn AddContentWithCrossProcessChaining<P0, P1, P2, P3>(&self, content: P0, device: P1, parentvisual: P2, childvisual: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectManipulationPrimaryContent>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddContentWithCrossProcessChaining)(windows_core::Interface::as_raw(self), content.param().abi(), device.param().abi(), parentvisual.param().abi(), childvisual.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationCompositor2_Vtbl {
    pub base__: IDirectManipulationCompositor_Vtbl,
    pub AddContentWithCrossProcessChaining: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDirectManipulationCompositor2_Impl: IDirectManipulationCompositor_Impl {
    fn AddContentWithCrossProcessChaining(&self, content: windows_core::Ref<IDirectManipulationPrimaryContent>, device: windows_core::Ref<windows_core::IUnknown>, parentvisual: windows_core::Ref<windows_core::IUnknown>, childvisual: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IDirectManipulationCompositor2_Vtbl {
    pub const fn new<Identity: IDirectManipulationCompositor2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddContentWithCrossProcessChaining<Identity: IDirectManipulationCompositor2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, content: *mut core::ffi::c_void, device: *mut core::ffi::c_void, parentvisual: *mut core::ffi::c_void, childvisual: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationCompositor2_Impl::AddContentWithCrossProcessChaining(this, core::mem::transmute_copy(&content), core::mem::transmute_copy(&device), core::mem::transmute_copy(&parentvisual), core::mem::transmute_copy(&childvisual)).into()
            }
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
impl windows_core::RuntimeName for IDirectManipulationCompositor2 {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IDirectManipulationCompositorPartner(pub u8);
windows_core::imp::define_interface!(IDirectManipulationContent, IDirectManipulationContent_Vtbl, 0xb89962cb_3d89_442b_bb58_5098fa0f9f16);
windows_core::imp::interface_hierarchy!(IDirectManipulationContent, windows_core::IUnknown);
impl IDirectManipulationContent {
    #[cfg(feature = "windef")]
    pub unsafe fn GetContentRect(&self) -> windows_core::Result<super::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContentRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetContentRect(&self, contentsize: *const super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetContentRect)(windows_core::Interface::as_raw(self), contentsize) }
    }
    pub unsafe fn GetViewport<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetViewport)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetTag<T>(&self, id: Option<*mut u32>, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        unsafe { (windows_core::Interface::vtable(self).GetTag)(windows_core::Interface::as_raw(self), &T::IID, result__ as *mut _ as *mut _, id.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetTag<P0>(&self, object: P0, id: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTag)(windows_core::Interface::as_raw(self), object.param().abi(), id) }
    }
    pub unsafe fn GetOutputTransform(&self, matrix: &mut [f32]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOutputTransform)(windows_core::Interface::as_raw(self), matrix.as_mut_ptr(), matrix.len().try_into().unwrap()) }
    }
    pub unsafe fn GetContentTransform(&self, matrix: &mut [f32]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetContentTransform)(windows_core::Interface::as_raw(self), matrix.as_mut_ptr(), matrix.len().try_into().unwrap()) }
    }
    pub unsafe fn SyncContentTransform(&self, matrix: &[f32]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SyncContentTransform)(windows_core::Interface::as_raw(self), matrix.as_ptr(), matrix.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationContent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub GetContentRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetContentRect: usize,
    #[cfg(feature = "windef")]
    pub SetContentRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetContentRect: usize,
    pub GetViewport: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetOutputTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32) -> windows_core::HRESULT,
    pub GetContentTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32) -> windows_core::HRESULT,
    pub SyncContentTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IDirectManipulationContent_Impl: windows_core::IUnknownImpl {
    fn GetContentRect(&self) -> windows_core::Result<super::RECT>;
    fn SetContentRect(&self, contentsize: *const super::RECT) -> windows_core::Result<()>;
    fn GetViewport(&self, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetTag(&self, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void, id: *mut u32) -> windows_core::Result<()>;
    fn SetTag(&self, object: windows_core::Ref<windows_core::IUnknown>, id: u32) -> windows_core::Result<()>;
    fn GetOutputTransform(&self, matrix: *mut f32, pointcount: u32) -> windows_core::Result<()>;
    fn GetContentTransform(&self, matrix: *mut f32, pointcount: u32) -> windows_core::Result<()>;
    fn SyncContentTransform(&self, matrix: *const f32, pointcount: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IDirectManipulationContent_Vtbl {
    pub const fn new<Identity: IDirectManipulationContent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetContentRect<Identity: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentsize: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectManipulationContent_Impl::GetContentRect(this) {
                    Ok(ok__) => {
                        contentsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContentRect<Identity: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, contentsize: *const super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationContent_Impl::SetContentRect(this, core::mem::transmute_copy(&contentsize)).into()
            }
        }
        unsafe extern "system" fn GetViewport<Identity: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationContent_Impl::GetViewport(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&object)).into()
            }
        }
        unsafe extern "system" fn GetTag<Identity: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void, id: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationContent_Impl::GetTag(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&object), core::mem::transmute_copy(&id)).into()
            }
        }
        unsafe extern "system" fn SetTag<Identity: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationContent_Impl::SetTag(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&id)).into()
            }
        }
        unsafe extern "system" fn GetOutputTransform<Identity: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationContent_Impl::GetOutputTransform(this, core::mem::transmute_copy(&matrix), core::mem::transmute_copy(&pointcount)).into()
            }
        }
        unsafe extern "system" fn GetContentTransform<Identity: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationContent_Impl::GetContentTransform(this, core::mem::transmute_copy(&matrix), core::mem::transmute_copy(&pointcount)).into()
            }
        }
        unsafe extern "system" fn SyncContentTransform<Identity: IDirectManipulationContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const f32, pointcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationContent_Impl::SyncContentTransform(this, core::mem::transmute_copy(&matrix), core::mem::transmute_copy(&pointcount)).into()
            }
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
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IDirectManipulationContent {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IDirectManipulationContent2(pub u8);
windows_core::imp::define_interface!(IDirectManipulationDeferContactService, IDirectManipulationDeferContactService_Vtbl, 0x652d5c71_fe60_4a98_be70_e5f21291e7f1);
windows_core::imp::interface_hierarchy!(IDirectManipulationDeferContactService, windows_core::IUnknown);
impl IDirectManipulationDeferContactService {
    pub unsafe fn DeferContact(&self, pointerid: u32, timeout: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeferContact)(windows_core::Interface::as_raw(self), pointerid, timeout) }
    }
    pub unsafe fn CancelContact(&self, pointerid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelContact)(windows_core::Interface::as_raw(self), pointerid) }
    }
    pub unsafe fn CancelDeferral(&self, pointerid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelDeferral)(windows_core::Interface::as_raw(self), pointerid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationDeferContactService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DeferContact: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub CancelContact: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub CancelDeferral: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IDirectManipulationDeferContactService_Impl: windows_core::IUnknownImpl {
    fn DeferContact(&self, pointerid: u32, timeout: u32) -> windows_core::Result<()>;
    fn CancelContact(&self, pointerid: u32) -> windows_core::Result<()>;
    fn CancelDeferral(&self, pointerid: u32) -> windows_core::Result<()>;
}
impl IDirectManipulationDeferContactService_Vtbl {
    pub const fn new<Identity: IDirectManipulationDeferContactService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DeferContact<Identity: IDirectManipulationDeferContactService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointerid: u32, timeout: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationDeferContactService_Impl::DeferContact(this, core::mem::transmute_copy(&pointerid), core::mem::transmute_copy(&timeout)).into()
            }
        }
        unsafe extern "system" fn CancelContact<Identity: IDirectManipulationDeferContactService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointerid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationDeferContactService_Impl::CancelContact(this, core::mem::transmute_copy(&pointerid)).into()
            }
        }
        unsafe extern "system" fn CancelDeferral<Identity: IDirectManipulationDeferContactService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointerid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationDeferContactService_Impl::CancelDeferral(this, core::mem::transmute_copy(&pointerid)).into()
            }
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
impl windows_core::RuntimeName for IDirectManipulationDeferContactService {}
windows_core::imp::define_interface!(IDirectManipulationDragDropBehavior, IDirectManipulationDragDropBehavior_Vtbl, 0x814b5af5_c2c8_4270_a9b7_a198ce8d02fa);
windows_core::imp::interface_hierarchy!(IDirectManipulationDragDropBehavior, windows_core::IUnknown);
impl IDirectManipulationDragDropBehavior {
    pub unsafe fn SetConfiguration(&self, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConfiguration)(windows_core::Interface::as_raw(self), configuration) }
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<DIRECTMANIPULATION_DRAG_DROP_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationDragDropBehavior_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIRECTMANIPULATION_DRAG_DROP_STATUS) -> windows_core::HRESULT,
}
pub trait IDirectManipulationDragDropBehavior_Impl: windows_core::IUnknownImpl {
    fn SetConfiguration(&self, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<DIRECTMANIPULATION_DRAG_DROP_STATUS>;
}
impl IDirectManipulationDragDropBehavior_Vtbl {
    pub const fn new<Identity: IDirectManipulationDragDropBehavior_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetConfiguration<Identity: IDirectManipulationDragDropBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationDragDropBehavior_Impl::SetConfiguration(this, core::mem::transmute_copy(&configuration)).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IDirectManipulationDragDropBehavior_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut DIRECTMANIPULATION_DRAG_DROP_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectManipulationDragDropBehavior_Impl::GetStatus(this) {
                    Ok(ok__) => {
                        status.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IDirectManipulationDragDropBehavior {}
windows_core::imp::define_interface!(IDirectManipulationDragDropEventHandler, IDirectManipulationDragDropEventHandler_Vtbl, 0x1fa11b10_701b_41ae_b5f2_49e36bd595aa);
windows_core::imp::interface_hierarchy!(IDirectManipulationDragDropEventHandler, windows_core::IUnknown);
impl IDirectManipulationDragDropEventHandler {
    pub unsafe fn OnDragDropStatusChange<P0>(&self, viewport: P0, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectManipulationViewport2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDragDropStatusChange)(windows_core::Interface::as_raw(self), viewport.param().abi(), current, previous) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationDragDropEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnDragDropStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DIRECTMANIPULATION_DRAG_DROP_STATUS, DIRECTMANIPULATION_DRAG_DROP_STATUS) -> windows_core::HRESULT,
}
pub trait IDirectManipulationDragDropEventHandler_Impl: windows_core::IUnknownImpl {
    fn OnDragDropStatusChange(&self, viewport: windows_core::Ref<IDirectManipulationViewport2>, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> windows_core::Result<()>;
}
impl IDirectManipulationDragDropEventHandler_Vtbl {
    pub const fn new<Identity: IDirectManipulationDragDropEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnDragDropStatusChange<Identity: IDirectManipulationDragDropEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewport: *mut core::ffi::c_void, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationDragDropEventHandler_Impl::OnDragDropStatusChange(this, core::mem::transmute_copy(&viewport), core::mem::transmute_copy(&current), core::mem::transmute_copy(&previous)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnDragDropStatusChange: OnDragDropStatusChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationDragDropEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectManipulationDragDropEventHandler {}
windows_core::imp::define_interface!(IDirectManipulationFrameInfoProvider, IDirectManipulationFrameInfoProvider_Vtbl, 0xfb759dba_6f4c_4c01_874e_19c8a05907f9);
windows_core::imp::interface_hierarchy!(IDirectManipulationFrameInfoProvider, windows_core::IUnknown);
impl IDirectManipulationFrameInfoProvider {
    pub unsafe fn GetNextFrameInfo(&self, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNextFrameInfo)(windows_core::Interface::as_raw(self), time as _, processtime as _, compositiontime as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationFrameInfoProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNextFrameInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64, *mut u64) -> windows_core::HRESULT,
}
pub trait IDirectManipulationFrameInfoProvider_Impl: windows_core::IUnknownImpl {
    fn GetNextFrameInfo(&self, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> windows_core::Result<()>;
}
impl IDirectManipulationFrameInfoProvider_Vtbl {
    pub const fn new<Identity: IDirectManipulationFrameInfoProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNextFrameInfo<Identity: IDirectManipulationFrameInfoProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationFrameInfoProvider_Impl::GetNextFrameInfo(this, core::mem::transmute_copy(&time), core::mem::transmute_copy(&processtime), core::mem::transmute_copy(&compositiontime)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetNextFrameInfo: GetNextFrameInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationFrameInfoProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectManipulationFrameInfoProvider {}
windows_core::imp::define_interface!(IDirectManipulationInteractionEventHandler, IDirectManipulationInteractionEventHandler_Vtbl, 0xe43f45b8_42b4_403e_b1f2_273b8f510830);
windows_core::imp::interface_hierarchy!(IDirectManipulationInteractionEventHandler, windows_core::IUnknown);
impl IDirectManipulationInteractionEventHandler {
    pub unsafe fn OnInteraction<P0>(&self, viewport: P0, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectManipulationViewport2>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnInteraction)(windows_core::Interface::as_raw(self), viewport.param().abi(), interaction) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationInteractionEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnInteraction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DIRECTMANIPULATION_INTERACTION_TYPE) -> windows_core::HRESULT,
}
pub trait IDirectManipulationInteractionEventHandler_Impl: windows_core::IUnknownImpl {
    fn OnInteraction(&self, viewport: windows_core::Ref<IDirectManipulationViewport2>, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> windows_core::Result<()>;
}
impl IDirectManipulationInteractionEventHandler_Vtbl {
    pub const fn new<Identity: IDirectManipulationInteractionEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnInteraction<Identity: IDirectManipulationInteractionEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewport: *mut core::ffi::c_void, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationInteractionEventHandler_Impl::OnInteraction(this, core::mem::transmute_copy(&viewport), core::mem::transmute_copy(&interaction)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnInteraction: OnInteraction::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationInteractionEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectManipulationInteractionEventHandler {}
windows_core::imp::define_interface!(IDirectManipulationManager, IDirectManipulationManager_Vtbl, 0xfbf5d3b4_70c7_4163_9322_5a6f660d6fbc);
windows_core::imp::interface_hierarchy!(IDirectManipulationManager, windows_core::IUnknown);
impl IDirectManipulationManager {
    #[cfg(feature = "windef")]
    pub unsafe fn Activate(&self, window: super::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Activate)(windows_core::Interface::as_raw(self), window) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn Deactivate(&self, window: super::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Deactivate)(windows_core::Interface::as_raw(self), window) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn RegisterHitTestTarget(&self, window: super::HWND, hittestwindow: Option<super::HWND>, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterHitTestTarget)(windows_core::Interface::as_raw(self), window, hittestwindow.unwrap_or(core::mem::zeroed()) as _, r#type) }
    }
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub unsafe fn ProcessInput(&self, message: *const super::MSG) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProcessInput)(windows_core::Interface::as_raw(self), message, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetUpdateManager<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetUpdateManager)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn CreateViewport<P0, T>(&self, frameinfo: P0, window: super::HWND) -> windows_core::Result<T>
    where
        P0: windows_core::Param<IDirectManipulationFrameInfoProvider>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateViewport)(windows_core::Interface::as_raw(self), frameinfo.param().abi(), window, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn CreateContent<P0, T>(&self, frameinfo: P0, clsid: *const windows_core::GUID) -> windows_core::Result<T>
    where
        P0: windows_core::Param<IDirectManipulationFrameInfoProvider>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateContent)(windows_core::Interface::as_raw(self), frameinfo.param().abi(), clsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub Activate: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Activate: usize,
    #[cfg(feature = "windef")]
    pub Deactivate: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Deactivate: usize,
    #[cfg(feature = "windef")]
    pub RegisterHitTestTarget: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, super::HWND, DIRECTMANIPULATION_HITTEST_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    RegisterHitTestTarget: usize,
    #[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
    pub ProcessInput: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::MSG, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef", feature = "winuser")))]
    ProcessInput: usize,
    pub GetUpdateManager: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub CreateViewport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::HWND, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    CreateViewport: usize,
    pub CreateContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub trait IDirectManipulationManager_Impl: windows_core::IUnknownImpl {
    fn Activate(&self, window: super::HWND) -> windows_core::Result<()>;
    fn Deactivate(&self, window: super::HWND) -> windows_core::Result<()>;
    fn RegisterHitTestTarget(&self, window: super::HWND, hittestwindow: super::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> windows_core::Result<()>;
    fn ProcessInput(&self, message: *const super::MSG) -> windows_core::Result<windows_core::BOOL>;
    fn GetUpdateManager(&self, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateViewport(&self, frameinfo: windows_core::Ref<IDirectManipulationFrameInfoProvider>, window: super::HWND, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CreateContent(&self, frameinfo: windows_core::Ref<IDirectManipulationFrameInfoProvider>, clsid: *const windows_core::GUID, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl IDirectManipulationManager_Vtbl {
    pub const fn new<Identity: IDirectManipulationManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Activate<Identity: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationManager_Impl::Activate(this, core::mem::transmute_copy(&window)).into()
            }
        }
        unsafe extern "system" fn Deactivate<Identity: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationManager_Impl::Deactivate(this, core::mem::transmute_copy(&window)).into()
            }
        }
        unsafe extern "system" fn RegisterHitTestTarget<Identity: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::HWND, hittestwindow: super::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationManager_Impl::RegisterHitTestTarget(this, core::mem::transmute_copy(&window), core::mem::transmute_copy(&hittestwindow), core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn ProcessInput<Identity: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, message: *const super::MSG, handled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectManipulationManager_Impl::ProcessInput(this, core::mem::transmute_copy(&message)) {
                    Ok(ok__) => {
                        handled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUpdateManager<Identity: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationManager_Impl::GetUpdateManager(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&object)).into()
            }
        }
        unsafe extern "system" fn CreateViewport<Identity: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frameinfo: *mut core::ffi::c_void, window: super::HWND, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationManager_Impl::CreateViewport(this, core::mem::transmute_copy(&frameinfo), core::mem::transmute_copy(&window), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&object)).into()
            }
        }
        unsafe extern "system" fn CreateContent<Identity: IDirectManipulationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frameinfo: *mut core::ffi::c_void, clsid: *const windows_core::GUID, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationManager_Impl::CreateContent(this, core::mem::transmute_copy(&frameinfo), core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&object)).into()
            }
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
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl windows_core::RuntimeName for IDirectManipulationManager {}
windows_core::imp::define_interface!(IDirectManipulationManager2, IDirectManipulationManager2_Vtbl, 0xfa1005e9_3d16_484c_bfc9_62b61e56ec4e);
impl core::ops::Deref for IDirectManipulationManager2 {
    type Target = IDirectManipulationManager;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationManager2, windows_core::IUnknown, IDirectManipulationManager);
impl IDirectManipulationManager2 {
    pub unsafe fn CreateBehavior<T>(&self, clsid: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateBehavior)(windows_core::Interface::as_raw(self), clsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationManager2_Vtbl {
    pub base__: IDirectManipulationManager_Vtbl,
    pub CreateBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub trait IDirectManipulationManager2_Impl: IDirectManipulationManager_Impl {
    fn CreateBehavior(&self, clsid: *const windows_core::GUID, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl IDirectManipulationManager2_Vtbl {
    pub const fn new<Identity: IDirectManipulationManager2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateBehavior<Identity: IDirectManipulationManager2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationManager2_Impl::CreateBehavior(this, core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&object)).into()
            }
        }
        Self { base__: IDirectManipulationManager_Vtbl::new::<Identity, OFFSET>(), CreateBehavior: CreateBehavior::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationManager2 as windows_core::Interface>::IID || iid == &<IDirectManipulationManager as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl windows_core::RuntimeName for IDirectManipulationManager2 {}
windows_core::imp::define_interface!(IDirectManipulationManager3, IDirectManipulationManager3_Vtbl, 0x2cb6b33d_ffe8_488c_b750_fbdfe88dca8c);
impl core::ops::Deref for IDirectManipulationManager3 {
    type Target = IDirectManipulationManager2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationManager3, windows_core::IUnknown, IDirectManipulationManager, IDirectManipulationManager2);
impl IDirectManipulationManager3 {
    pub unsafe fn GetService<T>(&self, clsid: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), clsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationManager3_Vtbl {
    pub base__: IDirectManipulationManager2_Vtbl,
    pub GetService: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub trait IDirectManipulationManager3_Impl: IDirectManipulationManager2_Impl {
    fn GetService(&self, clsid: *const windows_core::GUID, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl IDirectManipulationManager3_Vtbl {
    pub const fn new<Identity: IDirectManipulationManager3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetService<Identity: IDirectManipulationManager3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationManager3_Impl::GetService(this, core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&object)).into()
            }
        }
        Self { base__: IDirectManipulationManager2_Vtbl::new::<Identity, OFFSET>(), GetService: GetService::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationManager3 as windows_core::Interface>::IID || iid == &<IDirectManipulationManager as windows_core::Interface>::IID || iid == &<IDirectManipulationManager2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl windows_core::RuntimeName for IDirectManipulationManager3 {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IDirectManipulationManagerPartner(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IDirectManipulationParametricMotionBehavior(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IDirectManipulationParametricMotionCurve(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IDirectManipulationParametricRestPointBehavior(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IDirectManipulationParametricRestPointList(pub u8);
windows_core::imp::define_interface!(IDirectManipulationPrimaryContent, IDirectManipulationPrimaryContent_Vtbl, 0xc12851e4_1698_4625_b9b1_7ca3ec18630b);
windows_core::imp::interface_hierarchy!(IDirectManipulationPrimaryContent, windows_core::IUnknown);
impl IDirectManipulationPrimaryContent {
    pub unsafe fn SetSnapInterval(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSnapInterval)(windows_core::Interface::as_raw(self), motion, interval, offset) }
    }
    pub unsafe fn SetSnapPoints(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, points: Option<&[f32]>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSnapPoints)(windows_core::Interface::as_raw(self), motion, points.map_or(core::ptr::null(), |slice| slice.as_ptr()), points.map_or(0, |slice| slice.len().try_into().unwrap())) }
    }
    pub unsafe fn SetSnapType(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSnapType)(windows_core::Interface::as_raw(self), motion, r#type) }
    }
    pub unsafe fn SetSnapCoordinate(&self, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSnapCoordinate)(windows_core::Interface::as_raw(self), motion, coordinate, origin) }
    }
    pub unsafe fn SetZoomBoundaries(&self, zoomminimum: f32, zoommaximum: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetZoomBoundaries)(windows_core::Interface::as_raw(self), zoomminimum, zoommaximum) }
    }
    pub unsafe fn SetHorizontalAlignment(&self, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHorizontalAlignment)(windows_core::Interface::as_raw(self), alignment) }
    }
    pub unsafe fn SetVerticalAlignment(&self, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVerticalAlignment)(windows_core::Interface::as_raw(self), alignment) }
    }
    pub unsafe fn GetInertiaEndTransform(&self, matrix: &mut [f32]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInertiaEndTransform)(windows_core::Interface::as_raw(self), matrix.as_mut_ptr(), matrix.len().try_into().unwrap()) }
    }
    pub unsafe fn GetCenterPoint(&self, centerx: *mut f32, centery: *mut f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCenterPoint)(windows_core::Interface::as_raw(self), centerx as _, centery as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationPrimaryContent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSnapInterval: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_MOTION_TYPES, f32, f32) -> windows_core::HRESULT,
    pub SetSnapPoints: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_MOTION_TYPES, *const f32, u32) -> windows_core::HRESULT,
    pub SetSnapType: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_MOTION_TYPES, DIRECTMANIPULATION_SNAPPOINT_TYPE) -> windows_core::HRESULT,
    pub SetSnapCoordinate: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_MOTION_TYPES, DIRECTMANIPULATION_SNAPPOINT_COORDINATE, f32) -> windows_core::HRESULT,
    pub SetZoomBoundaries: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32) -> windows_core::HRESULT,
    pub SetHorizontalAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> windows_core::HRESULT,
    pub SetVerticalAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_VERTICALALIGNMENT) -> windows_core::HRESULT,
    pub GetInertiaEndTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, u32) -> windows_core::HRESULT,
    pub GetCenterPoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32, *mut f32) -> windows_core::HRESULT,
}
pub trait IDirectManipulationPrimaryContent_Impl: windows_core::IUnknownImpl {
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
impl IDirectManipulationPrimaryContent_Vtbl {
    pub const fn new<Identity: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSnapInterval<Identity: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationPrimaryContent_Impl::SetSnapInterval(this, core::mem::transmute_copy(&motion), core::mem::transmute_copy(&interval), core::mem::transmute_copy(&offset)).into()
            }
        }
        unsafe extern "system" fn SetSnapPoints<Identity: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationPrimaryContent_Impl::SetSnapPoints(this, core::mem::transmute_copy(&motion), core::mem::transmute_copy(&points), core::mem::transmute_copy(&pointcount)).into()
            }
        }
        unsafe extern "system" fn SetSnapType<Identity: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationPrimaryContent_Impl::SetSnapType(this, core::mem::transmute_copy(&motion), core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn SetSnapCoordinate<Identity: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationPrimaryContent_Impl::SetSnapCoordinate(this, core::mem::transmute_copy(&motion), core::mem::transmute_copy(&coordinate), core::mem::transmute_copy(&origin)).into()
            }
        }
        unsafe extern "system" fn SetZoomBoundaries<Identity: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, zoomminimum: f32, zoommaximum: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationPrimaryContent_Impl::SetZoomBoundaries(this, core::mem::transmute_copy(&zoomminimum), core::mem::transmute_copy(&zoommaximum)).into()
            }
        }
        unsafe extern "system" fn SetHorizontalAlignment<Identity: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationPrimaryContent_Impl::SetHorizontalAlignment(this, core::mem::transmute_copy(&alignment)).into()
            }
        }
        unsafe extern "system" fn SetVerticalAlignment<Identity: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationPrimaryContent_Impl::SetVerticalAlignment(this, core::mem::transmute_copy(&alignment)).into()
            }
        }
        unsafe extern "system" fn GetInertiaEndTransform<Identity: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationPrimaryContent_Impl::GetInertiaEndTransform(this, core::mem::transmute_copy(&matrix), core::mem::transmute_copy(&pointcount)).into()
            }
        }
        unsafe extern "system" fn GetCenterPoint<Identity: IDirectManipulationPrimaryContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, centerx: *mut f32, centery: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationPrimaryContent_Impl::GetCenterPoint(this, core::mem::transmute_copy(&centerx), core::mem::transmute_copy(&centery)).into()
            }
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
impl windows_core::RuntimeName for IDirectManipulationPrimaryContent {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IDirectManipulationPrimaryContent2(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IDirectManipulationSnapPointsInertiaBehavior(pub u8);
windows_core::imp::define_interface!(IDirectManipulationUpdateHandler, IDirectManipulationUpdateHandler_Vtbl, 0x790b6337_64f8_4ff5_a269_b32bc2af27a7);
windows_core::imp::interface_hierarchy!(IDirectManipulationUpdateHandler, windows_core::IUnknown);
impl IDirectManipulationUpdateHandler {
    pub unsafe fn Update(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationUpdateHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDirectManipulationUpdateHandler_Impl: windows_core::IUnknownImpl {
    fn Update(&self) -> windows_core::Result<()>;
}
impl IDirectManipulationUpdateHandler_Vtbl {
    pub const fn new<Identity: IDirectManipulationUpdateHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Update<Identity: IDirectManipulationUpdateHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationUpdateHandler_Impl::Update(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectManipulationUpdateHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectManipulationUpdateHandler {}
windows_core::imp::define_interface!(IDirectManipulationUpdateManager, IDirectManipulationUpdateManager_Vtbl, 0xb0ae62fd_be34_46e7_9caa_d361facbb9cc);
windows_core::imp::interface_hierarchy!(IDirectManipulationUpdateManager, windows_core::IUnknown);
impl IDirectManipulationUpdateManager {
    #[cfg(feature = "winnt")]
    pub unsafe fn RegisterWaitHandleCallback<P1>(&self, handle: super::HANDLE, eventhandler: P1) -> windows_core::Result<u32>
    where
        P1: windows_core::Param<IDirectManipulationUpdateHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterWaitHandleCallback)(windows_core::Interface::as_raw(self), handle, eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterWaitHandleCallback(&self, cookie: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnregisterWaitHandleCallback)(windows_core::Interface::as_raw(self), cookie) }
    }
    pub unsafe fn Update<P0>(&self, frameinfo: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectManipulationFrameInfoProvider>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), frameinfo.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationUpdateManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub RegisterWaitHandleCallback: unsafe extern "system" fn(*mut core::ffi::c_void, super::HANDLE, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    RegisterWaitHandleCallback: usize,
    pub UnregisterWaitHandleCallback: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "winnt")]
pub trait IDirectManipulationUpdateManager_Impl: windows_core::IUnknownImpl {
    fn RegisterWaitHandleCallback(&self, handle: super::HANDLE, eventhandler: windows_core::Ref<IDirectManipulationUpdateHandler>) -> windows_core::Result<u32>;
    fn UnregisterWaitHandleCallback(&self, cookie: u32) -> windows_core::Result<()>;
    fn Update(&self, frameinfo: windows_core::Ref<IDirectManipulationFrameInfoProvider>) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl IDirectManipulationUpdateManager_Vtbl {
    pub const fn new<Identity: IDirectManipulationUpdateManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterWaitHandleCallback<Identity: IDirectManipulationUpdateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: super::HANDLE, eventhandler: *mut core::ffi::c_void, cookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectManipulationUpdateManager_Impl::RegisterWaitHandleCallback(this, core::mem::transmute_copy(&handle), core::mem::transmute_copy(&eventhandler)) {
                    Ok(ok__) => {
                        cookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterWaitHandleCallback<Identity: IDirectManipulationUpdateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationUpdateManager_Impl::UnregisterWaitHandleCallback(this, core::mem::transmute_copy(&cookie)).into()
            }
        }
        unsafe extern "system" fn Update<Identity: IDirectManipulationUpdateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, frameinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationUpdateManager_Impl::Update(this, core::mem::transmute_copy(&frameinfo)).into()
            }
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
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IDirectManipulationUpdateManager {}
windows_core::imp::define_interface!(IDirectManipulationViewport, IDirectManipulationViewport_Vtbl, 0x28b85a3d_60a0_48bd_9ba1_5ce8d9ea3a6d);
windows_core::imp::interface_hierarchy!(IDirectManipulationViewport, windows_core::IUnknown);
impl IDirectManipulationViewport {
    pub unsafe fn Enable(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Enable)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Disable(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Disable)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetContact(&self, pointerid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetContact)(windows_core::Interface::as_raw(self), pointerid) }
    }
    pub unsafe fn ReleaseContact(&self, pointerid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseContact)(windows_core::Interface::as_raw(self), pointerid) }
    }
    pub unsafe fn ReleaseAllContacts(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseAllContacts)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetStatus(&self) -> windows_core::Result<DIRECTMANIPULATION_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTag<T>(&self, id: Option<*mut u32>, result__: *mut Option<T>) -> windows_core::Result<()>
    where
        T: windows_core::Interface,
    {
        unsafe { (windows_core::Interface::vtable(self).GetTag)(windows_core::Interface::as_raw(self), &T::IID, result__ as *mut _ as *mut _, id.unwrap_or(core::mem::zeroed()) as _).ok() }
    }
    pub unsafe fn SetTag<P0>(&self, object: P0, id: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTag)(windows_core::Interface::as_raw(self), object.param().abi(), id) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetViewportRect(&self) -> windows_core::Result<super::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetViewportRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetViewportRect(&self, viewport: *const super::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetViewportRect)(windows_core::Interface::as_raw(self), viewport) }
    }
    pub unsafe fn ZoomToRect(&self, left: f32, top: f32, right: f32, bottom: f32, animate: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ZoomToRect)(windows_core::Interface::as_raw(self), left, top, right, bottom, animate.into()) }
    }
    pub unsafe fn SetViewportTransform(&self, matrix: &[f32]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetViewportTransform)(windows_core::Interface::as_raw(self), matrix.as_ptr(), matrix.len().try_into().unwrap()) }
    }
    pub unsafe fn SyncDisplayTransform(&self, matrix: &[f32]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SyncDisplayTransform)(windows_core::Interface::as_raw(self), matrix.as_ptr(), matrix.len().try_into().unwrap()) }
    }
    pub unsafe fn GetPrimaryContent<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetPrimaryContent)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn AddContent<P0>(&self, content: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectManipulationContent>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddContent)(windows_core::Interface::as_raw(self), content.param().abi()) }
    }
    pub unsafe fn RemoveContent<P0>(&self, content: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectManipulationContent>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveContent)(windows_core::Interface::as_raw(self), content.param().abi()) }
    }
    pub unsafe fn SetViewportOptions(&self, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetViewportOptions)(windows_core::Interface::as_raw(self), options) }
    }
    pub unsafe fn AddConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddConfiguration)(windows_core::Interface::as_raw(self), configuration) }
    }
    pub unsafe fn RemoveConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveConfiguration)(windows_core::Interface::as_raw(self), configuration) }
    }
    pub unsafe fn ActivateConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ActivateConfiguration)(windows_core::Interface::as_raw(self), configuration) }
    }
    pub unsafe fn SetManualGesture(&self, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetManualGesture)(windows_core::Interface::as_raw(self), configuration) }
    }
    pub unsafe fn SetChaining(&self, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetChaining)(windows_core::Interface::as_raw(self), enabledtypes) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn AddEventHandler<P1>(&self, window: Option<super::HWND>, eventhandler: P1) -> windows_core::Result<u32>
    where
        P1: windows_core::Param<IDirectManipulationViewportEventHandler>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddEventHandler)(windows_core::Interface::as_raw(self), window.unwrap_or(core::mem::zeroed()) as _, eventhandler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveEventHandler(&self, cookie: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveEventHandler)(windows_core::Interface::as_raw(self), cookie) }
    }
    pub unsafe fn SetInputMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInputMode)(windows_core::Interface::as_raw(self), mode) }
    }
    pub unsafe fn SetUpdateMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUpdateMode)(windows_core::Interface::as_raw(self), mode) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Abandon(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Abandon)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationViewport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Enable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Disable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetContact: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReleaseContact: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReleaseAllContacts: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DIRECTMANIPULATION_STATUS) -> windows_core::HRESULT,
    pub GetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTag: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetViewportRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetViewportRect: usize,
    #[cfg(feature = "windef")]
    pub SetViewportRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetViewportRect: usize,
    pub ZoomToRect: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetViewportTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32) -> windows_core::HRESULT,
    pub SyncDisplayTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const f32, u32) -> windows_core::HRESULT,
    pub GetPrimaryContent: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetViewportOptions: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> windows_core::HRESULT,
    pub AddConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_CONFIGURATION) -> windows_core::HRESULT,
    pub RemoveConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_CONFIGURATION) -> windows_core::HRESULT,
    pub ActivateConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_CONFIGURATION) -> windows_core::HRESULT,
    pub SetManualGesture: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> windows_core::HRESULT,
    pub SetChaining: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_MOTION_TYPES) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub AddEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    AddEventHandler: usize,
    pub RemoveEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetInputMode: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_INPUT_MODE) -> windows_core::HRESULT,
    pub SetUpdateMode: unsafe extern "system" fn(*mut core::ffi::c_void, DIRECTMANIPULATION_INPUT_MODE) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Abandon: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IDirectManipulationViewport_Impl: windows_core::IUnknownImpl {
    fn Enable(&self) -> windows_core::Result<()>;
    fn Disable(&self) -> windows_core::Result<()>;
    fn SetContact(&self, pointerid: u32) -> windows_core::Result<()>;
    fn ReleaseContact(&self, pointerid: u32) -> windows_core::Result<()>;
    fn ReleaseAllContacts(&self) -> windows_core::Result<()>;
    fn GetStatus(&self) -> windows_core::Result<DIRECTMANIPULATION_STATUS>;
    fn GetTag(&self, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void, id: *mut u32) -> windows_core::Result<()>;
    fn SetTag(&self, object: windows_core::Ref<windows_core::IUnknown>, id: u32) -> windows_core::Result<()>;
    fn GetViewportRect(&self) -> windows_core::Result<super::RECT>;
    fn SetViewportRect(&self, viewport: *const super::RECT) -> windows_core::Result<()>;
    fn ZoomToRect(&self, left: f32, top: f32, right: f32, bottom: f32, animate: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetViewportTransform(&self, matrix: *const f32, pointcount: u32) -> windows_core::Result<()>;
    fn SyncDisplayTransform(&self, matrix: *const f32, pointcount: u32) -> windows_core::Result<()>;
    fn GetPrimaryContent(&self, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn AddContent(&self, content: windows_core::Ref<IDirectManipulationContent>) -> windows_core::Result<()>;
    fn RemoveContent(&self, content: windows_core::Ref<IDirectManipulationContent>) -> windows_core::Result<()>;
    fn SetViewportOptions(&self, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> windows_core::Result<()>;
    fn AddConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> windows_core::Result<()>;
    fn RemoveConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> windows_core::Result<()>;
    fn ActivateConfiguration(&self, configuration: DIRECTMANIPULATION_CONFIGURATION) -> windows_core::Result<()>;
    fn SetManualGesture(&self, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> windows_core::Result<()>;
    fn SetChaining(&self, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> windows_core::Result<()>;
    fn AddEventHandler(&self, window: super::HWND, eventhandler: windows_core::Ref<IDirectManipulationViewportEventHandler>) -> windows_core::Result<u32>;
    fn RemoveEventHandler(&self, cookie: u32) -> windows_core::Result<()>;
    fn SetInputMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> windows_core::Result<()>;
    fn SetUpdateMode(&self, mode: DIRECTMANIPULATION_INPUT_MODE) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Abandon(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IDirectManipulationViewport_Vtbl {
    pub const fn new<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Enable<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::Enable(this).into()
            }
        }
        unsafe extern "system" fn Disable<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::Disable(this).into()
            }
        }
        unsafe extern "system" fn SetContact<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointerid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::SetContact(this, core::mem::transmute_copy(&pointerid)).into()
            }
        }
        unsafe extern "system" fn ReleaseContact<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pointerid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::ReleaseContact(this, core::mem::transmute_copy(&pointerid)).into()
            }
        }
        unsafe extern "system" fn ReleaseAllContacts<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::ReleaseAllContacts(this).into()
            }
        }
        unsafe extern "system" fn GetStatus<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: *mut DIRECTMANIPULATION_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectManipulationViewport_Impl::GetStatus(this) {
                    Ok(ok__) => {
                        status.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTag<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void, id: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::GetTag(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&object), core::mem::transmute_copy(&id)).into()
            }
        }
        unsafe extern "system" fn SetTag<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, id: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::SetTag(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&id)).into()
            }
        }
        unsafe extern "system" fn GetViewportRect<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewport: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectManipulationViewport_Impl::GetViewportRect(this) {
                    Ok(ok__) => {
                        viewport.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetViewportRect<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewport: *const super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::SetViewportRect(this, core::mem::transmute_copy(&viewport)).into()
            }
        }
        unsafe extern "system" fn ZoomToRect<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, left: f32, top: f32, right: f32, bottom: f32, animate: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::ZoomToRect(this, core::mem::transmute_copy(&left), core::mem::transmute_copy(&top), core::mem::transmute_copy(&right), core::mem::transmute_copy(&bottom), core::mem::transmute_copy(&animate)).into()
            }
        }
        unsafe extern "system" fn SetViewportTransform<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const f32, pointcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::SetViewportTransform(this, core::mem::transmute_copy(&matrix), core::mem::transmute_copy(&pointcount)).into()
            }
        }
        unsafe extern "system" fn SyncDisplayTransform<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, matrix: *const f32, pointcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::SyncDisplayTransform(this, core::mem::transmute_copy(&matrix), core::mem::transmute_copy(&pointcount)).into()
            }
        }
        unsafe extern "system" fn GetPrimaryContent<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, object: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::GetPrimaryContent(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&object)).into()
            }
        }
        unsafe extern "system" fn AddContent<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, content: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::AddContent(this, core::mem::transmute_copy(&content)).into()
            }
        }
        unsafe extern "system" fn RemoveContent<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, content: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::RemoveContent(this, core::mem::transmute_copy(&content)).into()
            }
        }
        unsafe extern "system" fn SetViewportOptions<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::SetViewportOptions(this, core::mem::transmute_copy(&options)).into()
            }
        }
        unsafe extern "system" fn AddConfiguration<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::AddConfiguration(this, core::mem::transmute_copy(&configuration)).into()
            }
        }
        unsafe extern "system" fn RemoveConfiguration<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::RemoveConfiguration(this, core::mem::transmute_copy(&configuration)).into()
            }
        }
        unsafe extern "system" fn ActivateConfiguration<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::ActivateConfiguration(this, core::mem::transmute_copy(&configuration)).into()
            }
        }
        unsafe extern "system" fn SetManualGesture<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::SetManualGesture(this, core::mem::transmute_copy(&configuration)).into()
            }
        }
        unsafe extern "system" fn SetChaining<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::SetChaining(this, core::mem::transmute_copy(&enabledtypes)).into()
            }
        }
        unsafe extern "system" fn AddEventHandler<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, window: super::HWND, eventhandler: *mut core::ffi::c_void, cookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectManipulationViewport_Impl::AddEventHandler(this, core::mem::transmute_copy(&window), core::mem::transmute_copy(&eventhandler)) {
                    Ok(ok__) => {
                        cookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveEventHandler<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::RemoveEventHandler(this, core::mem::transmute_copy(&cookie)).into()
            }
        }
        unsafe extern "system" fn SetInputMode<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::SetInputMode(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn SetUpdateMode<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::SetUpdateMode(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Abandon<Identity: IDirectManipulationViewport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport_Impl::Abandon(this).into()
            }
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
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IDirectManipulationViewport {}
windows_core::imp::define_interface!(IDirectManipulationViewport2, IDirectManipulationViewport2_Vtbl, 0x923ccaac_61e1_4385_b726_017af189882a);
impl core::ops::Deref for IDirectManipulationViewport2 {
    type Target = IDirectManipulationViewport;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDirectManipulationViewport2, windows_core::IUnknown, IDirectManipulationViewport);
impl IDirectManipulationViewport2 {
    pub unsafe fn AddBehavior<P0>(&self, behavior: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddBehavior)(windows_core::Interface::as_raw(self), behavior.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemoveBehavior(&self, cookie: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveBehavior)(windows_core::Interface::as_raw(self), cookie) }
    }
    pub unsafe fn RemoveAllBehaviors(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllBehaviors)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationViewport2_Vtbl {
    pub base__: IDirectManipulationViewport_Vtbl,
    pub AddBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub RemoveBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RemoveAllBehaviors: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IDirectManipulationViewport2_Impl: IDirectManipulationViewport_Impl {
    fn AddBehavior(&self, behavior: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<u32>;
    fn RemoveBehavior(&self, cookie: u32) -> windows_core::Result<()>;
    fn RemoveAllBehaviors(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IDirectManipulationViewport2_Vtbl {
    pub const fn new<Identity: IDirectManipulationViewport2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddBehavior<Identity: IDirectManipulationViewport2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, behavior: *mut core::ffi::c_void, cookie: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectManipulationViewport2_Impl::AddBehavior(this, core::mem::transmute_copy(&behavior)) {
                    Ok(ok__) => {
                        cookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveBehavior<Identity: IDirectManipulationViewport2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cookie: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport2_Impl::RemoveBehavior(this, core::mem::transmute_copy(&cookie)).into()
            }
        }
        unsafe extern "system" fn RemoveAllBehaviors<Identity: IDirectManipulationViewport2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewport2_Impl::RemoveAllBehaviors(this).into()
            }
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
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IDirectManipulationViewport2 {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IDirectManipulationViewport3(pub u8);
windows_core::imp::define_interface!(IDirectManipulationViewportEventHandler, IDirectManipulationViewportEventHandler_Vtbl, 0x952121da_d69f_45f9_b0f9_f23944321a6d);
windows_core::imp::interface_hierarchy!(IDirectManipulationViewportEventHandler, windows_core::IUnknown);
impl IDirectManipulationViewportEventHandler {
    pub unsafe fn OnViewportStatusChanged<P0>(&self, viewport: P0, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectManipulationViewport>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnViewportStatusChanged)(windows_core::Interface::as_raw(self), viewport.param().abi(), current, previous) }
    }
    pub unsafe fn OnViewportUpdated<P0>(&self, viewport: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectManipulationViewport>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnViewportUpdated)(windows_core::Interface::as_raw(self), viewport.param().abi()) }
    }
    pub unsafe fn OnContentUpdated<P0, P1>(&self, viewport: P0, content: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDirectManipulationViewport>,
        P1: windows_core::Param<IDirectManipulationContent>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnContentUpdated)(windows_core::Interface::as_raw(self), viewport.param().abi(), content.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectManipulationViewportEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnViewportStatusChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, DIRECTMANIPULATION_STATUS, DIRECTMANIPULATION_STATUS) -> windows_core::HRESULT,
    pub OnViewportUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnContentUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDirectManipulationViewportEventHandler_Impl: windows_core::IUnknownImpl {
    fn OnViewportStatusChanged(&self, viewport: windows_core::Ref<IDirectManipulationViewport>, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> windows_core::Result<()>;
    fn OnViewportUpdated(&self, viewport: windows_core::Ref<IDirectManipulationViewport>) -> windows_core::Result<()>;
    fn OnContentUpdated(&self, viewport: windows_core::Ref<IDirectManipulationViewport>, content: windows_core::Ref<IDirectManipulationContent>) -> windows_core::Result<()>;
}
impl IDirectManipulationViewportEventHandler_Vtbl {
    pub const fn new<Identity: IDirectManipulationViewportEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnViewportStatusChanged<Identity: IDirectManipulationViewportEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewport: *mut core::ffi::c_void, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewportEventHandler_Impl::OnViewportStatusChanged(this, core::mem::transmute_copy(&viewport), core::mem::transmute_copy(&current), core::mem::transmute_copy(&previous)).into()
            }
        }
        unsafe extern "system" fn OnViewportUpdated<Identity: IDirectManipulationViewportEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewport: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewportEventHandler_Impl::OnViewportUpdated(this, core::mem::transmute_copy(&viewport)).into()
            }
        }
        unsafe extern "system" fn OnContentUpdated<Identity: IDirectManipulationViewportEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewport: *mut core::ffi::c_void, content: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectManipulationViewportEventHandler_Impl::OnContentUpdated(this, core::mem::transmute_copy(&viewport), core::mem::transmute_copy(&content)).into()
            }
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
impl windows_core::RuntimeName for IDirectManipulationViewportEventHandler {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IDirectManipulationViewportPartner(pub u8);
