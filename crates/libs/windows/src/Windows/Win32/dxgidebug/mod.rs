pub const DXGI_DEBUG_ALL: windows_core::GUID = windows_core::GUID::from_u128(0xe48ae283_da80_490b_87e6_43e9a9cfda08);
pub const DXGI_DEBUG_APP: windows_core::GUID = windows_core::GUID::from_u128(0x06cd6e01_4219_4ebd_8709_27ed23360c62);
pub const DXGI_DEBUG_BINARY_VERSION: u32 = 1;
pub const DXGI_DEBUG_DX: windows_core::GUID = windows_core::GUID::from_u128(0x35cdd7fc_13b2_421d_a5d7_7e4451287d64);
pub const DXGI_DEBUG_DXGI: windows_core::GUID = windows_core::GUID::from_u128(0x25cddaa4_b1c6_47e1_ac3e_98875b5a2e2a);
pub type DXGI_DEBUG_ID = windows_core::GUID;
pub const DXGI_DEBUG_RLO_ALL: DXGI_DEBUG_RLO_FLAGS = 7;
pub const DXGI_DEBUG_RLO_DETAIL: DXGI_DEBUG_RLO_FLAGS = 2;
pub type DXGI_DEBUG_RLO_FLAGS = i32;
pub const DXGI_DEBUG_RLO_IGNORE_INTERNAL: DXGI_DEBUG_RLO_FLAGS = 4;
pub const DXGI_DEBUG_RLO_SUMMARY: DXGI_DEBUG_RLO_FLAGS = 1;
pub const DXGI_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DXGI_INFO_QUEUE_FILTER {
    pub AllowList: DXGI_INFO_QUEUE_FILTER_DESC,
    pub DenyList: DXGI_INFO_QUEUE_FILTER_DESC,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut DXGI_INFO_QUEUE_MESSAGE_ID,
}
impl Default for DXGI_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DXGI_INFO_QUEUE_MESSAGE {
    pub Producer: DXGI_DEBUG_ID,
    pub Category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY,
    pub Severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY,
    pub ID: DXGI_INFO_QUEUE_MESSAGE_ID,
    pub pDescription: *const i8,
    pub DescriptionByteLength: usize,
}
impl Default for DXGI_INFO_QUEUE_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGI_INFO_QUEUE_MESSAGE_CATEGORY = i32;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_CLEANUP: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 3;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_COMPILATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 4;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_EXECUTION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 9;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_INITIALIZATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 2;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_MISCELLANEOUS: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 1;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 8;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_SHADER: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 10;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_CREATION: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 5;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_GETTING: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 7;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_SETTING: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 6;
pub const DXGI_INFO_QUEUE_MESSAGE_CATEGORY_UNKNOWN: DXGI_INFO_QUEUE_MESSAGE_CATEGORY = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DXGI_INFO_QUEUE_MESSAGE_ID(pub i32);
pub const DXGI_INFO_QUEUE_MESSAGE_ID_STRING_FROM_APPLICATION: u32 = 0;
pub type DXGI_INFO_QUEUE_MESSAGE_SEVERITY = i32;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_CORRUPTION: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 0;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_ERROR: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 1;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_INFO: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 3;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_MESSAGE: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 4;
pub const DXGI_INFO_QUEUE_MESSAGE_SEVERITY_WARNING: DXGI_INFO_QUEUE_MESSAGE_SEVERITY = 2;
windows_core::imp::define_interface!(IDXGIDebug, IDXGIDebug_Vtbl, 0x119e7452_de9e_40fe_8806_88f90c12b441);
windows_core::imp::interface_hierarchy!(IDXGIDebug, windows_core::IUnknown);
impl IDXGIDebug {
    pub unsafe fn ReportLiveObjects(&self, apiid: windows_core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReportLiveObjects)(windows_core::Interface::as_raw(self), core::mem::transmute(apiid), flags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDebug_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReportLiveObjects: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, DXGI_DEBUG_RLO_FLAGS) -> windows_core::HRESULT,
}
pub trait IDXGIDebug_Impl: windows_core::IUnknownImpl {
    fn ReportLiveObjects(&self, apiid: &windows_core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> windows_core::Result<()>;
}
impl IDXGIDebug_Vtbl {
    pub const fn new<Identity: IDXGIDebug_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReportLiveObjects<Identity: IDXGIDebug_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, apiid: windows_core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDebug_Impl::ReportLiveObjects(this, core::mem::transmute(&apiid), core::mem::transmute_copy(&flags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ReportLiveObjects: ReportLiveObjects::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDebug as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIDebug {}
windows_core::imp::define_interface!(IDXGIDebug1, IDXGIDebug1_Vtbl, 0xc5a05f0c_16f2_4adf_9f4d_a8c4d58ac550);
impl core::ops::Deref for IDXGIDebug1 {
    type Target = IDXGIDebug;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IDXGIDebug1, windows_core::IUnknown, IDXGIDebug);
impl IDXGIDebug1 {
    pub unsafe fn EnableLeakTrackingForThread(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).EnableLeakTrackingForThread)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn DisableLeakTrackingForThread(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).DisableLeakTrackingForThread)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn IsLeakTrackingEnabledForThread(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsLeakTrackingEnabledForThread)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIDebug1_Vtbl {
    pub base__: IDXGIDebug_Vtbl,
    pub EnableLeakTrackingForThread: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub DisableLeakTrackingForThread: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub IsLeakTrackingEnabledForThread: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
pub trait IDXGIDebug1_Impl: IDXGIDebug_Impl {
    fn EnableLeakTrackingForThread(&self);
    fn DisableLeakTrackingForThread(&self);
    fn IsLeakTrackingEnabledForThread(&self) -> windows_core::BOOL;
}
impl IDXGIDebug1_Vtbl {
    pub const fn new<Identity: IDXGIDebug1_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EnableLeakTrackingForThread<Identity: IDXGIDebug1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDebug1_Impl::EnableLeakTrackingForThread(this);
            }
        }
        unsafe extern "system" fn DisableLeakTrackingForThread<Identity: IDXGIDebug1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDebug1_Impl::DisableLeakTrackingForThread(this);
            }
        }
        unsafe extern "system" fn IsLeakTrackingEnabledForThread<Identity: IDXGIDebug1_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIDebug1_Impl::IsLeakTrackingEnabledForThread(this)
            }
        }
        Self {
            base__: IDXGIDebug_Vtbl::new::<Identity, OFFSET>(),
            EnableLeakTrackingForThread: EnableLeakTrackingForThread::<Identity, OFFSET>,
            DisableLeakTrackingForThread: DisableLeakTrackingForThread::<Identity, OFFSET>,
            IsLeakTrackingEnabledForThread: IsLeakTrackingEnabledForThread::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIDebug1 as windows_core::Interface>::IID || iid == &<IDXGIDebug as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIDebug1 {}
windows_core::imp::define_interface!(IDXGIInfoQueue, IDXGIInfoQueue_Vtbl, 0xd67441c7_672a_476f_9e82_cd55b44949ce);
windows_core::imp::interface_hierarchy!(IDXGIInfoQueue, windows_core::IUnknown);
impl IDXGIInfoQueue {
    pub unsafe fn SetMessageCountLimit(&self, producer: DXGI_DEBUG_ID, messagecountlimit: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMessageCountLimit)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), messagecountlimit) }
    }
    pub unsafe fn ClearStoredMessages(&self, producer: DXGI_DEBUG_ID) {
        unsafe {
            (windows_core::Interface::vtable(self).ClearStoredMessages)(windows_core::Interface::as_raw(self), core::mem::transmute(producer));
        }
    }
    pub unsafe fn GetMessage(&self, producer: DXGI_DEBUG_ID, messageindex: u64, pmessage: Option<*mut DXGI_INFO_QUEUE_MESSAGE>, pmessagebytelength: *mut usize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMessage)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), messageindex, pmessage.unwrap_or(core::mem::zeroed()) as _, pmessagebytelength as _) }
    }
    pub unsafe fn GetNumStoredMessagesAllowedByRetrievalFilters(&self, producer: DXGI_DEBUG_ID) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNumStoredMessagesAllowedByRetrievalFilters)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn GetNumStoredMessages(&self, producer: DXGI_DEBUG_ID) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNumStoredMessages)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn GetNumMessagesDiscardedByMessageCountLimit(&self, producer: DXGI_DEBUG_ID) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNumMessagesDiscardedByMessageCountLimit)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn GetMessageCountLimit(&self, producer: DXGI_DEBUG_ID) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetMessageCountLimit)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn GetNumMessagesAllowedByStorageFilter(&self, producer: DXGI_DEBUG_ID) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNumMessagesAllowedByStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn GetNumMessagesDeniedByStorageFilter(&self, producer: DXGI_DEBUG_ID) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetNumMessagesDeniedByStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn AddStorageFilterEntries(&self, producer: DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddStorageFilterEntries)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), pfilter) }
    }
    pub unsafe fn GetStorageFilter(&self, producer: DXGI_DEBUG_ID, pfilter: Option<*mut DXGI_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), pfilter.unwrap_or(core::mem::zeroed()) as _, pfilterbytelength as _) }
    }
    pub unsafe fn ClearStorageFilter(&self, producer: DXGI_DEBUG_ID) {
        unsafe {
            (windows_core::Interface::vtable(self).ClearStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer));
        }
    }
    pub unsafe fn PushEmptyStorageFilter(&self, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PushEmptyStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn PushDenyAllStorageFilter(&self, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PushDenyAllStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn PushCopyOfStorageFilter(&self, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PushCopyOfStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn PushStorageFilter(&self, producer: DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PushStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), pfilter) }
    }
    pub unsafe fn PopStorageFilter(&self, producer: DXGI_DEBUG_ID) {
        unsafe {
            (windows_core::Interface::vtable(self).PopStorageFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer));
        }
    }
    pub unsafe fn GetStorageFilterStackSize(&self, producer: DXGI_DEBUG_ID) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetStorageFilterStackSize)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn AddRetrievalFilterEntries(&self, producer: DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddRetrievalFilterEntries)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), pfilter) }
    }
    pub unsafe fn GetRetrievalFilter(&self, producer: DXGI_DEBUG_ID, pfilter: Option<*mut DXGI_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), pfilter.unwrap_or(core::mem::zeroed()) as _, pfilterbytelength as _) }
    }
    pub unsafe fn ClearRetrievalFilter(&self, producer: DXGI_DEBUG_ID) {
        unsafe {
            (windows_core::Interface::vtable(self).ClearRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer));
        }
    }
    pub unsafe fn PushEmptyRetrievalFilter(&self, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PushEmptyRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn PushDenyAllRetrievalFilter(&self, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PushDenyAllRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn PushCopyOfRetrievalFilter(&self, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PushCopyOfRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn PushRetrievalFilter(&self, producer: DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PushRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), pfilter) }
    }
    pub unsafe fn PopRetrievalFilter(&self, producer: DXGI_DEBUG_ID) {
        unsafe {
            (windows_core::Interface::vtable(self).PopRetrievalFilter)(windows_core::Interface::as_raw(self), core::mem::transmute(producer));
        }
    }
    pub unsafe fn GetRetrievalFilterStackSize(&self, producer: DXGI_DEBUG_ID) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetRetrievalFilterStackSize)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
    pub unsafe fn AddMessage<P4>(&self, producer: DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: DXGI_INFO_QUEUE_MESSAGE_ID, pdescription: P4) -> windows_core::HRESULT
    where
        P4: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddMessage)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), category, severity, id, pdescription.param().abi()) }
    }
    pub unsafe fn AddApplicationMessage<P1>(&self, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddApplicationMessage)(windows_core::Interface::as_raw(self), severity, pdescription.param().abi()) }
    }
    pub unsafe fn SetBreakOnCategory(&self, producer: DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBreakOnCategory)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), category, benable.into()) }
    }
    pub unsafe fn SetBreakOnSeverity(&self, producer: DXGI_DEBUG_ID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBreakOnSeverity)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), severity, benable.into()) }
    }
    pub unsafe fn SetBreakOnID(&self, producer: DXGI_DEBUG_ID, id: DXGI_INFO_QUEUE_MESSAGE_ID, benable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBreakOnID)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), id, benable.into()) }
    }
    pub unsafe fn GetBreakOnCategory(&self, producer: DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetBreakOnCategory)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), category) }
    }
    pub unsafe fn GetBreakOnSeverity(&self, producer: DXGI_DEBUG_ID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetBreakOnSeverity)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), severity) }
    }
    pub unsafe fn GetBreakOnID(&self, producer: DXGI_DEBUG_ID, id: DXGI_INFO_QUEUE_MESSAGE_ID) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetBreakOnID)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), id) }
    }
    pub unsafe fn SetMuteDebugOutput(&self, producer: DXGI_DEBUG_ID, bmute: bool) {
        unsafe {
            (windows_core::Interface::vtable(self).SetMuteDebugOutput)(windows_core::Interface::as_raw(self), core::mem::transmute(producer), bmute.into());
        }
    }
    pub unsafe fn GetMuteDebugOutput(&self, producer: DXGI_DEBUG_ID) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetMuteDebugOutput)(windows_core::Interface::as_raw(self), core::mem::transmute(producer)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDXGIInfoQueue_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMessageCountLimit: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, u64) -> windows_core::HRESULT,
    pub ClearStoredMessages: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID),
    pub GetMessage: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, u64, *mut DXGI_INFO_QUEUE_MESSAGE, *mut usize) -> windows_core::HRESULT,
    pub GetNumStoredMessagesAllowedByRetrievalFilters: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> u64,
    pub GetNumStoredMessages: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> u64,
    pub GetNumMessagesDiscardedByMessageCountLimit: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> u64,
    pub GetMessageCountLimit: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> u64,
    pub GetNumMessagesAllowedByStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> u64,
    pub GetNumMessagesDeniedByStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> u64,
    pub AddStorageFilterEntries: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub GetStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, *mut DXGI_INFO_QUEUE_FILTER, *mut usize) -> windows_core::HRESULT,
    pub ClearStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID),
    pub PushEmptyStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> windows_core::HRESULT,
    pub PushDenyAllStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> windows_core::HRESULT,
    pub PushCopyOfStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> windows_core::HRESULT,
    pub PushStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub PopStorageFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID),
    pub GetStorageFilterStackSize: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> u32,
    pub AddRetrievalFilterEntries: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub GetRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, *mut DXGI_INFO_QUEUE_FILTER, *mut usize) -> windows_core::HRESULT,
    pub ClearRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID),
    pub PushEmptyRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> windows_core::HRESULT,
    pub PushDenyAllRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> windows_core::HRESULT,
    pub PushCopyOfRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> windows_core::HRESULT,
    pub PushRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT,
    pub PopRetrievalFilter: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID),
    pub GetRetrievalFilterStackSize: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> u32,
    pub AddMessage: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, DXGI_INFO_QUEUE_MESSAGE_CATEGORY, DXGI_INFO_QUEUE_MESSAGE_SEVERITY, DXGI_INFO_QUEUE_MESSAGE_ID, windows_core::PCSTR) -> windows_core::HRESULT,
    pub AddApplicationMessage: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_INFO_QUEUE_MESSAGE_SEVERITY, windows_core::PCSTR) -> windows_core::HRESULT,
    pub SetBreakOnCategory: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, DXGI_INFO_QUEUE_MESSAGE_CATEGORY, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetBreakOnSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, DXGI_INFO_QUEUE_MESSAGE_SEVERITY, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetBreakOnID: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, DXGI_INFO_QUEUE_MESSAGE_ID, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetBreakOnCategory: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> windows_core::BOOL,
    pub GetBreakOnSeverity: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> windows_core::BOOL,
    pub GetBreakOnID: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, DXGI_INFO_QUEUE_MESSAGE_ID) -> windows_core::BOOL,
    pub SetMuteDebugOutput: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID, windows_core::BOOL),
    pub GetMuteDebugOutput: unsafe extern "system" fn(*mut core::ffi::c_void, DXGI_DEBUG_ID) -> windows_core::BOOL,
}
pub trait IDXGIInfoQueue_Impl: windows_core::IUnknownImpl {
    fn SetMessageCountLimit(&self, producer: &DXGI_DEBUG_ID, messagecountlimit: u64) -> windows_core::Result<()>;
    fn ClearStoredMessages(&self, producer: &DXGI_DEBUG_ID);
    fn GetMessage(&self, producer: &DXGI_DEBUG_ID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> windows_core::Result<()>;
    fn GetNumStoredMessagesAllowedByRetrievalFilters(&self, producer: &DXGI_DEBUG_ID) -> u64;
    fn GetNumStoredMessages(&self, producer: &DXGI_DEBUG_ID) -> u64;
    fn GetNumMessagesDiscardedByMessageCountLimit(&self, producer: &DXGI_DEBUG_ID) -> u64;
    fn GetMessageCountLimit(&self, producer: &DXGI_DEBUG_ID) -> u64;
    fn GetNumMessagesAllowedByStorageFilter(&self, producer: &DXGI_DEBUG_ID) -> u64;
    fn GetNumMessagesDeniedByStorageFilter(&self, producer: &DXGI_DEBUG_ID) -> u64;
    fn AddStorageFilterEntries(&self, producer: &DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn GetStorageFilter(&self, producer: &DXGI_DEBUG_ID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::Result<()>;
    fn ClearStorageFilter(&self, producer: &DXGI_DEBUG_ID);
    fn PushEmptyStorageFilter(&self, producer: &DXGI_DEBUG_ID) -> windows_core::Result<()>;
    fn PushDenyAllStorageFilter(&self, producer: &DXGI_DEBUG_ID) -> windows_core::Result<()>;
    fn PushCopyOfStorageFilter(&self, producer: &DXGI_DEBUG_ID) -> windows_core::Result<()>;
    fn PushStorageFilter(&self, producer: &DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn PopStorageFilter(&self, producer: &DXGI_DEBUG_ID);
    fn GetStorageFilterStackSize(&self, producer: &DXGI_DEBUG_ID) -> u32;
    fn AddRetrievalFilterEntries(&self, producer: &DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn GetRetrievalFilter(&self, producer: &DXGI_DEBUG_ID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::Result<()>;
    fn ClearRetrievalFilter(&self, producer: &DXGI_DEBUG_ID);
    fn PushEmptyRetrievalFilter(&self, producer: &DXGI_DEBUG_ID) -> windows_core::Result<()>;
    fn PushDenyAllRetrievalFilter(&self, producer: &DXGI_DEBUG_ID) -> windows_core::Result<()>;
    fn PushCopyOfRetrievalFilter(&self, producer: &DXGI_DEBUG_ID) -> windows_core::Result<()>;
    fn PushRetrievalFilter(&self, producer: &DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::Result<()>;
    fn PopRetrievalFilter(&self, producer: &DXGI_DEBUG_ID);
    fn GetRetrievalFilterStackSize(&self, producer: &DXGI_DEBUG_ID) -> u32;
    fn AddMessage(&self, producer: &DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: DXGI_INFO_QUEUE_MESSAGE_ID, pdescription: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn AddApplicationMessage(&self, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: &windows_core::PCSTR) -> windows_core::Result<()>;
    fn SetBreakOnCategory(&self, producer: &DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetBreakOnSeverity(&self, producer: &DXGI_DEBUG_ID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetBreakOnID(&self, producer: &DXGI_DEBUG_ID, id: DXGI_INFO_QUEUE_MESSAGE_ID, benable: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetBreakOnCategory(&self, producer: &DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> windows_core::BOOL;
    fn GetBreakOnSeverity(&self, producer: &DXGI_DEBUG_ID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> windows_core::BOOL;
    fn GetBreakOnID(&self, producer: &DXGI_DEBUG_ID, id: DXGI_INFO_QUEUE_MESSAGE_ID) -> windows_core::BOOL;
    fn SetMuteDebugOutput(&self, producer: &DXGI_DEBUG_ID, bmute: windows_core::BOOL);
    fn GetMuteDebugOutput(&self, producer: &DXGI_DEBUG_ID) -> windows_core::BOOL;
}
impl IDXGIInfoQueue_Vtbl {
    pub const fn new<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMessageCountLimit<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, messagecountlimit: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::SetMessageCountLimit(this, core::mem::transmute(&producer), core::mem::transmute_copy(&messagecountlimit)).into()
            }
        }
        unsafe extern "system" fn ClearStoredMessages<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::ClearStoredMessages(this, core::mem::transmute(&producer));
            }
        }
        unsafe extern "system" fn GetMessage<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetMessage(this, core::mem::transmute(&producer), core::mem::transmute_copy(&messageindex), core::mem::transmute_copy(&pmessage), core::mem::transmute_copy(&pmessagebytelength)).into()
            }
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilters<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetNumStoredMessagesAllowedByRetrievalFilters(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn GetNumStoredMessages<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetNumStoredMessages(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetNumMessagesDiscardedByMessageCountLimit(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn GetMessageCountLimit<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetMessageCountLimit(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetNumMessagesAllowedByStorageFilter(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetNumMessagesDeniedByStorageFilter(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn AddStorageFilterEntries<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::AddStorageFilterEntries(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter)).into()
            }
        }
        unsafe extern "system" fn GetStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetStorageFilter(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&pfilterbytelength)).into()
            }
        }
        unsafe extern "system" fn ClearStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::ClearStorageFilter(this, core::mem::transmute(&producer));
            }
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushEmptyStorageFilter(this, core::mem::transmute(&producer)).into()
            }
        }
        unsafe extern "system" fn PushDenyAllStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushDenyAllStorageFilter(this, core::mem::transmute(&producer)).into()
            }
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushCopyOfStorageFilter(this, core::mem::transmute(&producer)).into()
            }
        }
        unsafe extern "system" fn PushStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushStorageFilter(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter)).into()
            }
        }
        unsafe extern "system" fn PopStorageFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PopStorageFilter(this, core::mem::transmute(&producer));
            }
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetStorageFilterStackSize(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::AddRetrievalFilterEntries(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter)).into()
            }
        }
        unsafe extern "system" fn GetRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetRetrievalFilter(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter), core::mem::transmute_copy(&pfilterbytelength)).into()
            }
        }
        unsafe extern "system" fn ClearRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::ClearRetrievalFilter(this, core::mem::transmute(&producer));
            }
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushEmptyRetrievalFilter(this, core::mem::transmute(&producer)).into()
            }
        }
        unsafe extern "system" fn PushDenyAllRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushDenyAllRetrievalFilter(this, core::mem::transmute(&producer)).into()
            }
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushCopyOfRetrievalFilter(this, core::mem::transmute(&producer)).into()
            }
        }
        unsafe extern "system" fn PushRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PushRetrievalFilter(this, core::mem::transmute(&producer), core::mem::transmute_copy(&pfilter)).into()
            }
        }
        unsafe extern "system" fn PopRetrievalFilter<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::PopRetrievalFilter(this, core::mem::transmute(&producer));
            }
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetRetrievalFilterStackSize(this, core::mem::transmute(&producer))
            }
        }
        unsafe extern "system" fn AddMessage<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: DXGI_INFO_QUEUE_MESSAGE_ID, pdescription: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::AddMessage(this, core::mem::transmute(&producer), core::mem::transmute_copy(&category), core::mem::transmute_copy(&severity), core::mem::transmute_copy(&id), core::mem::transmute(&pdescription)).into()
            }
        }
        unsafe extern "system" fn AddApplicationMessage<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: windows_core::PCSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::AddApplicationMessage(this, core::mem::transmute_copy(&severity), core::mem::transmute(&pdescription)).into()
            }
        }
        unsafe extern "system" fn SetBreakOnCategory<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::SetBreakOnCategory(this, core::mem::transmute(&producer), core::mem::transmute_copy(&category), core::mem::transmute_copy(&benable)).into()
            }
        }
        unsafe extern "system" fn SetBreakOnSeverity<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::SetBreakOnSeverity(this, core::mem::transmute(&producer), core::mem::transmute_copy(&severity), core::mem::transmute_copy(&benable)).into()
            }
        }
        unsafe extern "system" fn SetBreakOnID<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, id: DXGI_INFO_QUEUE_MESSAGE_ID, benable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::SetBreakOnID(this, core::mem::transmute(&producer), core::mem::transmute_copy(&id), core::mem::transmute_copy(&benable)).into()
            }
        }
        unsafe extern "system" fn GetBreakOnCategory<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetBreakOnCategory(this, core::mem::transmute(&producer), core::mem::transmute_copy(&category))
            }
        }
        unsafe extern "system" fn GetBreakOnSeverity<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetBreakOnSeverity(this, core::mem::transmute(&producer), core::mem::transmute_copy(&severity))
            }
        }
        unsafe extern "system" fn GetBreakOnID<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, id: DXGI_INFO_QUEUE_MESSAGE_ID) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetBreakOnID(this, core::mem::transmute(&producer), core::mem::transmute_copy(&id))
            }
        }
        unsafe extern "system" fn SetMuteDebugOutput<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID, bmute: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::SetMuteDebugOutput(this, core::mem::transmute(&producer), core::mem::transmute_copy(&bmute));
            }
        }
        unsafe extern "system" fn GetMuteDebugOutput<Identity: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, producer: DXGI_DEBUG_ID) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDXGIInfoQueue_Impl::GetMuteDebugOutput(this, core::mem::transmute(&producer))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMessageCountLimit: SetMessageCountLimit::<Identity, OFFSET>,
            ClearStoredMessages: ClearStoredMessages::<Identity, OFFSET>,
            GetMessage: GetMessage::<Identity, OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilters: GetNumStoredMessagesAllowedByRetrievalFilters::<Identity, OFFSET>,
            GetNumStoredMessages: GetNumStoredMessages::<Identity, OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit: GetNumMessagesDiscardedByMessageCountLimit::<Identity, OFFSET>,
            GetMessageCountLimit: GetMessageCountLimit::<Identity, OFFSET>,
            GetNumMessagesAllowedByStorageFilter: GetNumMessagesAllowedByStorageFilter::<Identity, OFFSET>,
            GetNumMessagesDeniedByStorageFilter: GetNumMessagesDeniedByStorageFilter::<Identity, OFFSET>,
            AddStorageFilterEntries: AddStorageFilterEntries::<Identity, OFFSET>,
            GetStorageFilter: GetStorageFilter::<Identity, OFFSET>,
            ClearStorageFilter: ClearStorageFilter::<Identity, OFFSET>,
            PushEmptyStorageFilter: PushEmptyStorageFilter::<Identity, OFFSET>,
            PushDenyAllStorageFilter: PushDenyAllStorageFilter::<Identity, OFFSET>,
            PushCopyOfStorageFilter: PushCopyOfStorageFilter::<Identity, OFFSET>,
            PushStorageFilter: PushStorageFilter::<Identity, OFFSET>,
            PopStorageFilter: PopStorageFilter::<Identity, OFFSET>,
            GetStorageFilterStackSize: GetStorageFilterStackSize::<Identity, OFFSET>,
            AddRetrievalFilterEntries: AddRetrievalFilterEntries::<Identity, OFFSET>,
            GetRetrievalFilter: GetRetrievalFilter::<Identity, OFFSET>,
            ClearRetrievalFilter: ClearRetrievalFilter::<Identity, OFFSET>,
            PushEmptyRetrievalFilter: PushEmptyRetrievalFilter::<Identity, OFFSET>,
            PushDenyAllRetrievalFilter: PushDenyAllRetrievalFilter::<Identity, OFFSET>,
            PushCopyOfRetrievalFilter: PushCopyOfRetrievalFilter::<Identity, OFFSET>,
            PushRetrievalFilter: PushRetrievalFilter::<Identity, OFFSET>,
            PopRetrievalFilter: PopRetrievalFilter::<Identity, OFFSET>,
            GetRetrievalFilterStackSize: GetRetrievalFilterStackSize::<Identity, OFFSET>,
            AddMessage: AddMessage::<Identity, OFFSET>,
            AddApplicationMessage: AddApplicationMessage::<Identity, OFFSET>,
            SetBreakOnCategory: SetBreakOnCategory::<Identity, OFFSET>,
            SetBreakOnSeverity: SetBreakOnSeverity::<Identity, OFFSET>,
            SetBreakOnID: SetBreakOnID::<Identity, OFFSET>,
            GetBreakOnCategory: GetBreakOnCategory::<Identity, OFFSET>,
            GetBreakOnSeverity: GetBreakOnSeverity::<Identity, OFFSET>,
            GetBreakOnID: GetBreakOnID::<Identity, OFFSET>,
            SetMuteDebugOutput: SetMuteDebugOutput::<Identity, OFFSET>,
            GetMuteDebugOutput: GetMuteDebugOutput::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXGIInfoQueue as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDXGIInfoQueue {}
