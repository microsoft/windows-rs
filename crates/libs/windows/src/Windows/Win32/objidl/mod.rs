pub type ADVF = i32;
pub const ADVFCACHE_FORCEBUILTIN: ADVF = 16;
pub const ADVFCACHE_NOHANDLER: ADVF = 8;
pub const ADVFCACHE_ONSAVE: ADVF = 32;
pub const ADVF_DATAONSTOP: ADVF = 64;
pub const ADVF_NODATA: ADVF = 1;
pub const ADVF_ONLYONCE: ADVF = 4;
pub const ADVF_PRIMEFIRST: ADVF = 2;
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub type ASYNC_STGMEDIUM = STGMEDIUM;
pub type ApplicationType = i32;
windows_core::imp::define_interface!(AsyncIAdviseSink, AsyncIAdviseSink_Vtbl, 0x00000150_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(AsyncIAdviseSink, windows_core::IUnknown);
impl AsyncIAdviseSink {
    #[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub unsafe fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        unsafe {
            (windows_core::Interface::vtable(self).Begin_OnDataChange)(windows_core::Interface::as_raw(self), pformatetc, core::mem::transmute(pstgmed));
        }
    }
    pub unsafe fn Finish_OnDataChange(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Finish_OnDataChange)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32) {
        unsafe {
            (windows_core::Interface::vtable(self).Begin_OnViewChange)(windows_core::Interface::as_raw(self), dwaspect, lindex);
        }
    }
    pub unsafe fn Finish_OnViewChange(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Finish_OnViewChange)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Begin_OnRename<P0>(&self, pmk: P0)
    where
        P0: windows_core::Param<IMoniker>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Begin_OnRename)(windows_core::Interface::as_raw(self), pmk.param().abi());
        }
    }
    pub unsafe fn Finish_OnRename(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Finish_OnRename)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Begin_OnSave(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Begin_OnSave)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Finish_OnSave(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Finish_OnSave)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Begin_OnClose(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Begin_OnClose)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn Finish_OnClose(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Finish_OnClose)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAdviseSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub Begin_OnDataChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const FORMATETC, *const STGMEDIUM),
    #[cfg(not(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes")))]
    Begin_OnDataChange: usize,
    pub Finish_OnDataChange: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Begin_OnViewChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32),
    pub Finish_OnViewChange: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Begin_OnRename: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub Finish_OnRename: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Begin_OnSave: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Finish_OnSave: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Begin_OnClose: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub Finish_OnClose: unsafe extern "system" fn(*mut core::ffi::c_void),
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub trait AsyncIAdviseSink_Impl: windows_core::IUnknownImpl {
    fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM);
    fn Finish_OnDataChange(&self);
    fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32);
    fn Finish_OnViewChange(&self);
    fn Begin_OnRename(&self, pmk: windows_core::Ref<IMoniker>);
    fn Finish_OnRename(&self);
    fn Begin_OnSave(&self);
    fn Finish_OnSave(&self);
    fn Begin_OnClose(&self);
    fn Finish_OnClose(&self);
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl AsyncIAdviseSink_Vtbl {
    pub const fn new<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_OnDataChange<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink_Impl::Begin_OnDataChange(this, core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&pstgmed));
            }
        }
        unsafe extern "system" fn Finish_OnDataChange<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink_Impl::Finish_OnDataChange(this);
            }
        }
        unsafe extern "system" fn Begin_OnViewChange<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: u32, lindex: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink_Impl::Begin_OnViewChange(this, core::mem::transmute_copy(&dwaspect), core::mem::transmute_copy(&lindex));
            }
        }
        unsafe extern "system" fn Finish_OnViewChange<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink_Impl::Finish_OnViewChange(this);
            }
        }
        unsafe extern "system" fn Begin_OnRename<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink_Impl::Begin_OnRename(this, core::mem::transmute_copy(&pmk));
            }
        }
        unsafe extern "system" fn Finish_OnRename<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink_Impl::Finish_OnRename(this);
            }
        }
        unsafe extern "system" fn Begin_OnSave<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink_Impl::Begin_OnSave(this);
            }
        }
        unsafe extern "system" fn Finish_OnSave<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink_Impl::Finish_OnSave(this);
            }
        }
        unsafe extern "system" fn Begin_OnClose<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink_Impl::Begin_OnClose(this);
            }
        }
        unsafe extern "system" fn Finish_OnClose<Identity: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink_Impl::Finish_OnClose(this);
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Begin_OnDataChange: Begin_OnDataChange::<Identity, OFFSET>,
            Finish_OnDataChange: Finish_OnDataChange::<Identity, OFFSET>,
            Begin_OnViewChange: Begin_OnViewChange::<Identity, OFFSET>,
            Finish_OnViewChange: Finish_OnViewChange::<Identity, OFFSET>,
            Begin_OnRename: Begin_OnRename::<Identity, OFFSET>,
            Finish_OnRename: Finish_OnRename::<Identity, OFFSET>,
            Begin_OnSave: Begin_OnSave::<Identity, OFFSET>,
            Finish_OnSave: Finish_OnSave::<Identity, OFFSET>,
            Begin_OnClose: Begin_OnClose::<Identity, OFFSET>,
            Finish_OnClose: Finish_OnClose::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIAdviseSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl windows_core::RuntimeName for AsyncIAdviseSink {}
windows_core::imp::define_interface!(AsyncIAdviseSink2, AsyncIAdviseSink2_Vtbl, 0x00000151_0000_0000_c000_000000000046);
impl core::ops::Deref for AsyncIAdviseSink2 {
    type Target = AsyncIAdviseSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(AsyncIAdviseSink2, windows_core::IUnknown, AsyncIAdviseSink);
impl AsyncIAdviseSink2 {
    pub unsafe fn Begin_OnLinkSrcChange<P0>(&self, pmk: P0)
    where
        P0: windows_core::Param<IMoniker>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Begin_OnLinkSrcChange)(windows_core::Interface::as_raw(self), pmk.param().abi());
        }
    }
    pub unsafe fn Finish_OnLinkSrcChange(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Finish_OnLinkSrcChange)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AsyncIAdviseSink2_Vtbl {
    pub base__: AsyncIAdviseSink_Vtbl,
    pub Begin_OnLinkSrcChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub Finish_OnLinkSrcChange: unsafe extern "system" fn(*mut core::ffi::c_void),
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub trait AsyncIAdviseSink2_Impl: AsyncIAdviseSink_Impl {
    fn Begin_OnLinkSrcChange(&self, pmk: windows_core::Ref<IMoniker>);
    fn Finish_OnLinkSrcChange(&self);
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl AsyncIAdviseSink2_Vtbl {
    pub const fn new<Identity: AsyncIAdviseSink2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin_OnLinkSrcChange<Identity: AsyncIAdviseSink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink2_Impl::Begin_OnLinkSrcChange(this, core::mem::transmute_copy(&pmk));
            }
        }
        unsafe extern "system" fn Finish_OnLinkSrcChange<Identity: AsyncIAdviseSink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                AsyncIAdviseSink2_Impl::Finish_OnLinkSrcChange(this);
            }
        }
        Self {
            base__: AsyncIAdviseSink_Vtbl::new::<Identity, OFFSET>(),
            Begin_OnLinkSrcChange: Begin_OnLinkSrcChange::<Identity, OFFSET>,
            Finish_OnLinkSrcChange: Finish_OnLinkSrcChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<AsyncIAdviseSink2 as windows_core::Interface>::IID || iid == &<AsyncIAdviseSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl windows_core::RuntimeName for AsyncIAdviseSink2 {}
pub type BIND_FLAGS = i32;
pub const BIND_JUSTTESTEXISTENCE: BIND_FLAGS = 2;
pub const BIND_MAYBOTHERUSER: BIND_FLAGS = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BIND_OPTS {
    pub cbStruct: u32,
    pub grfFlags: u32,
    pub grfMode: u32,
    pub dwTickCountDeadline: u32,
}
#[repr(C)]
#[cfg(all(feature = "objidlbase", feature = "winnt", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BIND_OPTS2 {
    pub Base: BIND_OPTS,
    pub dwTrackFlags: u32,
    pub dwClassContext: u32,
    pub locale: super::winnt::LCID,
    pub pServerInfo: *mut super::objidlbase::COSERVERINFO,
}
#[cfg(all(feature = "objidlbase", feature = "winnt", feature = "wtypesbase"))]
impl Default for BIND_OPTS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BIND_OPTS3 {
    pub Base: BIND_OPTS2,
    pub hwnd: super::windef::HWND,
}
pub type CALLTYPE = i32;
pub const CALLTYPE_ASYNC: CALLTYPE = 3;
pub const CALLTYPE_ASYNC_CALLPENDING: CALLTYPE = 5;
pub const CALLTYPE_NESTED: CALLTYPE = 2;
pub const CALLTYPE_TOPLEVEL: CALLTYPE = 1;
pub const CALLTYPE_TOPLEVEL_CALLPENDING: CALLTYPE = 4;
pub type DATADIR = i32;
pub const DATADIR_GET: DATADIR = 1;
pub const DATADIR_SET: DATADIR = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DVTARGETDEVICE {
    pub tdSize: u32,
    pub tdDriverNameOffset: u16,
    pub tdDeviceNameOffset: u16,
    pub tdPortNameOffset: u16,
    pub tdExtDevmodeOffset: u16,
    pub tdData: [u8; 1],
}
impl Default for DVTARGETDEVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub struct FLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: STGMEDIUM,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl Clone for FLAG_STGMEDIUM {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl Default for FLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FORMATETC {
    pub cfFormat: super::wtypes::CLIPFORMAT,
    pub ptd: *mut DVTARGETDEVICE,
    pub dwAspect: u32,
    pub lindex: i32,
    pub tymed: u32,
}
#[cfg(feature = "wtypes")]
impl Default for FORMATETC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ForcedShutdown: ShutdownType = 1;
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct GDI_OBJECT {
    pub ObjectType: u32,
    pub u: GDI_OBJECT_0,
}
#[cfg(all(feature = "rpc", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
impl Default for GDI_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union GDI_OBJECT_0 {
    pub hBitmap: super::wtypes::wireHBITMAP,
    pub hPalette: super::wtypes::wireHPALETTE,
    pub hGeneric: super::wtypes::wireHGLOBAL,
}
#[cfg(all(feature = "rpc", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
impl Default for GDI_OBJECT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
windows_core::imp::define_interface!(IAdviseSink, IAdviseSink_Vtbl, 0x0000010f_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IAdviseSink, windows_core::IUnknown);
impl IAdviseSink {
    #[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub unsafe fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
        unsafe {
            (windows_core::Interface::vtable(self).OnDataChange)(windows_core::Interface::as_raw(self), pformatetc, core::mem::transmute(pstgmed));
        }
    }
    pub unsafe fn OnViewChange(&self, dwaspect: u32, lindex: i32) {
        unsafe {
            (windows_core::Interface::vtable(self).OnViewChange)(windows_core::Interface::as_raw(self), dwaspect, lindex);
        }
    }
    pub unsafe fn OnRename<P0>(&self, pmk: P0)
    where
        P0: windows_core::Param<IMoniker>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnRename)(windows_core::Interface::as_raw(self), pmk.param().abi());
        }
    }
    pub unsafe fn OnSave(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).OnSave)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn OnClose(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).OnClose)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub OnDataChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const FORMATETC, *const STGMEDIUM),
    #[cfg(not(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes")))]
    OnDataChange: usize,
    pub OnViewChange: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32),
    pub OnRename: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
    pub OnSave: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnClose: unsafe extern "system" fn(*mut core::ffi::c_void),
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub trait IAdviseSink_Impl: windows_core::IUnknownImpl {
    fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM);
    fn OnViewChange(&self, dwaspect: u32, lindex: i32);
    fn OnRename(&self, pmk: windows_core::Ref<IMoniker>);
    fn OnSave(&self);
    fn OnClose(&self);
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl IAdviseSink_Vtbl {
    pub const fn new<Identity: IAdviseSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnDataChange<Identity: IAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAdviseSink_Impl::OnDataChange(this, core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&pstgmed));
            }
        }
        unsafe extern "system" fn OnViewChange<Identity: IAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspect: u32, lindex: i32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAdviseSink_Impl::OnViewChange(this, core::mem::transmute_copy(&dwaspect), core::mem::transmute_copy(&lindex));
            }
        }
        unsafe extern "system" fn OnRename<Identity: IAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAdviseSink_Impl::OnRename(this, core::mem::transmute_copy(&pmk));
            }
        }
        unsafe extern "system" fn OnSave<Identity: IAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAdviseSink_Impl::OnSave(this);
            }
        }
        unsafe extern "system" fn OnClose<Identity: IAdviseSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAdviseSink_Impl::OnClose(this);
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnDataChange: OnDataChange::<Identity, OFFSET>,
            OnViewChange: OnViewChange::<Identity, OFFSET>,
            OnRename: OnRename::<Identity, OFFSET>,
            OnSave: OnSave::<Identity, OFFSET>,
            OnClose: OnClose::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAdviseSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl windows_core::RuntimeName for IAdviseSink {}
windows_core::imp::define_interface!(IAdviseSink2, IAdviseSink2_Vtbl, 0x00000125_0000_0000_c000_000000000046);
impl core::ops::Deref for IAdviseSink2 {
    type Target = IAdviseSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAdviseSink2, windows_core::IUnknown, IAdviseSink);
impl IAdviseSink2 {
    pub unsafe fn OnLinkSrcChange<P0>(&self, pmk: P0)
    where
        P0: windows_core::Param<IMoniker>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).OnLinkSrcChange)(windows_core::Interface::as_raw(self), pmk.param().abi());
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAdviseSink2_Vtbl {
    pub base__: IAdviseSink_Vtbl,
    pub OnLinkSrcChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void),
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub trait IAdviseSink2_Impl: IAdviseSink_Impl {
    fn OnLinkSrcChange(&self, pmk: windows_core::Ref<IMoniker>);
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl IAdviseSink2_Vtbl {
    pub const fn new<Identity: IAdviseSink2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnLinkSrcChange<Identity: IAdviseSink2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmk: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAdviseSink2_Impl::OnLinkSrcChange(this, core::mem::transmute_copy(&pmk));
            }
        }
        Self { base__: IAdviseSink_Vtbl::new::<Identity, OFFSET>(), OnLinkSrcChange: OnLinkSrcChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAdviseSink2 as windows_core::Interface>::IID || iid == &<IAdviseSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl windows_core::RuntimeName for IAdviseSink2 {}
windows_core::imp::define_interface!(IApartmentShutdown, IApartmentShutdown_Vtbl, 0xa2f05a09_27a2_42b5_bc0e_ac163ef49d9b);
windows_core::imp::interface_hierarchy!(IApartmentShutdown, windows_core::IUnknown);
impl IApartmentShutdown {
    pub unsafe fn OnUninitialize(&self, ui64apartmentidentifier: u64) {
        unsafe {
            (windows_core::Interface::vtable(self).OnUninitialize)(windows_core::Interface::as_raw(self), ui64apartmentidentifier);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IApartmentShutdown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnUninitialize: unsafe extern "system" fn(*mut core::ffi::c_void, u64),
}
pub trait IApartmentShutdown_Impl: windows_core::IUnknownImpl {
    fn OnUninitialize(&self, ui64apartmentidentifier: u64);
}
impl IApartmentShutdown_Vtbl {
    pub const fn new<Identity: IApartmentShutdown_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnUninitialize<Identity: IApartmentShutdown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ui64apartmentidentifier: u64) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IApartmentShutdown_Impl::OnUninitialize(this, core::mem::transmute_copy(&ui64apartmentidentifier));
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUninitialize: OnUninitialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IApartmentShutdown as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IApartmentShutdown {}
windows_core::imp::define_interface!(IBindCtx, IBindCtx_Vtbl, 0x0000000e_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IBindCtx, windows_core::IUnknown);
impl IBindCtx {
    pub unsafe fn RegisterObjectBound<P0>(&self, punk: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterObjectBound)(windows_core::Interface::as_raw(self), punk.param().abi()) }
    }
    pub unsafe fn RevokeObjectBound<P0>(&self, punk: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).RevokeObjectBound)(windows_core::Interface::as_raw(self), punk.param().abi()) }
    }
    pub unsafe fn ReleaseBoundObjects(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseBoundObjects)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetBindOptions(&self, pbindopts: *const BIND_OPTS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBindOptions)(windows_core::Interface::as_raw(self), pbindopts) }
    }
    pub unsafe fn GetBindOptions(&self, pbindopts: *mut BIND_OPTS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBindOptions)(windows_core::Interface::as_raw(self), pbindopts as _) }
    }
    pub unsafe fn GetRunningObjectTable(&self) -> windows_core::Result<IRunningObjectTable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRunningObjectTable)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RegisterObjectParam<P0, P1>(&self, pszkey: P0, punk: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterObjectParam)(windows_core::Interface::as_raw(self), pszkey.param().abi(), punk.param().abi()) }
    }
    pub unsafe fn GetObjectParam<P0>(&self, pszkey: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectParam)(windows_core::Interface::as_raw(self), pszkey.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn EnumObjectParam(&self) -> windows_core::Result<super::objidlbase::IEnumString> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumObjectParam)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RevokeObjectParam<P0>(&self, pszkey: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RevokeObjectParam)(windows_core::Interface::as_raw(self), pszkey.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindCtx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterObjectBound: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RevokeObjectBound: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseBoundObjects: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBindOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *const BIND_OPTS) -> windows_core::HRESULT,
    pub GetBindOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BIND_OPTS) -> windows_core::HRESULT,
    pub GetRunningObjectTable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterObjectParam: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObjectParam: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub EnumObjectParam: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    EnumObjectParam: usize,
    pub RevokeObjectParam: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "objidlbase")]
