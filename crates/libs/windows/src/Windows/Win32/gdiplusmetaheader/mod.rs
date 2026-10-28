#[repr(C, packed(8))]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct ENHMETAHEADER3 {
    pub iType: super::DWORD,
    pub nSize: super::DWORD,
    pub rclBounds: super::RECTL,
    pub rclFrame: super::RECTL,
    pub dSignature: super::DWORD,
    pub nVersion: super::DWORD,
    pub nBytes: super::DWORD,
    pub nRecords: super::DWORD,
    pub nHandles: super::WORD,
    pub sReserved: super::WORD,
    pub nDescription: super::DWORD,
    pub offDescription: super::DWORD,
    pub nPalEntries: super::DWORD,
    pub szlDevice: super::SIZEL,
    pub szlMillimeters: super::SIZEL,
}
pub const GDIP_EMFPLUSFLAGS_DISPLAY: i32 = 1;
#[repr(C, packed(8))]
#[cfg(all(feature = "gdiplusenums", feature = "gdiplustypes", feature = "shared", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy)]
pub struct MetafileHeader {
    pub Type: super::MetafileType,
    pub Size: super::UINT,
    pub Version: super::UINT,
    pub EmfPlusFlags: super::UINT,
    pub DpiX: super::REAL,
    pub DpiY: super::REAL,
    pub X: super::INT,
    pub Y: super::INT,
    pub Width: super::INT,
    pub Height: super::INT,
    pub Anonymous: MetafileHeader_0,
    pub EmfPlusHeaderSize: super::INT,
    pub LogicalDpiX: super::INT,
    pub LogicalDpiY: super::INT,
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplustypes", feature = "shared", feature = "windef", feature = "wingdi"))]
impl Default for MetafileHeader {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(8))]
#[cfg(all(feature = "gdiplusenums", feature = "gdiplustypes", feature = "shared", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy)]
pub union MetafileHeader_0 {
    pub WmfHeader: super::METAHEADER,
    pub EmfHeader: ENHMETAHEADER3,
}
#[cfg(all(feature = "gdiplusenums", feature = "gdiplustypes", feature = "shared", feature = "windef", feature = "wingdi"))]
impl Default for MetafileHeader_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PWMFRect16 {
    pub Left: super::INT16,
    pub Top: super::INT16,
    pub Right: super::INT16,
    pub Bottom: super::INT16,
}
#[repr(C, packed(2))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WmfPlaceableFileHeader {
    pub Key: super::UINT32,
    pub Hmf: super::INT16,
    pub BoundingBox: PWMFRect16,
    pub Inch: super::INT16,
    pub Reserved: super::UINT32,
    pub Checksum: super::INT16,
}
