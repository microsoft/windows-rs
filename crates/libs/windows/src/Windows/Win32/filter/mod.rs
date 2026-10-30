pub type CHUNKSTATE = i32;
pub type CHUNK_BREAKTYPE = i32;
pub const CHUNK_EOC: CHUNK_BREAKTYPE = 4;
pub const CHUNK_EOP: CHUNK_BREAKTYPE = 3;
pub const CHUNK_EOS: CHUNK_BREAKTYPE = 2;
pub const CHUNK_EOW: CHUNK_BREAKTYPE = 1;
pub const CHUNK_FILTER_OWNED_VALUE: CHUNKSTATE = 4;
pub const CHUNK_IMAGE: CHUNKSTATE = 8;
pub const CHUNK_NO_BREAK: CHUNK_BREAKTYPE = 0;
pub const CHUNK_TEXT: CHUNKSTATE = 1;
pub const CHUNK_VALUE: CHUNKSTATE = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILTERREGION {
    pub idChunk: super::ULONG,
    pub cwcStart: super::ULONG,
    pub cwcExtent: super::ULONG,
}
pub const FILTER_PIXELFORMAT_BGR8: IMAGE_PIXELFORMAT = 2;
pub const FILTER_PIXELFORMAT_BGRA8: IMAGE_PIXELFORMAT = 0;
pub const FILTER_PIXELFORMAT_PBGRA8: IMAGE_PIXELFORMAT = 1;
#[repr(C)]
#[cfg(all(feature = "propidlbase", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct FULLPROPSPEC {
    pub guidPropSet: windows_core::GUID,
    pub psProperty: super::PROPSPEC,
}
#[cfg(all(feature = "propidlbase", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
impl Default for FULLPROPSPEC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IFILTER_FLAGS = i32;
pub const IFILTER_FLAGS_OLE_PROPERTIES: IFILTER_FLAGS = 1;
pub type IFILTER_INIT = i32;
pub const IFILTER_INIT_APPLY_CRAWL_ATTRIBUTES: IFILTER_INIT = 256;
pub const IFILTER_INIT_APPLY_INDEX_ATTRIBUTES: IFILTER_INIT = 16;
pub const IFILTER_INIT_APPLY_OTHER_ATTRIBUTES: IFILTER_INIT = 32;
pub const IFILTER_INIT_CANON_HYPHENS: IFILTER_INIT = 4;
pub const IFILTER_INIT_CANON_PARAGRAPHS: IFILTER_INIT = 1;
pub const IFILTER_INIT_CANON_SPACES: IFILTER_INIT = 8;
pub const IFILTER_INIT_DISABLE_EMBEDDED: IFILTER_INIT = 2048;
pub const IFILTER_INIT_EMIT_FORMATTING: IFILTER_INIT = 4096;
pub const IFILTER_INIT_FILTER_AGGRESSIVE_BREAK: IFILTER_INIT = 1024;
pub const IFILTER_INIT_FILTER_OWNED_VALUE_OK: IFILTER_INIT = 512;
pub const IFILTER_INIT_HARD_LINE_BREAKS: IFILTER_INIT = 2;
pub const IFILTER_INIT_INDEXING_ONLY: IFILTER_INIT = 64;
pub const IFILTER_INIT_SEARCH_LINKS: IFILTER_INIT = 128;
windows_core::imp::define_interface!(IFilter, IFilter_Vtbl, 0x89bcb740_6119_101a_bcb7_00dd010655af);
windows_core::imp::interface_hierarchy!(IFilter, windows_core::IUnknown);
impl IFilter {
    #[cfg(all(feature = "propidlbase", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Init(&self, grfflags: super::ULONG, cattributes: super::ULONG, aattributes: *const FULLPROPSPEC, pflags: *mut super::ULONG) -> super::SCODE {
        unsafe { (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), grfflags, cattributes, aattributes, pflags as _) }
    }
    #[cfg(all(feature = "propidlbase", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetChunk(&self, pstat: *mut STAT_CHUNK) -> super::SCODE {
        unsafe { (windows_core::Interface::vtable(self).GetChunk)(windows_core::Interface::as_raw(self), pstat as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt", feature = "wtypesbase"))]
    pub unsafe fn GetText(&self, pcwcbuffer: *mut super::ULONG, awcbuffer: *mut super::WCHAR) -> super::SCODE {
        unsafe { (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), pcwcbuffer as _, awcbuffer as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetValue(&self, pppropvalue: *mut *mut super::PROPVARIANT) -> super::SCODE {
        unsafe { (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), pppropvalue as _) }
    }
    #[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypesbase"))]
    pub unsafe fn BindRegion(&self, origpos: FILTERREGION, riid: *const super::IID, ppunk: *mut *mut core::ffi::c_void) -> super::SCODE {
        unsafe { (windows_core::Interface::vtable(self).BindRegion)(windows_core::Interface::as_raw(self), origpos, riid, ppunk as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "propidlbase", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, super::ULONG, *const FULLPROPSPEC, *mut super::ULONG) -> super::SCODE,
    #[cfg(not(all(feature = "propidlbase", feature = "shared", feature = "wtypes", feature = "wtypesbase")))]
    Init: usize,
    #[cfg(all(feature = "propidlbase", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
    pub GetChunk: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STAT_CHUNK) -> super::SCODE,
    #[cfg(not(all(feature = "propidlbase", feature = "shared", feature = "wtypes", feature = "wtypesbase")))]
    GetChunk: usize,
    #[cfg(all(feature = "shared", feature = "winnt", feature = "wtypesbase"))]
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG, *mut super::WCHAR) -> super::SCODE,
    #[cfg(not(all(feature = "shared", feature = "winnt", feature = "wtypesbase")))]
    GetText: usize,
    #[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::PROPVARIANT) -> super::SCODE,
    #[cfg(not(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetValue: usize,
    #[cfg(all(feature = "guiddef", feature = "shared", feature = "wtypesbase"))]
    pub BindRegion: unsafe extern "system" fn(*mut core::ffi::c_void, FILTERREGION, *const super::IID, *mut *mut core::ffi::c_void) -> super::SCODE,
    #[cfg(not(all(feature = "guiddef", feature = "shared", feature = "wtypesbase")))]
    BindRegion: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IFilter_Impl: windows_core::IUnknownImpl {
    fn Init(&self, grfflags: super::ULONG, cattributes: super::ULONG, aattributes: *const FULLPROPSPEC, pflags: *mut super::ULONG) -> super::SCODE;
    fn GetChunk(&self, pstat: *mut STAT_CHUNK) -> super::SCODE;
    fn GetText(&self, pcwcbuffer: *mut super::ULONG, awcbuffer: *mut super::WCHAR) -> super::SCODE;
    fn GetValue(&self, pppropvalue: *mut *mut super::PROPVARIANT) -> super::SCODE;
    fn BindRegion(&self, origpos: &FILTERREGION, riid: *const super::IID, ppunk: *mut *mut core::ffi::c_void) -> super::SCODE;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IFilter_Vtbl {
    pub const fn new<Identity: IFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Init<Identity: IFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfflags: super::ULONG, cattributes: super::ULONG, aattributes: *const FULLPROPSPEC, pflags: *mut super::ULONG) -> super::SCODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFilter_Impl::Init(this, core::mem::transmute_copy(&grfflags), core::mem::transmute_copy(&cattributes), core::mem::transmute_copy(&aattributes), core::mem::transmute_copy(&pflags))
            }
        }
        unsafe extern "system" fn GetChunk<Identity: IFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstat: *mut STAT_CHUNK) -> super::SCODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFilter_Impl::GetChunk(this, core::mem::transmute_copy(&pstat))
            }
        }
        unsafe extern "system" fn GetText<Identity: IFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcwcbuffer: *mut super::ULONG, awcbuffer: *mut super::WCHAR) -> super::SCODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFilter_Impl::GetText(this, core::mem::transmute_copy(&pcwcbuffer), core::mem::transmute_copy(&awcbuffer))
            }
        }
        unsafe extern "system" fn GetValue<Identity: IFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropvalue: *mut *mut super::PROPVARIANT) -> super::SCODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFilter_Impl::GetValue(this, core::mem::transmute_copy(&pppropvalue))
            }
        }
        unsafe extern "system" fn BindRegion<Identity: IFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, origpos: FILTERREGION, riid: *const super::IID, ppunk: *mut *mut core::ffi::c_void) -> super::SCODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFilter_Impl::BindRegion(this, core::mem::transmute(&origpos), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunk))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            GetChunk: GetChunk::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            GetValue: GetValue::<Identity, OFFSET>,
            BindRegion: BindRegion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFilter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IFilter {}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IMAGE_INFO {
    pub Width: super::UINT,
    pub Height: super::UINT,
    pub Format: IMAGE_PIXELFORMAT,
}
pub type IMAGE_PIXELFORMAT = i32;
windows_core::imp::define_interface!(IPixelFilter, IPixelFilter_Vtbl, 0x3d7df9a7_8da6_4fbf_a45b_7592f06d93a9);
impl core::ops::Deref for IPixelFilter {
    type Target = IFilter;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IPixelFilter, windows_core::IUnknown, IFilter);