pub trait IBindCtx_Impl: windows_core::IUnknownImpl {
    fn RegisterObjectBound(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RevokeObjectBound(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ReleaseBoundObjects(&self) -> windows_core::Result<()>;
    fn SetBindOptions(&self, pbindopts: *const BIND_OPTS) -> windows_core::Result<()>;
    fn GetBindOptions(&self, pbindopts: *mut BIND_OPTS) -> windows_core::Result<()>;
    fn GetRunningObjectTable(&self) -> windows_core::Result<IRunningObjectTable>;
    fn RegisterObjectParam(&self, pszkey: &windows_core::PCWSTR, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetObjectParam(&self, pszkey: &windows_core::PCWSTR) -> windows_core::Result<windows_core::IUnknown>;
    fn EnumObjectParam(&self) -> windows_core::Result<super::objidlbase::IEnumString>;
    fn RevokeObjectParam(&self, pszkey: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "objidlbase")]
impl IBindCtx_Vtbl {
    pub const fn new<Identity: IBindCtx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterObjectBound<Identity: IBindCtx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindCtx_Impl::RegisterObjectBound(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn RevokeObjectBound<Identity: IBindCtx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindCtx_Impl::RevokeObjectBound(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn ReleaseBoundObjects<Identity: IBindCtx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindCtx_Impl::ReleaseBoundObjects(this).into()
            }
        }
        unsafe extern "system" fn SetBindOptions<Identity: IBindCtx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbindopts: *const BIND_OPTS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindCtx_Impl::SetBindOptions(this, core::mem::transmute_copy(&pbindopts)).into()
            }
        }
        unsafe extern "system" fn GetBindOptions<Identity: IBindCtx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbindopts: *mut BIND_OPTS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindCtx_Impl::GetBindOptions(this, core::mem::transmute_copy(&pbindopts)).into()
            }
        }
        unsafe extern "system" fn GetRunningObjectTable<Identity: IBindCtx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprot: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBindCtx_Impl::GetRunningObjectTable(this) {
                    Ok(ok__) => {
                        pprot.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterObjectParam<Identity: IBindCtx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszkey: windows_core::PCWSTR, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindCtx_Impl::RegisterObjectParam(this, core::mem::transmute(&pszkey), core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn GetObjectParam<Identity: IBindCtx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszkey: windows_core::PCWSTR, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBindCtx_Impl::GetObjectParam(this, core::mem::transmute(&pszkey)) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumObjectParam<Identity: IBindCtx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBindCtx_Impl::EnumObjectParam(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RevokeObjectParam<Identity: IBindCtx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszkey: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBindCtx_Impl::RevokeObjectParam(this, core::mem::transmute(&pszkey)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterObjectBound: RegisterObjectBound::<Identity, OFFSET>,
            RevokeObjectBound: RevokeObjectBound::<Identity, OFFSET>,
            ReleaseBoundObjects: ReleaseBoundObjects::<Identity, OFFSET>,
            SetBindOptions: SetBindOptions::<Identity, OFFSET>,
            GetBindOptions: GetBindOptions::<Identity, OFFSET>,
            GetRunningObjectTable: GetRunningObjectTable::<Identity, OFFSET>,
            RegisterObjectParam: RegisterObjectParam::<Identity, OFFSET>,
            GetObjectParam: GetObjectParam::<Identity, OFFSET>,
            EnumObjectParam: EnumObjectParam::<Identity, OFFSET>,
            RevokeObjectParam: RevokeObjectParam::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBindCtx as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for IBindCtx {}
windows_core::imp::define_interface!(IBlockingLock, IBlockingLock_Vtbl, 0x30f3d47a_6447_11d1_8e3c_00c04fb9386d);
windows_core::imp::interface_hierarchy!(IBlockingLock, windows_core::IUnknown);
impl IBlockingLock {
    pub unsafe fn Lock(&self, dwtimeout: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Lock)(windows_core::Interface::as_raw(self), dwtimeout) }
    }
    pub unsafe fn Unlock(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBlockingLock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Lock: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IBlockingLock_Impl: windows_core::IUnknownImpl {
    fn Lock(&self, dwtimeout: u32) -> windows_core::Result<()>;
    fn Unlock(&self) -> windows_core::Result<()>;
}
impl IBlockingLock_Vtbl {
    pub const fn new<Identity: IBlockingLock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Lock<Identity: IBlockingLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtimeout: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBlockingLock_Impl::Lock(this, core::mem::transmute_copy(&dwtimeout)).into()
            }
        }
        unsafe extern "system" fn Unlock<Identity: IBlockingLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBlockingLock_Impl::Unlock(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Lock: Lock::<Identity, OFFSET>, Unlock: Unlock::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBlockingLock as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBlockingLock {}
windows_core::imp::define_interface!(IClassActivator, IClassActivator_Vtbl, 0x00000140_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IClassActivator, windows_core::IUnknown);
impl IClassActivator {
    #[cfg(feature = "winnt")]
    pub unsafe fn GetClassObject<T>(&self, rclsid: *const windows_core::GUID, dwclasscontext: u32, locale: super::winnt::LCID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetClassObject)(windows_core::Interface::as_raw(self), rclsid, dwclasscontext, locale, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassActivator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "winnt")]
    pub GetClassObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, super::winnt::LCID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetClassObject: usize,
}
#[cfg(feature = "winnt")]
pub trait IClassActivator_Impl: windows_core::IUnknownImpl {
    fn GetClassObject(&self, rclsid: *const windows_core::GUID, dwclasscontext: u32, locale: super::winnt::LCID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "winnt")]
impl IClassActivator_Vtbl {
    pub const fn new<Identity: IClassActivator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClassObject<Identity: IClassActivator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, dwclasscontext: u32, locale: super::winnt::LCID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IClassActivator_Impl::GetClassObject(this, core::mem::transmute_copy(&rclsid), core::mem::transmute_copy(&dwclasscontext), core::mem::transmute_copy(&locale), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetClassObject: GetClassObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IClassActivator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "winnt")]
impl windows_core::RuntimeName for IClassActivator {}
windows_core::imp::define_interface!(IDataAdviseHolder, IDataAdviseHolder_Vtbl, 0x00000110_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IDataAdviseHolder, windows_core::IUnknown);
impl IDataAdviseHolder {
    #[cfg(feature = "wtypes")]
    pub unsafe fn Advise<P0, P3>(&self, pdataobject: P0, pfetc: *const FORMATETC, advf: u32, padvise: P3) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<IDataObject>,
        P3: windows_core::Param<IAdviseSink>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Advise)(windows_core::Interface::as_raw(self), pdataobject.param().abi(), pfetc, advf, padvise.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unadvise(&self, dwconnection: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), dwconnection) }
    }
    pub unsafe fn EnumAdvise(&self) -> windows_core::Result<IEnumSTATDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumAdvise)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SendOnDataChange<P0>(&self, pdataobject: P0, dwreserved: Option<u32>, advf: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDataObject>,
    {
        unsafe { (windows_core::Interface::vtable(self).SendOnDataChange)(windows_core::Interface::as_raw(self), pdataobject.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _, advf) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataAdviseHolder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypes")]
    pub Advise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const FORMATETC, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Advise: usize,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnumAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SendOnDataChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "wtypes")]
pub trait IDataAdviseHolder_Impl: windows_core::IUnknownImpl {
    fn Advise(&self, pdataobject: windows_core::Ref<IDataObject>, pfetc: *const FORMATETC, advf: u32, padvise: windows_core::Ref<IAdviseSink>) -> windows_core::Result<u32>;
    fn Unadvise(&self, dwconnection: u32) -> windows_core::Result<()>;
    fn EnumAdvise(&self) -> windows_core::Result<IEnumSTATDATA>;
    fn SendOnDataChange(&self, pdataobject: windows_core::Ref<IDataObject>, dwreserved: u32, advf: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "wtypes")]
impl IDataAdviseHolder_Vtbl {
    pub const fn new<Identity: IDataAdviseHolder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Advise<Identity: IDataAdviseHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void, pfetc: *const FORMATETC, advf: u32, padvise: *mut core::ffi::c_void, pdwconnection: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataAdviseHolder_Impl::Advise(this, core::mem::transmute_copy(&pdataobject), core::mem::transmute_copy(&pfetc), core::mem::transmute_copy(&advf), core::mem::transmute_copy(&padvise)) {
                    Ok(ok__) => {
                        pdwconnection.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unadvise<Identity: IDataAdviseHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconnection: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataAdviseHolder_Impl::Unadvise(this, core::mem::transmute_copy(&dwconnection)).into()
            }
        }
        unsafe extern "system" fn EnumAdvise<Identity: IDataAdviseHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumadvise: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataAdviseHolder_Impl::EnumAdvise(this) {
                    Ok(ok__) => {
                        ppenumadvise.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SendOnDataChange<Identity: IDataAdviseHolder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdataobject: *mut core::ffi::c_void, dwreserved: u32, advf: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataAdviseHolder_Impl::SendOnDataChange(this, core::mem::transmute_copy(&pdataobject), core::mem::transmute_copy(&dwreserved), core::mem::transmute_copy(&advf)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Advise: Advise::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
            EnumAdvise: EnumAdvise::<Identity, OFFSET>,
            SendOnDataChange: SendOnDataChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataAdviseHolder as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypes")]
impl windows_core::RuntimeName for IDataAdviseHolder {}
windows_core::imp::define_interface!(IDataObject, IDataObject_Vtbl, 0x0000010e_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IDataObject, windows_core::IUnknown);
impl IDataObject {
    #[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub unsafe fn GetData(&self, pformatetcin: *const FORMATETC) -> windows_core::Result<STGMEDIUM> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), pformatetcin, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub unsafe fn GetDataHere(&self, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDataHere)(windows_core::Interface::as_raw(self), pformatetc, core::mem::transmute(pmedium)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn QueryGetData(&self, pformatetc: *const FORMATETC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryGetData)(windows_core::Interface::as_raw(self), pformatetc) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn GetCanonicalFormatEtc(&self, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCanonicalFormatEtc)(windows_core::Interface::as_raw(self), pformatectin, pformatetcout as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub unsafe fn SetData(&self, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), pformatetc, core::mem::transmute(pmedium), frelease.into()) }
    }
    pub unsafe fn EnumFormatEtc(&self, dwdirection: u32) -> windows_core::Result<IEnumFORMATETC> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumFormatEtc)(windows_core::Interface::as_raw(self), dwdirection, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn DAdvise<P2>(&self, pformatetc: *const FORMATETC, advf: u32, padvsink: P2) -> windows_core::Result<u32>
    where
        P2: windows_core::Param<IAdviseSink>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DAdvise)(windows_core::Interface::as_raw(self), pformatetc, advf, padvsink.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DUnadvise(&self, dwconnection: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DUnadvise)(windows_core::Interface::as_raw(self), dwconnection) }
    }
    pub unsafe fn EnumDAdvise(&self) -> windows_core::Result<IEnumSTATDATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumDAdvise)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const FORMATETC, *mut STGMEDIUM) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes")))]
    GetData: usize,
    #[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub GetDataHere: unsafe extern "system" fn(*mut core::ffi::c_void, *const FORMATETC, *mut STGMEDIUM) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes")))]
    GetDataHere: usize,
    #[cfg(feature = "wtypes")]
    pub QueryGetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const FORMATETC) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    QueryGetData: usize,
    #[cfg(feature = "wtypes")]
    pub GetCanonicalFormatEtc: unsafe extern "system" fn(*mut core::ffi::c_void, *const FORMATETC, *mut FORMATETC) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    GetCanonicalFormatEtc: usize,
    #[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const FORMATETC, *const STGMEDIUM, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes")))]
    SetData: usize,
    pub EnumFormatEtc: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub DAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, *const FORMATETC, u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    DAdvise: usize,
    pub DUnadvise: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnumDAdvise: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub trait IDataObject_Impl: windows_core::IUnknownImpl {
    fn GetData(&self, pformatetcin: *const FORMATETC) -> windows_core::Result<STGMEDIUM>;
    fn GetDataHere(&self, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> windows_core::Result<()>;
    fn QueryGetData(&self, pformatetc: *const FORMATETC) -> windows_core::Result<()>;
    fn GetCanonicalFormatEtc(&self, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> windows_core::Result<()>;
    fn SetData(&self, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: windows_core::BOOL) -> windows_core::Result<()>;
    fn EnumFormatEtc(&self, dwdirection: u32) -> windows_core::Result<IEnumFORMATETC>;
    fn DAdvise(&self, pformatetc: *const FORMATETC, advf: u32, padvsink: windows_core::Ref<IAdviseSink>) -> windows_core::Result<u32>;
    fn DUnadvise(&self, dwconnection: u32) -> windows_core::Result<()>;
    fn EnumDAdvise(&self) -> windows_core::Result<IEnumSTATDATA>;
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl IDataObject_Vtbl {
    pub const fn new<Identity: IDataObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetData<Identity: IDataObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetcin: *const FORMATETC, pmedium: *mut STGMEDIUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataObject_Impl::GetData(this, core::mem::transmute_copy(&pformatetcin)) {
                    Ok(ok__) => {
                        pmedium.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDataHere<Identity: IDataObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataObject_Impl::GetDataHere(this, core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&pmedium)).into()
            }
        }
        unsafe extern "system" fn QueryGetData<Identity: IDataObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const FORMATETC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataObject_Impl::QueryGetData(this, core::mem::transmute_copy(&pformatetc)).into()
            }
        }
        unsafe extern "system" fn GetCanonicalFormatEtc<Identity: IDataObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataObject_Impl::GetCanonicalFormatEtc(this, core::mem::transmute_copy(&pformatectin), core::mem::transmute_copy(&pformatetcout)).into()
            }
        }
        unsafe extern "system" fn SetData<Identity: IDataObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataObject_Impl::SetData(this, core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&pmedium), core::mem::transmute_copy(&frelease)).into()
            }
        }
        unsafe extern "system" fn EnumFormatEtc<Identity: IDataObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdirection: u32, ppenumformatetc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataObject_Impl::EnumFormatEtc(this, core::mem::transmute_copy(&dwdirection)) {
                    Ok(ok__) => {
                        ppenumformatetc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DAdvise<Identity: IDataObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformatetc: *const FORMATETC, advf: u32, padvsink: *mut core::ffi::c_void, pdwconnection: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataObject_Impl::DAdvise(this, core::mem::transmute_copy(&pformatetc), core::mem::transmute_copy(&advf), core::mem::transmute_copy(&padvsink)) {
                    Ok(ok__) => {
                        pdwconnection.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DUnadvise<Identity: IDataObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconnection: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataObject_Impl::DUnadvise(this, core::mem::transmute_copy(&dwconnection)).into()
            }
        }
        unsafe extern "system" fn EnumDAdvise<Identity: IDataObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenumadvise: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataObject_Impl::EnumDAdvise(this) {
                    Ok(ok__) => {
                        ppenumadvise.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetData: GetData::<Identity, OFFSET>,
            GetDataHere: GetDataHere::<Identity, OFFSET>,
            QueryGetData: QueryGetData::<Identity, OFFSET>,
            GetCanonicalFormatEtc: GetCanonicalFormatEtc::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
            EnumFormatEtc: EnumFormatEtc::<Identity, OFFSET>,
            DAdvise: DAdvise::<Identity, OFFSET>,
            DUnadvise: DUnadvise::<Identity, OFFSET>,
            EnumDAdvise: EnumDAdvise::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataObject as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl windows_core::RuntimeName for IDataObject {}
windows_core::imp::define_interface!(IDirectWriterLock, IDirectWriterLock_Vtbl, 0x0e6d4d92_6738_11cf_9608_00aa00680db4);
windows_core::imp::interface_hierarchy!(IDirectWriterLock, windows_core::IUnknown);
impl IDirectWriterLock {
    pub unsafe fn WaitForWriteAccess(&self, dwtimeout: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WaitForWriteAccess)(windows_core::Interface::as_raw(self), dwtimeout) }
    }
    pub unsafe fn ReleaseWriteAccess(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseWriteAccess)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn HaveWriteAccess(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HaveWriteAccess)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectWriterLock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub WaitForWriteAccess: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ReleaseWriteAccess: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HaveWriteAccess: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDirectWriterLock_Impl: windows_core::IUnknownImpl {
    fn WaitForWriteAccess(&self, dwtimeout: u32) -> windows_core::Result<()>;
    fn ReleaseWriteAccess(&self) -> windows_core::Result<()>;
    fn HaveWriteAccess(&self) -> windows_core::Result<()>;
}
impl IDirectWriterLock_Vtbl {
    pub const fn new<Identity: IDirectWriterLock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WaitForWriteAccess<Identity: IDirectWriterLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtimeout: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectWriterLock_Impl::WaitForWriteAccess(this, core::mem::transmute_copy(&dwtimeout)).into()
            }
        }
        unsafe extern "system" fn ReleaseWriteAccess<Identity: IDirectWriterLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectWriterLock_Impl::ReleaseWriteAccess(this).into()
            }
        }
        unsafe extern "system" fn HaveWriteAccess<Identity: IDirectWriterLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectWriterLock_Impl::HaveWriteAccess(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WaitForWriteAccess: WaitForWriteAccess::<Identity, OFFSET>,
            ReleaseWriteAccess: ReleaseWriteAccess::<Identity, OFFSET>,
            HaveWriteAccess: HaveWriteAccess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectWriterLock as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDirectWriterLock {}
windows_core::imp::define_interface!(IDummyHICONIncluder, IDummyHICONIncluder_Vtbl, 0x947990de_cc28_11d2_a0f7_00805f858fb1);
windows_core::imp::interface_hierarchy!(IDummyHICONIncluder, windows_core::IUnknown);
impl IDummyHICONIncluder {
    #[cfg(feature = "windef")]
    pub unsafe fn Dummy(&self, h1: super::windef::HICON, h2: super::windef::HDC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Dummy)(windows_core::Interface::as_raw(self), h1, h2) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDummyHICONIncluder_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub Dummy: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HICON, super::windef::HDC) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    Dummy: usize,
}
#[cfg(feature = "windef")]
pub trait IDummyHICONIncluder_Impl: windows_core::IUnknownImpl {
    fn Dummy(&self, h1: super::windef::HICON, h2: super::windef::HDC) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IDummyHICONIncluder_Vtbl {
    pub const fn new<Identity: IDummyHICONIncluder_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Dummy<Identity: IDummyHICONIncluder_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, h1: super::windef::HICON, h2: super::windef::HDC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDummyHICONIncluder_Impl::Dummy(this, core::mem::transmute_copy(&h1), core::mem::transmute_copy(&h2)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Dummy: Dummy::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDummyHICONIncluder as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IDummyHICONIncluder {}
windows_core::imp::define_interface!(IEnumFORMATETC, IEnumFORMATETC_Vtbl, 0x00000103_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumFORMATETC, windows_core::IUnknown);
impl IEnumFORMATETC {
    #[cfg(feature = "wtypes")]
    pub unsafe fn Next(&self, rgelt: &mut [FORMATETC], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumFORMATETC_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypes")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut FORMATETC, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "wtypes")]
pub trait IEnumFORMATETC_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumFORMATETC>;
}
#[cfg(feature = "wtypes")]
impl IEnumFORMATETC_Vtbl {
    pub const fn new<Identity: IEnumFORMATETC_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumFORMATETC_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumFORMATETC_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumFORMATETC_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumFORMATETC_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumFORMATETC_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumFORMATETC_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumFORMATETC_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumFORMATETC_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumFORMATETC as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypes")]
impl windows_core::RuntimeName for IEnumFORMATETC {}
windows_core::imp::define_interface!(IEnumMoniker, IEnumMoniker_Vtbl, 0x00000102_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumMoniker, windows_core::IUnknown);
impl IEnumMoniker {
    pub unsafe fn Next(&self, rgelt: &mut [Option<IMoniker>], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumMoniker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IEnumMoniker_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut Option<IMoniker>, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumMoniker>;
}
impl IEnumMoniker_Vtbl {
    pub const fn new<Identity: IEnumMoniker_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut *mut core::ffi::c_void, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumMoniker_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumMoniker_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumMoniker_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumMoniker_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumMoniker as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEnumMoniker {}
windows_core::imp::define_interface!(IEnumSTATDATA, IEnumSTATDATA_Vtbl, 0x00000105_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumSTATDATA, windows_core::IUnknown);
impl IEnumSTATDATA {
    #[cfg(feature = "wtypes")]
    pub unsafe fn Next(&self, rgelt: &mut [STATDATA], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATDATA_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypes")]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut STATDATA, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "wtypes")]
pub trait IEnumSTATDATA_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut STATDATA, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSTATDATA>;
}
#[cfg(feature = "wtypes")]
impl IEnumSTATDATA_Vtbl {
    pub const fn new<Identity: IEnumSTATDATA_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumSTATDATA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut STATDATA, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATDATA_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumSTATDATA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATDATA_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumSTATDATA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATDATA_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumSTATDATA_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumSTATDATA_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSTATDATA as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypes")]
impl windows_core::RuntimeName for IEnumSTATDATA {}
windows_core::imp::define_interface!(IEnumSTATSTG, IEnumSTATSTG_Vtbl, 0x0000000d_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IEnumSTATSTG, windows_core::IUnknown);
impl IEnumSTATSTG {
    #[cfg(all(feature = "minwindef", feature = "objidlbase"))]
    pub unsafe fn Next(&self, rgelt: &mut [super::objidlbase::STATSTG], pceltfetched: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self), rgelt.len().try_into().unwrap(), core::mem::transmute(rgelt.as_ptr()), pceltfetched.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Skip(&self, celt: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Skip)(windows_core::Interface::as_raw(self), celt) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSTATSTG_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "objidlbase"))]
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::objidlbase::STATSTG, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "objidlbase")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
pub trait IEnumSTATSTG_Impl: windows_core::IUnknownImpl {
    fn Next(&self, celt: u32, rgelt: *mut super::objidlbase::STATSTG, pceltfetched: *mut u32) -> windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IEnumSTATSTG>;
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
impl IEnumSTATSTG_Vtbl {
    pub const fn new<Identity: IEnumSTATSTG_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Next<Identity: IEnumSTATSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32, rgelt: *mut super::objidlbase::STATSTG, pceltfetched: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATSTG_Impl::Next(this, core::mem::transmute_copy(&celt), core::mem::transmute_copy(&rgelt), core::mem::transmute_copy(&pceltfetched)).into()
            }
        }
        unsafe extern "system" fn Skip<Identity: IEnumSTATSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, celt: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATSTG_Impl::Skip(this, core::mem::transmute_copy(&celt)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IEnumSTATSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEnumSTATSTG_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IEnumSTATSTG_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEnumSTATSTG_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, OFFSET>,
            Skip: Skip::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEnumSTATSTG as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
impl windows_core::RuntimeName for IEnumSTATSTG {}
windows_core::imp::define_interface!(IFillLockBytes, IFillLockBytes_Vtbl, 0x99caf010_415e_11cf_8814_00aa00b569f5);
windows_core::imp::interface_hierarchy!(IFillLockBytes, windows_core::IUnknown);
impl IFillLockBytes {
    pub unsafe fn FillAppend(&self, pv: *const core::ffi::c_void, cb: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FillAppend)(windows_core::Interface::as_raw(self), pv, cb, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FillAt(&self, uloffset: u64, pv: *const core::ffi::c_void, cb: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FillAt)(windows_core::Interface::as_raw(self), uloffset, pv, cb, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFillSize(&self, ulsize: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFillSize)(windows_core::Interface::as_raw(self), ulsize) }
    }
    pub unsafe fn Terminate(&self, bcanceled: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self), bcanceled.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFillLockBytes_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FillAppend: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub FillAt: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub SetFillSize: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IFillLockBytes_Impl: windows_core::IUnknownImpl {
    fn FillAppend(&self, pv: *const core::ffi::c_void, cb: u32) -> windows_core::Result<u32>;
    fn FillAt(&self, uloffset: u64, pv: *const core::ffi::c_void, cb: u32) -> windows_core::Result<u32>;
    fn SetFillSize(&self, ulsize: u64) -> windows_core::Result<()>;
    fn Terminate(&self, bcanceled: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IFillLockBytes_Vtbl {
    pub const fn new<Identity: IFillLockBytes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FillAppend<Identity: IFillLockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFillLockBytes_Impl::FillAppend(this, core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb)) {
                    Ok(ok__) => {
                        pcbwritten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FillAt<Identity: IFillLockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloffset: u64, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFillLockBytes_Impl::FillAt(this, core::mem::transmute_copy(&uloffset), core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb)) {
                    Ok(ok__) => {
                        pcbwritten.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFillSize<Identity: IFillLockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulsize: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFillLockBytes_Impl::SetFillSize(this, core::mem::transmute_copy(&ulsize)).into()
            }
        }
        unsafe extern "system" fn Terminate<Identity: IFillLockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bcanceled: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFillLockBytes_Impl::Terminate(this, core::mem::transmute_copy(&bcanceled)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FillAppend: FillAppend::<Identity, OFFSET>,
            FillAt: FillAt::<Identity, OFFSET>,
            SetFillSize: SetFillSize::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFillLockBytes as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IFillLockBytes {}
windows_core::imp::define_interface!(IForegroundTransfer, IForegroundTransfer_Vtbl, 0x00000145_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IForegroundTransfer, windows_core::IUnknown);
impl IForegroundTransfer {
    pub unsafe fn AllowForegroundTransfer(&self, lpvreserved: Option<*const core::ffi::c_void>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AllowForegroundTransfer)(windows_core::Interface::as_raw(self), lpvreserved.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IForegroundTransfer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AllowForegroundTransfer: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IForegroundTransfer_Impl: windows_core::IUnknownImpl {
    fn AllowForegroundTransfer(&self, lpvreserved: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl IForegroundTransfer_Vtbl {
    pub const fn new<Identity: IForegroundTransfer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AllowForegroundTransfer<Identity: IForegroundTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpvreserved: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IForegroundTransfer_Impl::AllowForegroundTransfer(this, core::mem::transmute_copy(&lpvreserved)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AllowForegroundTransfer: AllowForegroundTransfer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IForegroundTransfer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IForegroundTransfer {}
windows_core::imp::define_interface!(IInitializeSpy, IInitializeSpy_Vtbl, 0x00000034_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IInitializeSpy, windows_core::IUnknown);
impl IInitializeSpy {
    pub unsafe fn PreInitialize(&self, dwcoinit: u32, dwcurthreadaptrefs: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PreInitialize)(windows_core::Interface::as_raw(self), dwcoinit, dwcurthreadaptrefs) }
    }
    pub unsafe fn PostInitialize(&self, hrcoinit: windows_core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PostInitialize)(windows_core::Interface::as_raw(self), hrcoinit, dwcoinit, dwnewthreadaptrefs) }
    }
    pub unsafe fn PreUninitialize(&self, dwcurthreadaptrefs: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PreUninitialize)(windows_core::Interface::as_raw(self), dwcurthreadaptrefs) }
    }
    pub unsafe fn PostUninitialize(&self, dwnewthreadaptrefs: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PostUninitialize)(windows_core::Interface::as_raw(self), dwnewthreadaptrefs) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInitializeSpy_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PreInitialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub PostInitialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, u32, u32) -> windows_core::HRESULT,
    pub PreUninitialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub PostUninitialize: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IInitializeSpy_Impl: windows_core::IUnknownImpl {
    fn PreInitialize(&self, dwcoinit: u32, dwcurthreadaptrefs: u32) -> windows_core::Result<()>;
    fn PostInitialize(&self, hrcoinit: windows_core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> windows_core::Result<()>;
    fn PreUninitialize(&self, dwcurthreadaptrefs: u32) -> windows_core::Result<()>;
    fn PostUninitialize(&self, dwnewthreadaptrefs: u32) -> windows_core::Result<()>;
}
impl IInitializeSpy_Vtbl {
    pub const fn new<Identity: IInitializeSpy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PreInitialize<Identity: IInitializeSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcoinit: u32, dwcurthreadaptrefs: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInitializeSpy_Impl::PreInitialize(this, core::mem::transmute_copy(&dwcoinit), core::mem::transmute_copy(&dwcurthreadaptrefs)).into()
            }
        }
        unsafe extern "system" fn PostInitialize<Identity: IInitializeSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrcoinit: windows_core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInitializeSpy_Impl::PostInitialize(this, core::mem::transmute_copy(&hrcoinit), core::mem::transmute_copy(&dwcoinit), core::mem::transmute_copy(&dwnewthreadaptrefs)).into()
            }
        }
        unsafe extern "system" fn PreUninitialize<Identity: IInitializeSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcurthreadaptrefs: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInitializeSpy_Impl::PreUninitialize(this, core::mem::transmute_copy(&dwcurthreadaptrefs)).into()
            }
        }
        unsafe extern "system" fn PostUninitialize<Identity: IInitializeSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwnewthreadaptrefs: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInitializeSpy_Impl::PostUninitialize(this, core::mem::transmute_copy(&dwnewthreadaptrefs)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PreInitialize: PreInitialize::<Identity, OFFSET>,
            PostInitialize: PostInitialize::<Identity, OFFSET>,
            PreUninitialize: PreUninitialize::<Identity, OFFSET>,
            PostUninitialize: PostUninitialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInitializeSpy as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInitializeSpy {}
windows_core::imp::define_interface!(ILayoutStorage, ILayoutStorage_Vtbl, 0x0e6d4d90_6738_11cf_9608_00aa00680db4);
windows_core::imp::interface_hierarchy!(ILayoutStorage, windows_core::IUnknown);
impl ILayoutStorage {
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn LayoutScript(&self, pstoragelayout: &[StorageLayout], glfinterleavedflag: Option<u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LayoutScript)(windows_core::Interface::as_raw(self), core::mem::transmute(pstoragelayout.as_ptr()), pstoragelayout.len().try_into().unwrap(), glfinterleavedflag.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn BeginMonitor(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginMonitor)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EndMonitor(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndMonitor)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn ReLayoutDocfile(&self, pwcsnewdfname: *const super::wtypesbase::OLECHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReLayoutDocfile)(windows_core::Interface::as_raw(self), pwcsnewdfname) }
    }
    pub unsafe fn ReLayoutDocfileOnILockBytes<P0>(&self, pilockbytes: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ILockBytes>,
    {
        unsafe { (windows_core::Interface::vtable(self).ReLayoutDocfileOnILockBytes)(windows_core::Interface::as_raw(self), pilockbytes.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILayoutStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypesbase")]
    pub LayoutScript: unsafe extern "system" fn(*mut core::ffi::c_void, *const StorageLayout, u32, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    LayoutScript: usize,
    pub BeginMonitor: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndMonitor: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypesbase")]
    pub ReLayoutDocfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypesbase::OLECHAR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    ReLayoutDocfile: usize,
    pub ReLayoutDocfileOnILockBytes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "wtypesbase")]
pub trait ILayoutStorage_Impl: windows_core::IUnknownImpl {
    fn LayoutScript(&self, pstoragelayout: *const StorageLayout, nentries: u32, glfinterleavedflag: u32) -> windows_core::Result<()>;
    fn BeginMonitor(&self) -> windows_core::Result<()>;
    fn EndMonitor(&self) -> windows_core::Result<()>;
    fn ReLayoutDocfile(&self, pwcsnewdfname: *const super::wtypesbase::OLECHAR) -> windows_core::Result<()>;
    fn ReLayoutDocfileOnILockBytes(&self, pilockbytes: windows_core::Ref<ILockBytes>) -> windows_core::Result<()>;
}
#[cfg(feature = "wtypesbase")]
impl ILayoutStorage_Vtbl {
    pub const fn new<Identity: ILayoutStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LayoutScript<Identity: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstoragelayout: *const StorageLayout, nentries: u32, glfinterleavedflag: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILayoutStorage_Impl::LayoutScript(this, core::mem::transmute_copy(&pstoragelayout), core::mem::transmute_copy(&nentries), core::mem::transmute_copy(&glfinterleavedflag)).into()
            }
        }
        unsafe extern "system" fn BeginMonitor<Identity: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILayoutStorage_Impl::BeginMonitor(this).into()
            }
        }
        unsafe extern "system" fn EndMonitor<Identity: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILayoutStorage_Impl::EndMonitor(this).into()
            }
        }
        unsafe extern "system" fn ReLayoutDocfile<Identity: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsnewdfname: *const super::wtypesbase::OLECHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILayoutStorage_Impl::ReLayoutDocfile(this, core::mem::transmute_copy(&pwcsnewdfname)).into()
            }
        }
        unsafe extern "system" fn ReLayoutDocfileOnILockBytes<Identity: ILayoutStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pilockbytes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILayoutStorage_Impl::ReLayoutDocfileOnILockBytes(this, core::mem::transmute_copy(&pilockbytes)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LayoutScript: LayoutScript::<Identity, OFFSET>,
            BeginMonitor: BeginMonitor::<Identity, OFFSET>,
            EndMonitor: EndMonitor::<Identity, OFFSET>,
            ReLayoutDocfile: ReLayoutDocfile::<Identity, OFFSET>,
            ReLayoutDocfileOnILockBytes: ReLayoutDocfileOnILockBytes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILayoutStorage as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypesbase")]
impl windows_core::RuntimeName for ILayoutStorage {}
windows_core::imp::define_interface!(ILockBytes, ILockBytes_Vtbl, 0x0000000a_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ILockBytes, windows_core::IUnknown);
impl ILockBytes {
    pub unsafe fn ReadAt(&self, uloffset: u64, pv: *mut core::ffi::c_void, cb: u32, pcbread: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReadAt)(windows_core::Interface::as_raw(self), uloffset, pv as _, cb, pcbread.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn WriteAt(&self, uloffset: u64, pv: *const core::ffi::c_void, cb: u32, pcbwritten: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WriteAt)(windows_core::Interface::as_raw(self), uloffset, pv, cb, pcbwritten.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Flush(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetSize(&self, cb: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSize)(windows_core::Interface::as_raw(self), cb) }
    }
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockRegion)(windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype) }
    }
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockRegion)(windows_core::Interface::as_raw(self), liboffset, cb, dwlocktype) }
    }
    #[cfg(all(feature = "minwindef", feature = "objidlbase"))]
    pub unsafe fn Stat(&self, pstatstg: *mut super::objidlbase::STATSTG, grfstatflag: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stat)(windows_core::Interface::as_raw(self), pstatstg as _, grfstatflag) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILockBytes_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ReadAt: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub WriteAt: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *const core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub LockRegion: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u32) -> windows_core::HRESULT,
    pub UnlockRegion: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "objidlbase"))]
    pub Stat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::objidlbase::STATSTG, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "objidlbase")))]
    Stat: usize,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
