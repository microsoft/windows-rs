#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILTERED_DATA_SOURCES {
    pub pwcsExtension: *const u16,
    pub pwcsMime: *const u16,
    pub pClsid: *const windows_core::GUID,
    pub pwcsOverride: *const u16,
}
impl Default for FILTERED_DATA_SOURCES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
windows_core::imp::define_interface!(ILoadFilter, ILoadFilter_Vtbl, 0xc7310722_ac80_11d1_8df3_00c04fb6ef4f);
windows_core::imp::interface_hierarchy!(ILoadFilter, windows_core::IUnknown);
impl ILoadFilter {
    #[cfg(feature = "filter")]
    pub unsafe fn LoadIFilter<P0, P2>(&self, pwcspath: P0, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: P2, fusedefault: bool, pfilterclsid: *mut windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut Option<super::filter::IFilter>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadIFilter)(windows_core::Interface::as_raw(self), pwcspath.param().abi(), pfilteredsources, punkouter.param().abi(), fusedefault.into(), pfilterclsid as _, searchdecsize as _, pwcssearchdesc as _, core::mem::transmute(ppifilt)) }
    }
    #[cfg(all(feature = "filter", feature = "objidl"))]
    pub unsafe fn LoadIFilterFromStorage<P0, P1, P2>(&self, pstg: P0, punkouter: P1, pwcsoverride: P2, fusedefault: bool, pfilterclsid: *mut windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut Option<super::filter::IFilter>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidl::IStorage>,
        P1: windows_core::Param<windows_core::IUnknown>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadIFilterFromStorage)(windows_core::Interface::as_raw(self), pstg.param().abi(), punkouter.param().abi(), pwcsoverride.param().abi(), fusedefault.into(), pfilterclsid as _, searchdecsize as _, pwcssearchdesc as _, core::mem::transmute(ppifilt)) }
    }
    #[cfg(all(feature = "filter", feature = "objidlbase"))]
    pub unsafe fn LoadIFilterFromStream<P0, P2>(&self, pstm: P0, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: P2, fusedefault: bool, pfilterclsid: *mut windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut Option<super::filter::IFilter>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadIFilterFromStream)(windows_core::Interface::as_raw(self), pstm.param().abi(), pfilteredsources, punkouter.param().abi(), fusedefault.into(), pfilterclsid as _, searchdecsize as _, pwcssearchdesc as _, core::mem::transmute(ppifilt)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoadFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "filter")]
    pub LoadIFilter: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const FILTERED_DATA_SOURCES, *mut core::ffi::c_void, windows_core::BOOL, *mut windows_core::GUID, *mut i32, *mut *mut u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "filter"))]
    LoadIFilter: usize,
    #[cfg(all(feature = "filter", feature = "objidl"))]
    pub LoadIFilterFromStorage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL, *mut windows_core::GUID, *mut i32, *mut *mut u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "filter", feature = "objidl")))]
    LoadIFilterFromStorage: usize,
    #[cfg(all(feature = "filter", feature = "objidlbase"))]
    pub LoadIFilterFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const FILTERED_DATA_SOURCES, *mut core::ffi::c_void, windows_core::BOOL, *mut windows_core::GUID, *mut i32, *mut *mut u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "filter", feature = "objidlbase")))]
    LoadIFilterFromStream: usize,
}
#[cfg(all(feature = "filter", feature = "objidl", feature = "objidlbase"))]
pub trait ILoadFilter_Impl: windows_core::IUnknownImpl {
    fn LoadIFilter(&self, pwcspath: &windows_core::PCWSTR, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: windows_core::Ref<windows_core::IUnknown>, fusedefault: windows_core::BOOL, pfilterclsid: *mut windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: windows_core::OutRef<super::filter::IFilter>) -> windows_core::Result<()>;
    fn LoadIFilterFromStorage(&self, pstg: windows_core::Ref<super::objidl::IStorage>, punkouter: windows_core::Ref<windows_core::IUnknown>, pwcsoverride: &windows_core::PCWSTR, fusedefault: windows_core::BOOL, pfilterclsid: *mut windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: windows_core::OutRef<super::filter::IFilter>) -> windows_core::Result<()>;
    fn LoadIFilterFromStream(&self, pstm: windows_core::Ref<super::objidlbase::IStream>, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: windows_core::Ref<windows_core::IUnknown>, fusedefault: windows_core::BOOL, pfilterclsid: *mut windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: windows_core::OutRef<super::filter::IFilter>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "filter", feature = "objidl", feature = "objidlbase"))]
