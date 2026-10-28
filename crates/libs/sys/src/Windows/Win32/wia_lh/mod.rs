#[cfg(feature = "shared")]
pub type PWIA_DATA_CALLBACK_HEADER = *mut WIA_DATA_CALLBACK_HEADER;
#[cfg(feature = "shared")]
pub type PWIA_DATA_TRANSFER_INFO = *mut WIA_DATA_TRANSFER_INFO;
#[cfg(feature = "shared")]
pub type PWIA_DEV_CAP = *mut WIA_DEV_CAP;
#[cfg(feature = "shared")]
pub type PWIA_DITHER_PATTERN_DATA = *mut WIA_DITHER_PATTERN_DATA;
#[cfg(feature = "shared")]
pub type PWIA_EVENT_HANDLER = *mut WIA_DEV_CAP;
#[cfg(feature = "shared")]
pub type PWIA_EXTENDED_TRANSFER_INFO = *mut WIA_EXTENDED_TRANSFER_INFO;
#[cfg(feature = "shared")]
pub type PWIA_FORMAT_INFO = *mut WIA_FORMAT_INFO;
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub type PWIA_PROPID_TO_NAME = *mut WIA_PROPID_TO_NAME;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WIA_DATA_CALLBACK_HEADER {
    pub lSize: super::LONG,
    pub guidFormatID: windows_sys::core::GUID,
    pub lBufferSize: super::LONG,
    pub lPageCount: super::LONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WIA_DATA_TRANSFER_INFO {
    pub ulSize: super::ULONG,
    pub ulSection: super::ULONG,
    pub ulBufferSize: super::ULONG,
    pub bDoubleBuffer: windows_sys::core::BOOL,
    pub ulReserved1: super::ULONG,
    pub ulReserved2: super::ULONG,
    pub ulReserved3: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WIA_DEV_CAP {
    pub guid: windows_sys::core::GUID,
    pub ulFlags: super::ULONG,
    pub bstrName: windows_sys::core::BSTR,
    pub bstrDescription: windows_sys::core::BSTR,
    pub bstrIcon: windows_sys::core::BSTR,
    pub bstrCommandline: windows_sys::core::BSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WIA_DITHER_PATTERN_DATA {
    pub lSize: super::LONG,
    pub bstrPatternName: windows_sys::core::BSTR,
    pub lPatternWidth: super::LONG,
    pub lPatternLength: super::LONG,
    pub cbPattern: super::LONG,
    pub pbPattern: *mut super::BYTE,
}
#[cfg(feature = "shared")]
pub type WIA_EVENT_HANDLER = WIA_DEV_CAP;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WIA_EXTENDED_TRANSFER_INFO {
    pub ulSize: super::ULONG,
    pub ulMinBufferSize: super::ULONG,
    pub ulOptimalBufferSize: super::ULONG,
    pub ulMaxBufferSize: super::ULONG,
    pub ulNumBuffers: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WIA_FORMAT_INFO {
    pub guidFormatID: windows_sys::core::GUID,
    pub lTymed: super::LONG,
}
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct WIA_PROPID_TO_NAME {
    pub propid: super::PROPID,
    pub pszName: super::LPOLESTR,
}
pub const WiaDevMgr: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa1f4e726_8cf1_11d1_bf92_0060081ed811);
pub const WiaDevMgr2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb6c292bc_7c88_41ee_8b54_8ec92617e599);
pub const WiaLog: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa1e75357_881a_419e_83e2_bb16db197c68);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WiaTransferParams {
    pub lMessage: super::LONG,
    pub lPercentComplete: super::LONG,
    pub ulTransferredBytes: super::ULONG64,
    pub hrErrorStatus: windows_sys::core::HRESULT,
}