pub trait ILockBytes_Impl: windows_core::IUnknownImpl {
    fn ReadAt(&self, uloffset: u64, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::Result<()>;
    fn WriteAt(&self, uloffset: u64, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::Result<()>;
    fn Flush(&self) -> windows_core::Result<()>;
    fn SetSize(&self, cb: u64) -> windows_core::Result<()>;
    fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::Result<()>;
    fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::Result<()>;
    fn Stat(&self, pstatstg: *mut super::objidlbase::STATSTG, grfstatflag: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
impl ILockBytes_Vtbl {
    pub const fn new<Identity: ILockBytes_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReadAt<Identity: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloffset: u64, pv: *mut core::ffi::c_void, cb: u32, pcbread: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILockBytes_Impl::ReadAt(this, core::mem::transmute_copy(&uloffset), core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbread)).into()
            }
        }
        unsafe extern "system" fn WriteAt<Identity: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uloffset: u64, pv: *const core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILockBytes_Impl::WriteAt(this, core::mem::transmute_copy(&uloffset), core::mem::transmute_copy(&pv), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcbwritten)).into()
            }
        }
        unsafe extern "system" fn Flush<Identity: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILockBytes_Impl::Flush(this).into()
            }
        }
        unsafe extern "system" fn SetSize<Identity: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cb: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILockBytes_Impl::SetSize(this, core::mem::transmute_copy(&cb)).into()
            }
        }
        unsafe extern "system" fn LockRegion<Identity: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILockBytes_Impl::LockRegion(this, core::mem::transmute_copy(&liboffset), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&dwlocktype)).into()
            }
        }
        unsafe extern "system" fn UnlockRegion<Identity: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILockBytes_Impl::UnlockRegion(this, core::mem::transmute_copy(&liboffset), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&dwlocktype)).into()
            }
        }
        unsafe extern "system" fn Stat<Identity: ILockBytes_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatstg: *mut super::objidlbase::STATSTG, grfstatflag: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILockBytes_Impl::Stat(this, core::mem::transmute_copy(&pstatstg), core::mem::transmute_copy(&grfstatflag)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ReadAt: ReadAt::<Identity, OFFSET>,
            WriteAt: WriteAt::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            LockRegion: LockRegion::<Identity, OFFSET>,
            UnlockRegion: UnlockRegion::<Identity, OFFSET>,
            Stat: Stat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILockBytes as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
impl windows_core::RuntimeName for ILockBytes {}
windows_core::imp::define_interface!(IMallocSpy, IMallocSpy_Vtbl, 0x0000001d_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IMallocSpy, windows_core::IUnknown);
impl IMallocSpy {
    pub unsafe fn PreAlloc(&self, cbrequest: usize) -> usize {
        unsafe { (windows_core::Interface::vtable(self).PreAlloc)(windows_core::Interface::as_raw(self), cbrequest) }
    }
    pub unsafe fn PostAlloc(&self, pactual: *const core::ffi::c_void) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).PostAlloc)(windows_core::Interface::as_raw(self), pactual) }
    }
    pub unsafe fn PreFree(&self, prequest: *const core::ffi::c_void, fspyed: bool) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).PreFree)(windows_core::Interface::as_raw(self), prequest, fspyed.into()) }
    }
    pub unsafe fn PostFree(&self, fspyed: bool) {
        unsafe {
            (windows_core::Interface::vtable(self).PostFree)(windows_core::Interface::as_raw(self), fspyed.into());
        }
    }
    pub unsafe fn PreRealloc(&self, prequest: *const core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut core::ffi::c_void, fspyed: bool) -> usize {
        unsafe { (windows_core::Interface::vtable(self).PreRealloc)(windows_core::Interface::as_raw(self), prequest, cbrequest, ppnewrequest as _, fspyed.into()) }
    }
    pub unsafe fn PostRealloc(&self, pactual: *const core::ffi::c_void, fspyed: bool) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).PostRealloc)(windows_core::Interface::as_raw(self), pactual, fspyed.into()) }
    }
    pub unsafe fn PreGetSize(&self, prequest: *const core::ffi::c_void, fspyed: bool) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).PreGetSize)(windows_core::Interface::as_raw(self), prequest, fspyed.into()) }
    }
    pub unsafe fn PostGetSize(&self, cbactual: usize, fspyed: bool) -> usize {
        unsafe { (windows_core::Interface::vtable(self).PostGetSize)(windows_core::Interface::as_raw(self), cbactual, fspyed.into()) }
    }
    pub unsafe fn PreDidAlloc(&self, prequest: *const core::ffi::c_void, fspyed: bool) -> *mut core::ffi::c_void {
        unsafe { (windows_core::Interface::vtable(self).PreDidAlloc)(windows_core::Interface::as_raw(self), prequest, fspyed.into()) }
    }
    pub unsafe fn PostDidAlloc(&self, prequest: *const core::ffi::c_void, fspyed: bool, factual: i32) -> i32 {
        unsafe { (windows_core::Interface::vtable(self).PostDidAlloc)(windows_core::Interface::as_raw(self), prequest, fspyed.into(), factual) }
    }
    pub unsafe fn PreHeapMinimize(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PreHeapMinimize)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn PostHeapMinimize(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).PostHeapMinimize)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMallocSpy_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub PreAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> usize,
    pub PostAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> *mut core::ffi::c_void,
    pub PreFree: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::BOOL) -> *mut core::ffi::c_void,
    pub PostFree: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL),
    pub PreRealloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, usize, *mut *mut core::ffi::c_void, windows_core::BOOL) -> usize,
    pub PostRealloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::BOOL) -> *mut core::ffi::c_void,
    pub PreGetSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::BOOL) -> *mut core::ffi::c_void,
    pub PostGetSize: unsafe extern "system" fn(*mut core::ffi::c_void, usize, windows_core::BOOL) -> usize,
    pub PreDidAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::BOOL) -> *mut core::ffi::c_void,
    pub PostDidAlloc: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, windows_core::BOOL, i32) -> i32,
    pub PreHeapMinimize: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub PostHeapMinimize: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IMallocSpy_Impl: windows_core::IUnknownImpl {
    fn PreAlloc(&self, cbrequest: usize) -> usize;
    fn PostAlloc(&self, pactual: *const core::ffi::c_void) -> *mut core::ffi::c_void;
    fn PreFree(&self, prequest: *const core::ffi::c_void, fspyed: windows_core::BOOL) -> *mut core::ffi::c_void;
    fn PostFree(&self, fspyed: windows_core::BOOL);
    fn PreRealloc(&self, prequest: *const core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut core::ffi::c_void, fspyed: windows_core::BOOL) -> usize;
    fn PostRealloc(&self, pactual: *const core::ffi::c_void, fspyed: windows_core::BOOL) -> *mut core::ffi::c_void;
    fn PreGetSize(&self, prequest: *const core::ffi::c_void, fspyed: windows_core::BOOL) -> *mut core::ffi::c_void;
    fn PostGetSize(&self, cbactual: usize, fspyed: windows_core::BOOL) -> usize;
    fn PreDidAlloc(&self, prequest: *const core::ffi::c_void, fspyed: windows_core::BOOL) -> *mut core::ffi::c_void;
    fn PostDidAlloc(&self, prequest: *const core::ffi::c_void, fspyed: windows_core::BOOL, factual: i32) -> i32;
    fn PreHeapMinimize(&self);
    fn PostHeapMinimize(&self);
}
impl IMallocSpy_Vtbl {
    pub const fn new<Identity: IMallocSpy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PreAlloc<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbrequest: usize) -> usize {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PreAlloc(this, core::mem::transmute_copy(&cbrequest))
            }
        }
        unsafe extern "system" fn PostAlloc<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pactual: *const core::ffi::c_void) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PostAlloc(this, core::mem::transmute_copy(&pactual))
            }
        }
        unsafe extern "system" fn PreFree<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequest: *const core::ffi::c_void, fspyed: windows_core::BOOL) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PreFree(this, core::mem::transmute_copy(&prequest), core::mem::transmute_copy(&fspyed))
            }
        }
        unsafe extern "system" fn PostFree<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fspyed: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PostFree(this, core::mem::transmute_copy(&fspyed));
            }
        }
        unsafe extern "system" fn PreRealloc<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequest: *const core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut core::ffi::c_void, fspyed: windows_core::BOOL) -> usize {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PreRealloc(this, core::mem::transmute_copy(&prequest), core::mem::transmute_copy(&cbrequest), core::mem::transmute_copy(&ppnewrequest), core::mem::transmute_copy(&fspyed))
            }
        }
        unsafe extern "system" fn PostRealloc<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pactual: *const core::ffi::c_void, fspyed: windows_core::BOOL) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PostRealloc(this, core::mem::transmute_copy(&pactual), core::mem::transmute_copy(&fspyed))
            }
        }
        unsafe extern "system" fn PreGetSize<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequest: *const core::ffi::c_void, fspyed: windows_core::BOOL) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PreGetSize(this, core::mem::transmute_copy(&prequest), core::mem::transmute_copy(&fspyed))
            }
        }
        unsafe extern "system" fn PostGetSize<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbactual: usize, fspyed: windows_core::BOOL) -> usize {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PostGetSize(this, core::mem::transmute_copy(&cbactual), core::mem::transmute_copy(&fspyed))
            }
        }
        unsafe extern "system" fn PreDidAlloc<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequest: *const core::ffi::c_void, fspyed: windows_core::BOOL) -> *mut core::ffi::c_void {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PreDidAlloc(this, core::mem::transmute_copy(&prequest), core::mem::transmute_copy(&fspyed))
            }
        }
        unsafe extern "system" fn PostDidAlloc<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequest: *const core::ffi::c_void, fspyed: windows_core::BOOL, factual: i32) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PostDidAlloc(this, core::mem::transmute_copy(&prequest), core::mem::transmute_copy(&fspyed), core::mem::transmute_copy(&factual))
            }
        }
        unsafe extern "system" fn PreHeapMinimize<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PreHeapMinimize(this);
            }
        }
        unsafe extern "system" fn PostHeapMinimize<Identity: IMallocSpy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMallocSpy_Impl::PostHeapMinimize(this);
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PreAlloc: PreAlloc::<Identity, OFFSET>,
            PostAlloc: PostAlloc::<Identity, OFFSET>,
            PreFree: PreFree::<Identity, OFFSET>,
            PostFree: PostFree::<Identity, OFFSET>,
            PreRealloc: PreRealloc::<Identity, OFFSET>,
            PostRealloc: PostRealloc::<Identity, OFFSET>,
            PreGetSize: PreGetSize::<Identity, OFFSET>,
            PostGetSize: PostGetSize::<Identity, OFFSET>,
            PreDidAlloc: PreDidAlloc::<Identity, OFFSET>,
            PostDidAlloc: PostDidAlloc::<Identity, OFFSET>,
            PreHeapMinimize: PreHeapMinimize::<Identity, OFFSET>,
            PostHeapMinimize: PostHeapMinimize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMallocSpy as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMallocSpy {}
