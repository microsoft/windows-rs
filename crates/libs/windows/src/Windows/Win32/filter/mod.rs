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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILTERREGION {
    pub idChunk: u32,
    pub cwcStart: u32,
    pub cwcExtent: u32,
}
pub const FILTER_PIXELFORMAT_BGR8: IMAGE_PIXELFORMAT = 2;
pub const FILTER_PIXELFORMAT_BGRA8: IMAGE_PIXELFORMAT = 0;
pub const FILTER_PIXELFORMAT_PBGRA8: IMAGE_PIXELFORMAT = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_propidlbase", feature = "Win32_wtypes"))]
#[derive(Clone, Copy)]
pub struct FULLPROPSPEC {
    pub guidPropSet: windows_core::GUID,
    pub psProperty: super::propidlbase::PROPSPEC,
}
#[cfg(all(feature = "Win32_propidlbase", feature = "Win32_wtypes"))]
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
    #[cfg(all(feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn Init(&self, grfflags: u32, cattributes: u32, aattributes: *const FULLPROPSPEC, pflags: *mut u32) -> super::wtypesbase::SCODE {
        unsafe { (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), grfflags, cattributes, aattributes, pflags as _) }
    }
    #[cfg(all(feature = "Win32_propidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetChunk(&self, pstat: *mut STAT_CHUNK) -> super::wtypesbase::SCODE {
        unsafe { (windows_core::Interface::vtable(self).GetChunk)(windows_core::Interface::as_raw(self), pstat as _) }
    }
    #[cfg(feature = "Win32_wtypesbase")]
    pub unsafe fn GetText(&self, pcwcbuffer: *mut u32, awcbuffer: *mut u16) -> super::wtypesbase::SCODE {
        unsafe { (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), pcwcbuffer as _, awcbuffer as _) }
    }
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetValue(&self, pppropvalue: *mut *mut super::propidlbase::PROPVARIANT) -> super::wtypesbase::SCODE {
        unsafe { (windows_core::Interface::vtable(self).GetValue)(windows_core::Interface::as_raw(self), pppropvalue as _) }
    }
    #[cfg(feature = "Win32_wtypesbase")]
    pub unsafe fn BindRegion(&self, origpos: FILTERREGION, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> super::wtypesbase::SCODE {
        unsafe { (windows_core::Interface::vtable(self).BindRegion)(windows_core::Interface::as_raw(self), core::mem::transmute(origpos), riid, ppunk as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const FULLPROPSPEC, *mut u32) -> super::wtypesbase::SCODE,
    #[cfg(not(all(feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    Init: usize,
    #[cfg(all(feature = "Win32_propidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetChunk: unsafe extern "system" fn(*mut core::ffi::c_void, *mut STAT_CHUNK) -> super::wtypesbase::SCODE,
    #[cfg(not(all(feature = "Win32_propidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetChunk: usize,
    #[cfg(feature = "Win32_wtypesbase")]
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u16) -> super::wtypesbase::SCODE,
    #[cfg(not(feature = "Win32_wtypesbase"))]
    GetText: usize,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::propidlbase::PROPVARIANT) -> super::wtypesbase::SCODE,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetValue: usize,
    #[cfg(feature = "Win32_wtypesbase")]
    pub BindRegion: unsafe extern "system" fn(*mut core::ffi::c_void, FILTERREGION, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> super::wtypesbase::SCODE,
    #[cfg(not(feature = "Win32_wtypesbase"))]
    BindRegion: usize,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IFilter_Impl: windows_core::IUnknownImpl {
    fn Init(&self, grfflags: u32, cattributes: u32, aattributes: *const FULLPROPSPEC, pflags: *mut u32) -> super::wtypesbase::SCODE;
    fn GetChunk(&self, pstat: *mut STAT_CHUNK) -> super::wtypesbase::SCODE;
    fn GetText(&self, pcwcbuffer: *mut u32, awcbuffer: *mut u16) -> super::wtypesbase::SCODE;
    fn GetValue(&self, pppropvalue: *mut *mut super::propidlbase::PROPVARIANT) -> super::wtypesbase::SCODE;
    fn BindRegion(&self, origpos: &FILTERREGION, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> super::wtypesbase::SCODE;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IFilter_Vtbl {
    pub const fn new<Identity: IFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Init<Identity: IFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfflags: u32, cattributes: u32, aattributes: *const FULLPROPSPEC, pflags: *mut u32) -> super::wtypesbase::SCODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFilter_Impl::Init(this, core::mem::transmute_copy(&grfflags), core::mem::transmute_copy(&cattributes), core::mem::transmute_copy(&aattributes), core::mem::transmute_copy(&pflags))
            }
        }
        unsafe extern "system" fn GetChunk<Identity: IFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstat: *mut STAT_CHUNK) -> super::wtypesbase::SCODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFilter_Impl::GetChunk(this, core::mem::transmute_copy(&pstat))
            }
        }
        unsafe extern "system" fn GetText<Identity: IFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcwcbuffer: *mut u32, awcbuffer: *mut u16) -> super::wtypesbase::SCODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFilter_Impl::GetText(this, core::mem::transmute_copy(&pcwcbuffer), core::mem::transmute_copy(&awcbuffer))
            }
        }
        unsafe extern "system" fn GetValue<Identity: IFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropvalue: *mut *mut super::propidlbase::PROPVARIANT) -> super::wtypesbase::SCODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFilter_Impl::GetValue(this, core::mem::transmute_copy(&pppropvalue))
            }
        }
        unsafe extern "system" fn BindRegion<Identity: IFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, origpos: FILTERREGION, riid: *const windows_core::GUID, ppunk: *mut *mut core::ffi::c_void) -> super::wtypesbase::SCODE {
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
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IFilter {}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_INFO {
    pub Width: u32,
    pub Height: u32,
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
    pub unsafe fn GetImageInfo(&self) -> windows_core::Result<IMAGE_INFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetImageInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn GetPixelsForImage(&self, scalingfactor: f32, sourcerect: *const super::windef::RECT, pixelbuffersize: u32) -> windows_core::Result<u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelsForImage)(windows_core::Interface::as_raw(self), scalingfactor, sourcerect, pixelbuffersize, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPixelFilter_Vtbl {
    pub base__: IFilter_Vtbl,
    pub GetImageInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAGE_INFO) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub GetPixelsForImage: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *const super::windef::RECT, u32, *mut u8) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    GetPixelsForImage: usize,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IPixelFilter_Impl: IFilter_Impl {
    fn GetImageInfo(&self) -> windows_core::Result<IMAGE_INFO>;
    fn GetPixelsForImage(&self, scalingfactor: f32, sourcerect: *const super::windef::RECT, pixelbuffersize: u32) -> windows_core::Result<u8>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IPixelFilter_Vtbl {
    pub const fn new<Identity: IPixelFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetImageInfo<Identity: IPixelFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imageinfo: *mut IMAGE_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPixelFilter_Impl::GetImageInfo(this) {
                    Ok(ok__) => {
                        imageinfo.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPixelsForImage<Identity: IPixelFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scalingfactor: f32, sourcerect: *const super::windef::RECT, pixelbuffersize: u32, pixelbuffer: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPixelFilter_Impl::GetPixelsForImage(this, core::mem::transmute_copy(&scalingfactor), core::mem::transmute_copy(&sourcerect), core::mem::transmute_copy(&pixelbuffersize)) {
                    Ok(ok__) => {
                        pixelbuffer.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_windef", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IPixelFilter {}
#[repr(C)]
#[cfg(all(feature = "Win32_propidlbase", feature = "Win32_winnt", feature = "Win32_wtypes"))]
#[derive(Clone, Copy)]
pub struct STAT_CHUNK {
    pub idChunk: u32,
    pub breakType: CHUNK_BREAKTYPE,
    pub flags: CHUNKSTATE,
    pub locale: super::winnt::LCID,
    pub attribute: FULLPROPSPEC,
    pub idChunkSource: u32,
    pub cwcStartSource: u32,
    pub cwcLenSource: u32,
}
#[cfg(all(feature = "Win32_propidlbase", feature = "Win32_winnt", feature = "Win32_wtypes"))]
impl Default for STAT_CHUNK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
