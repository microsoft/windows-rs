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
#[derive(Clone, Copy, Default)]
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
    pub guidPropSet: windows_sys::core::GUID,
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_INFO {
    pub Width: u32,
    pub Height: u32,
    pub Format: IMAGE_PIXELFORMAT,
}
pub type IMAGE_PIXELFORMAT = i32;
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