windows_core::imp::define_interface!(IMessageFilter, IMessageFilter_Vtbl, 0x00000016_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IMessageFilter, windows_core::IUnknown);
impl IMessageFilter {
    #[cfg(feature = "minwindef")]
    pub unsafe fn HandleInComingCall(&self, dwcalltype: u32, htaskcaller: super::minwindef::HTASK, dwtickcount: u32, lpinterfaceinfo: Option<*const INTERFACEINFO>) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).HandleInComingCall)(windows_core::Interface::as_raw(self), dwcalltype, htaskcaller, dwtickcount, lpinterfaceinfo.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn RetryRejectedCall(&self, htaskcallee: super::minwindef::HTASK, dwtickcount: u32, dwrejecttype: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).RetryRejectedCall)(windows_core::Interface::as_raw(self), htaskcallee, dwtickcount, dwrejecttype) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn MessagePending(&self, htaskcallee: super::minwindef::HTASK, dwtickcount: u32, dwpendingtype: u32) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).MessagePending)(windows_core::Interface::as_raw(self), htaskcallee, dwtickcount, dwpendingtype) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "minwindef")]
    pub HandleInComingCall: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::minwindef::HTASK, u32, *const INTERFACEINFO) -> u32,
    #[cfg(not(feature = "minwindef"))]
    HandleInComingCall: usize,
    #[cfg(feature = "minwindef")]
    pub RetryRejectedCall: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::HTASK, u32, u32) -> u32,
    #[cfg(not(feature = "minwindef"))]
    RetryRejectedCall: usize,
    #[cfg(feature = "minwindef")]
    pub MessagePending: unsafe extern "system" fn(*mut core::ffi::c_void, super::minwindef::HTASK, u32, u32) -> u32,
    #[cfg(not(feature = "minwindef"))]
    MessagePending: usize,
}
#[cfg(feature = "minwindef")]
pub trait IMessageFilter_Impl: windows_core::IUnknownImpl {
    fn HandleInComingCall(&self, dwcalltype: u32, htaskcaller: super::minwindef::HTASK, dwtickcount: u32, lpinterfaceinfo: *const INTERFACEINFO) -> u32;
    fn RetryRejectedCall(&self, htaskcallee: super::minwindef::HTASK, dwtickcount: u32, dwrejecttype: u32) -> u32;
    fn MessagePending(&self, htaskcallee: super::minwindef::HTASK, dwtickcount: u32, dwpendingtype: u32) -> u32;
}
#[cfg(feature = "minwindef")]
impl IMessageFilter_Vtbl {
    pub const fn new<Identity: IMessageFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleInComingCall<Identity: IMessageFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcalltype: u32, htaskcaller: super::minwindef::HTASK, dwtickcount: u32, lpinterfaceinfo: *const INTERFACEINFO) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMessageFilter_Impl::HandleInComingCall(this, core::mem::transmute_copy(&dwcalltype), core::mem::transmute_copy(&htaskcaller), core::mem::transmute_copy(&dwtickcount), core::mem::transmute_copy(&lpinterfaceinfo))
            }
        }
        unsafe extern "system" fn RetryRejectedCall<Identity: IMessageFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, htaskcallee: super::minwindef::HTASK, dwtickcount: u32, dwrejecttype: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMessageFilter_Impl::RetryRejectedCall(this, core::mem::transmute_copy(&htaskcallee), core::mem::transmute_copy(&dwtickcount), core::mem::transmute_copy(&dwrejecttype))
            }
        }
        unsafe extern "system" fn MessagePending<Identity: IMessageFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, htaskcallee: super::minwindef::HTASK, dwtickcount: u32, dwpendingtype: u32) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMessageFilter_Impl::MessagePending(this, core::mem::transmute_copy(&htaskcallee), core::mem::transmute_copy(&dwtickcount), core::mem::transmute_copy(&dwpendingtype))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            HandleInComingCall: HandleInComingCall::<Identity, OFFSET>,
            RetryRejectedCall: RetryRejectedCall::<Identity, OFFSET>,
            MessagePending: MessagePending::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMessageFilter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "minwindef")]
