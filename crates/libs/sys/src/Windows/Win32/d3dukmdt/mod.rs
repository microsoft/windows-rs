#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3DDDIARG_CREATERESOURCE {
    pub Format: D3DDDIFORMAT,
    pub Pool: D3DDDI_POOL,
    pub MultisampleType: D3DDDIMULTISAMPLE_TYPE,
    pub MultisampleQuality: u32,
    pub pSurfList: *const D3DDDI_SURFACEINFO,
    pub SurfCount: u32,
    pub MipLevels: u32,
    pub Fvf: u32,
    pub VidPnSourceId: D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub RefreshRate: D3DDDI_RATIONAL,
    pub hResource: super::winnt::HANDLE,
    pub Flags: D3DDDI_RESOURCEFLAGS,
    pub Rotation: D3DDDI_ROTATION,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3DDDIARG_CREATERESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3DDDIARG_CREATERESOURCE2 {
    pub Format: D3DDDIFORMAT,
    pub Pool: D3DDDI_POOL,
    pub MultisampleType: D3DDDIMULTISAMPLE_TYPE,
    pub MultisampleQuality: u32,
    pub pSurfList: *const D3DDDI_SURFACEINFO,
    pub SurfCount: u32,
    pub MipLevels: u32,
    pub Fvf: u32,
    pub VidPnSourceId: D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub RefreshRate: D3DDDI_RATIONAL,
    pub hResource: super::winnt::HANDLE,
    pub Flags: D3DDDI_RESOURCEFLAGS,
    pub Rotation: D3DDDI_ROTATION,
    pub Flags2: D3DDDI_RESOURCEFLAGS2,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3DDDIARG_CREATERESOURCE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDICB_DESTROYALLOCATION2FLAGS {
    pub Anonymous: D3DDDICB_DESTROYALLOCATION2FLAGS_0,
}
impl Default for D3DDDICB_DESTROYALLOCATION2FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDICB_DESTROYALLOCATION2FLAGS_0 {
    pub Anonymous: D3DDDICB_DESTROYALLOCATION2FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DDDICB_DESTROYALLOCATION2FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDICB_DESTROYALLOCATION2FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDICB_LOCK2FLAGS {
    pub Anonymous: D3DDDICB_LOCK2FLAGS_0,
}
impl Default for D3DDDICB_LOCK2FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDICB_LOCK2FLAGS_0 {
    pub Anonymous: D3DDDICB_LOCK2FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DDDICB_LOCK2FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDICB_LOCK2FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDICB_LOCKFLAGS {
    pub Anonymous: D3DDDICB_LOCKFLAGS_0,
}
impl Default for D3DDDICB_LOCKFLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDICB_LOCKFLAGS_0 {
    pub Anonymous: D3DDDICB_LOCKFLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DDDICB_LOCKFLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDICB_LOCKFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDICB_SIGNALFLAGS {
    pub Anonymous: D3DDDICB_SIGNALFLAGS_0,
}
impl Default for D3DDDICB_SIGNALFLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDICB_SIGNALFLAGS_0 {
    pub Anonymous: D3DDDICB_SIGNALFLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DDDICB_SIGNALFLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDICB_SIGNALFLAGS_0_0 {
    pub _bitfield: u32,
}
pub const D3DDDIFMT_A1: D3DDDIFORMAT = 118;
pub const D3DDDIFMT_A16B16G16R16: D3DDDIFORMAT = 36;
pub const D3DDDIFMT_A16B16G16R16F: D3DDDIFORMAT = 113;
pub const D3DDDIFMT_A1R5G5B5: D3DDDIFORMAT = 25;
pub const D3DDDIFMT_A2B10G10R10: D3DDDIFORMAT = 31;
pub const D3DDDIFMT_A2B10G10R10_XR_BIAS: D3DDDIFORMAT = 119;
pub const D3DDDIFMT_A2R10G10B10: D3DDDIFORMAT = 35;
pub const D3DDDIFMT_A2W10V10U10: D3DDDIFORMAT = 67;
pub const D3DDDIFMT_A32B32G32R32F: D3DDDIFORMAT = 116;
pub const D3DDDIFMT_A4L4: D3DDDIFORMAT = 52;
pub const D3DDDIFMT_A4R4G4B4: D3DDDIFORMAT = 26;
pub const D3DDDIFMT_A8: D3DDDIFORMAT = 28;
pub const D3DDDIFMT_A8B8G8R8: D3DDDIFORMAT = 32;
pub const D3DDDIFMT_A8L8: D3DDDIFORMAT = 51;
pub const D3DDDIFMT_A8P8: D3DDDIFORMAT = 40;
pub const D3DDDIFMT_A8R3G3B2: D3DDDIFORMAT = 29;
pub const D3DDDIFMT_A8R8G8B8: D3DDDIFORMAT = 21;
pub const D3DDDIFMT_BINARYBUFFER: D3DDDIFORMAT = 199;
pub const D3DDDIFMT_BITSTREAMDATA: D3DDDIFORMAT = 156;
pub const D3DDDIFMT_CxV8U8: D3DDDIFORMAT = 117;
pub const D3DDDIFMT_D15S1: D3DDDIFORMAT = 73;
pub const D3DDDIFMT_D16: D3DDDIFORMAT = 80;
pub const D3DDDIFMT_D16_LOCKABLE: D3DDDIFORMAT = 70;
pub const D3DDDIFMT_D24FS8: D3DDDIFORMAT = 83;
pub const D3DDDIFMT_D24S8: D3DDDIFORMAT = 75;
pub const D3DDDIFMT_D24X4S4: D3DDDIFORMAT = 79;
pub const D3DDDIFMT_D24X8: D3DDDIFORMAT = 77;
pub const D3DDDIFMT_D32: D3DDDIFORMAT = 71;
pub const D3DDDIFMT_D32F_LOCKABLE: D3DDDIFORMAT = 82;
pub const D3DDDIFMT_D32_LOCKABLE: D3DDDIFORMAT = 84;
pub const D3DDDIFMT_DEBLOCKINGDATA: D3DDDIFORMAT = 153;
pub const D3DDDIFMT_DXT1: D3DDDIFORMAT = 827611204;
pub const D3DDDIFMT_DXT2: D3DDDIFORMAT = 844388420;
pub const D3DDDIFMT_DXT3: D3DDDIFORMAT = 861165636;
pub const D3DDDIFMT_DXT4: D3DDDIFORMAT = 877942852;
pub const D3DDDIFMT_DXT5: D3DDDIFORMAT = 894720068;
pub const D3DDDIFMT_DXVACOMPBUFFER_BASE: D3DDDIFORMAT = 150;
pub const D3DDDIFMT_DXVACOMPBUFFER_MAX: D3DDDIFORMAT = 181;
pub const D3DDDIFMT_DXVA_RESERVED10: D3DDDIFORMAT = 160;
pub const D3DDDIFMT_DXVA_RESERVED11: D3DDDIFORMAT = 161;
pub const D3DDDIFMT_DXVA_RESERVED12: D3DDDIFORMAT = 162;
pub const D3DDDIFMT_DXVA_RESERVED13: D3DDDIFORMAT = 163;
pub const D3DDDIFMT_DXVA_RESERVED14: D3DDDIFORMAT = 164;
pub const D3DDDIFMT_DXVA_RESERVED15: D3DDDIFORMAT = 165;
pub const D3DDDIFMT_DXVA_RESERVED16: D3DDDIFORMAT = 166;
pub const D3DDDIFMT_DXVA_RESERVED17: D3DDDIFORMAT = 167;
pub const D3DDDIFMT_DXVA_RESERVED18: D3DDDIFORMAT = 168;
pub const D3DDDIFMT_DXVA_RESERVED19: D3DDDIFORMAT = 169;
pub const D3DDDIFMT_DXVA_RESERVED20: D3DDDIFORMAT = 170;
pub const D3DDDIFMT_DXVA_RESERVED21: D3DDDIFORMAT = 171;
pub const D3DDDIFMT_DXVA_RESERVED22: D3DDDIFORMAT = 172;
pub const D3DDDIFMT_DXVA_RESERVED23: D3DDDIFORMAT = 173;
pub const D3DDDIFMT_DXVA_RESERVED24: D3DDDIFORMAT = 174;
pub const D3DDDIFMT_DXVA_RESERVED25: D3DDDIFORMAT = 175;
pub const D3DDDIFMT_DXVA_RESERVED26: D3DDDIFORMAT = 176;
pub const D3DDDIFMT_DXVA_RESERVED27: D3DDDIFORMAT = 177;
pub const D3DDDIFMT_DXVA_RESERVED28: D3DDDIFORMAT = 178;
pub const D3DDDIFMT_DXVA_RESERVED29: D3DDDIFORMAT = 179;
pub const D3DDDIFMT_DXVA_RESERVED30: D3DDDIFORMAT = 180;
pub const D3DDDIFMT_DXVA_RESERVED31: D3DDDIFORMAT = 181;
pub const D3DDDIFMT_DXVA_RESERVED9: D3DDDIFORMAT = 159;
pub const D3DDDIFMT_FILMGRAINBUFFER: D3DDDIFORMAT = 158;
pub const D3DDDIFMT_FORCE_UINT: D3DDDIFORMAT = 2147483647;
pub const D3DDDIFMT_G16R16: D3DDDIFORMAT = 34;
pub const D3DDDIFMT_G16R16F: D3DDDIFORMAT = 112;
pub const D3DDDIFMT_G32R32F: D3DDDIFORMAT = 115;
pub const D3DDDIFMT_G8R8: D3DDDIFORMAT = 91;
pub const D3DDDIFMT_G8R8_G8B8: D3DDDIFORMAT = 1111970375;
pub const D3DDDIFMT_INDEX16: D3DDDIFORMAT = 101;
pub const D3DDDIFMT_INDEX32: D3DDDIFORMAT = 102;
pub const D3DDDIFMT_INVERSEQUANTIZATIONDATA: D3DDDIFORMAT = 154;
pub const D3DDDIFMT_L16: D3DDDIFORMAT = 81;
pub const D3DDDIFMT_L6V5U5: D3DDDIFORMAT = 61;
pub const D3DDDIFMT_L8: D3DDDIFORMAT = 50;
pub const D3DDDIFMT_MACROBLOCKDATA: D3DDDIFORMAT = 151;
pub const D3DDDIFMT_MOTIONVECTORBUFFER: D3DDDIFORMAT = 157;
pub const D3DDDIFMT_MULTI2_ARGB8: D3DDDIFORMAT = 827606349;
pub const D3DDDIFMT_P8: D3DDDIFORMAT = 41;
pub const D3DDDIFMT_PICTUREPARAMSDATA: D3DDDIFORMAT = 150;
pub const D3DDDIFMT_Q16W16V16U16: D3DDDIFORMAT = 110;
pub const D3DDDIFMT_Q8W8V8U8: D3DDDIFORMAT = 63;
pub const D3DDDIFMT_R16F: D3DDDIFORMAT = 111;
pub const D3DDDIFMT_R32F: D3DDDIFORMAT = 114;
pub const D3DDDIFMT_R3G3B2: D3DDDIFORMAT = 27;
pub const D3DDDIFMT_R5G6B5: D3DDDIFORMAT = 23;
pub const D3DDDIFMT_R8: D3DDDIFORMAT = 92;
pub const D3DDDIFMT_R8G8B8: D3DDDIFORMAT = 20;
pub const D3DDDIFMT_R8G8_B8G8: D3DDDIFORMAT = 1195525970;
pub const D3DDDIFMT_RESIDUALDIFFERENCEDATA: D3DDDIFORMAT = 152;
pub const D3DDDIFMT_S1D15: D3DDDIFORMAT = 72;
pub const D3DDDIFMT_S8D24: D3DDDIFORMAT = 74;
pub const D3DDDIFMT_S8_LOCKABLE: D3DDDIFORMAT = 85;
pub const D3DDDIFMT_SLICECONTROLDATA: D3DDDIFORMAT = 155;
pub const D3DDDIFMT_UNKNOWN: D3DDDIFORMAT = 0;
pub const D3DDDIFMT_UYVY: D3DDDIFORMAT = 1498831189;
pub const D3DDDIFMT_V16U16: D3DDDIFORMAT = 64;
pub const D3DDDIFMT_V8U8: D3DDDIFORMAT = 60;
pub const D3DDDIFMT_VERTEXDATA: D3DDDIFORMAT = 100;
pub const D3DDDIFMT_W11V11U10: D3DDDIFORMAT = 65;
pub const D3DDDIFMT_X1R5G5B5: D3DDDIFORMAT = 24;
pub const D3DDDIFMT_X4R4G4B4: D3DDDIFORMAT = 30;
pub const D3DDDIFMT_X4S4D24: D3DDDIFORMAT = 78;
pub const D3DDDIFMT_X8B8G8R8: D3DDDIFORMAT = 33;
pub const D3DDDIFMT_X8D24: D3DDDIFORMAT = 76;
pub const D3DDDIFMT_X8L8V8U8: D3DDDIFORMAT = 62;
pub const D3DDDIFMT_X8R8G8B8: D3DDDIFORMAT = 22;
pub const D3DDDIFMT_YUY2: D3DDDIFORMAT = 844715353;
pub type D3DDDIFORMAT = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDIGPUVIRTUALADDRESS_PROTECTION_TYPE {
    pub Anonymous: D3DDDIGPUVIRTUALADDRESS_PROTECTION_TYPE_0,
}
impl Default for D3DDDIGPUVIRTUALADDRESS_PROTECTION_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDIGPUVIRTUALADDRESS_PROTECTION_TYPE_0 {
    pub Anonymous: D3DDDIGPUVIRTUALADDRESS_PROTECTION_TYPE_0_0,
    pub Value: u64,
}
impl Default for D3DDDIGPUVIRTUALADDRESS_PROTECTION_TYPE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDIGPUVIRTUALADDRESS_PROTECTION_TYPE_0_0 {
    pub _bitfield: u64,
}
pub type D3DDDIGPUVIRTUALADDRESS_RESERVATION_TYPE = i32;
pub const D3DDDIGPUVIRTUALADDRESS_RESERVE_NO_ACCESS: D3DDDIGPUVIRTUALADDRESS_RESERVATION_TYPE = 0;
pub const D3DDDIGPUVIRTUALADDRESS_RESERVE_NO_COMMIT: D3DDDIGPUVIRTUALADDRESS_RESERVATION_TYPE = 2;
pub const D3DDDIGPUVIRTUALADDRESS_RESERVE_ZERO: D3DDDIGPUVIRTUALADDRESS_RESERVATION_TYPE = 1;
pub const D3DDDIMULTISAMPLE_10_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 10;
pub const D3DDDIMULTISAMPLE_11_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 11;
pub const D3DDDIMULTISAMPLE_12_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 12;
pub const D3DDDIMULTISAMPLE_13_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 13;
pub const D3DDDIMULTISAMPLE_14_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 14;
pub const D3DDDIMULTISAMPLE_15_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 15;
pub const D3DDDIMULTISAMPLE_16_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 16;
pub const D3DDDIMULTISAMPLE_2_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 2;
pub const D3DDDIMULTISAMPLE_3_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 3;
pub const D3DDDIMULTISAMPLE_4_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 4;
pub const D3DDDIMULTISAMPLE_5_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 5;
pub const D3DDDIMULTISAMPLE_6_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 6;
pub const D3DDDIMULTISAMPLE_7_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 7;
pub const D3DDDIMULTISAMPLE_8_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 8;
pub const D3DDDIMULTISAMPLE_9_SAMPLES: D3DDDIMULTISAMPLE_TYPE = 9;
pub const D3DDDIMULTISAMPLE_FORCE_UINT: D3DDDIMULTISAMPLE_TYPE = 2147483647;
pub const D3DDDIMULTISAMPLE_NONE: D3DDDIMULTISAMPLE_TYPE = 0;
pub const D3DDDIMULTISAMPLE_NONMASKABLE: D3DDDIMULTISAMPLE_TYPE = 1;
pub type D3DDDIMULTISAMPLE_TYPE = i32;
pub const D3DDDIPOOL_LOCALVIDMEM: D3DDDI_POOL = 3;
pub const D3DDDIPOOL_NONLOCALVIDMEM: D3DDDI_POOL = 4;
pub const D3DDDIPOOL_STAGINGMEM: D3DDDI_POOL = 5;
pub const D3DDDIPOOL_SYSTEMMEM: D3DDDI_POOL = 1;
pub const D3DDDIPOOL_VIDEOMEMORY: D3DDDI_POOL = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDIRECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_ALLOCATIONINFO {
    pub hAllocation: D3DKMT_HANDLE,
    pub pSystemMem: *const core::ffi::c_void,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
    pub VidPnSourceId: D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub Flags: D3DDDI_ALLOCATIONINFO_0,
}
impl Default for D3DDDI_ALLOCATIONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_ALLOCATIONINFO_0 {
    pub Anonymous: D3DDDI_ALLOCATIONINFO_0_0,
    pub Value: u32,
}
impl Default for D3DDDI_ALLOCATIONINFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_ALLOCATIONINFO_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3DDDI_ALLOCATIONINFO2 {
    pub hAllocation: D3DKMT_HANDLE,
    pub Anonymous: D3DDDI_ALLOCATIONINFO2_0,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
    pub VidPnSourceId: D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub Flags: D3DDDI_ALLOCATIONINFO2_1,
    pub GpuVirtualAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub Anonymous2: D3DDDI_ALLOCATIONINFO2_2,
    pub Reserved: [usize; 5],
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3DDDI_ALLOCATIONINFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub union D3DDDI_ALLOCATIONINFO2_0 {
    pub hSection: super::winnt::HANDLE,
    pub pSystemMem: *const core::ffi::c_void,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3DDDI_ALLOCATIONINFO2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub union D3DDDI_ALLOCATIONINFO2_1 {
    pub Anonymous: D3DDDI_ALLOCATIONINFO2_1_0,
    pub Value: u32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3DDDI_ALLOCATIONINFO2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_ALLOCATIONINFO2_1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub union D3DDDI_ALLOCATIONINFO2_2 {
    pub Priority: u32,
    pub Unused: usize,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3DDDI_ALLOCATIONINFO2_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_ALLOCATIONLIST {
    pub hAllocation: D3DKMT_HANDLE,
    pub Anonymous: D3DDDI_ALLOCATIONLIST_0,
}
impl Default for D3DDDI_ALLOCATIONLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_ALLOCATIONLIST_0 {
    pub Anonymous: D3DDDI_ALLOCATIONLIST_0_0,
    pub Value: u32,
}
impl Default for D3DDDI_ALLOCATIONLIST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_ALLOCATIONLIST_0_0 {
    pub _bitfield: u32,
}
pub const D3DDDI_ALLOCATIONPRIORITY_HIGH: u32 = 2684354560;
pub const D3DDDI_ALLOCATIONPRIORITY_LOW: u32 = 1342177280;
pub const D3DDDI_ALLOCATIONPRIORITY_MAXIMUM: u32 = 3355443200;
pub const D3DDDI_ALLOCATIONPRIORITY_MINIMUM: u32 = 671088640;
pub const D3DDDI_ALLOCATIONPRIORITY_NORMAL: u32 = 2013265920;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_BUILDTESTCOMMANDBUFFERFLAGS {
    pub Anonymous: D3DDDI_BUILDTESTCOMMANDBUFFERFLAGS_0,
}
impl Default for D3DDDI_BUILDTESTCOMMANDBUFFERFLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_BUILDTESTCOMMANDBUFFERFLAGS_0 {
    pub Anonymous: D3DDDI_BUILDTESTCOMMANDBUFFERFLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DDDI_BUILDTESTCOMMANDBUFFERFLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_BUILDTESTCOMMANDBUFFERFLAGS_0_0 {
    pub _bitfield: u32,
}
pub const D3DDDI_COLOR_SPACE_CUSTOM: D3DDDI_COLOR_SPACE_TYPE = -1;
pub const D3DDDI_COLOR_SPACE_RESERVED: D3DDDI_COLOR_SPACE_TYPE = 4;
pub const D3DDDI_COLOR_SPACE_RGB_FULL_G10_NONE_P2020: D3DDDI_COLOR_SPACE_TYPE = 25;
pub const D3DDDI_COLOR_SPACE_RGB_FULL_G10_NONE_P709: D3DDDI_COLOR_SPACE_TYPE = 1;
pub const D3DDDI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020: D3DDDI_COLOR_SPACE_TYPE = 12;
pub const D3DDDI_COLOR_SPACE_RGB_FULL_G22_NONE_P2020: D3DDDI_COLOR_SPACE_TYPE = 17;
pub const D3DDDI_COLOR_SPACE_RGB_FULL_G22_NONE_P709: D3DDDI_COLOR_SPACE_TYPE = 0;
pub const D3DDDI_COLOR_SPACE_RGB_STUDIO_G2084_NONE_P2020: D3DDDI_COLOR_SPACE_TYPE = 14;
pub const D3DDDI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P2020: D3DDDI_COLOR_SPACE_TYPE = 3;
pub const D3DDDI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P709: D3DDDI_COLOR_SPACE_TYPE = 2;
pub const D3DDDI_COLOR_SPACE_RGB_STUDIO_G24_NONE_P2020: D3DDDI_COLOR_SPACE_TYPE = 21;
pub const D3DDDI_COLOR_SPACE_RGB_STUDIO_G24_NONE_P709: D3DDDI_COLOR_SPACE_TYPE = 20;
pub type D3DDDI_COLOR_SPACE_TYPE = i32;
pub const D3DDDI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P2020: D3DDDI_COLOR_SPACE_TYPE = 11;
pub const D3DDDI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P601: D3DDDI_COLOR_SPACE_TYPE = 7;
pub const D3DDDI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P709: D3DDDI_COLOR_SPACE_TYPE = 9;
pub const D3DDDI_COLOR_SPACE_YCBCR_FULL_G22_NONE_P709_X601: D3DDDI_COLOR_SPACE_TYPE = 5;
pub const D3DDDI_COLOR_SPACE_YCBCR_FULL_GHLG_TOPLEFT_P2020: D3DDDI_COLOR_SPACE_TYPE = 19;
pub const D3DDDI_COLOR_SPACE_YCBCR_STUDIO_G2084_LEFT_P2020: D3DDDI_COLOR_SPACE_TYPE = 13;
pub const D3DDDI_COLOR_SPACE_YCBCR_STUDIO_G2084_TOPLEFT_P2020: D3DDDI_COLOR_SPACE_TYPE = 16;
pub const D3DDDI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P2020: D3DDDI_COLOR_SPACE_TYPE = 10;
pub const D3DDDI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P601: D3DDDI_COLOR_SPACE_TYPE = 6;
pub const D3DDDI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P709: D3DDDI_COLOR_SPACE_TYPE = 8;
pub const D3DDDI_COLOR_SPACE_YCBCR_STUDIO_G22_TOPLEFT_P2020: D3DDDI_COLOR_SPACE_TYPE = 15;
pub const D3DDDI_COLOR_SPACE_YCBCR_STUDIO_G24_LEFT_P2020: D3DDDI_COLOR_SPACE_TYPE = 23;
pub const D3DDDI_COLOR_SPACE_YCBCR_STUDIO_G24_LEFT_P709: D3DDDI_COLOR_SPACE_TYPE = 22;
pub const D3DDDI_COLOR_SPACE_YCBCR_STUDIO_G24_TOPLEFT_P2020: D3DDDI_COLOR_SPACE_TYPE = 24;
pub const D3DDDI_COLOR_SPACE_YCBCR_STUDIO_GHLG_TOPLEFT_P2020: D3DDDI_COLOR_SPACE_TYPE = 18;
pub const D3DDDI_CPU_NOTIFICATION: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_CREATECONTEXTFLAGS {
    pub Anonymous: D3DDDI_CREATECONTEXTFLAGS_0,
}
impl Default for D3DDDI_CREATECONTEXTFLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_CREATECONTEXTFLAGS_0 {
    pub Anonymous: D3DDDI_CREATECONTEXTFLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DDDI_CREATECONTEXTFLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_CREATECONTEXTFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_CREATEHWCONTEXTFLAGS {
    pub Anonymous: D3DDDI_CREATEHWCONTEXTFLAGS_0,
}
impl Default for D3DDDI_CREATEHWCONTEXTFLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_CREATEHWCONTEXTFLAGS_0 {
    pub Anonymous: D3DDDI_CREATEHWCONTEXTFLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DDDI_CREATEHWCONTEXTFLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_CREATEHWCONTEXTFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_CREATEHWQUEUEFLAGS {
    pub Anonymous: D3DDDI_CREATEHWQUEUEFLAGS_0,
}
impl Default for D3DDDI_CREATEHWQUEUEFLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_CREATEHWQUEUEFLAGS_0 {
    pub Anonymous: D3DDDI_CREATEHWQUEUEFLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DDDI_CREATEHWQUEUEFLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_CREATEHWQUEUEFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_CREATEHWQUEUEFORUSERMODESUBMISSION_FLAGS {
    pub Anonymous: D3DDDI_CREATEHWQUEUEFORUSERMODESUBMISSION_FLAGS_0,
}
impl Default for D3DDDI_CREATEHWQUEUEFORUSERMODESUBMISSION_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_CREATEHWQUEUEFORUSERMODESUBMISSION_FLAGS_0 {
    pub Anonymous: D3DDDI_CREATEHWQUEUEFORUSERMODESUBMISSION_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DDDI_CREATEHWQUEUEFORUSERMODESUBMISSION_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_CREATEHWQUEUEFORUSERMODESUBMISSION_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_DESTROYPAGINGQUEUE {
    pub hPagingQueue: D3DKMT_HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_DOORBELLMAPPING {
    pub CpuVa: *mut core::ffi::c_void,
    pub SecondaryCpuVa: *mut core::ffi::c_void,
}
impl Default for D3DDDI_DOORBELLMAPPING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DDDI_DOORBELLSTATUS = i32;
pub const D3DDDI_DOORBELLSTATUS_CONNECTED: D3DDDI_DOORBELLSTATUS = 0;
pub const D3DDDI_DOORBELLSTATUS_CONNECTED_NOTIFY_KMD: D3DDDI_DOORBELLSTATUS = 1;
pub const D3DDDI_DOORBELLSTATUS_DISCONNECTED_ABORT: D3DDDI_DOORBELLSTATUS = 3;
pub const D3DDDI_DOORBELLSTATUS_DISCONNECTED_RETRY: D3DDDI_DOORBELLSTATUS = 2;
pub const D3DDDI_DOORBELL_PRIVATEDATA_MAX_BYTES_WDDM3_1: u32 = 16;
pub type D3DDDI_DRIVERESCAPETYPE = i32;
pub const D3DDDI_DRIVERESCAPETYPE_BUILDTESTCOMMANDBUFFER: D3DDDI_DRIVERESCAPETYPE = 3;
pub const D3DDDI_DRIVERESCAPETYPE_CPUEVENTUSAGE: D3DDDI_DRIVERESCAPETYPE = 2;
pub const D3DDDI_DRIVERESCAPETYPE_MAX: D3DDDI_DRIVERESCAPETYPE = 4;
pub const D3DDDI_DRIVERESCAPETYPE_TRANSLATEALLOCATIONHANDLE: D3DDDI_DRIVERESCAPETYPE = 0;
pub const D3DDDI_DRIVERESCAPETYPE_TRANSLATERESOURCEHANDLE: D3DDDI_DRIVERESCAPETYPE = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_DRIVERESCAPE_BUILDTESTCOMMANDBUFFER {
    pub EscapeType: D3DDDI_DRIVERESCAPETYPE,
    pub hDevice: D3DKMT_HANDLE,
    pub hContext: D3DKMT_HANDLE,
    pub Flags: D3DDDI_BUILDTESTCOMMANDBUFFERFLAGS,
    pub Command: D3DDDI_TESTCOMMANDBUFFER,
    pub pDmaBuffer: *mut core::ffi::c_void,
    pub pDmaBufferPrivateData: *mut core::ffi::c_void,
    pub DmaBufferSize: u32,
    pub DmaBufferPrivateDataSize: u32,
    pub HardwareProgressFenceId: u64,
    pub HardwareProgressFenceGpuVa: u64,
}
impl Default for D3DDDI_DRIVERESCAPE_BUILDTESTCOMMANDBUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_DRIVERESCAPE_CPUEVENTUSAGE {
    pub EscapeType: D3DDDI_DRIVERESCAPETYPE,
    pub hSyncObject: D3DKMT_HANDLE,
    pub hKmdCpuEvent: u64,
    pub Usage: [u32; 8],
}
impl Default for D3DDDI_DRIVERESCAPE_CPUEVENTUSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_DRIVERESCAPE_TRANSLATEALLOCATIONEHANDLE {
    pub EscapeType: D3DDDI_DRIVERESCAPETYPE,
    pub hAllocation: D3DKMT_HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_DRIVERESCAPE_TRANSLATERESOURCEHANDLE {
    pub EscapeType: D3DDDI_DRIVERESCAPETYPE,
    pub hResource: D3DKMT_HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_DXGI_RGB {
    pub Red: f32,
    pub Green: f32,
    pub Blue: f32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_ESCAPEFLAGS {
    pub Anonymous: D3DDDI_ESCAPEFLAGS_0,
}
impl Default for D3DDDI_ESCAPEFLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_ESCAPEFLAGS_0 {
    pub Anonymous: D3DDDI_ESCAPEFLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DDDI_ESCAPEFLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_ESCAPEFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_EVICT_FLAGS {
    pub Anonymous: D3DDDI_EVICT_FLAGS_0,
}
impl Default for D3DDDI_EVICT_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_EVICT_FLAGS_0 {
    pub Anonymous: D3DDDI_EVICT_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DDDI_EVICT_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_EVICT_FLAGS_0_0 {
    pub _bitfield: u32,
}
pub const D3DDDI_FENCE: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE = 3;
pub const D3DDDI_FLIPINTERVAL_FOUR: D3DDDI_FLIPINTERVAL_TYPE = 4;
pub const D3DDDI_FLIPINTERVAL_IMMEDIATE: D3DDDI_FLIPINTERVAL_TYPE = 0;
pub const D3DDDI_FLIPINTERVAL_IMMEDIATE_ALLOW_TEARING: D3DDDI_FLIPINTERVAL_TYPE = 5;
pub const D3DDDI_FLIPINTERVAL_ONE: D3DDDI_FLIPINTERVAL_TYPE = 1;
pub const D3DDDI_FLIPINTERVAL_THREE: D3DDDI_FLIPINTERVAL_TYPE = 3;
pub const D3DDDI_FLIPINTERVAL_TWO: D3DDDI_FLIPINTERVAL_TYPE = 2;
pub type D3DDDI_FLIPINTERVAL_TYPE = i32;
pub const D3DDDI_GAMMARAMP_DEFAULT: D3DDDI_GAMMARAMP_TYPE = 1;
pub const D3DDDI_GAMMARAMP_DXGI_1: D3DDDI_GAMMARAMP_TYPE = 3;
pub const D3DDDI_GAMMARAMP_MATRIX_3x4: D3DDDI_GAMMARAMP_TYPE = 4;
pub const D3DDDI_GAMMARAMP_MATRIX_V2: D3DDDI_GAMMARAMP_TYPE = 5;
pub const D3DDDI_GAMMARAMP_RGB256x3x16: D3DDDI_GAMMARAMP_TYPE = 2;
pub type D3DDDI_GAMMARAMP_TYPE = i32;
pub const D3DDDI_GAMMARAMP_UNINITIALIZED: D3DDDI_GAMMARAMP_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_GAMMA_RAMP_DXGI_1 {
    pub Scale: D3DDDI_DXGI_RGB,
    pub Offset: D3DDDI_DXGI_RGB,
    pub GammaCurve: [D3DDDI_DXGI_RGB; 1025],
}
impl Default for D3DDDI_GAMMA_RAMP_DXGI_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_GAMMA_RAMP_RGB256x3x16 {
    pub Red: [u16; 256],
    pub Green: [u16; 256],
    pub Blue: [u16; 256],
}
impl Default for D3DDDI_GAMMA_RAMP_RGB256x3x16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_GETRESOURCEPRESENTPRIVATEDRIVERDATA {
    pub hResource: D3DKMT_HANDLE,
    pub PrivateDriverDataSize: u32,
    pub pPrivateDriverData: *mut core::ffi::c_void,
}
impl Default for D3DDDI_GETRESOURCEPRESENTPRIVATEDRIVERDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_HDR_METADATA_HDR10 {
    pub RedPrimary: [u16; 2],
    pub GreenPrimary: [u16; 2],
    pub BluePrimary: [u16; 2],
    pub WhitePoint: [u16; 2],
    pub MaxMasteringLuminance: u32,
    pub MinMasteringLuminance: u32,
    pub MaxContentLightLevel: u16,
    pub MaxFrameAverageLightLevel: u16,
}
impl Default for D3DDDI_HDR_METADATA_HDR10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_HDR_METADATA_HDR10PLUS {
    pub Data: [u8; 72],
}
impl Default for D3DDDI_HDR_METADATA_HDR10PLUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DDDI_HDR_METADATA_TYPE = i32;
pub const D3DDDI_HDR_METADATA_TYPE_HDR10: D3DDDI_HDR_METADATA_TYPE = 1;
pub const D3DDDI_HDR_METADATA_TYPE_HDR10PLUS: D3DDDI_HDR_METADATA_TYPE = 2;
pub const D3DDDI_HDR_METADATA_TYPE_NONE: D3DDDI_HDR_METADATA_TYPE = 0;
pub const D3DDDI_ID_ALL: i32 = -3;
pub const D3DDDI_ID_ANY: i32 = -2;
pub const D3DDDI_ID_NOTAPPLICABLE: u32 = 0;
pub const D3DDDI_ID_UNINITIALIZED: i32 = -1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_KERNELOVERLAYINFO {
    pub hAllocation: D3DKMT_HANDLE,
    pub DstRect: D3DDDIRECT,
    pub SrcRect: D3DDDIRECT,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
}
impl Default for D3DDDI_KERNELOVERLAYINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_MAKERESIDENT {
    pub hPagingQueue: D3DKMT_HANDLE,
    pub NumAllocations: u32,
    pub AllocationList: *const D3DKMT_HANDLE,
    pub PriorityList: *const u32,
    pub Flags: D3DDDI_MAKERESIDENT_FLAGS,
    pub PagingFenceValue: u64,
    pub NumBytesToTrim: u64,
}
impl Default for D3DDDI_MAKERESIDENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_MAKERESIDENT_FLAGS {
    pub Anonymous: D3DDDI_MAKERESIDENT_FLAGS_0,
}
impl Default for D3DDDI_MAKERESIDENT_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_MAKERESIDENT_FLAGS_0 {
    pub Anonymous: D3DDDI_MAKERESIDENT_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DDDI_MAKERESIDENT_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_MAKERESIDENT_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_MAPGPUVIRTUALADDRESS {
    pub hPagingQueue: D3DKMT_HANDLE,
    pub BaseAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub MinimumAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub MaximumAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub hAllocation: D3DKMT_HANDLE,
    pub OffsetInPages: D3DGPU_SIZE_T,
    pub SizeInPages: D3DGPU_SIZE_T,
    pub Protection: D3DDDIGPUVIRTUALADDRESS_PROTECTION_TYPE,
    pub DriverProtection: u64,
    pub Reserved0: u32,
    pub Reserved1: u64,
    pub VirtualAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub PagingFenceValue: u64,
}
impl Default for D3DDDI_MAPGPUVIRTUALADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DDDI_MAXTESTBUFFERPRIVATEDRIVERDATASIZE: u32 = 1024;
pub const D3DDDI_MAXTESTBUFFERSIZE: u32 = 4096;
pub const D3DDDI_MAX_BROADCAST_CONTEXT: u32 = 64;
pub const D3DDDI_MAX_MPO_PRESENT_DIRTY_RECTS: u32 = 4095;
pub const D3DDDI_MAX_OBJECT_SIGNALED: u32 = 32;
pub const D3DDDI_MAX_OBJECT_WAITED_ON: u32 = 32;
pub const D3DDDI_MAX_WRITTEN_PRIMARIES: u32 = 16;
pub const D3DDDI_MONITORED_FENCE: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE = 5;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_MULTISAMPLINGMETHOD {
    pub NumSamples: u32,
    pub NumQualityLevels: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_NATIVEFENCEINFO {
    pub InitialFenceValue: u64,
    pub EngineAffinity: u32,
    pub Type: D3DDDI_NATIVEFENCE_TYPE,
    pub Flags: D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS,
    pub NativeFenceMapping: D3DDDI_NATIVEFENCEMAPPING,
    pub PhysicalAdapterIndex: u32,
    pub Reserved: [u8; 24],
}
impl Default for D3DDDI_NATIVEFENCEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_NATIVEFENCELOGDETAIL {
    pub WaitLogNumberOfEntries: u32,
    pub SignalLogNumberOfEntries: u32,
    pub WaitLogGpuBaseAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub SignalLogGpuBaseAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub Reserved: [u8; 64],
}
impl Default for D3DDDI_NATIVEFENCELOGDETAIL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_NATIVEFENCEMAPPING {
    pub CurrentValueCpuVa: *mut core::ffi::c_void,
    pub CurrentValueGpuVa: D3DGPU_VIRTUAL_ADDRESS,
    pub MonitoredValueGpuVa: D3DGPU_VIRTUAL_ADDRESS,
    pub Reserved: [u8; 32],
}
impl Default for D3DDDI_NATIVEFENCEMAPPING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DDDI_NATIVEFENCE_TYPE = i32;
pub const D3DDDI_NATIVEFENCE_TYPE_DEFAULT: D3DDDI_NATIVEFENCE_TYPE = 0;
pub const D3DDDI_NATIVEFENCE_TYPE_INTRA_GPU: D3DDDI_NATIVEFENCE_TYPE = 1;
pub const D3DDDI_NATIVE_FENCE: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE = 7;
pub const D3DDDI_NATIVE_FENCE_PDD_SIZE: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_OFFER_FLAGS {
    pub Anonymous: D3DDDI_OFFER_FLAGS_0,
}
impl Default for D3DDDI_OFFER_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_OFFER_FLAGS_0 {
    pub Anonymous: D3DDDI_OFFER_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DDDI_OFFER_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_OFFER_FLAGS_0_0 {
    pub _bitfield: u32,
}
pub type D3DDDI_OFFER_PRIORITY = i32;
pub const D3DDDI_OFFER_PRIORITY_AUTO: D3DDDI_OFFER_PRIORITY = 4;
pub const D3DDDI_OFFER_PRIORITY_HIGH: D3DDDI_OFFER_PRIORITY = 3;
pub const D3DDDI_OFFER_PRIORITY_LOW: D3DDDI_OFFER_PRIORITY = 1;
pub const D3DDDI_OFFER_PRIORITY_NONE: D3DDDI_OFFER_PRIORITY = 0;
pub const D3DDDI_OFFER_PRIORITY_NORMAL: D3DDDI_OFFER_PRIORITY = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_OPENALLOCATIONINFO {
    pub hAllocation: D3DKMT_HANDLE,
    pub pPrivateDriverData: *const core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
}
impl Default for D3DDDI_OPENALLOCATIONINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_OPENALLOCATIONINFO2 {
    pub hAllocation: D3DKMT_HANDLE,
    pub pPrivateDriverData: *const core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
    pub GpuVirtualAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub Reserved: [usize; 6],
}
impl Default for D3DDDI_OPENALLOCATIONINFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DDDI_OUTPUT_WIRE_COLOR_SPACE_G2084_P2020: D3DDDI_OUTPUT_WIRE_COLOR_SPACE_TYPE = 12;
pub const D3DDDI_OUTPUT_WIRE_COLOR_SPACE_G2084_P2020_DVLL: D3DDDI_OUTPUT_WIRE_COLOR_SPACE_TYPE = 33;
pub const D3DDDI_OUTPUT_WIRE_COLOR_SPACE_G2084_P2020_HDR10PLUS: D3DDDI_OUTPUT_WIRE_COLOR_SPACE_TYPE = 32;
pub const D3DDDI_OUTPUT_WIRE_COLOR_SPACE_G22_P2020: D3DDDI_OUTPUT_WIRE_COLOR_SPACE_TYPE = 31;
pub const D3DDDI_OUTPUT_WIRE_COLOR_SPACE_G22_P709: D3DDDI_OUTPUT_WIRE_COLOR_SPACE_TYPE = 0;
pub const D3DDDI_OUTPUT_WIRE_COLOR_SPACE_G22_P709_WCG: D3DDDI_OUTPUT_WIRE_COLOR_SPACE_TYPE = 30;
pub const D3DDDI_OUTPUT_WIRE_COLOR_SPACE_RESERVED: D3DDDI_OUTPUT_WIRE_COLOR_SPACE_TYPE = 4;
pub type D3DDDI_OUTPUT_WIRE_COLOR_SPACE_TYPE = i32;
pub type D3DDDI_PAGINGQUEUE_PRIORITY = i32;
pub const D3DDDI_PAGINGQUEUE_PRIORITY_ABOVE_NORMAL: D3DDDI_PAGINGQUEUE_PRIORITY = 1;
pub const D3DDDI_PAGINGQUEUE_PRIORITY_BELOW_NORMAL: D3DDDI_PAGINGQUEUE_PRIORITY = -1;
pub const D3DDDI_PAGINGQUEUE_PRIORITY_NORMAL: D3DDDI_PAGINGQUEUE_PRIORITY = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_PATCHLOCATIONLIST {
    pub AllocationIndex: u32,
    pub Anonymous: D3DDDI_PATCHLOCATIONLIST_0,
    pub DriverId: u32,
    pub AllocationOffset: u32,
    pub PatchOffset: u32,
    pub SplitOffset: u32,
}
impl Default for D3DDDI_PATCHLOCATIONLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_PATCHLOCATIONLIST_0 {
    pub Anonymous: D3DDDI_PATCHLOCATIONLIST_0_0,
    pub Value: u32,
}
impl Default for D3DDDI_PATCHLOCATIONLIST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_PATCHLOCATIONLIST_0_0 {
    pub _bitfield: u32,
}
pub const D3DDDI_PERIODIC_MONITORED_FENCE: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE = 6;
pub type D3DDDI_POOL = i32;
pub const D3DDDI_QUERYREGISTRY_ADAPTERKEY: D3DDDI_QUERYREGISTRY_TYPE = 1;
pub const D3DDDI_QUERYREGISTRY_DRIVERIMAGEPATH: D3DDDI_QUERYREGISTRY_TYPE = 3;
pub const D3DDDI_QUERYREGISTRY_DRIVERSTOREPATH: D3DDDI_QUERYREGISTRY_TYPE = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_QUERYREGISTRY_FLAGS {
    pub Anonymous: D3DDDI_QUERYREGISTRY_FLAGS_0,
}
impl Default for D3DDDI_QUERYREGISTRY_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_QUERYREGISTRY_FLAGS_0 {
    pub Anonymous: D3DDDI_QUERYREGISTRY_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DDDI_QUERYREGISTRY_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_QUERYREGISTRY_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_QUERYREGISTRY_INFO {
    pub QueryType: D3DDDI_QUERYREGISTRY_TYPE,
    pub QueryFlags: D3DDDI_QUERYREGISTRY_FLAGS,
    pub ValueName: [u16; 260],
    pub ValueType: u32,
    pub PhysicalAdapterIndex: u32,
    pub OutputValueSize: u32,
    pub Status: D3DDDI_QUERYREGISTRY_STATUS,
    pub Anonymous: D3DDDI_QUERYREGISTRY_INFO_0,
}
impl Default for D3DDDI_QUERYREGISTRY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_QUERYREGISTRY_INFO_0 {
    pub OutputDword: u32,
    pub OutputQword: u64,
    pub OutputString: [u16; 1],
    pub OutputBinary: [u8; 1],
}
impl Default for D3DDDI_QUERYREGISTRY_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DDDI_QUERYREGISTRY_MAX: D3DDDI_QUERYREGISTRY_TYPE = 4;
pub const D3DDDI_QUERYREGISTRY_SERVICEKEY: D3DDDI_QUERYREGISTRY_TYPE = 0;
pub type D3DDDI_QUERYREGISTRY_STATUS = i32;
pub const D3DDDI_QUERYREGISTRY_STATUS_BUFFER_OVERFLOW: D3DDDI_QUERYREGISTRY_STATUS = 1;
pub const D3DDDI_QUERYREGISTRY_STATUS_FAIL: D3DDDI_QUERYREGISTRY_STATUS = 2;
pub const D3DDDI_QUERYREGISTRY_STATUS_MAX: D3DDDI_QUERYREGISTRY_STATUS = 3;
pub const D3DDDI_QUERYREGISTRY_STATUS_SUCCESS: D3DDDI_QUERYREGISTRY_STATUS = 0;
pub type D3DDDI_QUERYREGISTRY_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_RATIONAL {
    pub Numerator: u32,
    pub Denominator: u32,
}
pub type D3DDDI_RECLAIM_RESULT = i32;
pub const D3DDDI_RECLAIM_RESULT_DISCARDED: D3DDDI_RECLAIM_RESULT = 1;
pub const D3DDDI_RECLAIM_RESULT_NOT_COMMITTED: D3DDDI_RECLAIM_RESULT = 2;
pub const D3DDDI_RECLAIM_RESULT_OK: D3DDDI_RECLAIM_RESULT = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_RESERVEGPUVIRTUALADDRESS {
    pub Anonymous: D3DDDI_RESERVEGPUVIRTUALADDRESS_0,
    pub BaseAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub MinimumAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub MaximumAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub Size: D3DGPU_SIZE_T,
    pub Anonymous2: D3DDDI_RESERVEGPUVIRTUALADDRESS_1,
    pub Anonymous3: D3DDDI_RESERVEGPUVIRTUALADDRESS_2,
    pub VirtualAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub Anonymous4: D3DDDI_RESERVEGPUVIRTUALADDRESS_3,
}
impl Default for D3DDDI_RESERVEGPUVIRTUALADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_RESERVEGPUVIRTUALADDRESS_0 {
    pub hPagingQueue: D3DKMT_HANDLE,
    pub hAdapter: D3DKMT_HANDLE,
}
impl Default for D3DDDI_RESERVEGPUVIRTUALADDRESS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_RESERVEGPUVIRTUALADDRESS_1 {
    pub ReservationType: D3DDDIGPUVIRTUALADDRESS_RESERVATION_TYPE,
    pub Reserved0: u32,
}
impl Default for D3DDDI_RESERVEGPUVIRTUALADDRESS_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_RESERVEGPUVIRTUALADDRESS_2 {
    pub DriverProtection: u64,
    pub Reserved1: u64,
}
impl Default for D3DDDI_RESERVEGPUVIRTUALADDRESS_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_RESERVEGPUVIRTUALADDRESS_3 {
    pub PagingFenceValue: u64,
    pub Reserved2: u64,
}
impl Default for D3DDDI_RESERVEGPUVIRTUALADDRESS_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_RESOURCEFLAGS {
    pub Anonymous: D3DDDI_RESOURCEFLAGS_0,
}
impl Default for D3DDDI_RESOURCEFLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_RESOURCEFLAGS_0 {
    pub Anonymous: D3DDDI_RESOURCEFLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DDDI_RESOURCEFLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_RESOURCEFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_RESOURCEFLAGS2 {
    pub Anonymous: D3DDDI_RESOURCEFLAGS2_0,
}
impl Default for D3DDDI_RESOURCEFLAGS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_RESOURCEFLAGS2_0 {
    pub Anonymous: D3DDDI_RESOURCEFLAGS2_0_0,
    pub Value: u32,
}
impl Default for D3DDDI_RESOURCEFLAGS2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_RESOURCEFLAGS2_0_0 {
    pub _bitfield: u32,
}
pub type D3DDDI_ROTATION = i32;
pub const D3DDDI_ROTATION_180: D3DDDI_ROTATION = 3;
pub const D3DDDI_ROTATION_270: D3DDDI_ROTATION = 4;
pub const D3DDDI_ROTATION_90: D3DDDI_ROTATION = 2;
pub const D3DDDI_ROTATION_IDENTITY: D3DDDI_ROTATION = 1;
pub type D3DDDI_SCANLINEORDERING = i32;
pub const D3DDDI_SCANLINEORDERING_INTERLACED: D3DDDI_SCANLINEORDERING = 2;
pub const D3DDDI_SCANLINEORDERING_PROGRESSIVE: D3DDDI_SCANLINEORDERING = 1;
pub const D3DDDI_SCANLINEORDERING_UNKNOWN: D3DDDI_SCANLINEORDERING = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SEGMENTPREFERENCE {
    pub Anonymous: D3DDDI_SEGMENTPREFERENCE_0,
}
impl Default for D3DDDI_SEGMENTPREFERENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_SEGMENTPREFERENCE_0 {
    pub Anonymous: D3DDDI_SEGMENTPREFERENCE_0_0,
    pub Value: u32,
}
impl Default for D3DDDI_SEGMENTPREFERENCE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_SEGMENTPREFERENCE_0_0 {
    pub _bitfield: u32,
}
pub const D3DDDI_SEMAPHORE: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SURFACEINFO {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub pSysMem: *const core::ffi::c_void,
    pub SysMemPitch: u32,
    pub SysMemSlicePitch: u32,
}
impl Default for D3DDDI_SURFACEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO {
    pub Type: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE,
    pub Anonymous: D3DDDI_SYNCHRONIZATIONOBJECTINFO_0,
}
impl Default for D3DDDI_SYNCHRONIZATIONOBJECTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_SYNCHRONIZATIONOBJECTINFO_0 {
    pub SynchronizationMutex: D3DDDI_SYNCHRONIZATIONOBJECTINFO_0_0,
    pub Semaphore: D3DDDI_SYNCHRONIZATIONOBJECTINFO_0_1,
    pub Reserved: D3DDDI_SYNCHRONIZATIONOBJECTINFO_0_2,
}
impl Default for D3DDDI_SYNCHRONIZATIONOBJECTINFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO_0_0 {
    pub InitialState: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO_0_1 {
    pub MaxCount: u32,
    pub InitialCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO_0_2 {
    pub Reserved: [u32; 16],
}
impl Default for D3DDDI_SYNCHRONIZATIONOBJECTINFO_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2 {
    pub Type: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE,
    pub Flags: D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS,
    pub Anonymous: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0,
    pub SharedHandle: D3DKMT_HANDLE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3DDDI_SYNCHRONIZATIONOBJECTINFO2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub union D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0 {
    pub SynchronizationMutex: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_0,
    pub Semaphore: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_1,
    pub Fence: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_2,
    pub CPUNotification: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_3,
    pub MonitoredFence: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_4,
    pub PeriodicMonitoredFence: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_5,
    pub Reserved: D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_6,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_0 {
    pub InitialState: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_1 {
    pub MaxCount: u32,
    pub InitialCount: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_2 {
    pub FenceValue: u64,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_3 {
    pub Event: super::winnt::HANDLE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_4 {
    pub InitialFenceValue: u64,
    pub FenceValueCPUVirtualAddress: *mut core::ffi::c_void,
    pub FenceValueGPUVirtualAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub EngineAffinity: u32,
    pub Padding: u32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_5 {
    pub hAdapter: D3DKMT_HANDLE,
    pub VidPnTargetId: D3DDDI_VIDEO_PRESENT_TARGET_ID,
    pub Time: u64,
    pub FenceValueCPUVirtualAddress: *mut core::ffi::c_void,
    pub FenceValueGPUVirtualAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub EngineAffinity: u32,
    pub Padding: u32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_6 {
    pub Reserved: [u64; 8],
}
#[cfg(feature = "Win32_winnt")]
impl Default for D3DDDI_SYNCHRONIZATIONOBJECTINFO2_0_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS {
    pub Anonymous: D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS_0,
}
impl Default for D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS_0 {
    pub Anonymous: D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS_0_0 {
    pub _bitfield: u32,
}
pub type D3DDDI_SYNCHRONIZATIONOBJECT_TYPE = i32;
pub const D3DDDI_SYNCHRONIZATION_MUTEX: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE = 1;
pub const D3DDDI_SYNCHRONIZATION_TYPE_LIMIT: D3DDDI_SYNCHRONIZATIONOBJECT_TYPE = 8;
pub const D3DDDI_SYNC_OBJECT_ALL_ACCESS: u32 = 2031619;
pub const D3DDDI_SYNC_OBJECT_SIGNAL: u32 = 2;
pub const D3DDDI_SYNC_OBJECT_WAIT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_TESTCOMMANDBUFFER {
    pub Anonymous: D3DDDI_TESTCOMMANDBUFFER_0,
    pub Operation: D3DDDI_TESTCOMMANDBUFFEROP,
    pub Reserved1: u32,
}
impl Default for D3DDDI_TESTCOMMANDBUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_TESTCOMMANDBUFFER_0 {
    pub Copy: D3DDDI_TESTCOMMANDBUFFER_COPY,
    pub Fill: D3DDDI_TESTCOMMANDBUFFER_FILL,
    pub Reserved: [i8; 64],
}
impl Default for D3DDDI_TESTCOMMANDBUFFER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DDDI_TESTCOMMANDBUFFEROP = i32;
pub const D3DDDI_TESTCOMMANDBUFFEROP_COPY: D3DDDI_TESTCOMMANDBUFFEROP = 1;
pub const D3DDDI_TESTCOMMANDBUFFEROP_FAULT_AND_STALL: D3DDDI_TESTCOMMANDBUFFEROP = 5;
pub const D3DDDI_TESTCOMMANDBUFFEROP_FILL: D3DDDI_TESTCOMMANDBUFFEROP = 2;
pub const D3DDDI_TESTCOMMANDBUFFEROP_INFINITE_LOOP: D3DDDI_TESTCOMMANDBUFFEROP = 3;
pub const D3DDDI_TESTCOMMANDBUFFEROP_INFINITE_PREEMPTABLE_LOOP: D3DDDI_TESTCOMMANDBUFFEROP = 4;
pub const D3DDDI_TESTCOMMANDBUFFEROP_INVALID: D3DDDI_TESTCOMMANDBUFFEROP = 0;
pub const D3DDDI_TESTCOMMANDBUFFEROP_MAX: D3DDDI_TESTCOMMANDBUFFEROP = 6;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_TESTCOMMANDBUFFER_COPY {
    pub SrcAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub DstAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub NumBytes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_TESTCOMMANDBUFFER_FILL {
    pub DstAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub NumBytes: u32,
    pub Pattern: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_TRIMRESIDENCYSET_FLAGS {
    pub Anonymous: D3DDDI_TRIMRESIDENCYSET_FLAGS_0,
}
impl Default for D3DDDI_TRIMRESIDENCYSET_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_TRIMRESIDENCYSET_FLAGS_0 {
    pub Anonymous: D3DDDI_TRIMRESIDENCYSET_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DDDI_TRIMRESIDENCYSET_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_TRIMRESIDENCYSET_FLAGS_0_0 {
    pub _bitfield: u32,
}
pub const D3DDDI_UMS_PDD_SIZE: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_UPDATEALLOCPROPERTY {
    pub hPagingQueue: D3DKMT_HANDLE,
    pub hAllocation: D3DKMT_HANDLE,
    pub SupportedSegmentSet: u32,
    pub PreferredSegment: D3DDDI_SEGMENTPREFERENCE,
    pub Flags: D3DDDI_UPDATEALLOCPROPERTY_FLAGS,
    pub PagingFenceValue: u64,
    pub Anonymous: D3DDDI_UPDATEALLOCPROPERTY_0,
}
impl Default for D3DDDI_UPDATEALLOCPROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_UPDATEALLOCPROPERTY_0 {
    pub Anonymous: D3DDDI_UPDATEALLOCPROPERTY_0_0,
    pub PropertyMaskValue: u32,
}
impl Default for D3DDDI_UPDATEALLOCPROPERTY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_UPDATEALLOCPROPERTY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_UPDATEALLOCPROPERTY_FLAGS {
    pub Anonymous: D3DDDI_UPDATEALLOCPROPERTY_FLAGS_0,
}
impl Default for D3DDDI_UPDATEALLOCPROPERTY_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_UPDATEALLOCPROPERTY_FLAGS_0 {
    pub Anonymous: D3DDDI_UPDATEALLOCPROPERTY_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DDDI_UPDATEALLOCPROPERTY_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_UPDATEALLOCPROPERTY_FLAGS_0_0 {
    pub _bitfield: u32,
}
pub const D3DDDI_UPDATEGPUVIRTUALADDRESS_COPY: D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_TYPE = 2;
pub const D3DDDI_UPDATEGPUVIRTUALADDRESS_MAP: D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_TYPE = 0;
pub const D3DDDI_UPDATEGPUVIRTUALADDRESS_MAP_PROTECT: D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_TYPE = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION {
    pub OperationType: D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_TYPE,
    pub Anonymous: D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0,
}
impl Default for D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0 {
    pub Map: D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0_0,
    pub MapProtect: D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0_1,
    pub Unmap: D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0_2,
    pub Copy: D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0_3,
}
impl Default for D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0_0 {
    pub BaseAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub SizeInBytes: D3DGPU_SIZE_T,
    pub hAllocation: D3DKMT_HANDLE,
    pub AllocationOffsetInBytes: D3DGPU_SIZE_T,
    pub AllocationSizeInBytes: D3DGPU_SIZE_T,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0_1 {
    pub BaseAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub SizeInBytes: D3DGPU_SIZE_T,
    pub hAllocation: D3DKMT_HANDLE,
    pub AllocationOffsetInBytes: D3DGPU_SIZE_T,
    pub AllocationSizeInBytes: D3DGPU_SIZE_T,
    pub Protection: D3DDDIGPUVIRTUALADDRESS_PROTECTION_TYPE,
    pub DriverProtection: u64,
}
impl Default for D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0_2 {
    pub BaseAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub SizeInBytes: D3DGPU_SIZE_T,
    pub Protection: D3DDDIGPUVIRTUALADDRESS_PROTECTION_TYPE,
}
impl Default for D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_0_3 {
    pub SourceAddress: D3DGPU_VIRTUAL_ADDRESS,
    pub SizeInBytes: D3DGPU_SIZE_T,
    pub DestAddress: D3DGPU_VIRTUAL_ADDRESS,
}
pub type D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_TYPE = i32;
pub const D3DDDI_UPDATEGPUVIRTUALADDRESS_UNMAP: D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION_TYPE = 1;
pub type D3DDDI_VIDEO_PRESENT_SOURCE_ID = u32;
pub type D3DDDI_VIDEO_PRESENT_TARGET_ID = u32;
pub type D3DDDI_VIDEO_SIGNAL_SCANLINE_ORDERING = i32;
pub const D3DDDI_VSSLO_INTERLACED_LOWERFIELDFIRST: D3DDDI_VIDEO_SIGNAL_SCANLINE_ORDERING = 3;
pub const D3DDDI_VSSLO_INTERLACED_UPPERFIELDFIRST: D3DDDI_VIDEO_SIGNAL_SCANLINE_ORDERING = 2;
pub const D3DDDI_VSSLO_OTHER: D3DDDI_VIDEO_SIGNAL_SCANLINE_ORDERING = 255;
pub const D3DDDI_VSSLO_PROGRESSIVE: D3DDDI_VIDEO_SIGNAL_SCANLINE_ORDERING = 1;
pub const D3DDDI_VSSLO_UNINITIALIZED: D3DDDI_VIDEO_SIGNAL_SCANLINE_ORDERING = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DDDI_WAITFORSYNCHRONIZATIONOBJECTFROMCPU_FLAGS {
    pub Anonymous: D3DDDI_WAITFORSYNCHRONIZATIONOBJECTFROMCPU_FLAGS_0,
}
impl Default for D3DDDI_WAITFORSYNCHRONIZATIONOBJECTFROMCPU_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DDDI_WAITFORSYNCHRONIZATIONOBJECTFROMCPU_FLAGS_0 {
    pub Anonymous: D3DDDI_WAITFORSYNCHRONIZATIONOBJECTFROMCPU_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DDDI_WAITFORSYNCHRONIZATIONOBJECTFROMCPU_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DDDI_WAITFORSYNCHRONIZATIONOBJECTFROMCPU_FLAGS_0_0 {
    pub _bitfield: u32,
}
pub const D3DGPU_NULL: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DGPU_PHYSICAL_ADDRESS {
    pub SegmentId: u32,
    pub Padding: u32,
    pub SegmentOffset: u64,
}
pub type D3DGPU_SIZE_T = u64;
pub const D3DGPU_UNIQUE_DRIVER_PROTECTION: u64 = 9223372036854775808;
pub type D3DGPU_VIRTUAL_ADDRESS = u64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_3x4_COLORSPACE_TRANSFORM {
    pub ColorMatrix3x4: [[f32; 4]; 3],
    pub ScalarMultiplier: f32,
    pub LookupTable1D: [D3DDDI_DXGI_RGB; 4096],
}
impl Default for D3DKMDT_3x4_COLORSPACE_TRANSFORM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMDT_COLORSPACE_TRANSFORM_MATRIX_V2 {
    pub StageControlLookupTable1DDegamma: D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL,
    pub LookupTable1DDegamma: [D3DDDI_DXGI_RGB; 4096],
    pub StageControlColorMatrix3x3: D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL,
    pub ColorMatrix3x3: [[f32; 3]; 3],
    pub StageControlLookupTable1DRegamma: D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL,
    pub LookupTable1DRegamma: [D3DDDI_DXGI_RGB; 4096],
}
impl Default for D3DKMDT_COLORSPACE_TRANSFORM_MATRIX_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL = i32;
pub const D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL_BYPASS: D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL = 2;
pub const D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL_ENABLE: D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL = 1;
pub const D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL_NO_CHANGE: D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL = 0;
pub const D3DKMDT_MAX_VIDPN_SOURCES: u32 = 16;
pub const D3DKMDT_MAX_VIDPN_SOURCES_BITCOUNT: u32 = 4;
pub const D3DKMT_CROSS_ADAPTER_RESOURCE_HEIGHT_ALIGNMENT: u32 = 4;
pub const D3DKMT_CROSS_ADAPTER_RESOURCE_PITCH_ALIGNMENT: u32 = 128;
pub type D3DKMT_HANDLE = u32;
#[cfg(feature = "Win32_winnt")]
pub type D3DKMT_PTR_TYPE = super::winnt::HANDLE;
pub type D3DKMT_SIZE_T = usize;
pub type D3DKMT_UINT_PTR = usize;
pub type D3DKMT_ULONG_PTR = usize;
pub const D3D_UMD_INTERFACE_VERSION: u32 = 69633;
pub const D3D_UMD_INTERFACE_VERSION_VISTA: u32 = 12;
pub const D3D_UMD_INTERFACE_VERSION_WDDM1_3: u32 = 16386;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_0: u32 = 20482;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_0_M1: u32 = 20480;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_0_M1_3: u32 = 20481;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_0_M2_2: u32 = 20482;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_1: u32 = 24579;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_1_1: u32 = 24576;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_1_2: u32 = 24577;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_1_3: u32 = 24578;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_1_4: u32 = 24579;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_2: u32 = 28673;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_2_1: u32 = 28672;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_2_2: u32 = 28673;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_3: u32 = 32769;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_3_1: u32 = 32768;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_3_2: u32 = 32769;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_4: u32 = 36865;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_4_1: u32 = 36864;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_4_2: u32 = 36865;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_5: u32 = 40962;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_5_1: u32 = 40960;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_5_2: u32 = 40961;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_5_3: u32 = 40962;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_6: u32 = 45059;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_6_1: u32 = 45056;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_6_2: u32 = 45057;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_6_3: u32 = 45058;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_6_4: u32 = 45059;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_7: u32 = 49153;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_7_1: u32 = 49152;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_7_2: u32 = 49153;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_8: u32 = 53248;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_8_1: u32 = 53248;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_9: u32 = 57344;
pub const D3D_UMD_INTERFACE_VERSION_WDDM2_9_1: u32 = 57344;
pub const D3D_UMD_INTERFACE_VERSION_WDDM3_0: u32 = 61440;
pub const D3D_UMD_INTERFACE_VERSION_WDDM3_0_1: u32 = 61440;
pub const D3D_UMD_INTERFACE_VERSION_WDDM3_1: u32 = 65536;
pub const D3D_UMD_INTERFACE_VERSION_WDDM3_1_1: u32 = 65536;
pub const D3D_UMD_INTERFACE_VERSION_WDDM3_2: u32 = 69633;
pub const D3D_UMD_INTERFACE_VERSION_WDDM3_2_1: u32 = 69632;
pub const D3D_UMD_INTERFACE_VERSION_WDDM3_2_2: u32 = 69633;
pub const D3D_UMD_INTERFACE_VERSION_WIN7: u32 = 8195;
pub const D3D_UMD_INTERFACE_VERSION_WIN8: u32 = 12292;
pub const D3D_UMD_INTERFACE_VERSION_WIN8_CP: u32 = 12290;
pub const D3D_UMD_INTERFACE_VERSION_WIN8_M3: u32 = 12289;
pub const D3D_UMD_INTERFACE_VERSION_WIN8_RC: u32 = 12291;
pub const DXGKDDI_INTERFACE_VERSION: u32 = 69640;
pub const DXGKDDI_INTERFACE_VERSION_VISTA: u32 = 4178;
pub const DXGKDDI_INTERFACE_VERSION_VISTA_SP1: u32 = 4179;
pub const DXGKDDI_INTERFACE_VERSION_WDDM1_3: u32 = 16386;
pub const DXGKDDI_INTERFACE_VERSION_WDDM1_3_PATH_INDEPENDENT_ROTATION: u32 = 16387;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_0: u32 = 20515;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_1: u32 = 24579;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_1_5: u32 = 24592;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_1_6: u32 = 24593;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_2: u32 = 28682;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_3: u32 = 32769;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_4: u32 = 36870;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_5: u32 = 40971;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_6: u32 = 45060;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_7: u32 = 49156;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_8: u32 = 53249;
pub const DXGKDDI_INTERFACE_VERSION_WDDM2_9: u32 = 57347;
pub const DXGKDDI_INTERFACE_VERSION_WDDM3_0: u32 = 61443;
pub const DXGKDDI_INTERFACE_VERSION_WDDM3_1: u32 = 65540;
pub const DXGKDDI_INTERFACE_VERSION_WDDM3_2: u32 = 69640;
pub const DXGKDDI_INTERFACE_VERSION_WIN7: u32 = 8197;
pub const DXGKDDI_INTERFACE_VERSION_WIN8: u32 = 12302;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct DXGKVGPU_ESCAPE_HEAD {
    pub Luid: GPUP_DRIVER_ESCAPE_INPUT,
    pub Type: DXGKVGPU_ESCAPE_TYPE,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct DXGKVGPU_ESCAPE_INITIALIZE {
    pub Header: DXGKVGPU_ESCAPE_HEAD,
    pub VmGuid: windows_sys::core::GUID,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct DXGKVGPU_ESCAPE_PAUSE {
    pub Header: DXGKVGPU_ESCAPE_HEAD,
    pub DeviceLuid: super::winnt::LUID,
    pub Anonymous: DXGKVGPU_ESCAPE_PAUSE_0,
}
#[cfg(feature = "Win32_winnt")]
impl Default for DXGKVGPU_ESCAPE_PAUSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub union DXGKVGPU_ESCAPE_PAUSE_0 {
    pub Anonymous: DXGKVGPU_ESCAPE_PAUSE_0_0,
    pub Flags: u32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for DXGKVGPU_ESCAPE_PAUSE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct DXGKVGPU_ESCAPE_PAUSE_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct DXGKVGPU_ESCAPE_POWERTRANSITIONCOMPLETE {
    pub Header: DXGKVGPU_ESCAPE_HEAD,
    pub PowerState: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct DXGKVGPU_ESCAPE_READ_PCI_CONFIG {
    pub Header: DXGKVGPU_ESCAPE_HEAD,
    pub Offset: u32,
    pub Size: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct DXGKVGPU_ESCAPE_READ_VGPU_TYPE {
    pub Header: DXGKVGPU_ESCAPE_HEAD,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct DXGKVGPU_ESCAPE_RELEASE {
    pub Header: DXGKVGPU_ESCAPE_HEAD,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct DXGKVGPU_ESCAPE_RESUME {
    pub Header: DXGKVGPU_ESCAPE_HEAD,
    pub DeviceLuid: super::winnt::LUID,
    pub Flags: u32,
}
pub type DXGKVGPU_ESCAPE_TYPE = i32;
pub const DXGKVGPU_ESCAPE_TYPE_GET_VGPU_TYPE: DXGKVGPU_ESCAPE_TYPE = 4;
pub const DXGKVGPU_ESCAPE_TYPE_INITIALIZE: DXGKVGPU_ESCAPE_TYPE = 2;
pub const DXGKVGPU_ESCAPE_TYPE_PAUSE: DXGKVGPU_ESCAPE_TYPE = 6;
pub const DXGKVGPU_ESCAPE_TYPE_POWERTRANSITIONCOMPLETE: DXGKVGPU_ESCAPE_TYPE = 5;
pub const DXGKVGPU_ESCAPE_TYPE_READ_PCI_CONFIG: DXGKVGPU_ESCAPE_TYPE = 0;
pub const DXGKVGPU_ESCAPE_TYPE_RELEASE: DXGKVGPU_ESCAPE_TYPE = 3;
pub const DXGKVGPU_ESCAPE_TYPE_RESUME: DXGKVGPU_ESCAPE_TYPE = 7;
pub const DXGKVGPU_ESCAPE_TYPE_WRITE_PCI_CONFIG: DXGKVGPU_ESCAPE_TYPE = 1;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct DXGKVGPU_ESCAPE_WRITE_PCI_CONFIG {
    pub Header: DXGKVGPU_ESCAPE_HEAD,
    pub Offset: u32,
    pub Size: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXGK_DETAILED_FEATURE_ID {
    pub Anonymous: DXGK_DETAILED_FEATURE_ID_0,
    pub Value: DXGK_FEATURE_ID,
}
impl Default for DXGK_DETAILED_FEATURE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGK_DETAILED_FEATURE_ID_0 {
    pub _bitfield: u32,
}
pub type DXGK_DRIVER_FEATURE = i32;
pub const DXGK_DRIVER_FEATURE_64K_PT_DEMOTION_FIX: DXGK_DRIVER_FEATURE = 34;
pub const DXGK_DRIVER_FEATURE_EXTENDED_SEGMENT_FLAGS: DXGK_DRIVER_FEATURE = 38;
pub const DXGK_DRIVER_FEATURE_FAULT_AND_STALL: DXGK_DRIVER_FEATURE = 39;
pub const DXGK_DRIVER_FEATURE_GPUPV_PRESENT_HWQUEUE: DXGK_DRIVER_FEATURE = 35;
pub const DXGK_DRIVER_FEATURE_GPUVAIOMMU: DXGK_DRIVER_FEATURE = 36;
pub const DXGK_DRIVER_FEATURE_HWFLIPQUEUE: DXGK_DRIVER_FEATURE = 1;
pub const DXGK_DRIVER_FEATURE_HWSCH: DXGK_DRIVER_FEATURE = 0;
pub const DXGK_DRIVER_FEATURE_KERNEL_MODE_TESTING: DXGK_DRIVER_FEATURE = 33;
pub const DXGK_DRIVER_FEATURE_KMD_SIGNAL_CPU_EVENT: DXGK_DRIVER_FEATURE = 3;
pub const DXGK_DRIVER_FEATURE_LDA_GPUPV: DXGK_DRIVER_FEATURE = 2;
pub const DXGK_DRIVER_FEATURE_MAX: DXGK_DRIVER_FEATURE = 47;
pub const DXGK_DRIVER_FEATURE_NATIVE_FENCE: DXGK_DRIVER_FEATURE = 37;
pub const DXGK_DRIVER_FEATURE_NOTIFY_RESIDENCY2: DXGK_DRIVER_FEATURE = 43;
pub const DXGK_DRIVER_FEATURE_PAGE_BASED_MEMORY_MANAGER: DXGK_DRIVER_FEATURE = 32;
pub const DXGK_DRIVER_FEATURE_PANEL_BUFFER_CONTROL: DXGK_DRIVER_FEATURE = 46;
pub const DXGK_DRIVER_FEATURE_PROCESS_DEBUG_BLOB_COLLECTION: DXGK_DRIVER_FEATURE = 45;
pub const DXGK_DRIVER_FEATURE_RESERVED_1: DXGK_DRIVER_FEATURE = 6;
pub const DXGK_DRIVER_FEATURE_RESERVED_10: DXGK_DRIVER_FEATURE = 15;
pub const DXGK_DRIVER_FEATURE_RESERVED_11: DXGK_DRIVER_FEATURE = 16;
pub const DXGK_DRIVER_FEATURE_RESERVED_12: DXGK_DRIVER_FEATURE = 17;
pub const DXGK_DRIVER_FEATURE_RESERVED_13: DXGK_DRIVER_FEATURE = 18;
pub const DXGK_DRIVER_FEATURE_RESERVED_14: DXGK_DRIVER_FEATURE = 19;
pub const DXGK_DRIVER_FEATURE_RESERVED_15: DXGK_DRIVER_FEATURE = 20;
pub const DXGK_DRIVER_FEATURE_RESERVED_16: DXGK_DRIVER_FEATURE = 21;
pub const DXGK_DRIVER_FEATURE_RESERVED_17: DXGK_DRIVER_FEATURE = 22;
pub const DXGK_DRIVER_FEATURE_RESERVED_18: DXGK_DRIVER_FEATURE = 23;
pub const DXGK_DRIVER_FEATURE_RESERVED_19: DXGK_DRIVER_FEATURE = 24;
pub const DXGK_DRIVER_FEATURE_RESERVED_2: DXGK_DRIVER_FEATURE = 7;
pub const DXGK_DRIVER_FEATURE_RESERVED_20: DXGK_DRIVER_FEATURE = 25;
pub const DXGK_DRIVER_FEATURE_RESERVED_21: DXGK_DRIVER_FEATURE = 26;
pub const DXGK_DRIVER_FEATURE_RESERVED_22: DXGK_DRIVER_FEATURE = 27;
pub const DXGK_DRIVER_FEATURE_RESERVED_23: DXGK_DRIVER_FEATURE = 28;
pub const DXGK_DRIVER_FEATURE_RESERVED_24: DXGK_DRIVER_FEATURE = 29;
pub const DXGK_DRIVER_FEATURE_RESERVED_25: DXGK_DRIVER_FEATURE = 30;
pub const DXGK_DRIVER_FEATURE_RESERVED_3: DXGK_DRIVER_FEATURE = 8;
pub const DXGK_DRIVER_FEATURE_RESERVED_4: DXGK_DRIVER_FEATURE = 9;
pub const DXGK_DRIVER_FEATURE_RESERVED_5: DXGK_DRIVER_FEATURE = 10;
pub const DXGK_DRIVER_FEATURE_RESERVED_6: DXGK_DRIVER_FEATURE = 11;
pub const DXGK_DRIVER_FEATURE_RESERVED_7: DXGK_DRIVER_FEATURE = 12;
pub const DXGK_DRIVER_FEATURE_RESERVED_8: DXGK_DRIVER_FEATURE = 13;
pub const DXGK_DRIVER_FEATURE_RESERVED_9: DXGK_DRIVER_FEATURE = 14;
pub const DXGK_DRIVER_FEATURE_SAMPLE: DXGK_DRIVER_FEATURE = 31;
pub const DXGK_DRIVER_FEATURE_SHARE_BACKING_STORE_WITH_KMD: DXGK_DRIVER_FEATURE = 5;
pub const DXGK_DRIVER_FEATURE_SINGLE_ADAPTER_HYBRID_MODE: DXGK_DRIVER_FEATURE = 40;
pub const DXGK_DRIVER_FEATURE_SYNC_PRESENT_RENDER_HWQ_ONLY: DXGK_DRIVER_FEATURE = 41;
pub const DXGK_DRIVER_FEATURE_UNIFIED_SCHEDULING_MODEL: DXGK_DRIVER_FEATURE = 42;
pub const DXGK_DRIVER_FEATURE_USER_MODE_SUBMISSION: DXGK_DRIVER_FEATURE = 4;
pub const DXGK_FEATURE_64K_PT_DEMOTION_FIX: DXGK_FEATURE_ID = 34;
pub type DXGK_FEATURE_CATEGORY = i32;
pub const DXGK_FEATURE_CATEGORY_BUGFIX: DXGK_FEATURE_CATEGORY = 2;
pub const DXGK_FEATURE_CATEGORY_DRIVER: DXGK_FEATURE_CATEGORY = 0;
pub const DXGK_FEATURE_CATEGORY_MAX: DXGK_FEATURE_CATEGORY = 16;
pub const DXGK_FEATURE_CATEGORY_OS: DXGK_FEATURE_CATEGORY = 1;
pub const DXGK_FEATURE_CATEGORY_RESERVED10: DXGK_FEATURE_CATEGORY = 10;
pub const DXGK_FEATURE_CATEGORY_RESERVED11: DXGK_FEATURE_CATEGORY = 11;
pub const DXGK_FEATURE_CATEGORY_RESERVED12: DXGK_FEATURE_CATEGORY = 12;
pub const DXGK_FEATURE_CATEGORY_RESERVED13: DXGK_FEATURE_CATEGORY = 13;
pub const DXGK_FEATURE_CATEGORY_RESERVED14: DXGK_FEATURE_CATEGORY = 14;
pub const DXGK_FEATURE_CATEGORY_RESERVED15: DXGK_FEATURE_CATEGORY = 15;
pub const DXGK_FEATURE_CATEGORY_RESERVED4: DXGK_FEATURE_CATEGORY = 4;
pub const DXGK_FEATURE_CATEGORY_RESERVED5: DXGK_FEATURE_CATEGORY = 5;
pub const DXGK_FEATURE_CATEGORY_RESERVED6: DXGK_FEATURE_CATEGORY = 6;
pub const DXGK_FEATURE_CATEGORY_RESERVED7: DXGK_FEATURE_CATEGORY = 7;
pub const DXGK_FEATURE_CATEGORY_RESERVED8: DXGK_FEATURE_CATEGORY = 8;
pub const DXGK_FEATURE_CATEGORY_RESERVED9: DXGK_FEATURE_CATEGORY = 9;
pub const DXGK_FEATURE_CATEGORY_TEST: DXGK_FEATURE_CATEGORY = 3;
pub const DXGK_FEATURE_EXTENDED_SEGMENT_FLAGS: DXGK_FEATURE_ID = 38;
pub const DXGK_FEATURE_EXTENDED_SEGMENT_FLAGS_VERSION: u32 = 1;
pub const DXGK_FEATURE_EXTENDED_SEGMENT_FLAGS_VERSION_APERTUREPRESERVEDDURINGSTANDBY: u32 = 1;
pub const DXGK_FEATURE_FAULT_AND_STALL: DXGK_FEATURE_ID = 39;
pub const DXGK_FEATURE_FEATURE_INTERFACE_EXTENSIONS: DXGK_FEATURE_ID = 268435461;
pub const DXGK_FEATURE_FENCE_SIGNAL_FROM_SWS_NODE: DXGK_FEATURE_ID = 268435459;
pub const DXGK_FEATURE_GPUPV_PRESENT_HWQUEUE: DXGK_FEATURE_ID = 35;
pub const DXGK_FEATURE_GPUVAIOMMU: DXGK_FEATURE_ID = 36;
pub const DXGK_FEATURE_HWFLIPQUEUE: DXGK_FEATURE_ID = 1;
pub const DXGK_FEATURE_HWSCH: DXGK_FEATURE_ID = 0;
pub type DXGK_FEATURE_ID = i32;
pub const DXGK_FEATURE_ID_CATEGORY_BITS: u32 = 4;
pub const DXGK_FEATURE_ID_CATEGORY_SHIFT: u32 = 28;
pub const DXGK_FEATURE_ID_FEATURE_BITS: u32 = 28;
pub const DXGK_FEATURE_ID_MASK: u32 = 268435455;
pub const DXGK_FEATURE_KERNEL_MODE_TESTING: DXGK_FEATURE_ID = 33;
pub const DXGK_FEATURE_KMD_SIGNAL_CPU_EVENT: DXGK_FEATURE_ID = 3;
pub const DXGK_FEATURE_LDA_GPUPV: DXGK_FEATURE_ID = 2;
pub const DXGK_FEATURE_MAX: u32 = 47;
pub const DXGK_FEATURE_NATIVE_FENCE: DXGK_FEATURE_ID = 37;
pub const DXGK_FEATURE_NOTIFY_RESIDENCY2: DXGK_FEATURE_ID = 43;
pub const DXGK_FEATURE_OPPORTUNISTIC_64KB_PAGES: DXGK_FEATURE_ID = 268435463;
pub const DXGK_FEATURE_PAGE_BASED_MEMORY_MANAGER: DXGK_FEATURE_ID = 32;
pub const DXGK_FEATURE_PANEL_BUFFER_CONTROL: DXGK_FEATURE_ID = 46;
pub const DXGK_FEATURE_PER_PTE_PAGE_SIZE: DXGK_FEATURE_ID = 268435458;
pub const DXGK_FEATURE_PROCESS_DEBUG_BLOB_COLLECTION: DXGK_FEATURE_ID = 45;
pub const DXGK_FEATURE_QUERYSTATISTICS_EXTENSIONS: DXGK_FEATURE_ID = 268435456;
pub const DXGK_FEATURE_READONLY_EXISTINGSYSMEM: DXGK_FEATURE_ID = 268435462;
pub const DXGK_FEATURE_RESERVE_GPUVA_ZERO_BASE_ADDRESS: DXGK_FEATURE_ID = 268435457;
pub const DXGK_FEATURE_SAMPLE: DXGK_FEATURE_ID = 31;
pub const DXGK_FEATURE_SHARE_BACKING_STORE_WITH_KMD: DXGK_FEATURE_ID = 5;
pub const DXGK_FEATURE_SINGLE_ADAPTER_HYBRID_MODE: DXGK_FEATURE_ID = 40;
pub const DXGK_FEATURE_SUPPRESSVSYNC_INTERRUPTS: DXGK_FEATURE_ID = 268435460;
pub const DXGK_FEATURE_SYNC_PRESENT_RENDER_HWQ_ONLY: DXGK_FEATURE_ID = 41;
pub const DXGK_FEATURE_UNIFIED_SCHEDULING_MODEL: DXGK_FEATURE_ID = 42;
pub const DXGK_FEATURE_USER_MODE_SUBMISSION: DXGK_FEATURE_ID = 4;
pub type DXGK_FEATURE_VERSION = u16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_ISFEATUREENABLED_RESULT {
    pub Version: u16,
    pub Anonymous: DXGK_ISFEATUREENABLED_RESULT_0,
}
impl Default for DXGK_ISFEATUREENABLED_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXGK_ISFEATUREENABLED_RESULT_0 {
    pub Anonymous: DXGK_ISFEATUREENABLED_RESULT_0_0,
    pub Value: DXGK_FEATURE_VERSION,
}
impl Default for DXGK_ISFEATUREENABLED_RESULT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGK_ISFEATUREENABLED_RESULT_0_0 {
    pub _bitfield: u16,
}
pub const DXGK_MAX_GPU_VA_BIT_COUNT: u32 = 63;
pub const DXGK_MAX_PAGE_TABLE_LEVEL_COUNT: u32 = 6;
pub const DXGK_MIN_PAGE_TABLE_LEVEL_COUNT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXGK_MIRACAST_CHUNK_ID {
    pub Anonymous: DXGK_MIRACAST_CHUNK_ID_0,
    pub Value: u64,
}
impl Default for DXGK_MIRACAST_CHUNK_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGK_MIRACAST_CHUNK_ID_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_MIRACAST_CHUNK_INFO {
    pub ChunkType: DXGK_MIRACAST_CHUNK_TYPE,
    pub ChunkId: DXGK_MIRACAST_CHUNK_ID,
    pub ProcessingTime: u32,
    pub EncodeRate: u32,
}
impl Default for DXGK_MIRACAST_CHUNK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGK_MIRACAST_CHUNK_TYPE = i32;
pub const DXGK_MIRACAST_CHUNK_TYPE_COLOR_CONVERT_COMPLETE: DXGK_MIRACAST_CHUNK_TYPE = 1;
pub const DXGK_MIRACAST_CHUNK_TYPE_ENCODE_COMPLETE: DXGK_MIRACAST_CHUNK_TYPE = 2;
pub const DXGK_MIRACAST_CHUNK_TYPE_ENCODE_DRIVER_DEFINED_1: DXGK_MIRACAST_CHUNK_TYPE = -2147483648;
pub const DXGK_MIRACAST_CHUNK_TYPE_ENCODE_DRIVER_DEFINED_2: DXGK_MIRACAST_CHUNK_TYPE = -2147483647;
pub const DXGK_MIRACAST_CHUNK_TYPE_FRAME_DROPPED: DXGK_MIRACAST_CHUNK_TYPE = 4;
pub const DXGK_MIRACAST_CHUNK_TYPE_FRAME_START: DXGK_MIRACAST_CHUNK_TYPE = 3;
pub const DXGK_MIRACAST_CHUNK_TYPE_UNKNOWN: DXGK_MIRACAST_CHUNK_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_NATIVE_FENCE_LOG_BUFFER {
    pub Header: DXGK_NATIVE_FENCE_LOG_HEADER,
    pub Entries: [DXGK_NATIVE_FENCE_LOG_ENTRY; 1],
}
impl Default for DXGK_NATIVE_FENCE_LOG_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGK_NATIVE_FENCE_LOG_ENTRY {
    pub FenceValue: u64,
    pub hNativeFence: D3DKMT_HANDLE,
    pub OperationType: u32,
    pub Reserved0: u64,
    pub FenceObservedGpuTimestamp: u64,
    pub Reserved1: u64,
    pub FenceEndGpuTimestamp: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_NATIVE_FENCE_LOG_HEADER {
    pub Anonymous: DXGK_NATIVE_FENCE_LOG_HEADER_0,
    pub Type: DXGK_NATIVE_FENCE_LOG_TYPE,
    pub NumberOfEntries: u64,
    pub Reserved: [u64; 2],
}
impl Default for DXGK_NATIVE_FENCE_LOG_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXGK_NATIVE_FENCE_LOG_HEADER_0 {
    pub Anonymous: DXGK_NATIVE_FENCE_LOG_HEADER_0_0,
    pub AtomicWraparoundAndEntryIndex: u64,
}
impl Default for DXGK_NATIVE_FENCE_LOG_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGK_NATIVE_FENCE_LOG_HEADER_0_0 {
    pub FirstFreeEntryIndex: u32,
    pub WraparoundCount: u32,
}
pub type DXGK_NATIVE_FENCE_LOG_OPERATION = i32;
pub const DXGK_NATIVE_FENCE_LOG_OPERATION_SIGNAL_EXECUTED: DXGK_NATIVE_FENCE_LOG_OPERATION = 0;
pub const DXGK_NATIVE_FENCE_LOG_OPERATION_WAIT_UNBLOCKED: DXGK_NATIVE_FENCE_LOG_OPERATION = 1;
pub type DXGK_NATIVE_FENCE_LOG_TYPE = i32;
pub const DXGK_NATIVE_FENCE_LOG_TYPE_SIGNALS: DXGK_NATIVE_FENCE_LOG_TYPE = 2;
pub const DXGK_NATIVE_FENCE_LOG_TYPE_WAITS: DXGK_NATIVE_FENCE_LOG_TYPE = 1;
pub type DXGK_OS_FEATURE = i32;
pub const DXGK_OS_FEATURE_FEATURE_INTERFACE_EXTENSIONS: DXGK_OS_FEATURE = 5;
pub const DXGK_OS_FEATURE_FENCE_SIGNAL_FROM_SWS_NODE: DXGK_OS_FEATURE = 3;
pub const DXGK_OS_FEATURE_MAX: DXGK_OS_FEATURE = 8;
pub const DXGK_OS_FEATURE_OPPORTUNISTIC_64KB_PAGES: DXGK_OS_FEATURE = 7;
pub const DXGK_OS_FEATURE_PER_PTE_PAGE_SIZE: DXGK_OS_FEATURE = 2;
pub const DXGK_OS_FEATURE_QUERYSTATISTICS_EXTENSIONS: DXGK_OS_FEATURE = 0;
pub const DXGK_OS_FEATURE_READONLY_EXISTINGSYSMEM: DXGK_OS_FEATURE = 6;
pub const DXGK_OS_FEATURE_RESERVE_GPUVA_ZERO_BASE_ADDRESS: DXGK_OS_FEATURE = 1;
pub const DXGK_OS_FEATURE_SUPPRESSVSYNC_INTERRUPTS: DXGK_OS_FEATURE = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_PTE {
    pub Anonymous: DXGK_PTE_0,
    pub Anonymous2: DXGK_PTE_1,
}
impl Default for DXGK_PTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXGK_PTE_0 {
    pub Anonymous: DXGK_PTE_0_0,
    pub Flags: u64,
}
impl Default for DXGK_PTE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DXGK_PTE_0_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DXGK_PTE_1 {
    pub PageAddress: u64,
    pub PageTableAddress: u64,
}
impl Default for DXGK_PTE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DXGK_PTE_BLOCK_ACCESS_MODE = i32;
pub const DXGK_PTE_BLOCK_ACCESS_NONE: DXGK_PTE_BLOCK_ACCESS_MODE = 0;
pub const DXGK_PTE_BLOCK_ACCESS_READWRITE: DXGK_PTE_BLOCK_ACCESS_MODE = 2;
pub const DXGK_PTE_BLOCK_ACCESS_WRITE: DXGK_PTE_BLOCK_ACCESS_MODE = 1;
pub type DXGK_PTE_PAGE_SIZE = i32;
pub const DXGK_PTE_PAGE_TABLE_PAGE_4KB: DXGK_PTE_PAGE_SIZE = 0;
pub const DXGK_PTE_PAGE_TABLE_PAGE_64KB: DXGK_PTE_PAGE_SIZE = 1;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct GPUP_DRIVER_ESCAPE_INPUT {
    pub vfLUID: super::winnt::LUID,
}
pub const IOCTL_GPUP_DRIVER_ESCAPE: u32 = 2253920;
pub type PD3DDDI_3x4_COLORSPACE_TRANSFORM = *mut D3DKMDT_3x4_COLORSPACE_TRANSFORM;
pub type PD3DKMDT_COLORSPACE_TRANSFORM_MATRIX_V2 = *mut D3DKMDT_COLORSPACE_TRANSFORM_MATRIX_V2;
pub type PD3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL = *mut D3DKMDT_COLORSPACE_TRANSFORM_STAGE_CONTROL;
#[cfg(feature = "Win32_winnt")]
pub type PGPUP_DRIVER_ESCAPE_INPUT = *mut GPUP_DRIVER_ESCAPE_INPUT;
