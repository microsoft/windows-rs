#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVMODEW {
    pub dmDeviceName: [u16; 32],
    pub dmSpecVersion: u16,
    pub dmDriverVersion: u16,
    pub dmSize: u16,
    pub dmDriverExtra: u16,
    pub dmFields: DEVMODE_FIELD_FLAGS,
    pub Anonymous1: DEVMODEW_0,
    pub dmColor: DEVMODE_COLOR,
    pub dmDuplex: DEVMODE_DUPLEX,
    pub dmYResolution: i16,
    pub dmTTOption: DEVMODE_TRUETYPE_OPTION,
    pub dmCollate: DEVMODE_COLLATE,
    pub dmFormName: [u16; 32],
    pub dmLogPixels: u16,
    pub dmBitsPerPel: u32,
    pub dmPelsWidth: u32,
    pub dmPelsHeight: u32,
    pub Anonymous2: DEVMODEW_1,
    pub dmDisplayFrequency: u32,
    pub dmICMMethod: u32,
    pub dmICMIntent: u32,
    pub dmMediaType: u32,
    pub dmDitherType: u32,
    pub dmReserved1: u32,
    pub dmReserved2: u32,
    pub dmPanningWidth: u32,
    pub dmPanningHeight: u32,
}
impl Default for DEVMODEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DEVMODEW_0 {
    pub Anonymous1: DEVMODEW_0_0,
    pub Anonymous2: DEVMODEW_0_1,
}
impl Default for DEVMODEW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEVMODEW_0_0 {
    pub dmOrientation: i16,
    pub dmPaperSize: i16,
    pub dmPaperLength: i16,
    pub dmPaperWidth: i16,
    pub dmScale: i16,
    pub dmCopies: i16,
    pub dmDefaultSource: i16,
    pub dmPrintQuality: i16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEVMODEW_0_1 {
    pub dmPosition: POINTL,
    pub dmDisplayOrientation: DEVMODE_DISPLAY_ORIENTATION,
    pub dmDisplayFixedOutput: DEVMODE_DISPLAY_FIXED_OUTPUT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DEVMODEW_1 {
    pub dmDisplayFlags: u32,
    pub dmNup: u32,
}
impl Default for DEVMODEW_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DEVMODE_COLLATE = i16;
pub type DEVMODE_COLOR = i16;
pub type DEVMODE_DISPLAY_FIXED_OUTPUT = u32;
pub type DEVMODE_DISPLAY_ORIENTATION = u32;
pub type DEVMODE_DUPLEX = i16;
pub type DEVMODE_FIELD_FLAGS = u32;
pub type DEVMODE_TRUETYPE_OPTION = i16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
impl GUID {
    pub const fn from_u128(uuid: u128) -> Self {
        Self {
            data1: (uuid >> 96) as u32,
            data2: (uuid >> 80 & 0xffff) as u16,
            data3: (uuid >> 64 & 0xffff) as u16,
            data4: (uuid as u64).to_be_bytes(),
        }
    }
}
pub type HBITMAP = *mut core::ffi::c_void;
pub type HENHMETAFILE = *mut core::ffi::c_void;
pub type HGDIOBJ = *mut core::ffi::c_void;
pub type HGLOBAL = *mut core::ffi::c_void;
pub type HRESULT = i32;
pub const IID_ISequentialStream: GUID = GUID::from_u128(0x0c733a30_2a1c_11ce_ade5_00aa0044773d);
#[repr(C)]
pub struct ISequentialStream_Vtbl {
    pub base__: IUnknown_Vtbl,
    pub Read: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u32,
        *mut u32,
    ) -> HRESULT,
    pub Write: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const core::ffi::c_void,
        u32,
        *mut u32,
    ) -> HRESULT,
}
pub const IID_IStorage: GUID = GUID::from_u128(0x0000000b_0000_0000_c000_000000000046);
#[repr(C)]
pub struct IStorage_Vtbl {
    pub base__: IUnknown_Vtbl,
    CreateStream: usize,
    OpenStream: usize,
    CreateStorage: usize,
    OpenStorage: usize,
    pub CopyTo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        u32,
        *const GUID,
        *const *const u16,
        *mut core::ffi::c_void,
    ) -> HRESULT,
    MoveElementTo: usize,
    pub Commit: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> HRESULT,
    pub Revert: unsafe extern "system" fn(*mut core::ffi::c_void) -> HRESULT,
    EnumElements: usize,
    pub DestroyElement: unsafe extern "system" fn(*mut core::ffi::c_void, PCWSTR) -> HRESULT,
    pub RenameElement: unsafe extern "system" fn(*mut core::ffi::c_void, PCWSTR, PCWSTR) -> HRESULT,
    SetElementTimes: usize,
    pub SetClass: unsafe extern "system" fn(*mut core::ffi::c_void, *const GUID) -> HRESULT,
    pub SetStateBits: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> HRESULT,
    Stat: usize,
}
pub const IID_IStream: GUID = GUID::from_u128(0x0000000c_0000_0000_c000_000000000046);
#[repr(C)]
pub struct IStream_Vtbl {
    pub base__: ISequentialStream_Vtbl,
    Seek: usize,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> HRESULT,
    pub CopyTo: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        u64,
        *mut u64,
        *mut u64,
    ) -> HRESULT,
    Commit: usize,
    pub Revert: unsafe extern "system" fn(*mut core::ffi::c_void) -> HRESULT,
    LockRegion: usize,
    pub UnlockRegion: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u32) -> HRESULT,
    Stat: usize,
    pub Clone:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> HRESULT,
}
pub const IID_IUnknown: GUID = GUID::from_u128(0x00000000_0000_0000_c000_000000000046);
#[repr(C)]
pub struct IUnknown_Vtbl {
    pub QueryInterface: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        iid: *const GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> u32,
}
pub type PCWSTR = *const u16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct POINTL {
    pub x: i32,
    pub y: i32,
}
pub type PWSTR = *mut u16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STGMEDIUM {
    pub tymed: u32,
    pub u: STGMEDIUM_0,
    pub pUnkForRelease: *mut core::ffi::c_void,
}
impl Default for STGMEDIUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union STGMEDIUM_0 {
    pub hBitmap: HBITMAP,
    pub hMetaFilePict: *mut core::ffi::c_void,
    pub hEnhMetaFile: HENHMETAFILE,
    pub hGlobal: HGLOBAL,
    pub lpszFileName: PWSTR,
    pub pstm: *mut core::ffi::c_void,
    pub pstg: *mut core::ffi::c_void,
}
impl Default for STGMEDIUM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