impl ILoadFilter_Vtbl {
    pub const fn new<Identity: ILoadFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LoadIFilter<Identity: ILoadFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwcspath: windows_core::PCWSTR, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: *mut core::ffi::c_void, fusedefault: windows_core::BOOL, pfilterclsid: *mut windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILoadFilter_Impl::LoadIFilter(this, core::mem::transmute(&pwcspath), core::mem::transmute_copy(&pfilteredsources), core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&fusedefault), core::mem::transmute_copy(&pfilterclsid), core::mem::transmute_copy(&searchdecsize), core::mem::transmute_copy(&pwcssearchdesc), core::mem::transmute_copy(&ppifilt)).into()
            }
        }
        unsafe extern "system" fn LoadIFilterFromStorage<Identity: ILoadFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstg: *mut core::ffi::c_void, punkouter: *mut core::ffi::c_void, pwcsoverride: windows_core::PCWSTR, fusedefault: windows_core::BOOL, pfilterclsid: *mut windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILoadFilter_Impl::LoadIFilterFromStorage(this, core::mem::transmute_copy(&pstg), core::mem::transmute_copy(&punkouter), core::mem::transmute(&pwcsoverride), core::mem::transmute_copy(&fusedefault), core::mem::transmute_copy(&pfilterclsid), core::mem::transmute_copy(&searchdecsize), core::mem::transmute_copy(&pwcssearchdesc), core::mem::transmute_copy(&ppifilt)).into()
            }
        }
        unsafe extern "system" fn LoadIFilterFromStream<Identity: ILoadFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstm: *mut core::ffi::c_void, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: *mut core::ffi::c_void, fusedefault: windows_core::BOOL, pfilterclsid: *mut windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILoadFilter_Impl::LoadIFilterFromStream(this, core::mem::transmute_copy(&pstm), core::mem::transmute_copy(&pfilteredsources), core::mem::transmute_copy(&punkouter), core::mem::transmute_copy(&fusedefault), core::mem::transmute_copy(&pfilterclsid), core::mem::transmute_copy(&searchdecsize), core::mem::transmute_copy(&pwcssearchdesc), core::mem::transmute_copy(&ppifilt)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadIFilter: LoadIFilter::<Identity, OFFSET>,
            LoadIFilterFromStorage: LoadIFilterFromStorage::<Identity, OFFSET>,
            LoadIFilterFromStream: LoadIFilterFromStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILoadFilter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "filter", feature = "objidl", feature = "objidlbase"))]
impl windows_core::RuntimeName for ILoadFilter {}
windows_core::imp::define_interface!(ILoadFilterWithPrivateComActivation, ILoadFilterWithPrivateComActivation_Vtbl, 0x40bdbd34_780b_48d3_9bb6_12ebd4ad2e75);
impl core::ops::Deref for ILoadFilterWithPrivateComActivation {
    type Target = ILoadFilter;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILoadFilterWithPrivateComActivation, windows_core::IUnknown, ILoadFilter);
impl ILoadFilterWithPrivateComActivation {
    #[cfg(feature = "filter")]
    pub unsafe fn LoadIFilterWithPrivateComActivation(&self, filteredsources: *const FILTERED_DATA_SOURCES, usedefault: bool, filterclsid: *mut windows_core::GUID, isfilterprivatecomactivated: *mut windows_core::BOOL, filterobj: *mut Option<super::filter::IFilter>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LoadIFilterWithPrivateComActivation)(windows_core::Interface::as_raw(self), filteredsources, usedefault.into(), filterclsid as _, isfilterprivatecomactivated as _, core::mem::transmute(filterobj)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoadFilterWithPrivateComActivation_Vtbl {
    pub base__: ILoadFilter_Vtbl,
    #[cfg(feature = "filter")]
    pub LoadIFilterWithPrivateComActivation: unsafe extern "system" fn(*mut core::ffi::c_void, *const FILTERED_DATA_SOURCES, windows_core::BOOL, *mut windows_core::GUID, *mut windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "filter"))]
    LoadIFilterWithPrivateComActivation: usize,
}
#[cfg(all(feature = "filter", feature = "objidl", feature = "objidlbase"))]
pub trait ILoadFilterWithPrivateComActivation_Impl: ILoadFilter_Impl {
    fn LoadIFilterWithPrivateComActivation(&self, filteredsources: *const FILTERED_DATA_SOURCES, usedefault: windows_core::BOOL, filterclsid: *mut windows_core::GUID, isfilterprivatecomactivated: *mut windows_core::BOOL, filterobj: windows_core::OutRef<super::filter::IFilter>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "filter", feature = "objidl", feature = "objidlbase"))]
impl ILoadFilterWithPrivateComActivation_Vtbl {
    pub const fn new<Identity: ILoadFilterWithPrivateComActivation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LoadIFilterWithPrivateComActivation<Identity: ILoadFilterWithPrivateComActivation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filteredsources: *const FILTERED_DATA_SOURCES, usedefault: windows_core::BOOL, filterclsid: *mut windows_core::GUID, isfilterprivatecomactivated: *mut windows_core::BOOL, filterobj: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILoadFilterWithPrivateComActivation_Impl::LoadIFilterWithPrivateComActivation(this, core::mem::transmute_copy(&filteredsources), core::mem::transmute_copy(&usedefault), core::mem::transmute_copy(&filterclsid), core::mem::transmute_copy(&isfilterprivatecomactivated), core::mem::transmute_copy(&filterobj)).into()
            }
        }
        Self {
            base__: ILoadFilter_Vtbl::new::<Identity, OFFSET>(),
            LoadIFilterWithPrivateComActivation: LoadIFilterWithPrivateComActivation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILoadFilterWithPrivateComActivation as windows_core::Interface>::IID || iid == &<ILoadFilter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "filter", feature = "objidl", feature = "objidlbase"))]
impl windows_core::RuntimeName for ILoadFilterWithPrivateComActivation {}
