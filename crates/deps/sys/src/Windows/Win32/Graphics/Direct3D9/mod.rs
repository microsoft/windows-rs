#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DPERF_BeginEvent(col: u32, wszname: super::super::Foundation::PWSTR) -> i32;
    pub fn D3DPERF_EndEvent() -> i32;
    pub fn D3DPERF_GetStatus() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DPERF_QueryRepeatFrame() -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DPERF_SetMarker(col: u32, wszname: super::super::Foundation::PWSTR);
    pub fn D3DPERF_SetOptions(dwoptions: u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn D3DPERF_SetRegion(col: u32, wszname: super::super::Foundation::PWSTR);
    pub fn Direct3DCreate9(sdkversion: u32) -> ::core::option::Option<IDirect3D9>;
    pub fn Direct3DCreate9Ex(sdkversion: u32, param1: *mut IDirect3D9Ex) -> ::windows_sys::core::HRESULT;
}
pub const D3D9_RESOURCE_PRIORITY_HIGH: u32 = 2684354560u32;
pub const D3D9_RESOURCE_PRIORITY_LOW: u32 = 1342177280u32;
pub const D3D9_RESOURCE_PRIORITY_MAXIMUM: u32 = 3355443200u32;
pub const D3D9_RESOURCE_PRIORITY_MINIMUM: u32 = 671088640u32;
pub const D3D9_RESOURCE_PRIORITY_NORMAL: u32 = 2013265920u32;
pub const D3D9b_SDK_VERSION: u32 = 31u32;
pub const D3DADAPTER_DEFAULT: u32 = 0u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DADAPTER_IDENTIFIER9(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DADAPTER_IDENTIFIER9(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct D3DAES_CTR_IV(i32);
#[cfg(any(target_arch = "x86",))]
pub struct D3DAES_CTR_IV(i32);
pub struct D3DAUTHENTICATEDCHANNELTYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURECRYPTOSESSION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREINITIALIZE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREPROTECTION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURESHAREDRESOURCE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGUREUNCOMPRESSEDENCRYPTION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURE_INPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_CONFIGURE_OUTPUT(i32);
pub struct D3DAUTHENTICATEDCHANNEL_PROCESSIDENTIFIERTYPE(i32);
pub struct D3DAUTHENTICATEDCHANNEL_PROTECTION_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCHANNELTYPE_OUTPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_INPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYCRYPTOSESSION_OUTPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYDEVICEHANDLE_OUTPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUIDCOUNT_OUTPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_INPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYEVICTIONENCRYPTIONGUID_OUTPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYINFOBUSTYPE_OUTPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_INPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTIDCOUNT_OUTPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_INPUT(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYOUTPUTID_OUTPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYPROTECTION_OUTPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESSCOUNT_OUTPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_INPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYRESTRICTEDSHAREDRESOURCEPROCESS_OUTPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYUNCOMPRESSEDENCRYPTIONLEVEL_OUTPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERYUNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT_OUTPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERY_INPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DAUTHENTICATEDCHANNEL_QUERY_OUTPUT(i32);
pub const D3DAUTHENTICATEDCONFIGURE_CRYPTOSESSION: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1665584212, data2: 11516, data3: 19156, data4: [130, 36, 209, 88, 55, 222, 119, 0] };
pub const D3DAUTHENTICATEDCONFIGURE_ENCRYPTIONWHENACCESSIBLE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1107292806,
    data2: 27360,
    data3: 19779,
    data4: [157, 85, 164, 110, 158, 253, 21, 138],
};
pub const D3DAUTHENTICATEDCONFIGURE_INITIALIZE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 101796827,
    data2: 13603,
    data3: 18186,
    data4: [141, 202, 251, 194, 132, 81, 84, 240],
};
pub const D3DAUTHENTICATEDCONFIGURE_PROTECTION: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1346721368,
    data2: 16199,
    data3: 17250,
    data4: [191, 153, 191, 223, 205, 233, 237, 41],
};
pub const D3DAUTHENTICATEDCONFIGURE_SHAREDRESOURCE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 124964935, data2: 6976, data3: 18664, data4: [156, 166, 181, 245, 16, 222, 159, 1] };
pub const D3DAUTHENTICATEDQUERY_ACCESSIBILITYATTRIBUTES: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1645533650,
    data2: 17196,
    data3: 19131,
    data4: [159, 206, 33, 110, 234, 38, 158, 59],
};
pub const D3DAUTHENTICATEDQUERY_CHANNELTYPE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3155892389,
    data2: 45563,
    data3: 17067,
    data4: [189, 148, 181, 130, 139, 75, 247, 190],
};
pub const D3DAUTHENTICATEDQUERY_CRYPTOSESSION: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 640960926, data2: 53272, data3: 19828, data4: [172, 23, 127, 114, 64, 89, 82, 141] };
pub const D3DAUTHENTICATEDQUERY_CURRENTENCRYPTIONWHENACCESSIBLE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3960967623,
    data2: 56019,
    data3: 20245,
    data4: [158, 195, 250, 169, 61, 96, 212, 240],
};
pub const D3DAUTHENTICATEDQUERY_DEVICEHANDLE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3961279389,
    data2: 36095,
    data3: 20010,
    data4: [188, 196, 245, 105, 47, 153, 244, 128],
};
pub const D3DAUTHENTICATEDQUERY_ENCRYPTIONWHENACCESSIBLEGUID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4164573528, data2: 59782, data3: 19418, data4: [190, 176, 65, 31, 106, 122, 1, 183] };
pub const D3DAUTHENTICATEDQUERY_ENCRYPTIONWHENACCESSIBLEGUIDCOUNT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3004133478, data2: 8252, data3: 19207, data4: [147, 252, 206, 170, 253, 97, 36, 30] };
pub const D3DAUTHENTICATEDQUERY_OUTPUTID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2208160931,
    data2: 39758,
    data3: 16868,
    data4: [176, 83, 137, 43, 210, 161, 30, 231],
};
pub const D3DAUTHENTICATEDQUERY_OUTPUTIDCOUNT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 738470750,
    data2: 35847,
    data3: 18133,
    data4: [170, 190, 143, 117, 203, 173, 76, 49],
};
pub const D3DAUTHENTICATEDQUERY_PROTECTION: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2823730564,
    data2: 50325,
    data3: 18602,
    data4: [185, 77, 139, 210, 214, 251, 206, 5],
};
pub const D3DAUTHENTICATEDQUERY_RESTRICTEDSHAREDRESOURCEPROCESS: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1687927515, data2: 61684, data3: 17977, data4: [161, 91, 36, 57, 63, 195, 171, 172] };
pub const D3DAUTHENTICATEDQUERY_RESTRICTEDSHAREDRESOURCEPROCESSCOUNT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 229771187,
    data2: 37968,
    data3: 18086,
    data4: [130, 222, 27, 150, 212, 79, 156, 242],
};
pub const D3DAUTHENTICATEDQUERY_UNRESTRICTEDPROTECTEDSHAREDRESOURCECOUNT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 19860438, data2: 58978, data3: 17524, data4: [190, 253, 170, 83, 229, 20, 60, 109] };
pub struct D3DBACKBUFFER_TYPE(i32);
pub struct D3DBASISTYPE(i32);
pub struct D3DBLEND(i32);
pub struct D3DBLENDOP(i32);
pub struct D3DBOX(i32);
pub struct D3DBUSTYPE(i32);
pub const D3DCAPS2_CANAUTOGENMIPMAP: i32 = 1073741824i32;
pub const D3DCAPS2_CANCALIBRATEGAMMA: i32 = 1048576i32;
pub const D3DCAPS2_CANMANAGERESOURCE: i32 = 268435456i32;
pub const D3DCAPS2_CANSHARERESOURCE: i32 = -2147483648i32;
pub const D3DCAPS2_DYNAMICTEXTURES: i32 = 536870912i32;
pub const D3DCAPS2_FULLSCREENGAMMA: i32 = 131072i32;
pub const D3DCAPS2_RESERVED: i32 = 33554432i32;
pub const D3DCAPS3_ALPHA_FULLSCREEN_FLIP_OR_DISCARD: i32 = 32i32;
pub const D3DCAPS3_COPY_TO_SYSTEMMEM: i32 = 512i32;
pub const D3DCAPS3_COPY_TO_VIDMEM: i32 = 256i32;
pub const D3DCAPS3_DXVAHD: i32 = 1024i32;
pub const D3DCAPS3_DXVAHD_LIMITED: i32 = 2048i32;
pub const D3DCAPS3_LINEAR_TO_SRGB_PRESENTATION: i32 = 128i32;
pub const D3DCAPS3_RESERVED: i32 = -2147483617i32;
pub struct D3DCAPS9(i32);
pub const D3DCAPS_OVERLAY: i32 = 2048i32;
pub const D3DCAPS_READ_SCANLINE: i32 = 131072i32;
pub struct D3DCLIPSTATUS9(i32);
pub struct D3DCMPFUNC(i32);
pub struct D3DCOLORVALUE(i32);
pub struct D3DCOMPOSERECTDESC(i32);
pub struct D3DCOMPOSERECTDESTINATION(i32);
pub struct D3DCOMPOSERECTSOP(i32);
pub const D3DCOMPOSERECTS_MAXNUMRECTS: u32 = 65535u32;
pub const D3DCONVOLUTIONMONO_MAXHEIGHT: u32 = 7u32;
pub const D3DCONVOLUTIONMONO_MAXWIDTH: u32 = 7u32;
pub const D3DCPCAPS_CONTENTKEY: u32 = 16u32;
pub const D3DCPCAPS_ENCRYPTEDREADBACK: u32 = 64u32;
pub const D3DCPCAPS_ENCRYPTEDREADBACKKEY: u32 = 128u32;
pub const D3DCPCAPS_ENCRYPTSLICEDATAONLY: u32 = 512u32;
pub const D3DCPCAPS_FRESHENSESSIONKEY: u32 = 32u32;
pub const D3DCPCAPS_HARDWARE: u32 = 2u32;
pub const D3DCPCAPS_PARTIALDECRYPTION: u32 = 8u32;
pub const D3DCPCAPS_PROTECTIONALWAYSON: u32 = 4u32;
pub const D3DCPCAPS_SEQUENTIAL_CTR_IV: u32 = 256u32;
pub const D3DCPCAPS_SOFTWARE: u32 = 1u32;
pub const D3DCREATE_ADAPTERGROUP_DEVICE: i32 = 512i32;
pub const D3DCREATE_DISABLE_DRIVER_MANAGEMENT: i32 = 256i32;
pub const D3DCREATE_DISABLE_DRIVER_MANAGEMENT_EX: i32 = 1024i32;
pub const D3DCREATE_DISABLE_PRINTSCREEN: i32 = 32768i32;
pub const D3DCREATE_DISABLE_PSGP_THREADING: i32 = 8192i32;
pub const D3DCREATE_ENABLE_PRESENTSTATS: i32 = 16384i32;
pub const D3DCREATE_FPU_PRESERVE: i32 = 2i32;
pub const D3DCREATE_HARDWARE_VERTEXPROCESSING: i32 = 64i32;
pub const D3DCREATE_MIXED_VERTEXPROCESSING: i32 = 128i32;
pub const D3DCREATE_MULTITHREADED: i32 = 4i32;
pub const D3DCREATE_NOWINDOWCHANGES: i32 = 2048i32;
pub const D3DCREATE_PUREDEVICE: i32 = 16i32;
pub const D3DCREATE_SCREENSAVER: i32 = 268435456i32;
pub const D3DCREATE_SOFTWARE_VERTEXPROCESSING: i32 = 32i32;
pub const D3DCRYPTOTYPE_AES128_CTR: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2607535889,
    data2: 20340,
    data3: 16841,
    data4: [158, 123, 11, 226, 215, 217, 59, 79],
};
pub const D3DCRYPTOTYPE_PROPRIETARY: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2874055421, data2: 7452, data3: 18150, data4: [167, 47, 8, 105, 145, 123, 13, 232] };
pub const D3DCS_BACK: i32 = 32i32;
pub const D3DCS_BOTTOM: i32 = 8i32;
pub const D3DCS_FRONT: i32 = 16i32;
pub const D3DCS_LEFT: i32 = 1i32;
pub const D3DCS_PLANE0: i32 = 64i32;
pub const D3DCS_PLANE1: i32 = 128i32;
pub const D3DCS_PLANE2: i32 = 256i32;
pub const D3DCS_PLANE3: i32 = 512i32;
pub const D3DCS_PLANE4: i32 = 1024i32;
pub const D3DCS_PLANE5: i32 = 2048i32;
pub const D3DCS_RIGHT: i32 = 2i32;
pub const D3DCS_TOP: i32 = 4i32;
pub struct D3DCUBEMAP_FACES(i32);
pub struct D3DCULL(i32);
pub const D3DCURSORCAPS_COLOR: i32 = 1i32;
pub const D3DCURSORCAPS_LOWRES: i32 = 2i32;
pub const D3DCURSOR_IMMEDIATE_UPDATE: i32 = 1i32;
pub struct D3DDEBUGMONITORTOKENS(i32);
pub struct D3DDECLMETHOD(i32);
pub struct D3DDECLTYPE(i32);
pub struct D3DDECLUSAGE(i32);
pub struct D3DDEGREETYPE(i32);
pub const D3DDEVCAPS2_ADAPTIVETESSNPATCH: i32 = 8i32;
pub const D3DDEVCAPS2_ADAPTIVETESSRTPATCH: i32 = 4i32;
pub const D3DDEVCAPS2_CAN_STRETCHRECT_FROM_TEXTURES: i32 = 16i32;
pub const D3DDEVCAPS2_DMAPNPATCH: i32 = 2i32;
pub const D3DDEVCAPS2_PRESAMPLEDDMAPNPATCH: i32 = 32i32;
pub const D3DDEVCAPS2_STREAMOFFSET: i32 = 1i32;
pub const D3DDEVCAPS2_VERTEXELEMENTSCANSHARESTREAMOFFSET: i32 = 64i32;
pub const D3DDEVCAPS_NPATCHES: i32 = 16777216i32;
pub const D3DDEVCAPS_PUREDEVICE: i32 = 1048576i32;
pub const D3DDEVCAPS_QUINTICRTPATCHES: i32 = 2097152i32;
pub const D3DDEVCAPS_RTPATCHES: i32 = 4194304i32;
pub const D3DDEVCAPS_RTPATCHHANDLEZERO: i32 = 8388608i32;
#[cfg(feature = "Win32_Foundation")]
pub struct D3DDEVICE_CREATION_PARAMETERS(i32);
pub struct D3DDEVINFO_D3D9BANDWIDTHTIMINGS(i32);
pub struct D3DDEVINFO_D3D9CACHEUTILIZATION(i32);
pub struct D3DDEVINFO_D3D9INTERFACETIMINGS(i32);
pub struct D3DDEVINFO_D3D9PIPELINETIMINGS(i32);
pub struct D3DDEVINFO_D3D9STAGETIMINGS(i32);
pub struct D3DDEVINFO_D3DVERTEXSTATS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DDEVINFO_RESOURCEMANAGER(i32);
pub struct D3DDEVINFO_VCACHE(i32);
pub struct D3DDEVTYPE(i32);
pub struct D3DDISPLAYMODE(i32);
pub struct D3DDISPLAYMODEEX(i32);
pub struct D3DDISPLAYMODEFILTER(i32);
pub struct D3DDISPLAYROTATION(i32);
pub const D3DDMAPSAMPLER: u32 = 256u32;
pub const D3DDTCAPS_DEC3N: i32 = 128i32;
pub const D3DDTCAPS_FLOAT16_2: i32 = 256i32;
pub const D3DDTCAPS_FLOAT16_4: i32 = 512i32;
pub const D3DDTCAPS_SHORT2N: i32 = 4i32;
pub const D3DDTCAPS_SHORT4N: i32 = 8i32;
pub const D3DDTCAPS_UBYTE4: i32 = 1i32;
pub const D3DDTCAPS_UBYTE4N: i32 = 2i32;
pub const D3DDTCAPS_UDEC3: i32 = 64i32;
pub const D3DDTCAPS_USHORT2N: i32 = 16i32;
pub const D3DDTCAPS_USHORT4N: i32 = 32i32;
pub struct D3DENCRYPTED_BLOCK_INFO(i32);
pub const D3DENUM_NO_DRIVERVERSION: i32 = 4i32;
pub const D3DENUM_WHQL_LEVEL: i32 = 2i32;
pub struct D3DFILLMODE(i32);
pub const D3DFMT_A1_SURFACE_MAXHEIGHT: u32 = 2048u32;
pub const D3DFMT_A1_SURFACE_MAXWIDTH: u32 = 8192u32;
pub struct D3DFOGMODE(i32);
pub struct D3DFORMAT(i32);
pub const D3DFVFCAPS_PSIZE: i32 = 1048576i32;
pub const D3DFVF_LASTBETA_D3DCOLOR: u32 = 32768u32;
pub const D3DFVF_LASTBETA_UBYTE4: u32 = 4096u32;
pub const D3DFVF_PSIZE: u32 = 32u32;
pub const D3DFVF_XYZW: u32 = 16386u32;
pub struct D3DGAMMARAMP(i32);
pub const D3DGETDATA_FLUSH: u32 = 1u32;
pub struct D3DINDEXBUFFER_DESC(i32);
pub const D3DISSUE_BEGIN: u32 = 2u32;
pub const D3DISSUE_END: u32 = 1u32;
pub const D3DKEYEXCHANGE_DXVA: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1137932124,
    data2: 14565,
    data3: 18724,
    data4: [141, 134, 211, 252, 207, 21, 62, 155],
};
pub const D3DKEYEXCHANGE_RSAES_OAEP: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3247741077, data2: 55082, data3: 18973, data4: [142, 93, 237, 133, 125, 23, 21, 32] };
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub struct D3DLIGHT9(i32);
pub struct D3DLIGHTTYPE(i32);
pub const D3DLINECAPS_ALPHACMP: i32 = 8i32;
pub const D3DLINECAPS_ANTIALIAS: i32 = 32i32;
pub const D3DLINECAPS_BLEND: i32 = 4i32;
pub const D3DLINECAPS_FOG: i32 = 16i32;
pub const D3DLINECAPS_TEXTURE: i32 = 1i32;
pub const D3DLINECAPS_ZTEST: i32 = 2i32;
pub struct D3DLOCKED_BOX(i32);
pub struct D3DLOCKED_RECT(i32);
pub const D3DLOCK_DISCARD: i32 = 8192i32;
pub const D3DLOCK_DONOTWAIT: i32 = 16384i32;
pub const D3DLOCK_NOOVERWRITE: i32 = 4096i32;
pub const D3DLOCK_NOSYSLOCK: i32 = 2048i32;
pub const D3DLOCK_NO_DIRTY_UPDATE: i32 = 32768i32;
pub const D3DLOCK_READONLY: i32 = 16i32;
pub struct D3DMATERIAL9(i32);
pub struct D3DMATERIALCOLORSOURCE(i32);
pub const D3DMAX30SHADERINSTRUCTIONS: u32 = 32768u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct D3DMEMORYPRESSURE(i32);
#[cfg(any(target_arch = "x86",))]
pub struct D3DMEMORYPRESSURE(i32);
pub const D3DMIN30SHADERINSTRUCTIONS: u32 = 512u32;
pub struct D3DMULTISAMPLE_TYPE(i32);
pub const D3DOVERLAYCAPS_FULLRANGERGB: u32 = 1u32;
pub const D3DOVERLAYCAPS_LIMITEDRANGERGB: u32 = 2u32;
pub const D3DOVERLAYCAPS_STRETCHX: u32 = 64u32;
pub const D3DOVERLAYCAPS_STRETCHY: u32 = 128u32;
pub const D3DOVERLAYCAPS_YCbCr_BT601: u32 = 4u32;
pub const D3DOVERLAYCAPS_YCbCr_BT601_xvYCC: u32 = 16u32;
pub const D3DOVERLAYCAPS_YCbCr_BT709: u32 = 8u32;
pub const D3DOVERLAYCAPS_YCbCr_BT709_xvYCC: u32 = 32u32;
pub struct D3DPATCHEDGESTYLE(i32);
pub const D3DPBLENDCAPS_BLENDFACTOR: i32 = 8192i32;
pub const D3DPBLENDCAPS_INVSRCCOLOR2: i32 = 32768i32;
pub const D3DPBLENDCAPS_SRCCOLOR2: i32 = 16384i32;
pub const D3DPMISCCAPS_BLENDOP: i32 = 2048i32;
pub const D3DPMISCCAPS_CLIPPLANESCALEDPOINTS: i32 = 256i32;
pub const D3DPMISCCAPS_CLIPTLVERTS: i32 = 512i32;
pub const D3DPMISCCAPS_COLORWRITEENABLE: i32 = 128i32;
pub const D3DPMISCCAPS_FOGANDSPECULARALPHA: i32 = 65536i32;
pub const D3DPMISCCAPS_FOGVERTEXCLAMPED: i32 = 1048576i32;
pub const D3DPMISCCAPS_INDEPENDENTWRITEMASKS: i32 = 16384i32;
pub const D3DPMISCCAPS_MRTINDEPENDENTBITDEPTHS: i32 = 262144i32;
pub const D3DPMISCCAPS_MRTPOSTPIXELSHADERBLENDING: i32 = 524288i32;
pub const D3DPMISCCAPS_NULLREFERENCE: i32 = 4096i32;
pub const D3DPMISCCAPS_PERSTAGECONSTANT: i32 = 32768i32;
pub const D3DPMISCCAPS_POSTBLENDSRGBCONVERT: i32 = 2097152i32;
pub const D3DPMISCCAPS_SEPARATEALPHABLEND: i32 = 131072i32;
pub const D3DPMISCCAPS_TSSARGTEMP: i32 = 1024i32;
pub struct D3DPOOL(i32);
pub const D3DPRASTERCAPS_COLORPERSPECTIVE: i32 = 4194304i32;
pub const D3DPRASTERCAPS_DEPTHBIAS: i32 = 67108864i32;
pub const D3DPRASTERCAPS_MULTISAMPLE_TOGGLE: i32 = 134217728i32;
pub const D3DPRASTERCAPS_SCISSORTEST: i32 = 16777216i32;
pub const D3DPRASTERCAPS_SLOPESCALEDEPTHBIAS: i32 = 33554432i32;
pub const D3DPRESENTFLAG_DEVICECLIP: u32 = 4u32;
pub const D3DPRESENTFLAG_DISCARD_DEPTHSTENCIL: u32 = 2u32;
pub const D3DPRESENTFLAG_LOCKABLE_BACKBUFFER: u32 = 1u32;
pub const D3DPRESENTFLAG_NOAUTOROTATE: u32 = 32u32;
pub const D3DPRESENTFLAG_OVERLAY_LIMITEDRGB: u32 = 128u32;
pub const D3DPRESENTFLAG_OVERLAY_YCbCr_BT709: u32 = 256u32;
pub const D3DPRESENTFLAG_OVERLAY_YCbCr_xvYCC: u32 = 512u32;
pub const D3DPRESENTFLAG_RESTRICTED_CONTENT: u32 = 1024u32;
pub const D3DPRESENTFLAG_RESTRICT_SHARED_RESOURCE_DRIVER: u32 = 2048u32;
pub const D3DPRESENTFLAG_UNPRUNEDMODE: u32 = 64u32;
pub const D3DPRESENTFLAG_VIDEO: u32 = 16u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct D3DPRESENTSTATS(i32);
#[cfg(any(target_arch = "x86",))]
pub struct D3DPRESENTSTATS(i32);
pub const D3DPRESENT_BACK_BUFFERS_MAX: i32 = 3i32;
pub const D3DPRESENT_BACK_BUFFERS_MAX_EX: i32 = 30i32;
pub const D3DPRESENT_DONOTFLIP: i32 = 4i32;
pub const D3DPRESENT_DONOTWAIT: i32 = 1i32;
pub const D3DPRESENT_FLIPRESTART: i32 = 8i32;
pub const D3DPRESENT_FORCEIMMEDIATE: i32 = 256i32;
pub const D3DPRESENT_HIDEOVERLAY: i32 = 64i32;
pub const D3DPRESENT_INTERVAL_DEFAULT: i32 = 0i32;
pub const D3DPRESENT_INTERVAL_FOUR: i32 = 8i32;
pub const D3DPRESENT_INTERVAL_IMMEDIATE: i32 = -2147483648i32;
pub const D3DPRESENT_INTERVAL_ONE: i32 = 1i32;
pub const D3DPRESENT_INTERVAL_THREE: i32 = 4i32;
pub const D3DPRESENT_INTERVAL_TWO: i32 = 2i32;
pub const D3DPRESENT_LINEAR_CONTENT: i32 = 2i32;
#[cfg(feature = "Win32_Foundation")]
pub struct D3DPRESENT_PARAMETERS(i32);
pub const D3DPRESENT_RATE_DEFAULT: u32 = 0u32;
pub const D3DPRESENT_UPDATECOLORKEY: i32 = 128i32;
pub const D3DPRESENT_UPDATEOVERLAYONLY: i32 = 32i32;
pub const D3DPRESENT_VIDEO_RESTRICT_TO_MONITOR: i32 = 16i32;
pub struct D3DPRIMITIVETYPE(i32);
pub const D3DPS20CAPS_ARBITRARYSWIZZLE: u32 = 1u32;
pub const D3DPS20CAPS_GRADIENTINSTRUCTIONS: u32 = 2u32;
pub const D3DPS20CAPS_NODEPENDENTREADLIMIT: u32 = 8u32;
pub const D3DPS20CAPS_NOTEXINSTRUCTIONLIMIT: u32 = 16u32;
pub const D3DPS20CAPS_PREDICATION: u32 = 4u32;
pub const D3DPS20_MAX_DYNAMICFLOWCONTROLDEPTH: u32 = 24u32;
pub const D3DPS20_MAX_NUMINSTRUCTIONSLOTS: u32 = 512u32;
pub const D3DPS20_MAX_NUMTEMPS: u32 = 32u32;
pub const D3DPS20_MAX_STATICFLOWCONTROLDEPTH: u32 = 4u32;
pub const D3DPS20_MIN_DYNAMICFLOWCONTROLDEPTH: u32 = 0u32;
pub const D3DPS20_MIN_NUMINSTRUCTIONSLOTS: u32 = 96u32;
pub const D3DPS20_MIN_NUMTEMPS: u32 = 12u32;
pub const D3DPS20_MIN_STATICFLOWCONTROLDEPTH: u32 = 0u32;
pub struct D3DPSHADERCAPS2_0(i32);
pub const D3DPTADDRESSCAPS_MIRRORONCE: i32 = 32i32;
pub const D3DPTEXTURECAPS_CUBEMAP_POW2: i32 = 131072i32;
pub const D3DPTEXTURECAPS_MIPCUBEMAP: i32 = 65536i32;
pub const D3DPTEXTURECAPS_MIPMAP: i32 = 16384i32;
pub const D3DPTEXTURECAPS_MIPVOLUMEMAP: i32 = 32768i32;
pub const D3DPTEXTURECAPS_NOPROJECTEDBUMPENV: i32 = 2097152i32;
pub const D3DPTEXTURECAPS_VOLUMEMAP: i32 = 8192i32;
pub const D3DPTEXTURECAPS_VOLUMEMAP_POW2: i32 = 262144i32;
pub const D3DPTFILTERCAPS_CONVOLUTIONMONO: i32 = 262144i32;
pub const D3DPTFILTERCAPS_MAGFGAUSSIANQUAD: i32 = 268435456i32;
pub const D3DPTFILTERCAPS_MAGFPYRAMIDALQUAD: i32 = 134217728i32;
pub const D3DPTFILTERCAPS_MINFGAUSSIANQUAD: i32 = 4096i32;
pub const D3DPTFILTERCAPS_MINFPYRAMIDALQUAD: i32 = 2048i32;
pub struct D3DQUERYTYPE(i32);
pub struct D3DRANGE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DRASTER_STATUS(i32);
pub struct D3DRECT(i32);
pub struct D3DRECTPATCH_INFO(i32);
pub struct D3DRENDERSTATETYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct D3DRESOURCESTATS(i32);
pub struct D3DRESOURCETYPE(i32);
pub const D3DRTYPECOUNT: u32 = 8u32;
pub struct D3DSAMPLERSTATETYPE(i32);
pub struct D3DSAMPLER_TEXTURE_TYPE(i32);
pub struct D3DSCANLINEORDERING(i32);
pub const D3DSGR_CALIBRATE: i32 = 1i32;
pub const D3DSGR_NO_CALIBRATION: i32 = 0i32;
pub struct D3DSHADEMODE(i32);
pub const D3DSHADER_ADDRESSMODE_SHIFT: u32 = 13u32;
pub struct D3DSHADER_ADDRESSMODE_TYPE(i32);
pub struct D3DSHADER_COMPARISON(i32);
pub const D3DSHADER_COMPARISON_SHIFT: u32 = 16u32;
pub struct D3DSHADER_INSTRUCTION_OPCODE_TYPE(i32);
pub struct D3DSHADER_MIN_PRECISION(i32);
pub struct D3DSHADER_MISCTYPE_OFFSETS(i32);
pub struct D3DSHADER_PARAM_REGISTER_TYPE(i32);
pub struct D3DSHADER_PARAM_SRCMOD_TYPE(i32);
pub const D3DSI_COISSUE: u32 = 1073741824u32;
pub const D3DSI_COMMENTSIZE_MASK: u32 = 2147418112u32;
pub const D3DSI_COMMENTSIZE_SHIFT: u32 = 16u32;
pub const D3DSI_INSTLENGTH_MASK: u32 = 251658240u32;
pub const D3DSI_INSTLENGTH_SHIFT: u32 = 24u32;
pub const D3DSI_OPCODE_MASK: u32 = 65535u32;
pub const D3DSPD_IUNKNOWN: i32 = 1i32;
pub const D3DSP_DCL_USAGEINDEX_MASK: u32 = 983040u32;
pub const D3DSP_DCL_USAGEINDEX_SHIFT: u32 = 16u32;
pub const D3DSP_DCL_USAGE_MASK: u32 = 15u32;
pub const D3DSP_DCL_USAGE_SHIFT: u32 = 0u32;
pub const D3DSP_DSTMOD_MASK: u32 = 15728640u32;
pub const D3DSP_DSTMOD_SHIFT: u32 = 20u32;
pub const D3DSP_DSTSHIFT_MASK: u32 = 251658240u32;
pub const D3DSP_DSTSHIFT_SHIFT: u32 = 24u32;
pub const D3DSP_MIN_PRECISION_MASK: u32 = 49152u32;
pub const D3DSP_MIN_PRECISION_SHIFT: u32 = 14u32;
pub const D3DSP_OPCODESPECIFICCONTROL_MASK: u32 = 16711680u32;
pub const D3DSP_OPCODESPECIFICCONTROL_SHIFT: u32 = 16u32;
pub const D3DSP_REGNUM_MASK: u32 = 2047u32;
pub const D3DSP_REGTYPE_MASK: u32 = 1879048192u32;
pub const D3DSP_REGTYPE_MASK2: u32 = 6144u32;
pub const D3DSP_REGTYPE_SHIFT: u32 = 28u32;
pub const D3DSP_REGTYPE_SHIFT2: u32 = 8u32;
pub const D3DSP_SRCMOD_MASK: u32 = 251658240u32;
pub const D3DSP_SRCMOD_SHIFT: u32 = 24u32;
pub const D3DSP_SWIZZLE_MASK: u32 = 16711680u32;
pub const D3DSP_SWIZZLE_SHIFT: u32 = 16u32;
pub const D3DSP_TEXTURETYPE_MASK: u32 = 2013265920u32;
pub const D3DSP_TEXTURETYPE_SHIFT: u32 = 27u32;
pub const D3DSP_WRITEMASK_0: u32 = 65536u32;
pub const D3DSP_WRITEMASK_1: u32 = 131072u32;
pub const D3DSP_WRITEMASK_2: u32 = 262144u32;
pub const D3DSP_WRITEMASK_3: u32 = 524288u32;
pub const D3DSP_WRITEMASK_ALL: u32 = 983040u32;
pub struct D3DSTATEBLOCKTYPE(i32);
pub const D3DSTENCILCAPS_TWOSIDED: i32 = 256i32;
pub struct D3DSTENCILOP(i32);
pub const D3DSTREAMSOURCE_INDEXEDDATA: u32 = 1073741824u32;
pub const D3DSTREAMSOURCE_INSTANCEDATA: u32 = 2147483648u32;
pub struct D3DSURFACE_DESC(i32);
pub struct D3DSWAPEFFECT(i32);
pub const D3DTA_CONSTANT: u32 = 6u32;
pub const D3DTA_TEMP: u32 = 5u32;
pub const D3DTEXOPCAPS_LERP: i32 = 33554432i32;
pub const D3DTEXOPCAPS_MULTIPLYADD: i32 = 16777216i32;
pub struct D3DTEXTUREADDRESS(i32);
pub struct D3DTEXTUREFILTERTYPE(i32);
pub struct D3DTEXTUREOP(i32);
pub struct D3DTEXTURESTAGESTATETYPE(i32);
pub struct D3DTEXTURETRANSFORMFLAGS(i32);
pub struct D3DTRANSFORMSTATETYPE(i32);
pub struct D3DTRIPATCH_INFO(i32);
pub const D3DTSS_TCI_SPHEREMAP: u32 = 262144u32;
pub const D3DUSAGE_AUTOGENMIPMAP: i32 = 1024i32;
pub const D3DUSAGE_DEPTHSTENCIL: i32 = 2i32;
pub const D3DUSAGE_DMAP: i32 = 16384i32;
pub const D3DUSAGE_DONOTCLIP: i32 = 32i32;
pub const D3DUSAGE_DYNAMIC: i32 = 512i32;
pub const D3DUSAGE_NONSECURE: i32 = 8388608i32;
pub const D3DUSAGE_NPATCHES: i32 = 256i32;
pub const D3DUSAGE_POINTS: i32 = 64i32;
pub const D3DUSAGE_QUERY_FILTER: i32 = 131072i32;
pub const D3DUSAGE_QUERY_LEGACYBUMPMAP: i32 = 32768i32;
pub const D3DUSAGE_QUERY_POSTPIXELSHADER_BLENDING: i32 = 524288i32;
pub const D3DUSAGE_QUERY_SRGBREAD: i32 = 65536i32;
pub const D3DUSAGE_QUERY_SRGBWRITE: i32 = 262144i32;
pub const D3DUSAGE_QUERY_VERTEXTEXTURE: i32 = 1048576i32;
pub const D3DUSAGE_QUERY_WRAPANDMIP: i32 = 2097152i32;
pub const D3DUSAGE_RENDERTARGET: i32 = 1i32;
pub const D3DUSAGE_RESTRICTED_CONTENT: i32 = 2048i32;
pub const D3DUSAGE_RESTRICT_SHARED_RESOURCE: i32 = 8192i32;
pub const D3DUSAGE_RESTRICT_SHARED_RESOURCE_DRIVER: i32 = 4096i32;
pub const D3DUSAGE_RTPATCHES: i32 = 128i32;
pub const D3DUSAGE_SOFTWAREPROCESSING: i32 = 16i32;
pub const D3DUSAGE_TEXTAPI: i32 = 268435456i32;
pub const D3DUSAGE_WRITEONLY: i32 = 8i32;
pub struct D3DVERTEXBLENDFLAGS(i32);
pub struct D3DVERTEXBUFFER_DESC(i32);
pub struct D3DVERTEXELEMENT9(i32);
pub const D3DVERTEXTEXTURESAMPLER0: u32 = 257u32;
pub const D3DVERTEXTEXTURESAMPLER1: u32 = 258u32;
pub const D3DVERTEXTEXTURESAMPLER2: u32 = 259u32;
pub const D3DVERTEXTEXTURESAMPLER3: u32 = 260u32;
pub struct D3DVIEWPORT9(i32);
pub struct D3DVOLUME_DESC(i32);
pub const D3DVS20CAPS_PREDICATION: u32 = 1u32;
pub const D3DVS20_MAX_DYNAMICFLOWCONTROLDEPTH: u32 = 24u32;
pub const D3DVS20_MAX_NUMTEMPS: u32 = 32u32;
pub const D3DVS20_MAX_STATICFLOWCONTROLDEPTH: u32 = 4u32;
pub const D3DVS20_MIN_DYNAMICFLOWCONTROLDEPTH: u32 = 0u32;
pub const D3DVS20_MIN_NUMTEMPS: u32 = 12u32;
pub const D3DVS20_MIN_STATICFLOWCONTROLDEPTH: u32 = 1u32;
pub struct D3DVSHADERCAPS2_0(i32);
pub const D3DVS_ADDRESSMODE_SHIFT: u32 = 13u32;
pub struct D3DVS_ADDRESSMODE_TYPE(i32);
pub struct D3DVS_RASTOUT_OFFSETS(i32);
pub const D3DVS_SWIZZLE_MASK: u32 = 16711680u32;
pub const D3DVS_SWIZZLE_SHIFT: u32 = 16u32;
pub const D3DVTXPCAPS_NO_TEXGEN_NONLOCALVIEWER: i32 = 512i32;
pub const D3DVTXPCAPS_TEXGEN_SPHEREMAP: i32 = 256i32;
pub const D3DVTXPCAPS_TWEENING: i32 = 64i32;
pub const D3DWRAP_W: i32 = 4i32;
pub struct D3DZBUFFERTYPE(i32);
pub const D3D_MAX_SIMULTANEOUS_RENDERTARGETS: u32 = 4u32;
pub struct D3D_OMAC(i32);
pub const D3D_OMAC_SIZE: u32 = 16u32;
pub const D3D_SDK_VERSION: u32 = 32u32;
#[repr(transparent)]
pub struct IDirect3D9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3D9Ex(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3DBaseTexture9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3DCubeTexture9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3DDevice9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3DDevice9Ex(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3DIndexBuffer9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3DPixelShader9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3DQuery9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3DResource9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3DStateBlock9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3DSurface9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3DSwapChain9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3DSwapChain9Ex(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3DTexture9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3DVertexBuffer9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3DVertexDeclaration9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3DVertexShader9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3DVolume9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDirect3DVolumeTexture9(pub *mut ::core::ffi::c_void);
pub const MAXD3DDECLLENGTH: u32 = 64u32;
pub const MAXD3DDECLUSAGEINDEX: u32 = 15u32;
pub const MAX_DEVICE_IDENTIFIER_STRING: u32 = 512u32;
pub const _FACD3D: u32 = 2166u32;
