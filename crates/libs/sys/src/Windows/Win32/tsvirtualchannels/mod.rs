#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct BITMAP_RENDERER_STATISTICS {
    pub dwFramesDelivered: super::DWORD,
    pub dwFramesDropped: super::DWORD,
}
pub const Desktop: RdpSessionType = 0;
pub const E_DUPLICATE_WINDOW_HINT: windows_sys::core::HRESULT = 0x800700B7_u32 as _;
pub const E_MAPPEDRENDERER_SHUTDOWN: windows_sys::core::HRESULT = 0x8007139F_u32 as _;
pub const IID_IWTSBitmapRenderService: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xea326091_05fe_40c1_b49c_3d2ef4626a0e);
pub const IID_IWTSBitmapRenderer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5b7acc97_f3c9_46f7_8c5b_fa685d3441b1);
pub const IID_IWTSBitmapRendererCallback: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd782928e_fe4e_4e77_ae90_9cd0b3e3b353);
pub const IID_IWTSListener: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa1230206_9a39_4d58_8674_cdb4dff4e73b);
pub const IID_IWTSListenerCallback: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa1230203_d6a7_11d8_b9fd_000bdbd1f198);
pub const IID_IWTSPlugin: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa1230201_1439_4e62_a414_190d0ac3d40e);
pub const IID_IWTSPluginServiceProvider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd3e07363_087c_476c_86a7_dbb15f46ddb4);
pub const IID_IWTSVirtualChannel: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa1230207_d6a7_11d8_b9fd_000bdbd1f198);
pub const IID_IWTSVirtualChannelCallback: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa1230204_d6a7_11d8_b9fd_000bdbd1f198);
pub const IID_IWTSVirtualChannelManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa1230205_d6a7_11d8_b9fd_000bdbd1f198);
pub const IID_IWTSWindowChangedCallback: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe8a47fd3_1af5_4d7f_8e15_515ccae1c00c);
pub const IID_IWTSWindowInfoService: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2a002c7b_0120_4d04_bfb1_3c73110c8581);
#[cfg(feature = "shared")]
pub type PBITMAP_RENDERER_STATISTICS = *mut BITMAP_RENDERER_STATISTICS;
pub const RDCLIENT_BITMAP_RENDER_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe4cc08cb_942e_4b19_8504_bd5a89a747f5);
pub const RDCLIENT_WINDOW_INFO_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa9a17a6b_cdb3_4e81_b5c8_bd63822a8d6c);
pub type RdpSessionType = i32;
pub const RemoteApp: RdpSessionType = 1;
pub const TS_VC_LISTENER_STATIC_CHANNEL: i32 = 1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct WTSWindowInfo {
    pub Hwnd: super::HWND,
    pub Height: i32,
    pub Width: i32,
    pub ViewWidth: i32,
    pub ViewHeight: i32,
    pub ViewOffsetX: i32,
    pub ViewOffsetY: i32,
    pub Scale: f32,
}
pub const WTS_PROPERTY_DEFAULT_CONFIG: windows_sys::core::PCWSTR = windows_sys::core::w!("DefaultConfig");
