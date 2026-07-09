pub type PWIA_DATA_CALLBACK_HEADER = *mut WIA_DATA_CALLBACK_HEADER;
pub type PWIA_DATA_TRANSFER_INFO = *mut WIA_DATA_TRANSFER_INFO;
pub type PWIA_DEV_CAP = *mut WIA_DEV_CAP;
pub type PWIA_DITHER_PATTERN_DATA = *mut WIA_DITHER_PATTERN_DATA;
pub type PWIA_EVENT_HANDLER = *mut WIA_DEV_CAP;
pub type PWIA_EXTENDED_TRANSFER_INFO = *mut WIA_EXTENDED_TRANSFER_INFO;
pub type PWIA_FORMAT_INFO = *mut WIA_FORMAT_INFO;
#[cfg(feature = "Win32_wtypes")]
pub type PWIA_PROPID_TO_NAME = *mut WIA_PROPID_TO_NAME;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WIA_DATA_CALLBACK_HEADER {
    pub lSize: i32,
    pub guidFormatID: windows_sys::core::GUID,
    pub lBufferSize: i32,
    pub lPageCount: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WIA_DATA_TRANSFER_INFO {
    pub ulSize: u32,
    pub ulSection: u32,
    pub ulBufferSize: u32,
    pub bDoubleBuffer: windows_sys::core::BOOL,
    pub ulReserved1: u32,
    pub ulReserved2: u32,
    pub ulReserved3: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WIA_DEV_CAP {
    pub guid: windows_sys::core::GUID,
    pub ulFlags: u32,
    pub bstrName: windows_sys::core::BSTR,
    pub bstrDescription: windows_sys::core::BSTR,
    pub bstrIcon: windows_sys::core::BSTR,
    pub bstrCommandline: windows_sys::core::BSTR,
}
impl Default for WIA_DEV_CAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WIA_DITHER_PATTERN_DATA {
    pub lSize: i32,
    pub bstrPatternName: windows_sys::core::BSTR,
    pub lPatternWidth: i32,
    pub lPatternLength: i32,
    pub cbPattern: i32,
    pub pbPattern: *mut u8,
}
impl Default for WIA_DITHER_PATTERN_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WIA_EVENT_HANDLER = WIA_DEV_CAP;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WIA_EXTENDED_TRANSFER_INFO {
    pub ulSize: u32,
    pub ulMinBufferSize: u32,
    pub ulOptimalBufferSize: u32,
    pub ulMaxBufferSize: u32,
    pub ulNumBuffers: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WIA_FORMAT_INFO {
    pub guidFormatID: windows_sys::core::GUID,
    pub lTymed: i32,
}
#[repr(C)]
#[cfg(feature = "Win32_wtypes")]
#[derive(Clone, Copy)]
pub struct WIA_PROPID_TO_NAME {
    pub propid: super::wtypes::PROPID,
    pub pszName: windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_wtypes")]
impl Default for WIA_PROPID_TO_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WiaDevMgr: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa1f4e726_8cf1_11d1_bf92_0060081ed811);
pub const WiaDevMgr2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb6c292bc_7c88_41ee_8b54_8ec92617e599);
pub const WiaLog: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa1e75357_881a_419e_83e2_bb16db197c68);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WiaTransferParams {
    pub lMessage: i32,
    pub lPercentComplete: i32,
    pub ulTransferredBytes: u64,
    pub hrErrorStatus: windows_sys::core::HRESULT,
}