impl windows_core::RuntimeName for IMessageFilter {}
windows_core::imp::define_interface!(IMoniker, IMoniker_Vtbl, 0x0000000f_0000_0000_c000_000000000046);
impl core::ops::Deref for IMoniker {
    type Target = IPersistStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMoniker, windows_core::IUnknown, IPersist, IPersistStream);
impl IMoniker {
    pub unsafe fn BindToObject<P0, P1, T>(&self, pbc: P0, pmktoleft: P1) -> windows_core::Result<T>
    where
        P0: windows_core::Param<IBindCtx>,
        P1: windows_core::Param<Self>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).BindToObject)(windows_core::Interface::as_raw(self), pbc.param().abi(), pmktoleft.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn BindToStorage<P0, P1, T>(&self, pbc: P0, pmktoleft: P1) -> windows_core::Result<T>
    where
        P0: windows_core::Param<IBindCtx>,
        P1: windows_core::Param<Self>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).BindToStorage)(windows_core::Interface::as_raw(self), pbc.param().abi(), pmktoleft.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn Reduce<P0>(&self, pbc: P0, dwreducehowfar: u32, ppmktoleft: *mut Option<Self>, ppmkreduced: *mut Option<Self>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBindCtx>,
    {
        unsafe { (windows_core::Interface::vtable(self).Reduce)(windows_core::Interface::as_raw(self), pbc.param().abi(), dwreducehowfar, core::mem::transmute(ppmktoleft), core::mem::transmute(ppmkreduced)) }
    }
    pub unsafe fn ComposeWith<P0>(&self, pmkright: P0, fonlyifnotgeneric: bool) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ComposeWith)(windows_core::Interface::as_raw(self), pmkright.param().abi(), fonlyifnotgeneric.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Enum(&self, fforward: bool) -> windows_core::Result<IEnumMoniker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Enum)(windows_core::Interface::as_raw(self), fforward.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsEqual<P0>(&self, pmkothermoniker: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsEqual)(windows_core::Interface::as_raw(self), pmkothermoniker.param().abi()) }
    }
    pub unsafe fn Hash(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Hash)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsRunning<P0, P1, P2>(&self, pbc: P0, pmktoleft: P1, pmknewlyrunning: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBindCtx>,
        P1: windows_core::Param<Self>,
        P2: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsRunning)(windows_core::Interface::as_raw(self), pbc.param().abi(), pmktoleft.param().abi(), pmknewlyrunning.param().abi()) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetTimeOfLastChange<P0, P1>(&self, pbc: P0, pmktoleft: P1) -> windows_core::Result<super::minwindef::FILETIME>
    where
        P0: windows_core::Param<IBindCtx>,
        P1: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTimeOfLastChange)(windows_core::Interface::as_raw(self), pbc.param().abi(), pmktoleft.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Inverse(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Inverse)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CommonPrefixWith<P0>(&self, pmkother: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CommonPrefixWith)(windows_core::Interface::as_raw(self), pmkother.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RelativePathTo<P0>(&self, pmkother: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RelativePathTo)(windows_core::Interface::as_raw(self), pmkother.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetDisplayName<P0, P1>(&self, pbc: P0, pmktoleft: P1) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<IBindCtx>,
        P1: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), pbc.param().abi(), pmktoleft.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ParseDisplayName<P0, P1, P2>(&self, pbc: P0, pmktoleft: P1, pszdisplayname: P2, pcheaten: *mut u32, ppmkout: *mut Option<Self>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBindCtx>,
        P1: windows_core::Param<Self>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ParseDisplayName)(windows_core::Interface::as_raw(self), pbc.param().abi(), pmktoleft.param().abi(), pszdisplayname.param().abi(), pcheaten as _, core::mem::transmute(ppmkout)) }
    }
    pub unsafe fn IsSystemMoniker(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSystemMoniker)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMoniker_Vtbl {
    pub base__: IPersistStream_Vtbl,
    pub BindToObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BindToStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reduce: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ComposeWith: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Enum: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsEqual: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Hash: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub IsRunning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "minwindef")]
    pub GetTimeOfLastChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::minwindef::FILETIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetTimeOfLastChange: usize,
    pub Inverse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommonPrefixWith: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RelativePathTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub ParseDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSystemMoniker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
pub trait IMoniker_Impl: IPersistStream_Impl {
    fn BindToObject(&self, pbc: windows_core::Ref<IBindCtx>, pmktoleft: windows_core::Ref<IMoniker>, riidresult: *const windows_core::GUID, ppvresult: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn BindToStorage(&self, pbc: windows_core::Ref<IBindCtx>, pmktoleft: windows_core::Ref<IMoniker>, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Reduce(&self, pbc: windows_core::Ref<IBindCtx>, dwreducehowfar: u32, ppmktoleft: windows_core::OutRef<IMoniker>, ppmkreduced: windows_core::OutRef<IMoniker>) -> windows_core::Result<()>;
    fn ComposeWith(&self, pmkright: windows_core::Ref<IMoniker>, fonlyifnotgeneric: windows_core::BOOL) -> windows_core::Result<IMoniker>;
    fn Enum(&self, fforward: windows_core::BOOL) -> windows_core::Result<IEnumMoniker>;
    fn IsEqual(&self, pmkothermoniker: windows_core::Ref<IMoniker>) -> windows_core::Result<()>;
    fn Hash(&self) -> windows_core::Result<u32>;
    fn IsRunning(&self, pbc: windows_core::Ref<IBindCtx>, pmktoleft: windows_core::Ref<IMoniker>, pmknewlyrunning: windows_core::Ref<IMoniker>) -> windows_core::Result<()>;
    fn GetTimeOfLastChange(&self, pbc: windows_core::Ref<IBindCtx>, pmktoleft: windows_core::Ref<IMoniker>) -> windows_core::Result<super::minwindef::FILETIME>;
    fn Inverse(&self) -> windows_core::Result<IMoniker>;
    fn CommonPrefixWith(&self, pmkother: windows_core::Ref<IMoniker>) -> windows_core::Result<IMoniker>;
    fn RelativePathTo(&self, pmkother: windows_core::Ref<IMoniker>) -> windows_core::Result<IMoniker>;
    fn GetDisplayName(&self, pbc: windows_core::Ref<IBindCtx>, pmktoleft: windows_core::Ref<IMoniker>) -> windows_core::Result<windows_core::PWSTR>;
    fn ParseDisplayName(&self, pbc: windows_core::Ref<IBindCtx>, pmktoleft: windows_core::Ref<IMoniker>, pszdisplayname: &windows_core::PCWSTR, pcheaten: *mut u32, ppmkout: windows_core::OutRef<IMoniker>) -> windows_core::Result<()>;
    fn IsSystemMoniker(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
impl IMoniker_Vtbl {
    pub const fn new<Identity: IMoniker_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BindToObject<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pmktoleft: *mut core::ffi::c_void, riidresult: *const windows_core::GUID, ppvresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMoniker_Impl::BindToObject(this, core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&pmktoleft), core::mem::transmute_copy(&riidresult), core::mem::transmute_copy(&ppvresult)).into()
            }
        }
        unsafe extern "system" fn BindToStorage<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pmktoleft: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMoniker_Impl::BindToStorage(this, core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&pmktoleft), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobj)).into()
            }
        }
        unsafe extern "system" fn Reduce<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, dwreducehowfar: u32, ppmktoleft: *mut *mut core::ffi::c_void, ppmkreduced: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMoniker_Impl::Reduce(this, core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&dwreducehowfar), core::mem::transmute_copy(&ppmktoleft), core::mem::transmute_copy(&ppmkreduced)).into()
            }
        }
        unsafe extern "system" fn ComposeWith<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkright: *mut core::ffi::c_void, fonlyifnotgeneric: windows_core::BOOL, ppmkcomposite: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMoniker_Impl::ComposeWith(this, core::mem::transmute_copy(&pmkright), core::mem::transmute_copy(&fonlyifnotgeneric)) {
                    Ok(ok__) => {
                        ppmkcomposite.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Enum<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fforward: windows_core::BOOL, ppenummoniker: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMoniker_Impl::Enum(this, core::mem::transmute_copy(&fforward)) {
                    Ok(ok__) => {
                        ppenummoniker.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEqual<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkothermoniker: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMoniker_Impl::IsEqual(this, core::mem::transmute_copy(&pmkothermoniker)).into()
            }
        }
        unsafe extern "system" fn Hash<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhash: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMoniker_Impl::Hash(this) {
                    Ok(ok__) => {
                        pdwhash.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsRunning<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pmktoleft: *mut core::ffi::c_void, pmknewlyrunning: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMoniker_Impl::IsRunning(this, core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&pmktoleft), core::mem::transmute_copy(&pmknewlyrunning)).into()
            }
        }
        unsafe extern "system" fn GetTimeOfLastChange<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pmktoleft: *mut core::ffi::c_void, pfiletime: *mut super::minwindef::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMoniker_Impl::GetTimeOfLastChange(this, core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&pmktoleft)) {
                    Ok(ok__) => {
                        pfiletime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Inverse<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMoniker_Impl::Inverse(this) {
                    Ok(ok__) => {
                        ppmk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CommonPrefixWith<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkother: *mut core::ffi::c_void, ppmkprefix: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMoniker_Impl::CommonPrefixWith(this, core::mem::transmute_copy(&pmkother)) {
                    Ok(ok__) => {
                        ppmkprefix.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RelativePathTo<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkother: *mut core::ffi::c_void, ppmkrelpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMoniker_Impl::RelativePathTo(this, core::mem::transmute_copy(&pmkother)) {
                    Ok(ok__) => {
                        ppmkrelpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pmktoleft: *mut core::ffi::c_void, ppszdisplayname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMoniker_Impl::GetDisplayName(this, core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&pmktoleft)) {
                    Ok(ok__) => {
                        ppszdisplayname.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ParseDisplayName<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void, pmktoleft: *mut core::ffi::c_void, pszdisplayname: windows_core::PCWSTR, pcheaten: *mut u32, ppmkout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMoniker_Impl::ParseDisplayName(this, core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&pmktoleft), core::mem::transmute(&pszdisplayname), core::mem::transmute_copy(&pcheaten), core::mem::transmute_copy(&ppmkout)).into()
            }
        }
        unsafe extern "system" fn IsSystemMoniker<Identity: IMoniker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmksys: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMoniker_Impl::IsSystemMoniker(this) {
                    Ok(ok__) => {
                        pdwmksys.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IPersistStream_Vtbl::new::<Identity, OFFSET>(),
            BindToObject: BindToObject::<Identity, OFFSET>,
            BindToStorage: BindToStorage::<Identity, OFFSET>,
            Reduce: Reduce::<Identity, OFFSET>,
            ComposeWith: ComposeWith::<Identity, OFFSET>,
            Enum: Enum::<Identity, OFFSET>,
            IsEqual: IsEqual::<Identity, OFFSET>,
            Hash: Hash::<Identity, OFFSET>,
            IsRunning: IsRunning::<Identity, OFFSET>,
            GetTimeOfLastChange: GetTimeOfLastChange::<Identity, OFFSET>,
            Inverse: Inverse::<Identity, OFFSET>,
            CommonPrefixWith: CommonPrefixWith::<Identity, OFFSET>,
            RelativePathTo: RelativePathTo::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            ParseDisplayName: ParseDisplayName::<Identity, OFFSET>,
            IsSystemMoniker: IsSystemMoniker::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMoniker as windows_core::Interface>::IID || iid == &<IPersist as windows_core::Interface>::IID || iid == &<IPersistStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
impl windows_core::RuntimeName for IMoniker {}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct INTERFACEINFO {
    pub pUnk: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub iid: windows_core::GUID,
    pub wMethod: u16,
}
windows_core::imp::define_interface!(IOplockStorage, IOplockStorage_Vtbl, 0x8d19c834_8879_11d1_83e9_00c04fc2c6d4);
windows_core::imp::interface_hierarchy!(IOplockStorage, windows_core::IUnknown);
impl IOplockStorage {
    pub unsafe fn CreateStorageEx<P0, T>(&self, pwcsname: P0, grfmode: u32, stgfmt: u32, grfattrs: u32) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateStorageEx)(windows_core::Interface::as_raw(self), pwcsname.param().abi(), grfmode, stgfmt, grfattrs, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn OpenStorageEx<P0, T>(&self, pwcsname: P0, grfmode: u32, stgfmt: u32, grfattrs: u32) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).OpenStorageEx)(windows_core::Interface::as_raw(self), pwcsname.param().abi(), grfmode, stgfmt, grfattrs, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IOplockStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateStorageEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenStorageEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IOplockStorage_Impl: windows_core::IUnknownImpl {
    fn CreateStorageEx(&self, pwcsname: &windows_core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const windows_core::GUID, ppstgopen: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn OpenStorageEx(&self, pwcsname: &windows_core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const windows_core::GUID, ppstgopen: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IOplockStorage_Vtbl {
    pub const fn new<Identity: IOplockStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateStorageEx<Identity: IOplockStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: windows_core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const windows_core::GUID, ppstgopen: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOplockStorage_Impl::CreateStorageEx(this, core::mem::transmute(&pwcsname), core::mem::transmute_copy(&grfmode), core::mem::transmute_copy(&stgfmt), core::mem::transmute_copy(&grfattrs), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppstgopen)).into()
            }
        }
        unsafe extern "system" fn OpenStorageEx<Identity: IOplockStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: windows_core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const windows_core::GUID, ppstgopen: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IOplockStorage_Impl::OpenStorageEx(this, core::mem::transmute(&pwcsname), core::mem::transmute_copy(&grfmode), core::mem::transmute_copy(&stgfmt), core::mem::transmute_copy(&grfattrs), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppstgopen)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateStorageEx: CreateStorageEx::<Identity, OFFSET>,
            OpenStorageEx: OpenStorageEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOplockStorage as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IOplockStorage {}
windows_core::imp::define_interface!(IPersist, IPersist_Vtbl, 0x0000010c_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IPersist, windows_core::IUnknown);
impl IPersist {
    pub unsafe fn GetClassID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClassID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersist_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClassID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IPersist_Impl: windows_core::IUnknownImpl {
    fn GetClassID(&self) -> windows_core::Result<windows_core::GUID>;
}
impl IPersist_Vtbl {
    pub const fn new<Identity: IPersist_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClassID<Identity: IPersist_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclassid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPersist_Impl::GetClassID(this) {
                    Ok(ok__) => {
                        pclassid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetClassID: GetClassID::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersist as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPersist {}
windows_core::imp::define_interface!(IPersistFile, IPersistFile_Vtbl, 0x0000010b_0000_0000_c000_000000000046);
impl core::ops::Deref for IPersistFile {
    type Target = IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPersistFile, windows_core::IUnknown, IPersist);
impl IPersistFile {
    pub unsafe fn IsDirty(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Load<P0>(&self, pszfilename: P0, dwmode: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), pszfilename.param().abi(), dwmode) }
    }
    pub unsafe fn Save<P0>(&self, pszfilename: P0, fremember: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), pszfilename.param().abi(), fremember.into()) }
    }
    pub unsafe fn SaveCompleted<P0>(&self, pszfilename: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveCompleted)(windows_core::Interface::as_raw(self), pszfilename.param().abi()) }
    }
    pub unsafe fn GetCurFile(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurFile)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistFile_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL) -> windows_core::HRESULT,
    pub SaveCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetCurFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IPersistFile_Impl: IPersist_Impl {
    fn IsDirty(&self) -> windows_core::Result<()>;
    fn Load(&self, pszfilename: &windows_core::PCWSTR, dwmode: u32) -> windows_core::Result<()>;
    fn Save(&self, pszfilename: &windows_core::PCWSTR, fremember: windows_core::BOOL) -> windows_core::Result<()>;
    fn SaveCompleted(&self, pszfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetCurFile(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IPersistFile_Vtbl {
    pub const fn new<Identity: IPersistFile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDirty<Identity: IPersistFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistFile_Impl::IsDirty(this).into()
            }
        }
        unsafe extern "system" fn Load<Identity: IPersistFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilename: windows_core::PCWSTR, dwmode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistFile_Impl::Load(this, core::mem::transmute(&pszfilename), core::mem::transmute_copy(&dwmode)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IPersistFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilename: windows_core::PCWSTR, fremember: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistFile_Impl::Save(this, core::mem::transmute(&pszfilename), core::mem::transmute_copy(&fremember)).into()
            }
        }
        unsafe extern "system" fn SaveCompleted<Identity: IPersistFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfilename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistFile_Impl::SaveCompleted(this, core::mem::transmute(&pszfilename)).into()
            }
        }
        unsafe extern "system" fn GetCurFile<Identity: IPersistFile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszfilename: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPersistFile_Impl::GetCurFile(this) {
                    Ok(ok__) => {
                        ppszfilename.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IPersist_Vtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            SaveCompleted: SaveCompleted::<Identity, OFFSET>,
            GetCurFile: GetCurFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistFile as windows_core::Interface>::IID || iid == &<IPersist as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPersistFile {}
windows_core::imp::define_interface!(IPersistStorage, IPersistStorage_Vtbl, 0x0000010a_0000_0000_c000_000000000046);
impl core::ops::Deref for IPersistStorage {
    type Target = IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPersistStorage, windows_core::IUnknown, IPersist);
impl IPersistStorage {
    pub unsafe fn IsDirty(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn InitNew<P0>(&self, pstg: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitNew)(windows_core::Interface::as_raw(self), pstg.param().abi()) }
    }
    pub unsafe fn Load<P0>(&self, pstg: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), pstg.param().abi()) }
    }
    pub unsafe fn Save<P0>(&self, pstgsave: P0, fsameasload: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), pstgsave.param().abi(), fsameasload.into()) }
    }
    pub unsafe fn SaveCompleted<P0>(&self, pstgnew: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveCompleted)(windows_core::Interface::as_raw(self), pstgnew.param().abi()) }
    }
    pub unsafe fn HandsOffStorage(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HandsOffStorage)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStorage_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitNew: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SaveCompleted: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HandsOffStorage: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IPersistStorage_Impl: IPersist_Impl {
    fn IsDirty(&self) -> windows_core::Result<()>;
    fn InitNew(&self, pstg: windows_core::Ref<IStorage>) -> windows_core::Result<()>;
    fn Load(&self, pstg: windows_core::Ref<IStorage>) -> windows_core::Result<()>;
    fn Save(&self, pstgsave: windows_core::Ref<IStorage>, fsameasload: windows_core::BOOL) -> windows_core::Result<()>;
    fn SaveCompleted(&self, pstgnew: windows_core::Ref<IStorage>) -> windows_core::Result<()>;
    fn HandsOffStorage(&self) -> windows_core::Result<()>;
}
impl IPersistStorage_Vtbl {
    pub const fn new<Identity: IPersistStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDirty<Identity: IPersistStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStorage_Impl::IsDirty(this).into()
            }
        }
        unsafe extern "system" fn InitNew<Identity: IPersistStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstg: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStorage_Impl::InitNew(this, core::mem::transmute_copy(&pstg)).into()
            }
        }
        unsafe extern "system" fn Load<Identity: IPersistStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstg: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStorage_Impl::Load(this, core::mem::transmute_copy(&pstg)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IPersistStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstgsave: *mut core::ffi::c_void, fsameasload: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStorage_Impl::Save(this, core::mem::transmute_copy(&pstgsave), core::mem::transmute_copy(&fsameasload)).into()
            }
        }
        unsafe extern "system" fn SaveCompleted<Identity: IPersistStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstgnew: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStorage_Impl::SaveCompleted(this, core::mem::transmute_copy(&pstgnew)).into()
            }
        }
        unsafe extern "system" fn HandsOffStorage<Identity: IPersistStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStorage_Impl::HandsOffStorage(this).into()
            }
        }
        Self {
            base__: IPersist_Vtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, OFFSET>,
            InitNew: InitNew::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            SaveCompleted: SaveCompleted::<Identity, OFFSET>,
            HandsOffStorage: HandsOffStorage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistStorage as windows_core::Interface>::IID || iid == &<IPersist as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IPersistStorage {}
windows_core::imp::define_interface!(IPersistStream, IPersistStream_Vtbl, 0x00000109_0000_0000_c000_000000000046);
impl core::ops::Deref for IPersistStream {
    type Target = IPersist;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPersistStream, windows_core::IUnknown, IPersist);
impl IPersistStream {
    pub unsafe fn IsDirty(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsDirty)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn Load<P0>(&self, pstm: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), pstm.param().abi()) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn Save<P0>(&self, pstm: P0, fcleardirty: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self), pstm.param().abi(), fcleardirty.into()) }
    }
    pub unsafe fn GetSizeMax(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSizeMax)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPersistStream_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    Load: usize,
    #[cfg(feature = "objidlbase")]
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    Save: usize,
    pub GetSizeMax: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