impl IPixelFilter {
    #[cfg(feature = "shared")]
    pub unsafe fn GetImageInfo(&self, imageinfo: *mut IMAGE_INFO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetImageInfo)(windows_core::Interface::as_raw(self), imageinfo as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
    pub unsafe fn GetPixelsForImage(&self, scalingfactor: super::FLOAT, sourcerect: Option<*const super::RECT>, pixelbuffersize: super::ULONG, pixelbuffer: *mut super::BYTE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPixelsForImage)(windows_core::Interface::as_raw(self), scalingfactor, sourcerect.unwrap_or(core::mem::zeroed()) as _, pixelbuffersize, pixelbuffer as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPixelFilter_Vtbl {
    pub base__: IFilter_Vtbl,
    #[cfg(feature = "shared")]
    pub GetImageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAGE_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetImageInfo: usize,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
    pub GetPixelsForImage: unsafe extern "system" fn(*mut core::ffi::c_void, super::FLOAT, *const super::RECT, super::ULONG, *mut super::BYTE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "windef")))]
    GetPixelsForImage: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IPixelFilter_Impl: IFilter_Impl {
    fn GetImageInfo(&self, imageinfo: *mut IMAGE_INFO) -> windows_core::Result<()>;
    fn GetPixelsForImage(&self, scalingfactor: super::FLOAT, sourcerect: *const super::RECT, pixelbuffersize: super::ULONG, pixelbuffer: *mut super::BYTE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IPixelFilter_Vtbl {
    pub const fn new<Identity: IPixelFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetImageInfo<Identity: IPixelFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageinfo: *mut IMAGE_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPixelFilter_Impl::GetImageInfo(this, core::mem::transmute_copy(&imageinfo)).into()
            }
        }
        unsafe extern "system" fn GetPixelsForImage<Identity: IPixelFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scalingfactor: super::FLOAT, sourcerect: *const super::RECT, pixelbuffersize: super::ULONG, pixelbuffer: *mut super::BYTE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPixelFilter_Impl::GetPixelsForImage(this, core::mem::transmute_copy(&scalingfactor), core::mem::transmute_copy(&sourcerect), core::mem::transmute_copy(&pixelbuffersize), core::mem::transmute_copy(&pixelbuffer)).into()
            }
        }
        Self {
            base__: IFilter_Vtbl::new::<Identity, OFFSET>(),
            GetImageInfo: GetImageInfo::<Identity, OFFSET>,
            GetPixelsForImage: GetPixelsForImage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPixelFilter as windows_core::Interface>::IID || iid == &<IFilter as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IPixelFilter {}
#[repr(C)]
#[cfg(all(feature = "propidlbase", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct STAT_CHUNK {
    pub idChunk: super::ULONG,
    pub breakType: CHUNK_BREAKTYPE,
    pub flags: CHUNKSTATE,
    pub locale: super::LCID,
    pub attribute: FULLPROPSPEC,
    pub idChunkSource: super::ULONG,
    pub cwcStartSource: super::ULONG,
    pub cwcLenSource: super::ULONG,
}
#[cfg(all(feature = "propidlbase", feature = "shared", feature = "wtypes", feature = "wtypesbase"))]
impl Default for STAT_CHUNK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