#[cfg(feature = "objidlbase")]
pub trait IPersistStream_Impl: IPersist_Impl {
    fn IsDirty(&self) -> windows_core::Result<()>;
    fn Load(&self, pstm: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn Save(&self, pstm: windows_core::Ref<super::objidlbase::IStream>, fcleardirty: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetSizeMax(&self) -> windows_core::Result<u64>;
}
#[cfg(feature = "objidlbase")]
impl IPersistStream_Vtbl {
    pub const fn new<Identity: IPersistStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsDirty<Identity: IPersistStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStream_Impl::IsDirty(this).into()
            }
        }
        unsafe extern "system" fn Load<Identity: IPersistStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStream_Impl::Load(this, core::mem::transmute_copy(&pstm)).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IPersistStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void, fcleardirty: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPersistStream_Impl::Save(this, core::mem::transmute_copy(&pstm), core::mem::transmute_copy(&fcleardirty)).into()
            }
        }
        unsafe extern "system" fn GetSizeMax<Identity: IPersistStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcbsize: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPersistStream_Impl::GetSizeMax(this) {
                    Ok(ok__) => {
                        pcbsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IPersist_Vtbl::new::<Identity, OFFSET>(),
            IsDirty: IsDirty::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            GetSizeMax: GetSizeMax::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPersistStream as windows_core::Interface>::IID || iid == &<IPersist as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for IPersistStream {}
windows_core::imp::define_interface!(IProcessLock, IProcessLock_Vtbl, 0x000001d5_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IProcessLock, windows_core::IUnknown);
impl IProcessLock {
    pub unsafe fn AddRefOnProcess(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).AddRefOnProcess)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ReleaseRefOnProcess(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).ReleaseRefOnProcess)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProcessLock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddRefOnProcess: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub ReleaseRefOnProcess: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
pub trait IProcessLock_Impl: windows_core::IUnknownImpl {
    fn AddRefOnProcess(&self) -> u32;
    fn ReleaseRefOnProcess(&self) -> u32;
}
impl IProcessLock_Vtbl {
    pub const fn new<Identity: IProcessLock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddRefOnProcess<Identity: IProcessLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProcessLock_Impl::AddRefOnProcess(this)
            }
        }
        unsafe extern "system" fn ReleaseRefOnProcess<Identity: IProcessLock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProcessLock_Impl::ReleaseRefOnProcess(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddRefOnProcess: AddRefOnProcess::<Identity, OFFSET>,
            ReleaseRefOnProcess: ReleaseRefOnProcess::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProcessLock as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IProcessLock {}
windows_core::imp::define_interface!(IProgressNotify, IProgressNotify_Vtbl, 0xa9d758a0_4617_11cf_95fc_00aa00680db4);
windows_core::imp::interface_hierarchy!(IProgressNotify, windows_core::IUnknown);
impl IProgressNotify {
    pub unsafe fn OnProgress(&self, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: bool, fowner: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnProgress)(windows_core::Interface::as_raw(self), dwprogresscurrent, dwprogressmaximum, faccurate.into(), fowner.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProgressNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnProgress: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IProgressNotify_Impl: windows_core::IUnknownImpl {
    fn OnProgress(&self, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: windows_core::BOOL, fowner: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IProgressNotify_Vtbl {
    pub const fn new<Identity: IProgressNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnProgress<Identity: IProgressNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: windows_core::BOOL, fowner: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProgressNotify_Impl::OnProgress(this, core::mem::transmute_copy(&dwprogresscurrent), core::mem::transmute_copy(&dwprogressmaximum), core::mem::transmute_copy(&faccurate), core::mem::transmute_copy(&fowner)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnProgress: OnProgress::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProgressNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IProgressNotify {}
windows_core::imp::define_interface!(IROTData, IROTData_Vtbl, 0xf29f6bc0_5021_11ce_aa15_00006901293f);
windows_core::imp::interface_hierarchy!(IROTData, windows_core::IUnknown);
impl IROTData {
    #[cfg(feature = "rpc")]
    pub unsafe fn GetComparisonData(&self, pbdata: *mut super::rpc::byte, cbmax: u32, pcbdata: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetComparisonData)(windows_core::Interface::as_raw(self), pbdata as _, cbmax, pcbdata as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IROTData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "rpc")]
    pub GetComparisonData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::rpc::byte, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "rpc"))]
    GetComparisonData: usize,
}
#[cfg(feature = "rpc")]
pub trait IROTData_Impl: windows_core::IUnknownImpl {
    fn GetComparisonData(&self, pbdata: *mut super::rpc::byte, cbmax: u32, pcbdata: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "rpc")]
impl IROTData_Vtbl {
    pub const fn new<Identity: IROTData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetComparisonData<Identity: IROTData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdata: *mut super::rpc::byte, cbmax: u32, pcbdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IROTData_Impl::GetComparisonData(this, core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&cbmax), core::mem::transmute_copy(&pcbdata)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetComparisonData: GetComparisonData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IROTData as windows_core::Interface>::IID
    }
}
#[cfg(feature = "rpc")]
impl windows_core::RuntimeName for IROTData {}
windows_core::imp::define_interface!(IRootStorage, IRootStorage_Vtbl, 0x00000012_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IRootStorage, windows_core::IUnknown);
impl IRootStorage {
    pub unsafe fn SwitchToFile<P0>(&self, pszfile: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SwitchToFile)(windows_core::Interface::as_raw(self), pszfile.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRootStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SwitchToFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IRootStorage_Impl: windows_core::IUnknownImpl {
    fn SwitchToFile(&self, pszfile: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IRootStorage_Vtbl {
    pub const fn new<Identity: IRootStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SwitchToFile<Identity: IRootStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszfile: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRootStorage_Impl::SwitchToFile(this, core::mem::transmute(&pszfile)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SwitchToFile: SwitchToFile::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRootStorage as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRootStorage {}
windows_core::imp::define_interface!(IRunnableObject, IRunnableObject_Vtbl, 0x00000126_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IRunnableObject, windows_core::IUnknown);
impl IRunnableObject {
    pub unsafe fn GetRunningClass(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRunningClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Run<P0>(&self, pbc: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IBindCtx>,
    {
        unsafe { (windows_core::Interface::vtable(self).Run)(windows_core::Interface::as_raw(self), pbc.param().abi()) }
    }
    pub unsafe fn IsRunning(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsRunning)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn LockRunning(&self, flock: bool, flastunlockcloses: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockRunning)(windows_core::Interface::as_raw(self), flock.into(), flastunlockcloses.into()) }
    }
    pub unsafe fn SetContainedObject(&self, fcontained: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetContainedObject)(windows_core::Interface::as_raw(self), fcontained.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunnableObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRunningClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub Run: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsRunning: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub LockRunning: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetContainedObject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IRunnableObject_Impl: windows_core::IUnknownImpl {
    fn GetRunningClass(&self) -> windows_core::Result<windows_core::GUID>;
    fn Run(&self, pbc: windows_core::Ref<IBindCtx>) -> windows_core::Result<()>;
    fn IsRunning(&self) -> windows_core::BOOL;
    fn LockRunning(&self, flock: windows_core::BOOL, flastunlockcloses: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetContainedObject(&self, fcontained: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IRunnableObject_Vtbl {
    pub const fn new<Identity: IRunnableObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRunningClass<Identity: IRunnableObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lpclsid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunnableObject_Impl::GetRunningClass(this) {
                    Ok(ok__) => {
                        lpclsid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Run<Identity: IRunnableObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbc: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRunnableObject_Impl::Run(this, core::mem::transmute_copy(&pbc)).into()
            }
        }
        unsafe extern "system" fn IsRunning<Identity: IRunnableObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRunnableObject_Impl::IsRunning(this)
            }
        }
        unsafe extern "system" fn LockRunning<Identity: IRunnableObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flock: windows_core::BOOL, flastunlockcloses: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRunnableObject_Impl::LockRunning(this, core::mem::transmute_copy(&flock), core::mem::transmute_copy(&flastunlockcloses)).into()
            }
        }
        unsafe extern "system" fn SetContainedObject<Identity: IRunnableObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fcontained: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRunnableObject_Impl::SetContainedObject(this, core::mem::transmute_copy(&fcontained)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRunningClass: GetRunningClass::<Identity, OFFSET>,
            Run: Run::<Identity, OFFSET>,
            IsRunning: IsRunning::<Identity, OFFSET>,
            LockRunning: LockRunning::<Identity, OFFSET>,
            SetContainedObject: SetContainedObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRunnableObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRunnableObject {}
windows_core::imp::define_interface!(IRunningObjectTable, IRunningObjectTable_Vtbl, 0x00000010_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IRunningObjectTable, windows_core::IUnknown);
impl IRunningObjectTable {
    pub unsafe fn Register<P1, P2>(&self, grfflags: u32, punkobject: P1, pmkobjectname: P2) -> windows_core::Result<u32>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<IMoniker>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Register)(windows_core::Interface::as_raw(self), grfflags, punkobject.param().abi(), pmkobjectname.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Revoke(&self, dwregister: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Revoke)(windows_core::Interface::as_raw(self), dwregister) }
    }
    pub unsafe fn IsRunning<P0>(&self, pmkobjectname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMoniker>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsRunning)(windows_core::Interface::as_raw(self), pmkobjectname.param().abi()) }
    }
    pub unsafe fn GetObject<P0>(&self, pmkobjectname: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<IMoniker>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), pmkobjectname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn NoteChangeTime(&self, dwregister: u32, pfiletime: *const super::minwindef::FILETIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NoteChangeTime)(windows_core::Interface::as_raw(self), dwregister, pfiletime) }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetTimeOfLastChange<P0>(&self, pmkobjectname: P0) -> windows_core::Result<super::minwindef::FILETIME>
    where
        P0: windows_core::Param<IMoniker>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTimeOfLastChange)(windows_core::Interface::as_raw(self), pmkobjectname.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnumRunning(&self) -> windows_core::Result<IEnumMoniker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumRunning)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRunningObjectTable_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Revoke: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub IsRunning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "minwindef")]
    pub NoteChangeTime: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::minwindef::FILETIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    NoteChangeTime: usize,
    #[cfg(feature = "minwindef")]
    pub GetTimeOfLastChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::minwindef::FILETIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetTimeOfLastChange: usize,
    pub EnumRunning: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "minwindef")]
pub trait IRunningObjectTable_Impl: windows_core::IUnknownImpl {
    fn Register(&self, grfflags: u32, punkobject: windows_core::Ref<windows_core::IUnknown>, pmkobjectname: windows_core::Ref<IMoniker>) -> windows_core::Result<u32>;
    fn Revoke(&self, dwregister: u32) -> windows_core::Result<()>;
    fn IsRunning(&self, pmkobjectname: windows_core::Ref<IMoniker>) -> windows_core::Result<()>;
    fn GetObject(&self, pmkobjectname: windows_core::Ref<IMoniker>) -> windows_core::Result<windows_core::IUnknown>;
    fn NoteChangeTime(&self, dwregister: u32, pfiletime: *const super::minwindef::FILETIME) -> windows_core::Result<()>;
    fn GetTimeOfLastChange(&self, pmkobjectname: windows_core::Ref<IMoniker>) -> windows_core::Result<super::minwindef::FILETIME>;
    fn EnumRunning(&self) -> windows_core::Result<IEnumMoniker>;
}
#[cfg(feature = "minwindef")]
impl IRunningObjectTable_Vtbl {
    pub const fn new<Identity: IRunningObjectTable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Register<Identity: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfflags: u32, punkobject: *mut core::ffi::c_void, pmkobjectname: *mut core::ffi::c_void, pdwregister: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningObjectTable_Impl::Register(this, core::mem::transmute_copy(&grfflags), core::mem::transmute_copy(&punkobject), core::mem::transmute_copy(&pmkobjectname)) {
                    Ok(ok__) => {
                        pdwregister.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Revoke<Identity: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwregister: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRunningObjectTable_Impl::Revoke(this, core::mem::transmute_copy(&dwregister)).into()
            }
        }
        unsafe extern "system" fn IsRunning<Identity: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkobjectname: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRunningObjectTable_Impl::IsRunning(this, core::mem::transmute_copy(&pmkobjectname)).into()
            }
        }
        unsafe extern "system" fn GetObject<Identity: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkobjectname: *mut core::ffi::c_void, ppunkobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningObjectTable_Impl::GetObject(this, core::mem::transmute_copy(&pmkobjectname)) {
                    Ok(ok__) => {
                        ppunkobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NoteChangeTime<Identity: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwregister: u32, pfiletime: *const super::minwindef::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRunningObjectTable_Impl::NoteChangeTime(this, core::mem::transmute_copy(&dwregister), core::mem::transmute_copy(&pfiletime)).into()
            }
        }
        unsafe extern "system" fn GetTimeOfLastChange<Identity: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmkobjectname: *mut core::ffi::c_void, pfiletime: *mut super::minwindef::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningObjectTable_Impl::GetTimeOfLastChange(this, core::mem::transmute_copy(&pmkobjectname)) {
                    Ok(ok__) => {
                        pfiletime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnumRunning<Identity: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppenummoniker: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRunningObjectTable_Impl::EnumRunning(this) {
                    Ok(ok__) => {
                        ppenummoniker.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Register: Register::<Identity, OFFSET>,
            Revoke: Revoke::<Identity, OFFSET>,
            IsRunning: IsRunning::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            NoteChangeTime: NoteChangeTime::<Identity, OFFSET>,
            GetTimeOfLastChange: GetTimeOfLastChange::<Identity, OFFSET>,
            EnumRunning: EnumRunning::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRunningObjectTable as windows_core::Interface>::IID
    }
}
#[cfg(feature = "minwindef")]
impl windows_core::RuntimeName for IRunningObjectTable {}
windows_core::imp::define_interface!(IStorage, IStorage_Vtbl, 0x0000000b_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IStorage, windows_core::IUnknown);
impl IStorage {
    #[cfg(all(feature = "objidlbase", feature = "wtypesbase"))]
    pub unsafe fn CreateStream(&self, pwcsname: *const super::wtypesbase::OLECHAR, grfmode: u32, reserved1: u32, reserved2: u32) -> windows_core::Result<super::objidlbase::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStream)(windows_core::Interface::as_raw(self), pwcsname, grfmode, reserved1, reserved2, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "objidlbase", feature = "wtypesbase"))]
    pub unsafe fn OpenStream(&self, pwcsname: *const super::wtypesbase::OLECHAR, reserved1: Option<*const core::ffi::c_void>, grfmode: u32, reserved2: u32) -> windows_core::Result<super::objidlbase::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenStream)(windows_core::Interface::as_raw(self), pwcsname, reserved1.unwrap_or(core::mem::zeroed()) as _, grfmode, reserved2, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn CreateStorage(&self, pwcsname: *const super::wtypesbase::OLECHAR, grfmode: u32, reserved1: u32, reserved2: u32) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateStorage)(windows_core::Interface::as_raw(self), pwcsname, grfmode, reserved1, reserved2, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn OpenStorage<P1>(&self, pwcsname: *const super::wtypesbase::OLECHAR, pstgpriority: P1, grfmode: u32, snbexclude: *const windows_core::PCWSTR, reserved: u32) -> windows_core::Result<Self>
    where
        P1: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenStorage)(windows_core::Interface::as_raw(self), pwcsname, pstgpriority.param().abi(), grfmode, snbexclude, reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CopyTo<P3>(&self, rgiidexclude: Option<&[windows_core::GUID]>, snbexclude: Option<*const windows_core::PCWSTR>, pstgdest: P3) -> windows_core::HRESULT
    where
        P3: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).CopyTo)(windows_core::Interface::as_raw(self), rgiidexclude.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgiidexclude.map_or(core::ptr::null(), |slice| slice.as_ptr())), snbexclude.unwrap_or(core::mem::zeroed()) as _, pstgdest.param().abi()) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn MoveElementTo<P1>(&self, pwcsname: *const super::wtypesbase::OLECHAR, pstgdest: P1, pwcsnewname: *const super::wtypesbase::OLECHAR, grfflags: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).MoveElementTo)(windows_core::Interface::as_raw(self), pwcsname, pstgdest.param().abi(), pwcsnewname, grfflags) }
    }
    pub unsafe fn Commit(&self, grfcommitflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Commit)(windows_core::Interface::as_raw(self), grfcommitflags) }
    }
    pub unsafe fn Revert(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Revert)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EnumElements(&self, reserved1: Option<u32>, reserved2: Option<*const core::ffi::c_void>, reserved3: Option<u32>) -> windows_core::Result<IEnumSTATSTG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EnumElements)(windows_core::Interface::as_raw(self), reserved1.unwrap_or(core::mem::zeroed()) as _, reserved2.unwrap_or(core::mem::zeroed()) as _, reserved3.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn DestroyElement(&self, pwcsname: *const super::wtypesbase::OLECHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DestroyElement)(windows_core::Interface::as_raw(self), pwcsname) }
    }
    #[cfg(feature = "wtypesbase")]
    pub unsafe fn RenameElement(&self, pwcsoldname: *const super::wtypesbase::OLECHAR, pwcsnewname: *const super::wtypesbase::OLECHAR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RenameElement)(windows_core::Interface::as_raw(self), pwcsoldname, pwcsnewname) }
    }
    #[cfg(all(feature = "minwindef", feature = "wtypesbase"))]
    pub unsafe fn SetElementTimes(&self, pwcsname: *const super::wtypesbase::OLECHAR, pctime: *const super::minwindef::FILETIME, patime: *const super::minwindef::FILETIME, pmtime: *const super::minwindef::FILETIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetElementTimes)(windows_core::Interface::as_raw(self), pwcsname, pctime, patime, pmtime) }
    }
    pub unsafe fn SetClass(&self, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClass)(windows_core::Interface::as_raw(self), clsid) }
    }
    pub unsafe fn SetStateBits(&self, grfstatebits: u32, grfmask: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStateBits)(windows_core::Interface::as_raw(self), grfstatebits, grfmask) }
    }
    #[cfg(all(feature = "minwindef", feature = "objidlbase"))]
    pub unsafe fn Stat(&self, pstatstg: *mut super::objidlbase::STATSTG, grfstatflag: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stat)(windows_core::Interface::as_raw(self), pstatstg as _, grfstatflag) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorage_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "objidlbase", feature = "wtypesbase"))]
    pub CreateStream: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypesbase::OLECHAR, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidlbase", feature = "wtypesbase")))]
    CreateStream: usize,
    #[cfg(all(feature = "objidlbase", feature = "wtypesbase"))]
    pub OpenStream: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypesbase::OLECHAR, *const core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidlbase", feature = "wtypesbase")))]
    OpenStream: usize,
    #[cfg(feature = "wtypesbase")]
    pub CreateStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypesbase::OLECHAR, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    CreateStorage: usize,
    #[cfg(feature = "wtypesbase")]
    pub OpenStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypesbase::OLECHAR, *mut core::ffi::c_void, u32, *const windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    OpenStorage: usize,
    pub CopyTo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *const windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypesbase")]
    pub MoveElementTo: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypesbase::OLECHAR, *mut core::ffi::c_void, *const super::wtypesbase::OLECHAR, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    MoveElementTo: usize,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Revert: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnumElements: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypesbase")]
    pub DestroyElement: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypesbase::OLECHAR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    DestroyElement: usize,
    #[cfg(feature = "wtypesbase")]
    pub RenameElement: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypesbase::OLECHAR, *const super::wtypesbase::OLECHAR) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypesbase"))]
    RenameElement: usize,
    #[cfg(all(feature = "minwindef", feature = "wtypesbase"))]
    pub SetElementTimes: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypesbase::OLECHAR, *const super::minwindef::FILETIME, *const super::minwindef::FILETIME, *const super::minwindef::FILETIME) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "wtypesbase")))]
    SetElementTimes: usize,
    pub SetClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetStateBits: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "objidlbase"))]
    pub Stat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::objidlbase::STATSTG, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "objidlbase")))]
    Stat: usize,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "wtypesbase"))]
pub trait IStorage_Impl: windows_core::IUnknownImpl {
    fn CreateStream(&self, pwcsname: *const super::wtypesbase::OLECHAR, grfmode: u32, reserved1: u32, reserved2: u32) -> windows_core::Result<super::objidlbase::IStream>;
    fn OpenStream(&self, pwcsname: *const super::wtypesbase::OLECHAR, reserved1: *const core::ffi::c_void, grfmode: u32, reserved2: u32) -> windows_core::Result<super::objidlbase::IStream>;
    fn CreateStorage(&self, pwcsname: *const super::wtypesbase::OLECHAR, grfmode: u32, reserved1: u32, reserved2: u32) -> windows_core::Result<IStorage>;
    fn OpenStorage(&self, pwcsname: *const super::wtypesbase::OLECHAR, pstgpriority: windows_core::Ref<IStorage>, grfmode: u32, snbexclude: *const windows_core::PCWSTR, reserved: u32) -> windows_core::Result<IStorage>;
    fn CopyTo(&self, ciidexclude: u32, rgiidexclude: *const windows_core::GUID, snbexclude: *const windows_core::PCWSTR, pstgdest: windows_core::Ref<IStorage>) -> windows_core::Result<()>;
    fn MoveElementTo(&self, pwcsname: *const super::wtypesbase::OLECHAR, pstgdest: windows_core::Ref<IStorage>, pwcsnewname: *const super::wtypesbase::OLECHAR, grfflags: u32) -> windows_core::Result<()>;
    fn Commit(&self, grfcommitflags: u32) -> windows_core::Result<()>;
    fn Revert(&self) -> windows_core::Result<()>;
    fn EnumElements(&self, reserved1: u32, reserved2: *const core::ffi::c_void, reserved3: u32) -> windows_core::Result<IEnumSTATSTG>;
    fn DestroyElement(&self, pwcsname: *const super::wtypesbase::OLECHAR) -> windows_core::Result<()>;
    fn RenameElement(&self, pwcsoldname: *const super::wtypesbase::OLECHAR, pwcsnewname: *const super::wtypesbase::OLECHAR) -> windows_core::Result<()>;
    fn SetElementTimes(&self, pwcsname: *const super::wtypesbase::OLECHAR, pctime: *const super::minwindef::FILETIME, patime: *const super::minwindef::FILETIME, pmtime: *const super::minwindef::FILETIME) -> windows_core::Result<()>;
    fn SetClass(&self, clsid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetStateBits(&self, grfstatebits: u32, grfmask: u32) -> windows_core::Result<()>;
    fn Stat(&self, pstatstg: *mut super::objidlbase::STATSTG, grfstatflag: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "wtypesbase"))]
impl IStorage_Vtbl {
    pub const fn new<Identity: IStorage_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateStream<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: *const super::wtypesbase::OLECHAR, grfmode: u32, reserved1: u32, reserved2: u32, ppstm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorage_Impl::CreateStream(this, core::mem::transmute_copy(&pwcsname), core::mem::transmute_copy(&grfmode), core::mem::transmute_copy(&reserved1), core::mem::transmute_copy(&reserved2)) {
                    Ok(ok__) => {
                        ppstm.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenStream<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: *const super::wtypesbase::OLECHAR, reserved1: *const core::ffi::c_void, grfmode: u32, reserved2: u32, ppstm: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorage_Impl::OpenStream(this, core::mem::transmute_copy(&pwcsname), core::mem::transmute_copy(&reserved1), core::mem::transmute_copy(&grfmode), core::mem::transmute_copy(&reserved2)) {
                    Ok(ok__) => {
                        ppstm.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateStorage<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: *const super::wtypesbase::OLECHAR, grfmode: u32, reserved1: u32, reserved2: u32, ppstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorage_Impl::CreateStorage(this, core::mem::transmute_copy(&pwcsname), core::mem::transmute_copy(&grfmode), core::mem::transmute_copy(&reserved1), core::mem::transmute_copy(&reserved2)) {
                    Ok(ok__) => {
                        ppstg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenStorage<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: *const super::wtypesbase::OLECHAR, pstgpriority: *mut core::ffi::c_void, grfmode: u32, snbexclude: *const windows_core::PCWSTR, reserved: u32, ppstg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorage_Impl::OpenStorage(this, core::mem::transmute_copy(&pwcsname), core::mem::transmute_copy(&pstgpriority), core::mem::transmute_copy(&grfmode), core::mem::transmute_copy(&snbexclude), core::mem::transmute_copy(&reserved)) {
                    Ok(ok__) => {
                        ppstg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CopyTo<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ciidexclude: u32, rgiidexclude: *const windows_core::GUID, snbexclude: *const windows_core::PCWSTR, pstgdest: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::CopyTo(this, core::mem::transmute_copy(&ciidexclude), core::mem::transmute_copy(&rgiidexclude), core::mem::transmute_copy(&snbexclude), core::mem::transmute_copy(&pstgdest)).into()
            }
        }
        unsafe extern "system" fn MoveElementTo<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: *const super::wtypesbase::OLECHAR, pstgdest: *mut core::ffi::c_void, pwcsnewname: *const super::wtypesbase::OLECHAR, grfflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::MoveElementTo(this, core::mem::transmute_copy(&pwcsname), core::mem::transmute_copy(&pstgdest), core::mem::transmute_copy(&pwcsnewname), core::mem::transmute_copy(&grfflags)).into()
            }
        }
        unsafe extern "system" fn Commit<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfcommitflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::Commit(this, core::mem::transmute_copy(&grfcommitflags)).into()
            }
        }
        unsafe extern "system" fn Revert<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::Revert(this).into()
            }
        }
        unsafe extern "system" fn EnumElements<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reserved1: u32, reserved2: *const core::ffi::c_void, reserved3: u32, ppenum: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStorage_Impl::EnumElements(this, core::mem::transmute_copy(&reserved1), core::mem::transmute_copy(&reserved2), core::mem::transmute_copy(&reserved3)) {
                    Ok(ok__) => {
                        ppenum.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DestroyElement<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: *const super::wtypesbase::OLECHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::DestroyElement(this, core::mem::transmute_copy(&pwcsname)).into()
            }
        }
        unsafe extern "system" fn RenameElement<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsoldname: *const super::wtypesbase::OLECHAR, pwcsnewname: *const super::wtypesbase::OLECHAR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::RenameElement(this, core::mem::transmute_copy(&pwcsoldname), core::mem::transmute_copy(&pwcsnewname)).into()
            }
        }
        unsafe extern "system" fn SetElementTimes<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcsname: *const super::wtypesbase::OLECHAR, pctime: *const super::minwindef::FILETIME, patime: *const super::minwindef::FILETIME, pmtime: *const super::minwindef::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::SetElementTimes(this, core::mem::transmute_copy(&pwcsname), core::mem::transmute_copy(&pctime), core::mem::transmute_copy(&patime), core::mem::transmute_copy(&pmtime)).into()
            }
        }
        unsafe extern "system" fn SetClass<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::SetClass(this, core::mem::transmute_copy(&clsid)).into()
            }
        }
        unsafe extern "system" fn SetStateBits<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfstatebits: u32, grfmask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::SetStateBits(this, core::mem::transmute_copy(&grfstatebits), core::mem::transmute_copy(&grfmask)).into()
            }
        }
        unsafe extern "system" fn Stat<Identity: IStorage_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatstg: *mut super::objidlbase::STATSTG, grfstatflag: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStorage_Impl::Stat(this, core::mem::transmute_copy(&pstatstg), core::mem::transmute_copy(&grfstatflag)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateStream: CreateStream::<Identity, OFFSET>,
            OpenStream: OpenStream::<Identity, OFFSET>,
            CreateStorage: CreateStorage::<Identity, OFFSET>,
            OpenStorage: OpenStorage::<Identity, OFFSET>,
            CopyTo: CopyTo::<Identity, OFFSET>,
            MoveElementTo: MoveElementTo::<Identity, OFFSET>,
            Commit: Commit::<Identity, OFFSET>,
            Revert: Revert::<Identity, OFFSET>,
            EnumElements: EnumElements::<Identity, OFFSET>,
            DestroyElement: DestroyElement::<Identity, OFFSET>,
            RenameElement: RenameElement::<Identity, OFFSET>,
            SetElementTimes: SetElementTimes::<Identity, OFFSET>,
            SetClass: SetClass::<Identity, OFFSET>,
            SetStateBits: SetStateBits::<Identity, OFFSET>,
            Stat: Stat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStorage as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IStorage {}
windows_core::imp::define_interface!(ISurrogateService, ISurrogateService_Vtbl, 0x000001d4_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(ISurrogateService, windows_core::IUnknown);
impl ISurrogateService {
    pub unsafe fn Init<P1>(&self, rguidprocessid: *const windows_core::GUID, pprocesslock: P1) -> windows_core::Result<windows_core::BOOL>
    where
        P1: windows_core::Param<IProcessLock>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), rguidprocessid, pprocesslock.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ApplicationLaunch(&self, rguidapplid: *const windows_core::GUID, apptype: ApplicationType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ApplicationLaunch)(windows_core::Interface::as_raw(self), rguidapplid, apptype) }
    }
    pub unsafe fn ApplicationFree(&self, rguidapplid: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ApplicationFree)(windows_core::Interface::as_raw(self), rguidapplid) }
    }
    pub unsafe fn CatalogRefresh(&self, ulreserved: Option<u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CatalogRefresh)(windows_core::Interface::as_raw(self), ulreserved.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn ProcessShutdown(&self, shutdowntype: ShutdownType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessShutdown)(windows_core::Interface::as_raw(self), shutdowntype) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurrogateService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub ApplicationLaunch: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, ApplicationType) -> windows_core::HRESULT,
    pub ApplicationFree: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub CatalogRefresh: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ProcessShutdown: unsafe extern "system" fn(*mut core::ffi::c_void, ShutdownType) -> windows_core::HRESULT,
}
pub trait ISurrogateService_Impl: windows_core::IUnknownImpl {
    fn Init(&self, rguidprocessid: *const windows_core::GUID, pprocesslock: windows_core::Ref<IProcessLock>) -> windows_core::Result<windows_core::BOOL>;
    fn ApplicationLaunch(&self, rguidapplid: *const windows_core::GUID, apptype: ApplicationType) -> windows_core::Result<()>;
    fn ApplicationFree(&self, rguidapplid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn CatalogRefresh(&self, ulreserved: u32) -> windows_core::Result<()>;
    fn ProcessShutdown(&self, shutdowntype: ShutdownType) -> windows_core::Result<()>;
}
impl ISurrogateService_Vtbl {
    pub const fn new<Identity: ISurrogateService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Init<Identity: ISurrogateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidprocessid: *const windows_core::GUID, pprocesslock: *mut core::ffi::c_void, pfapplicationaware: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISurrogateService_Impl::Init(this, core::mem::transmute_copy(&rguidprocessid), core::mem::transmute_copy(&pprocesslock)) {
                    Ok(ok__) => {
                        pfapplicationaware.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ApplicationLaunch<Identity: ISurrogateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidapplid: *const windows_core::GUID, apptype: ApplicationType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurrogateService_Impl::ApplicationLaunch(this, core::mem::transmute_copy(&rguidapplid), core::mem::transmute_copy(&apptype)).into()
            }
        }
        unsafe extern "system" fn ApplicationFree<Identity: ISurrogateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidapplid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurrogateService_Impl::ApplicationFree(this, core::mem::transmute_copy(&rguidapplid)).into()
            }
        }
        unsafe extern "system" fn CatalogRefresh<Identity: ISurrogateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurrogateService_Impl::CatalogRefresh(this, core::mem::transmute_copy(&ulreserved)).into()
            }
        }
        unsafe extern "system" fn ProcessShutdown<Identity: ISurrogateService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, shutdowntype: ShutdownType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISurrogateService_Impl::ProcessShutdown(this, core::mem::transmute_copy(&shutdowntype)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            ApplicationLaunch: ApplicationLaunch::<Identity, OFFSET>,
            ApplicationFree: ApplicationFree::<Identity, OFFSET>,
            CatalogRefresh: CatalogRefresh::<Identity, OFFSET>,
            ProcessShutdown: ProcessShutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISurrogateService as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISurrogateService {}
windows_core::imp::define_interface!(IThumbnailExtractor, IThumbnailExtractor_Vtbl, 0x969dc708_5c76_11d1_8d86_0000f804b057);
windows_core::imp::interface_hierarchy!(IThumbnailExtractor, windows_core::IUnknown);
impl IThumbnailExtractor {
    #[cfg(feature = "windef")]
    pub unsafe fn ExtractThumbnail<P0>(&self, pstg: P0, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::windef::HBITMAP) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).ExtractThumbnail)(windows_core::Interface::as_raw(self), pstg.param().abi(), ullength, ulheight, puloutputlength as _, puloutputheight as _, phoutputbitmap as _) }
    }
    pub unsafe fn OnFileUpdated<P0>(&self, pstg: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IStorage>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnFileUpdated)(windows_core::Interface::as_raw(self), pstg.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IThumbnailExtractor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub ExtractThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut u32, *mut u32, *mut super::windef::HBITMAP) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ExtractThumbnail: usize,
    pub OnFileUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IThumbnailExtractor_Impl: windows_core::IUnknownImpl {
    fn ExtractThumbnail(&self, pstg: windows_core::Ref<IStorage>, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::windef::HBITMAP) -> windows_core::Result<()>;
    fn OnFileUpdated(&self, pstg: windows_core::Ref<IStorage>) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IThumbnailExtractor_Vtbl {
    pub const fn new<Identity: IThumbnailExtractor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ExtractThumbnail<Identity: IThumbnailExtractor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstg: *mut core::ffi::c_void, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::windef::HBITMAP) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IThumbnailExtractor_Impl::ExtractThumbnail(this, core::mem::transmute_copy(&pstg), core::mem::transmute_copy(&ullength), core::mem::transmute_copy(&ulheight), core::mem::transmute_copy(&puloutputlength), core::mem::transmute_copy(&puloutputheight), core::mem::transmute_copy(&phoutputbitmap)).into()
            }
        }
        unsafe extern "system" fn OnFileUpdated<Identity: IThumbnailExtractor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstg: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IThumbnailExtractor_Impl::OnFileUpdated(this, core::mem::transmute_copy(&pstg)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ExtractThumbnail: ExtractThumbnail::<Identity, OFFSET>,
            OnFileUpdated: OnFileUpdated::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IThumbnailExtractor as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IThumbnailExtractor {}
windows_core::imp::define_interface!(ITimeAndNoticeControl, ITimeAndNoticeControl_Vtbl, 0xbc0bf6ae_8878_11d1_83e9_00c04fc2c6d4);
windows_core::imp::interface_hierarchy!(ITimeAndNoticeControl, windows_core::IUnknown);
impl ITimeAndNoticeControl {
    pub unsafe fn SuppressChanges(&self, res1: u32, res2: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SuppressChanges)(windows_core::Interface::as_raw(self), res1, res2) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITimeAndNoticeControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SuppressChanges: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
pub trait ITimeAndNoticeControl_Impl: windows_core::IUnknownImpl {
    fn SuppressChanges(&self, res1: u32, res2: u32) -> windows_core::Result<()>;
}
impl ITimeAndNoticeControl_Vtbl {
    pub const fn new<Identity: ITimeAndNoticeControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SuppressChanges<Identity: ITimeAndNoticeControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, res1: u32, res2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITimeAndNoticeControl_Impl::SuppressChanges(this, core::mem::transmute_copy(&res1), core::mem::transmute_copy(&res2)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SuppressChanges: SuppressChanges::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITimeAndNoticeControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITimeAndNoticeControl {}
windows_core::imp::define_interface!(IUrlMon, IUrlMon_Vtbl, 0x00000026_0000_0000_c000_000000000046);
windows_core::imp::interface_hierarchy!(IUrlMon, windows_core::IUnknown);
impl IUrlMon {
    pub unsafe fn AsyncGetClassBits<P1, P2, P5, P6>(&self, rclsid: *const windows_core::GUID, psztype: P1, pszext: P2, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: P5, pbc: P6, dwclasscontext: u32, riid: *const windows_core::GUID, flags: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
        P6: windows_core::Param<IBindCtx>,
    {
        unsafe { (windows_core::Interface::vtable(self).AsyncGetClassBits)(windows_core::Interface::as_raw(self), rclsid, psztype.param().abi(), pszext.param().abi(), dwfileversionms, dwfileversionls, pszcodebase.param().abi(), pbc.param().abi(), dwclasscontext, riid, flags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUrlMon_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AsyncGetClassBits: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, windows_core::PCWSTR, windows_core::PCWSTR, u32, u32, windows_core::PCWSTR, *mut core::ffi::c_void, u32, *const windows_core::GUID, u32) -> windows_core::HRESULT,
}
pub trait IUrlMon_Impl: windows_core::IUnknownImpl {
    fn AsyncGetClassBits(&self, rclsid: *const windows_core::GUID, psztype: &windows_core::PCWSTR, pszext: &windows_core::PCWSTR, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: &windows_core::PCWSTR, pbc: windows_core::Ref<IBindCtx>, dwclasscontext: u32, riid: *const windows_core::GUID, flags: u32) -> windows_core::Result<()>;
}
impl IUrlMon_Vtbl {
    pub const fn new<Identity: IUrlMon_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AsyncGetClassBits<Identity: IUrlMon_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rclsid: *const windows_core::GUID, psztype: windows_core::PCWSTR, pszext: windows_core::PCWSTR, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: windows_core::PCWSTR, pbc: *mut core::ffi::c_void, dwclasscontext: u32, riid: *const windows_core::GUID, flags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUrlMon_Impl::AsyncGetClassBits(this, core::mem::transmute_copy(&rclsid), core::mem::transmute(&psztype), core::mem::transmute(&pszext), core::mem::transmute_copy(&dwfileversionms), core::mem::transmute_copy(&dwfileversionls), core::mem::transmute(&pszcodebase), core::mem::transmute_copy(&pbc), core::mem::transmute_copy(&dwclasscontext), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&flags)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AsyncGetClassBits: AsyncGetClassBits::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUrlMon as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUrlMon {}
pub const IdleShutdown: ShutdownType = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPBIND_OPTS(pub *mut BIND_OPTS);
impl LPBIND_OPTS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPBIND_OPTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "objidlbase", feature = "winnt", feature = "wtypesbase"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPBIND_OPTS2(pub *mut BIND_OPTS2);
#[cfg(all(feature = "objidlbase", feature = "winnt", feature = "wtypesbase"))]
impl LPBIND_OPTS2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "objidlbase", feature = "winnt", feature = "wtypesbase"))]
impl Default for LPBIND_OPTS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypesbase"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPBIND_OPTS3(pub *mut BIND_OPTS3);
#[cfg(all(feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypesbase"))]
impl LPBIND_OPTS3 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypesbase"))]
impl Default for LPBIND_OPTS3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "wtypes")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCLIPFORMAT(pub *mut super::wtypes::CLIPFORMAT);
#[cfg(feature = "wtypes")]
impl LPCLIPFORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "wtypes")]
impl Default for LPCLIPFORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "wtypes")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPFORMATETC(pub *mut FORMATETC);
#[cfg(feature = "wtypes")]
impl LPFORMATETC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "wtypes")]
impl Default for LPFORMATETC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPINTERFACEINFO(pub *mut INTERFACEINFO);
impl LPINTERFACEINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPINTERFACEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "wtypes")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSTATDATA(pub *mut STATDATA);
#[cfg(feature = "wtypes")]
impl LPSTATDATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "wtypes")]
impl Default for LPSTATDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSTGMEDIUM(pub *mut STGMEDIUM);
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl LPSTGMEDIUM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl Default for LPSTGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LibraryApplication: ApplicationType = 1;
pub type MKRREDUCE = i32;
pub const MKRREDUCE_ALL: MKRREDUCE = 0;
pub const MKRREDUCE_ONE: MKRREDUCE = 196608;
pub const MKRREDUCE_THROUGHUSER: MKRREDUCE = 65536;
pub const MKRREDUCE_TOUSER: MKRREDUCE = 131072;
pub type MKSYS = i32;
pub const MKSYS_ANTIMONIKER: MKSYS = 3;
pub const MKSYS_CLASSMONIKER: MKSYS = 7;
pub const MKSYS_FILEMONIKER: MKSYS = 2;
pub const MKSYS_GENERICCOMPOSITE: MKSYS = 1;
pub const MKSYS_ITEMMONIKER: MKSYS = 4;
pub const MKSYS_LUAMONIKER: MKSYS = 10;
pub const MKSYS_NONE: MKSYS = 0;
pub const MKSYS_OBJREFMONIKER: MKSYS = 8;
pub const MKSYS_POINTERMONIKER: MKSYS = 5;
pub const MKSYS_SESSIONMONIKER: MKSYS = 9;
pub type PENDINGMSG = i32;
pub const PENDINGMSG_CANCELCALL: PENDINGMSG = 0;
pub const PENDINGMSG_WAITDEFPROCESS: PENDINGMSG = 2;
pub const PENDINGMSG_WAITNOPROCESS: PENDINGMSG = 1;
pub type PENDINGTYPE = i32;
pub const PENDINGTYPE_NESTED: PENDINGTYPE = 2;
pub const PENDINGTYPE_TOPLEVEL: PENDINGTYPE = 1;
#[repr(C)]
#[cfg(feature = "wtypesbase")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RemSNB {
    pub ulCntStr: u32,
    pub ulCntChar: u32,
    pub rgString: [super::wtypesbase::OLECHAR; 1],
}
#[cfg(feature = "wtypesbase")]
impl Default for RemSNB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "rpc")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RemSTGMEDIUM {
    pub tymed: u32,
    pub dwHandleType: u32,
    pub pData: u32,
    pub pUnkForRelease: u32,
    pub cbData: u32,
    pub data: [super::rpc::byte; 1],
}
#[cfg(feature = "rpc")]
impl Default for RemSTGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SERVERCALL = i32;
pub const SERVERCALL_ISHANDLED: SERVERCALL = 0;
pub const SERVERCALL_REJECTED: SERVERCALL = 1;
pub const SERVERCALL_RETRYLATER: SERVERCALL = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SNB(pub *mut windows_core::PWSTR);
impl SNB {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for SNB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct STATDATA {
    pub formatetc: FORMATETC,
    pub advf: u32,
    pub pAdvSink: core::mem::ManuallyDrop<Option<IAdviseSink>>,
    pub dwConnection: u32,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub type STGMEDIUM = uSTGMEDIUM;
pub const ServerApplication: ApplicationType = 0;
pub type ShutdownType = i32;
#[repr(C)]
#[cfg(feature = "wtypesbase")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StorageLayout {
    pub LayoutType: u32,
    pub pwcsElementName: *mut super::wtypesbase::OLECHAR,
    pub cOffset: i64,
    pub cBytes: i64,
}
#[cfg(feature = "wtypesbase")]
impl Default for StorageLayout {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type TYMED = i32;
pub const TYMED_ENHMF: TYMED = 64;
pub const TYMED_FILE: TYMED = 2;
pub const TYMED_GDI: TYMED = 16;
pub const TYMED_HGLOBAL: TYMED = 1;
pub const TYMED_ISTORAGE: TYMED = 8;
pub const TYMED_ISTREAM: TYMED = 4;
pub const TYMED_MFPICT: TYMED = 32;
pub const TYMED_NULL: TYMED = 0;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub struct uSTGMEDIUM {
    pub tymed: u32,
    pub Anonymous: uSTGMEDIUM_0,
    pub pUnkForRelease: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl Clone for uSTGMEDIUM {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl Default for uSTGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
pub union uSTGMEDIUM_0 {
    pub hBitmap: super::windef::HBITMAP,
    pub hMetaFilePict: super::wtypes::HMETAFILEPICT,
    pub hEnhMetaFile: super::windef::HENHMETAFILE,
    pub hGlobal: super::minwindef::HGLOBAL,
    pub lpszFileName: windows_core::PWSTR,
    pub pstm: core::mem::ManuallyDrop<Option<super::objidlbase::IStream>>,
    pub pstg: core::mem::ManuallyDrop<Option<IStorage>>,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl Clone for uSTGMEDIUM_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase", feature = "windef", feature = "winnt", feature = "wtypes"))]
impl Default for uSTGMEDIUM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct userFLAG_STGMEDIUM {
    pub ContextFlags: i32,
    pub fPassOwnership: i32,
    pub Stgmed: userSTGMEDIUM,
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct userSTGMEDIUM {
    pub pUnkForRelease: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct userSTGMEDIUM_0 {
    pub tymed: u32,
    pub u: userSTGMEDIUM_0_0,
}
#[cfg(all(feature = "rpc", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
impl Default for userSTGMEDIUM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "rpc", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub union userSTGMEDIUM_0_0 {
    pub hMetaFilePict: super::wtypes::wireHMETAFILEPICT,
    pub hHEnhMetaFile: super::wtypes::wireHENHMETAFILE,
    pub hGdiHandle: *mut GDI_OBJECT,
    pub hGlobal: super::wtypes::wireHGLOBAL,
    pub lpszFileName: windows_core::PWSTR,
    pub pstm: *mut super::wtypesbase::BYTE_BLOB,
    pub pstg: *mut super::wtypesbase::BYTE_BLOB,
}
#[cfg(all(feature = "rpc", feature = "wingdi", feature = "wtypes", feature = "wtypesbase"))]
impl Default for userSTGMEDIUM_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireASYNC_STGMEDIUM(pub *mut userSTGMEDIUM);
impl wireASYNC_STGMEDIUM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for wireASYNC_STGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireFLAG_STGMEDIUM(pub *mut userFLAG_STGMEDIUM);
impl wireFLAG_STGMEDIUM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for wireFLAG_STGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "wtypesbase")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireSNB(pub *mut RemSNB);
#[cfg(feature = "wtypesbase")]
impl wireSNB {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "wtypesbase")]
impl Default for wireSNB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct wireSTGMEDIUM(pub *mut userSTGMEDIUM);
impl wireSTGMEDIUM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for wireSTGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
