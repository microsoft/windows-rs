#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi"))]
    pub fn D3D11CreateDevice(padapter: super::Dxgi::IDXGIAdapter, drivertype: super::Direct3D::D3D_DRIVER_TYPE, software: super::super::Foundation::HINSTANCE, flags: D3D11_CREATE_DEVICE_FLAG, pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, sdkversion: u32, ppdevice: *mut ID3D11Device, pfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL, ppimmediatecontext: *mut ID3D11DeviceContext) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
    pub fn D3D11CreateDeviceAndSwapChain(
        padapter: super::Dxgi::IDXGIAdapter,
        drivertype: super::Direct3D::D3D_DRIVER_TYPE,
        software: super::super::Foundation::HINSTANCE,
        flags: D3D11_CREATE_DEVICE_FLAG,
        pfeaturelevels: *const super::Direct3D::D3D_FEATURE_LEVEL,
        featurelevels: u32,
        sdkversion: u32,
        pswapchaindesc: *const super::Dxgi::DXGI_SWAP_CHAIN_DESC,
        ppswapchain: *mut super::Dxgi::IDXGISwapChain,
        ppdevice: *mut ID3D11Device,
        pfeaturelevel: *mut super::Direct3D::D3D_FEATURE_LEVEL,
        ppimmediatecontext: *mut ID3D11DeviceContext,
    ) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub fn D3DDisassemble11Trace(psrcdata: *const ::core::ffi::c_void, srcdatasize: usize, ptrace: ID3D11ShaderTrace, startstep: u32, numsteps: u32, flags: u32, ppdisassembly: *mut super::Direct3D::ID3DBlob) -> ::windows_sys::core::HRESULT;
    pub fn D3DX11CreateFFT(pdevicecontext: ID3D11DeviceContext, pdesc: *const D3DX11_FFT_DESC, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ID3DX11FFT) -> ::windows_sys::core::HRESULT;
    pub fn D3DX11CreateFFT1DComplex(pdevicecontext: ID3D11DeviceContext, x: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ID3DX11FFT) -> ::windows_sys::core::HRESULT;
    pub fn D3DX11CreateFFT1DReal(pdevicecontext: ID3D11DeviceContext, x: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ID3DX11FFT) -> ::windows_sys::core::HRESULT;
    pub fn D3DX11CreateFFT2DComplex(pdevicecontext: ID3D11DeviceContext, x: u32, y: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ID3DX11FFT) -> ::windows_sys::core::HRESULT;
    pub fn D3DX11CreateFFT2DReal(pdevicecontext: ID3D11DeviceContext, x: u32, y: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ID3DX11FFT) -> ::windows_sys::core::HRESULT;
    pub fn D3DX11CreateFFT3DComplex(pdevicecontext: ID3D11DeviceContext, x: u32, y: u32, z: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ID3DX11FFT) -> ::windows_sys::core::HRESULT;
    pub fn D3DX11CreateFFT3DReal(pdevicecontext: ID3D11DeviceContext, x: u32, y: u32, z: u32, flags: u32, pbufferinfo: *mut D3DX11_FFT_BUFFER_INFO, ppfft: *mut ID3DX11FFT) -> ::windows_sys::core::HRESULT;
    pub fn D3DX11CreateScan(pdevicecontext: ID3D11DeviceContext, maxelementscansize: u32, maxscancount: u32, ppscan: *mut ID3DX11Scan) -> ::windows_sys::core::HRESULT;
    pub fn D3DX11CreateSegmentedScan(pdevicecontext: ID3D11DeviceContext, maxelementscansize: u32, ppscan: *mut ID3DX11SegmentedScan) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
pub struct CD3D11_VIDEO_DEFAULT(pub u8);
pub const D3D11_16BIT_INDEX_STRIP_CUT_VALUE: u32 = 65535u32;
pub const D3D11_1_CREATE_DEVICE_CONTEXT_STATE_SINGLETHREADED: i32 = 1i32;
pub const D3D11_1_UAV_SLOT_COUNT: u32 = 64u32;
pub const D3D11_2_TILED_RESOURCE_TILE_SIZE_IN_BYTES: u32 = 65536u32;
pub const D3D11_32BIT_INDEX_STRIP_CUT_VALUE: u32 = 4294967295u32;
pub const D3D11_4_VIDEO_DECODER_HISTOGRAM_OFFSET_ALIGNMENT: u32 = 256u32;
pub const D3D11_4_VIDEO_DECODER_MAX_HISTOGRAM_COMPONENTS: u32 = 4u32;
pub const D3D11_8BIT_INDEX_STRIP_CUT_VALUE: u32 = 255u32;
#[repr(C)]
pub struct D3D11_AES_CTR_IV {
    pub IV: u64,
    pub Count: u64,
}
impl ::core::marker::Copy for D3D11_AES_CTR_IV {}
impl ::core::clone::Clone for D3D11_AES_CTR_IV {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_ANISOTROPIC_FILTERING_BIT: u32 = 64u32;
pub const D3D11_APPEND_ALIGNED_ELEMENT: u32 = 4294967295u32;
pub const D3D11_ARRAY_AXIS_ADDRESS_RANGE_BIT_COUNT: u32 = 9u32;
pub const D3D11_ASYNC_GETDATA_DONOTFLUSH: i32 = 1i32;
pub const D3D11_AUTHENTICATED_CHANNEL_D3D11: i32 = 1i32;
pub const D3D11_AUTHENTICATED_CHANNEL_DRIVER_SOFTWARE: i32 = 2i32;
pub const D3D11_AUTHENTICATED_CHANNEL_DRIVER_HARDWARE: i32 = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub EncryptionGuid: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1665584212, data2: 11516, data3: 19156, data4: [130, 36, 209, 88, 55, 222, 119, 0] };
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub DecoderHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub DeviceHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_AUTHENTICATED_CONFIGURE_ENCRYPTION_WHEN_ACCESSIBLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1107292806,
    data2: 27360,
    data3: 19779,
    data4: [157, 85, 164, 110, 158, 253, 21, 138],
};
pub const D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 101796827,
    data2: 13603,
    data3: 18186,
    data4: [141, 202, 251, 194, 132, 81, 84, 240],
};
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub StartSequenceQuery: u32,
    pub StartSequenceConfigure: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_CONFIGURE_INPUT {
    pub omac: D3D11_OMAC,
    pub ConfigureType: ::windows_sys::core::GUID,
    pub hChannel: super::super::Foundation::HANDLE,
    pub SequenceNumber: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_CONFIGURE_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_CONFIGURE_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_CONFIGURE_OUTPUT {
    pub omac: D3D11_OMAC,
    pub ConfigureType: ::windows_sys::core::GUID,
    pub hChannel: super::super::Foundation::HANDLE,
    pub SequenceNumber: u32,
    pub ReturnCode: ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_CONFIGURE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_CONFIGURE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_AUTHENTICATED_CONFIGURE_PROTECTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1346721368,
    data2: 16199,
    data3: 17250,
    data4: [191, 153, 191, 223, 205, 233, 237, 41],
};
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub Protections: D3D11_AUTHENTICATED_PROTECTION_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_CONFIGURE_PROTECTION_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 124964935, data2: 6976, data3: 18664, data4: [156, 166, 181, 245, 16, 222, 159, 1] };
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT {
    pub Parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,
    pub ProcessType: D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE,
    pub ProcessHandle: super::super::Foundation::HANDLE,
    pub AllowAccess: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_CONFIGURE_SHARED_RESOURCE_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_PROCESSIDTYPE_UNKNOWN: i32 = 0i32;
pub const D3D11_PROCESSIDTYPE_DWM: i32 = 1i32;
pub const D3D11_PROCESSIDTYPE_HANDLE: i32 = 2i32;
#[repr(C)]
pub union D3D11_AUTHENTICATED_PROTECTION_FLAGS {
    pub Flags: D3D11_AUTHENTICATED_PROTECTION_FLAGS_0,
    pub Value: u32,
}
impl ::core::marker::Copy for D3D11_AUTHENTICATED_PROTECTION_FLAGS {}
impl ::core::clone::Clone for D3D11_AUTHENTICATED_PROTECTION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_AUTHENTICATED_PROTECTION_FLAGS_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for D3D11_AUTHENTICATED_PROTECTION_FLAGS_0 {}
impl ::core::clone::Clone for D3D11_AUTHENTICATED_PROTECTION_FLAGS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ATTRIBUTES: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1645533650,
    data2: 17196,
    data3: 19131,
    data4: [159, 206, 33, 110, 234, 38, 158, 59],
};
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub EncryptionGuidCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub EncryptionGuidIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub EncryptionGuidIndex: u32,
    pub EncryptionGuid: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub BusType: D3D11_BUS_TYPE,
    pub AccessibleInContiguousBlocks: super::super::Foundation::BOOL,
    pub AccessibleInNonContiguousBlocks: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3155892389,
    data2: 45563,
    data3: 17067,
    data4: [189, 148, 181, 130, 139, 75, 247, 190],
};
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub ChannelType: D3D11_AUTHENTICATED_CHANNEL_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 640960926, data2: 53272, data3: 19828, data4: [172, 23, 127, 114, 64, 89, 82, 141] };
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub DecoderHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub DecoderHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub DeviceHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub EncryptionGuid: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_QUERY_CURRENT_ACCESSIBILITY_ENCRYPTION_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_AUTHENTICATED_QUERY_CURRENT_ENCRYPTION_WHEN_ACCESSIBLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3960967623,
    data2: 56019,
    data3: 20245,
    data4: [158, 195, 250, 169, 61, 96, 212, 240],
};
pub const D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3961279389,
    data2: 36095,
    data3: 20010,
    data4: [188, 196, 245, 105, 47, 153, 244, 128],
};
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4164573528, data2: 59782, data3: 19418, data4: [190, 176, 65, 31, 106, 122, 1, 183] };
pub const D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID_COUNT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3004133478, data2: 8252, data3: 19207, data4: [147, 252, 206, 170, 253, 97, 36, 30] };
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_QUERY_INPUT {
    pub QueryType: ::windows_sys::core::GUID,
    pub hChannel: super::super::Foundation::HANDLE,
    pub SequenceNumber: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_QUERY_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_QUERY_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT {
    pub omac: D3D11_OMAC,
    pub QueryType: ::windows_sys::core::GUID,
    pub hChannel: super::super::Foundation::HANDLE,
    pub SequenceNumber: u32,
    pub ReturnCode: ::windows_sys::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_QUERY_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_QUERY_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_AUTHENTICATED_QUERY_OUTPUT_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2208160931,
    data2: 39758,
    data3: 16868,
    data4: [176, 83, 137, 43, 210, 161, 30, 231],
};
pub const D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 738470750,
    data2: 35847,
    data3: 18133,
    data4: [170, 190, 143, 117, 203, 173, 76, 49],
};
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub OutputIDCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_COUNT_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub OutputIDIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub DeviceHandle: super::super::Foundation::HANDLE,
    pub CryptoSessionHandle: super::super::Foundation::HANDLE,
    pub OutputIDIndex: u32,
    pub OutputID: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_AUTHENTICATED_QUERY_PROTECTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2823730564,
    data2: 50325,
    data3: 18602,
    data4: [185, 77, 139, 210, 214, 251, 206, 5],
};
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub ProtectionFlags: D3D11_AUTHENTICATED_PROTECTION_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1687927515, data2: 61684, data3: 17977, data4: [161, 91, 36, 57, 63, 195, 171, 172] };
pub const D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 229771187,
    data2: 37968,
    data3: 18086,
    data4: [130, 222, 27, 150, 212, 79, 156, 242],
};
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub RestrictedSharedResourceProcessCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT {
    pub Input: D3D11_AUTHENTICATED_QUERY_INPUT,
    pub ProcessIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub ProcessIndex: u32,
    pub ProcessIdentifier: D3D11_AUTHENTICATED_PROCESS_IDENTIFIER_TYPE,
    pub ProcessHandle: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 19860438, data2: 58978, data3: 17524, data4: [190, 253, 170, 83, 229, 20, 60, 109] };
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT_OUTPUT {
    pub Output: D3D11_AUTHENTICATED_QUERY_OUTPUT,
    pub UnrestrictedProtectedSharedResourceCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_AUTHENTICATED_QUERY_UNRESTRICTED_PROTECTED_SHARED_RESOURCE_COUNT_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_BIND_VERTEX_BUFFER: u32 = 1u32;
pub const D3D11_BIND_INDEX_BUFFER: u32 = 2u32;
pub const D3D11_BIND_CONSTANT_BUFFER: u32 = 4u32;
pub const D3D11_BIND_SHADER_RESOURCE: u32 = 8u32;
pub const D3D11_BIND_STREAM_OUTPUT: u32 = 16u32;
pub const D3D11_BIND_RENDER_TARGET: u32 = 32u32;
pub const D3D11_BIND_DEPTH_STENCIL: u32 = 64u32;
pub const D3D11_BIND_UNORDERED_ACCESS: u32 = 128u32;
pub const D3D11_BIND_DECODER: u32 = 512u32;
pub const D3D11_BIND_VIDEO_ENCODER: u32 = 1024u32;
pub const D3D11_BLEND_ZERO: i32 = 1i32;
pub const D3D11_BLEND_ONE: i32 = 2i32;
pub const D3D11_BLEND_SRC_COLOR: i32 = 3i32;
pub const D3D11_BLEND_INV_SRC_COLOR: i32 = 4i32;
pub const D3D11_BLEND_SRC_ALPHA: i32 = 5i32;
pub const D3D11_BLEND_INV_SRC_ALPHA: i32 = 6i32;
pub const D3D11_BLEND_DEST_ALPHA: i32 = 7i32;
pub const D3D11_BLEND_INV_DEST_ALPHA: i32 = 8i32;
pub const D3D11_BLEND_DEST_COLOR: i32 = 9i32;
pub const D3D11_BLEND_INV_DEST_COLOR: i32 = 10i32;
pub const D3D11_BLEND_SRC_ALPHA_SAT: i32 = 11i32;
pub const D3D11_BLEND_BLEND_FACTOR: i32 = 14i32;
pub const D3D11_BLEND_INV_BLEND_FACTOR: i32 = 15i32;
pub const D3D11_BLEND_SRC1_COLOR: i32 = 16i32;
pub const D3D11_BLEND_INV_SRC1_COLOR: i32 = 17i32;
pub const D3D11_BLEND_SRC1_ALPHA: i32 = 18i32;
pub const D3D11_BLEND_INV_SRC1_ALPHA: i32 = 19i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_BLEND_DESC {
    pub AlphaToCoverageEnable: super::super::Foundation::BOOL,
    pub IndependentBlendEnable: super::super::Foundation::BOOL,
    pub RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC; 8],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_BLEND_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_BLEND_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_BLEND_DESC1 {
    pub AlphaToCoverageEnable: super::super::Foundation::BOOL,
    pub IndependentBlendEnable: super::super::Foundation::BOOL,
    pub RenderTarget: [D3D11_RENDER_TARGET_BLEND_DESC1; 8],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_BLEND_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_BLEND_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_BLEND_OP_ADD: i32 = 1i32;
pub const D3D11_BLEND_OP_SUBTRACT: i32 = 2i32;
pub const D3D11_BLEND_OP_REV_SUBTRACT: i32 = 3i32;
pub const D3D11_BLEND_OP_MIN: i32 = 4i32;
pub const D3D11_BLEND_OP_MAX: i32 = 5i32;
#[repr(C)]
pub struct D3D11_BOX {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}
impl ::core::marker::Copy for D3D11_BOX {}
impl ::core::clone::Clone for D3D11_BOX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_BUFFEREX_SRV {
    pub FirstElement: u32,
    pub NumElements: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for D3D11_BUFFEREX_SRV {}
impl ::core::clone::Clone for D3D11_BUFFEREX_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_BUFFEREX_SRV_FLAG_RAW: i32 = 1i32;
#[repr(C)]
pub struct D3D11_BUFFER_DESC {
    pub ByteWidth: u32,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
    pub StructureByteStride: u32,
}
impl ::core::marker::Copy for D3D11_BUFFER_DESC {}
impl ::core::clone::Clone for D3D11_BUFFER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_BUFFER_RTV {
    pub Anonymous1: D3D11_BUFFER_RTV_0,
    pub Anonymous2: D3D11_BUFFER_RTV_1,
}
impl ::core::marker::Copy for D3D11_BUFFER_RTV {}
impl ::core::clone::Clone for D3D11_BUFFER_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D3D11_BUFFER_RTV_0 {
    pub FirstElement: u32,
    pub ElementOffset: u32,
}
impl ::core::marker::Copy for D3D11_BUFFER_RTV_0 {}
impl ::core::clone::Clone for D3D11_BUFFER_RTV_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D3D11_BUFFER_RTV_1 {
    pub NumElements: u32,
    pub ElementWidth: u32,
}
impl ::core::marker::Copy for D3D11_BUFFER_RTV_1 {}
impl ::core::clone::Clone for D3D11_BUFFER_RTV_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_BUFFER_SRV {
    pub Anonymous1: D3D11_BUFFER_SRV_0,
    pub Anonymous2: D3D11_BUFFER_SRV_1,
}
impl ::core::marker::Copy for D3D11_BUFFER_SRV {}
impl ::core::clone::Clone for D3D11_BUFFER_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D3D11_BUFFER_SRV_0 {
    pub FirstElement: u32,
    pub ElementOffset: u32,
}
impl ::core::marker::Copy for D3D11_BUFFER_SRV_0 {}
impl ::core::clone::Clone for D3D11_BUFFER_SRV_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D3D11_BUFFER_SRV_1 {
    pub NumElements: u32,
    pub ElementWidth: u32,
}
impl ::core::marker::Copy for D3D11_BUFFER_SRV_1 {}
impl ::core::clone::Clone for D3D11_BUFFER_SRV_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_BUFFER_UAV {
    pub FirstElement: u32,
    pub NumElements: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for D3D11_BUFFER_UAV {}
impl ::core::clone::Clone for D3D11_BUFFER_UAV {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_BUFFER_UAV_FLAG_RAW: i32 = 1i32;
pub const D3D11_BUFFER_UAV_FLAG_APPEND: i32 = 2i32;
pub const D3D11_BUFFER_UAV_FLAG_COUNTER: i32 = 4i32;
pub const D3D11_BUS_TYPE_OTHER: i32 = 0i32;
pub const D3D11_BUS_TYPE_PCI: i32 = 1i32;
pub const D3D11_BUS_TYPE_PCIX: i32 = 2i32;
pub const D3D11_BUS_TYPE_PCIEXPRESS: i32 = 3i32;
pub const D3D11_BUS_TYPE_AGP: i32 = 4i32;
pub const D3D11_BUS_IMPL_MODIFIER_INSIDE_OF_CHIPSET: i32 = 65536i32;
pub const D3D11_BUS_IMPL_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_CHIP: i32 = 131072i32;
pub const D3D11_BUS_IMPL_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_SOCKET: i32 = 196608i32;
pub const D3D11_BUS_IMPL_MODIFIER_DAUGHTER_BOARD_CONNECTOR: i32 = 262144i32;
pub const D3D11_BUS_IMPL_MODIFIER_DAUGHTER_BOARD_CONNECTOR_INSIDE_OF_NUAE: i32 = 327680i32;
pub const D3D11_BUS_IMPL_MODIFIER_NON_STANDARD: i32 = -2147483648i32;
pub const D3D11_CHECK_MULTISAMPLE_QUALITY_LEVELS_TILED_RESOURCE: i32 = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_CLASS_INSTANCE_DESC {
    pub InstanceId: u32,
    pub InstanceIndex: u32,
    pub TypeId: u32,
    pub ConstantBuffer: u32,
    pub BaseConstantBufferOffset: u32,
    pub BaseTexture: u32,
    pub BaseSampler: u32,
    pub Created: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_CLASS_INSTANCE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_CLASS_INSTANCE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_CLEAR_DEPTH: i32 = 1i32;
pub const D3D11_CLEAR_STENCIL: i32 = 2i32;
pub const D3D11_CLIP_OR_CULL_DISTANCE_COUNT: u32 = 8u32;
pub const D3D11_CLIP_OR_CULL_DISTANCE_ELEMENT_COUNT: u32 = 2u32;
pub const D3D11_COLOR_WRITE_ENABLE_RED: i32 = 1i32;
pub const D3D11_COLOR_WRITE_ENABLE_GREEN: i32 = 2i32;
pub const D3D11_COLOR_WRITE_ENABLE_BLUE: i32 = 4i32;
pub const D3D11_COLOR_WRITE_ENABLE_ALPHA: i32 = 8i32;
pub const D3D11_COLOR_WRITE_ENABLE_ALL: i32 = 15i32;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT: u32 = 14u32;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_COMPONENTS: u32 = 4u32;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_HW_SLOT_COUNT: u32 = 15u32;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_PARTIAL_UPDATE_EXTENTS_BYTE_ALIGNMENT: u32 = 16u32;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 15u32;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_COMMONSHADER_FLOWCONTROL_NESTING_LIMIT: u32 = 64u32;
pub const D3D11_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D11_COMMONSHADER_IMMEDIATE_CONSTANT_BUFFER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_COMMONSHADER_IMMEDIATE_VALUE_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_REGISTER_COUNT: u32 = 128u32;
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT: u32 = 128u32;
pub const D3D11_COMMONSHADER_SAMPLER_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_COMMONSHADER_SAMPLER_REGISTER_COUNT: u32 = 16u32;
pub const D3D11_COMMONSHADER_SAMPLER_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D11_COMMONSHADER_SAMPLER_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT: u32 = 16u32;
pub const D3D11_COMMONSHADER_SUBROUTINE_NESTING_LIMIT: u32 = 32u32;
pub const D3D11_COMMONSHADER_TEMP_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_COMMONSHADER_TEMP_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_COMMONSHADER_TEMP_REGISTER_COUNT: u32 = 4096u32;
pub const D3D11_COMMONSHADER_TEMP_REGISTER_READS_PER_INST: u32 = 3u32;
pub const D3D11_COMMONSHADER_TEMP_REGISTER_READ_PORTS: u32 = 3u32;
pub const D3D11_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MAX: u32 = 10u32;
pub const D3D11_COMMONSHADER_TEXCOORD_RANGE_REDUCTION_MIN: i32 = -10i32;
pub const D3D11_COMMONSHADER_TEXEL_OFFSET_MAX_NEGATIVE: i32 = -8i32;
pub const D3D11_COMMONSHADER_TEXEL_OFFSET_MAX_POSITIVE: u32 = 7u32;
pub const D3D11_COMPARISON_FILTERING_BIT: u32 = 128u32;
pub const D3D11_COMPARISON_NEVER: i32 = 1i32;
pub const D3D11_COMPARISON_LESS: i32 = 2i32;
pub const D3D11_COMPARISON_EQUAL: i32 = 3i32;
pub const D3D11_COMPARISON_LESS_EQUAL: i32 = 4i32;
pub const D3D11_COMPARISON_GREATER: i32 = 5i32;
pub const D3D11_COMPARISON_NOT_EQUAL: i32 = 6i32;
pub const D3D11_COMPARISON_GREATER_EQUAL: i32 = 7i32;
pub const D3D11_COMPARISON_ALWAYS: i32 = 8i32;
#[repr(C)]
pub struct D3D11_COMPUTE_SHADER_TRACE_DESC {
    pub Invocation: u64,
    pub ThreadIDInGroup: [u32; 3],
    pub ThreadGroupID: [u32; 3],
}
impl ::core::marker::Copy for D3D11_COMPUTE_SHADER_TRACE_DESC {}
impl ::core::clone::Clone for D3D11_COMPUTE_SHADER_TRACE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_CONSERVATIVE_RASTERIZATION_MODE_OFF: i32 = 0i32;
pub const D3D11_CONSERVATIVE_RASTERIZATION_MODE_ON: i32 = 1i32;
pub const D3D11_CONSERVATIVE_RASTERIZATION_NOT_SUPPORTED: i32 = 0i32;
pub const D3D11_CONSERVATIVE_RASTERIZATION_TIER_1: i32 = 1i32;
pub const D3D11_CONSERVATIVE_RASTERIZATION_TIER_2: i32 = 2i32;
pub const D3D11_CONSERVATIVE_RASTERIZATION_TIER_3: i32 = 3i32;
pub const D3D11_CONTENT_PROTECTION_CAPS_SOFTWARE: i32 = 1i32;
pub const D3D11_CONTENT_PROTECTION_CAPS_HARDWARE: i32 = 2i32;
pub const D3D11_CONTENT_PROTECTION_CAPS_PROTECTION_ALWAYS_ON: i32 = 4i32;
pub const D3D11_CONTENT_PROTECTION_CAPS_PARTIAL_DECRYPTION: i32 = 8i32;
pub const D3D11_CONTENT_PROTECTION_CAPS_CONTENT_KEY: i32 = 16i32;
pub const D3D11_CONTENT_PROTECTION_CAPS_FRESHEN_SESSION_KEY: i32 = 32i32;
pub const D3D11_CONTENT_PROTECTION_CAPS_ENCRYPTED_READ_BACK: i32 = 64i32;
pub const D3D11_CONTENT_PROTECTION_CAPS_ENCRYPTED_READ_BACK_KEY: i32 = 128i32;
pub const D3D11_CONTENT_PROTECTION_CAPS_SEQUENTIAL_CTR_IV: i32 = 256i32;
pub const D3D11_CONTENT_PROTECTION_CAPS_ENCRYPT_SLICEDATA_ONLY: i32 = 512i32;
pub const D3D11_CONTENT_PROTECTION_CAPS_DECRYPTION_BLT: i32 = 1024i32;
pub const D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_PROTECT_UNCOMPRESSED: i32 = 2048i32;
pub const D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_PROTECTED_MEMORY_PAGEABLE: i32 = 4096i32;
pub const D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_TEARDOWN: i32 = 8192i32;
pub const D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_DRM_COMMUNICATION: i32 = 16384i32;
pub const D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_DRM_COMMUNICATION_MULTI_THREADED: i32 = 32768i32;
pub const D3D11_CONTEXT_TYPE_ALL: i32 = 0i32;
pub const D3D11_CONTEXT_TYPE_3D: i32 = 1i32;
pub const D3D11_CONTEXT_TYPE_COMPUTE: i32 = 2i32;
pub const D3D11_CONTEXT_TYPE_COPY: i32 = 3i32;
pub const D3D11_CONTEXT_TYPE_VIDEO: i32 = 4i32;
pub const D3D11_COPY_NO_OVERWRITE: i32 = 1i32;
pub const D3D11_COPY_DISCARD: i32 = 2i32;
pub const D3D11_COUNTER_DEVICE_DEPENDENT_0: i32 = 1073741824i32;
#[repr(C)]
pub struct D3D11_COUNTER_DESC {
    pub Counter: D3D11_COUNTER,
    pub MiscFlags: u32,
}
impl ::core::marker::Copy for D3D11_COUNTER_DESC {}
impl ::core::clone::Clone for D3D11_COUNTER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_COUNTER_INFO {
    pub LastDeviceDependentCounter: D3D11_COUNTER,
    pub NumSimultaneousCounters: u32,
    pub NumDetectableParallelUnits: u8,
}
impl ::core::marker::Copy for D3D11_COUNTER_INFO {}
impl ::core::clone::Clone for D3D11_COUNTER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_COUNTER_TYPE_FLOAT32: i32 = 0i32;
pub const D3D11_COUNTER_TYPE_UINT16: i32 = 1i32;
pub const D3D11_COUNTER_TYPE_UINT32: i32 = 2i32;
pub const D3D11_COUNTER_TYPE_UINT64: i32 = 3i32;
pub const D3D11_CPU_ACCESS_WRITE: u32 = 65536u32;
pub const D3D11_CPU_ACCESS_READ: u32 = 131072u32;
pub const D3D11_CREATE_DEVICE_SINGLETHREADED: u32 = 1u32;
pub const D3D11_CREATE_DEVICE_DEBUG: u32 = 2u32;
pub const D3D11_CREATE_DEVICE_SWITCH_TO_REF: u32 = 4u32;
pub const D3D11_CREATE_DEVICE_PREVENT_INTERNAL_THREADING_OPTIMIZATIONS: u32 = 8u32;
pub const D3D11_CREATE_DEVICE_BGRA_SUPPORT: u32 = 32u32;
pub const D3D11_CREATE_DEVICE_DEBUGGABLE: u32 = 64u32;
pub const D3D11_CREATE_DEVICE_PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY: u32 = 128u32;
pub const D3D11_CREATE_DEVICE_DISABLE_GPU_TIMEOUT: u32 = 256u32;
pub const D3D11_CREATE_DEVICE_VIDEO_SUPPORT: u32 = 2048u32;
pub const D3D11_CRYPTO_SESSION_KEY_EXCHANGE_FLAG_NONE: u32 = 0u32;
pub const D3D11_CRYPTO_SESSION_STATUS_OK: i32 = 0i32;
pub const D3D11_CRYPTO_SESSION_STATUS_KEY_LOST: i32 = 1i32;
pub const D3D11_CRYPTO_SESSION_STATUS_KEY_AND_CONTENT_LOST: i32 = 2i32;
pub const D3D11_CRYPTO_TYPE_AES128_CTR: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2607535889,
    data2: 20340,
    data3: 16841,
    data4: [158, 123, 11, 226, 215, 217, 59, 79],
};
pub const D3D11_CS_4_X_BUCKET00_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 256u32;
pub const D3D11_CS_4_X_BUCKET00_MAX_NUM_THREADS_PER_GROUP: u32 = 64u32;
pub const D3D11_CS_4_X_BUCKET01_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 240u32;
pub const D3D11_CS_4_X_BUCKET01_MAX_NUM_THREADS_PER_GROUP: u32 = 68u32;
pub const D3D11_CS_4_X_BUCKET02_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 224u32;
pub const D3D11_CS_4_X_BUCKET02_MAX_NUM_THREADS_PER_GROUP: u32 = 72u32;
pub const D3D11_CS_4_X_BUCKET03_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 208u32;
pub const D3D11_CS_4_X_BUCKET03_MAX_NUM_THREADS_PER_GROUP: u32 = 76u32;
pub const D3D11_CS_4_X_BUCKET04_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 192u32;
pub const D3D11_CS_4_X_BUCKET04_MAX_NUM_THREADS_PER_GROUP: u32 = 84u32;
pub const D3D11_CS_4_X_BUCKET05_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 176u32;
pub const D3D11_CS_4_X_BUCKET05_MAX_NUM_THREADS_PER_GROUP: u32 = 92u32;
pub const D3D11_CS_4_X_BUCKET06_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 160u32;
pub const D3D11_CS_4_X_BUCKET06_MAX_NUM_THREADS_PER_GROUP: u32 = 100u32;
pub const D3D11_CS_4_X_BUCKET07_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 144u32;
pub const D3D11_CS_4_X_BUCKET07_MAX_NUM_THREADS_PER_GROUP: u32 = 112u32;
pub const D3D11_CS_4_X_BUCKET08_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 128u32;
pub const D3D11_CS_4_X_BUCKET08_MAX_NUM_THREADS_PER_GROUP: u32 = 128u32;
pub const D3D11_CS_4_X_BUCKET09_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 112u32;
pub const D3D11_CS_4_X_BUCKET09_MAX_NUM_THREADS_PER_GROUP: u32 = 144u32;
pub const D3D11_CS_4_X_BUCKET10_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 96u32;
pub const D3D11_CS_4_X_BUCKET10_MAX_NUM_THREADS_PER_GROUP: u32 = 168u32;
pub const D3D11_CS_4_X_BUCKET11_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 80u32;
pub const D3D11_CS_4_X_BUCKET11_MAX_NUM_THREADS_PER_GROUP: u32 = 204u32;
pub const D3D11_CS_4_X_BUCKET12_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 64u32;
pub const D3D11_CS_4_X_BUCKET12_MAX_NUM_THREADS_PER_GROUP: u32 = 256u32;
pub const D3D11_CS_4_X_BUCKET13_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 48u32;
pub const D3D11_CS_4_X_BUCKET13_MAX_NUM_THREADS_PER_GROUP: u32 = 340u32;
pub const D3D11_CS_4_X_BUCKET14_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 32u32;
pub const D3D11_CS_4_X_BUCKET14_MAX_NUM_THREADS_PER_GROUP: u32 = 512u32;
pub const D3D11_CS_4_X_BUCKET15_MAX_BYTES_TGSM_WRITABLE_PER_THREAD: u32 = 16u32;
pub const D3D11_CS_4_X_BUCKET15_MAX_NUM_THREADS_PER_GROUP: u32 = 768u32;
pub const D3D11_CS_4_X_DISPATCH_MAX_THREAD_GROUPS_IN_Z_DIMENSION: u32 = 1u32;
pub const D3D11_CS_4_X_RAW_UAV_BYTE_ALIGNMENT: u32 = 256u32;
pub const D3D11_CS_4_X_THREAD_GROUP_MAX_THREADS_PER_GROUP: u32 = 768u32;
pub const D3D11_CS_4_X_THREAD_GROUP_MAX_X: u32 = 768u32;
pub const D3D11_CS_4_X_THREAD_GROUP_MAX_Y: u32 = 768u32;
pub const D3D11_CS_4_X_UAV_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_CS_DISPATCH_MAX_THREAD_GROUPS_PER_DIMENSION: u32 = 65535u32;
pub const D3D11_CS_TGSM_REGISTER_COUNT: u32 = 8192u32;
pub const D3D11_CS_TGSM_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D11_CS_TGSM_RESOURCE_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_CS_TGSM_RESOURCE_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_CS_THREADGROUPID_REGISTER_COMPONENTS: u32 = 3u32;
pub const D3D11_CS_THREADGROUPID_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_CS_THREADIDINGROUPFLATTENED_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_CS_THREADIDINGROUPFLATTENED_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_CS_THREADIDINGROUP_REGISTER_COMPONENTS: u32 = 3u32;
pub const D3D11_CS_THREADIDINGROUP_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_CS_THREADID_REGISTER_COMPONENTS: u32 = 3u32;
pub const D3D11_CS_THREADID_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_CS_THREAD_GROUP_MAX_THREADS_PER_GROUP: u32 = 1024u32;
pub const D3D11_CS_THREAD_GROUP_MAX_X: u32 = 1024u32;
pub const D3D11_CS_THREAD_GROUP_MAX_Y: u32 = 1024u32;
pub const D3D11_CS_THREAD_GROUP_MAX_Z: u32 = 64u32;
pub const D3D11_CS_THREAD_GROUP_MIN_X: u32 = 1u32;
pub const D3D11_CS_THREAD_GROUP_MIN_Y: u32 = 1u32;
pub const D3D11_CS_THREAD_GROUP_MIN_Z: u32 = 1u32;
pub const D3D11_CS_THREAD_LOCAL_TEMP_REGISTER_POOL: u32 = 16384u32;
pub const D3D11_CULL_NONE: i32 = 1i32;
pub const D3D11_CULL_FRONT: i32 = 2i32;
pub const D3D11_CULL_BACK: i32 = 3i32;
pub const D3D11_DEBUG_FEATURE_ALWAYS_DISCARD_OFFERED_RESOURCE: u32 = 8u32;
pub const D3D11_DEBUG_FEATURE_AVOID_BEHAVIOR_CHANGING_DEBUG_AIDS: u32 = 64u32;
pub const D3D11_DEBUG_FEATURE_DISABLE_TILED_RESOURCE_MAPPING_TRACKING_AND_VALIDATION: u32 = 128u32;
pub const D3D11_DEBUG_FEATURE_FINISH_PER_RENDER_OP: u32 = 2u32;
pub const D3D11_DEBUG_FEATURE_FLUSH_PER_RENDER_OP: u32 = 1u32;
pub const D3D11_DEBUG_FEATURE_NEVER_DISCARD_OFFERED_RESOURCE: u32 = 16u32;
pub const D3D11_DEBUG_FEATURE_PRESENT_PER_RENDER_OP: u32 = 4u32;
pub const D3D11_DECODER_BITSTREAM_ENCRYPTION_TYPE_CBCS: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1110283033, data2: 40225, data3: 19383, data4: [147, 113, 250, 245, 168, 44, 62, 4] };
pub const D3D11_DECODER_BITSTREAM_ENCRYPTION_TYPE_CENC: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2957005365,
    data2: 49469,
    data3: 17650,
    data4: [154, 229, 221, 72, 224, 142, 91, 103],
};
pub const D3D11_DECODER_ENCRYPTION_HW_CENC: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2312547407,
    data2: 2546,
    data3: 16937,
    data4: [178, 205, 55, 116, 10, 109, 253, 129],
};
pub const D3D11_DECODER_PROFILE_AV1_VLD_12BIT_PROFILE2: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 387084297,
    data2: 40975,
    data3: 19681,
    data4: [153, 78, 191, 64, 129, 246, 243, 240],
};
pub const D3D11_DECODER_PROFILE_AV1_VLD_12BIT_PROFILE2_420: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 763412182,
    data2: 40108,
    data3: 18485,
    data4: [158, 145, 50, 123, 188, 79, 158, 232],
};
pub const D3D11_DECODER_PROFILE_AV1_VLD_PROFILE0: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3099479243,
    data2: 53075,
    data3: 18106,
    data4: [141, 89, 214, 184, 166, 218, 93, 42],
};
pub const D3D11_DECODER_PROFILE_AV1_VLD_PROFILE1: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1765211919,
    data2: 17841,
    data3: 16739,
    data4: [156, 193, 100, 110, 246, 148, 97, 8],
};
pub const D3D11_DECODER_PROFILE_AV1_VLD_PROFILE2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 207563425, data2: 58689, data3: 16521, data4: [187, 123, 152, 17, 10, 25, 215, 200] };
pub const D3D11_DECODER_PROFILE_H264_IDCT_FGT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 461487719, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_H264_IDCT_NOFGT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 461487718, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_H264_MOCOMP_FGT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 461487717, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_H264_MOCOMP_NOFGT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 461487716, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_H264_VLD_FGT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 461487721, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_H264_VLD_MULTIVIEW_NOFGT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1885052290,
    data2: 30415,
    data3: 18902,
    data4: [183, 230, 172, 136, 114, 219, 1, 60],
};
pub const D3D11_DECODER_PROFILE_H264_VLD_NOFGT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 461487720, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_H264_VLD_STEREO_NOFGT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4188720315, data2: 49846, data3: 19708, data4: [135, 121, 87, 7, 177, 118, 5, 82] };
pub const D3D11_DECODER_PROFILE_H264_VLD_STEREO_PROGRESSIVE_NOFGT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3617319130, data2: 3313, data3: 19585, data4: [184, 42, 105, 164, 226, 54, 244, 61] };
pub const D3D11_DECODER_PROFILE_H264_VLD_WITHFMOASO_NOFGT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3589296121,
    data2: 13336,
    data3: 17880,
    data4: [149, 97, 50, 167, 106, 174, 45, 221],
};
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1527895323, data2: 12108, data3: 17490, data4: [188, 195, 9, 242, 161, 22, 12, 192] };
pub const D3D11_DECODER_PROFILE_HEVC_VLD_MAIN10: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 276492512, data2: 61210, data3: 19737, data4: [171, 168, 103, 161, 99, 7, 61, 19] };
pub const D3D11_DECODER_PROFILE_MPEG1_VLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1866385177,
    data2: 14133,
    data3: 17100,
    data4: [128, 99, 101, 204, 60, 179, 102, 22],
};
pub const D3D11_DECODER_PROFILE_MPEG2_IDCT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3206720768, data2: 1002, data3: 18064, data4: [128, 119, 71, 51, 70, 32, 155, 126] };
pub const D3D11_DECODER_PROFILE_MPEG2_MOCOMP: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3869897803,
    data2: 25008,
    data3: 17763,
    data4: [158, 164, 99, 210, 163, 198, 254, 102],
};
pub const D3D11_DECODER_PROFILE_MPEG2_VLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3995550079, data2: 24104, data3: 20069, data4: [190, 234, 29, 38, 181, 8, 173, 201] };
pub const D3D11_DECODER_PROFILE_MPEG2and1_VLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2255052562,
    data2: 13326,
    data3: 20228,
    data4: [159, 211, 146, 83, 221, 50, 116, 96],
};
pub const D3D11_DECODER_PROFILE_MPEG4PT2_VLD_ADVSIMPLE_GMC: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2878966619,
    data2: 16984,
    data3: 17577,
    data4: [159, 235, 148, 229, 151, 166, 186, 174],
};
pub const D3D11_DECODER_PROFILE_MPEG4PT2_VLD_ADVSIMPLE_NOGMC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3980495519, data2: 269, data3: 20186, data4: [154, 227, 154, 101, 53, 141, 141, 46] };
pub const D3D11_DECODER_PROFILE_MPEG4PT2_VLD_SIMPLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4023799156,
    data2: 51688,
    data3: 16855,
    data4: [165, 233, 233, 176, 227, 159, 163, 25],
};
pub const D3D11_DECODER_PROFILE_VC1_D2010: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 461487780, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_VC1_IDCT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 461487778, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_VC1_MOCOMP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 461487777, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_VC1_POSTPROC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 461487776, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_VC1_VLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 461487779, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_VP8_VLD: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2428017130,
    data2: 14946,
    data3: 18181,
    data4: [136, 179, 141, 240, 75, 39, 68, 231],
};
pub const D3D11_DECODER_PROFILE_VP9_VLD_10BIT_PROFILE2: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2764524015, data2: 28367, data3: 18602, data4: [132, 72, 80, 167, 161, 22, 95, 247] };
pub const D3D11_DECODER_PROFILE_VP9_VLD_PROFILE0: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1178011640,
    data2: 41424,
    data3: 17797,
    data4: [135, 109, 131, 170, 109, 96, 184, 158],
};
pub const D3D11_DECODER_PROFILE_WMV8_MOCOMP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 461487745, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_WMV8_POSTPROC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 461487744, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_WMV9_IDCT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 461487764, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_WMV9_MOCOMP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 461487761, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DECODER_PROFILE_WMV9_POSTPROC: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 461487760, data2: 41159, data3: 4563, data4: [185, 132, 0, 192, 79, 46, 115, 197] };
pub const D3D11_DEFAULT_BLEND_FACTOR_ALPHA: f32 = 1f32;
pub const D3D11_DEFAULT_BLEND_FACTOR_BLUE: f32 = 1f32;
pub const D3D11_DEFAULT_BLEND_FACTOR_GREEN: f32 = 1f32;
pub const D3D11_DEFAULT_BLEND_FACTOR_RED: f32 = 1f32;
pub const D3D11_DEFAULT_BORDER_COLOR_COMPONENT: f32 = 0f32;
pub const D3D11_DEFAULT_DEPTH_BIAS: u32 = 0u32;
pub const D3D11_DEFAULT_DEPTH_BIAS_CLAMP: f32 = 0f32;
pub const D3D11_DEFAULT_MAX_ANISOTROPY: u32 = 16u32;
pub const D3D11_DEFAULT_MIP_LOD_BIAS: f32 = 0f32;
pub const D3D11_DEFAULT_RENDER_TARGET_ARRAY_INDEX: u32 = 0u32;
pub const D3D11_DEFAULT_SAMPLE_MASK: u32 = 4294967295u32;
pub const D3D11_DEFAULT_SCISSOR_ENDX: u32 = 0u32;
pub const D3D11_DEFAULT_SCISSOR_ENDY: u32 = 0u32;
pub const D3D11_DEFAULT_SCISSOR_STARTX: u32 = 0u32;
pub const D3D11_DEFAULT_SCISSOR_STARTY: u32 = 0u32;
pub const D3D11_DEFAULT_SLOPE_SCALED_DEPTH_BIAS: f32 = 0f32;
pub const D3D11_DEFAULT_STENCIL_READ_MASK: u32 = 255u32;
pub const D3D11_DEFAULT_STENCIL_REFERENCE: u32 = 0u32;
pub const D3D11_DEFAULT_STENCIL_WRITE_MASK: u32 = 255u32;
pub const D3D11_DEFAULT_VIEWPORT_AND_SCISSORRECT_INDEX: u32 = 0u32;
pub const D3D11_DEFAULT_VIEWPORT_HEIGHT: u32 = 0u32;
pub const D3D11_DEFAULT_VIEWPORT_MAX_DEPTH: f32 = 0f32;
pub const D3D11_DEFAULT_VIEWPORT_MIN_DEPTH: f32 = 0f32;
pub const D3D11_DEFAULT_VIEWPORT_TOPLEFTX: u32 = 0u32;
pub const D3D11_DEFAULT_VIEWPORT_TOPLEFTY: u32 = 0u32;
pub const D3D11_DEFAULT_VIEWPORT_WIDTH: u32 = 0u32;
#[repr(C)]
pub struct D3D11_DEPTH_STENCILOP_DESC {
    pub StencilFailOp: D3D11_STENCIL_OP,
    pub StencilDepthFailOp: D3D11_STENCIL_OP,
    pub StencilPassOp: D3D11_STENCIL_OP,
    pub StencilFunc: D3D11_COMPARISON_FUNC,
}
impl ::core::marker::Copy for D3D11_DEPTH_STENCILOP_DESC {}
impl ::core::clone::Clone for D3D11_DEPTH_STENCILOP_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_DEPTH_STENCIL_DESC {
    pub DepthEnable: super::super::Foundation::BOOL,
    pub DepthWriteMask: D3D11_DEPTH_WRITE_MASK,
    pub DepthFunc: D3D11_COMPARISON_FUNC,
    pub StencilEnable: super::super::Foundation::BOOL,
    pub StencilReadMask: u8,
    pub StencilWriteMask: u8,
    pub FrontFace: D3D11_DEPTH_STENCILOP_DESC,
    pub BackFace: D3D11_DEPTH_STENCILOP_DESC,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_DEPTH_STENCIL_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_DEPTH_STENCIL_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D11_DEPTH_STENCIL_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D11_DSV_DIMENSION,
    pub Flags: u32,
    pub Anonymous: D3D11_DEPTH_STENCIL_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_DEPTH_STENCIL_VIEW_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_DEPTH_STENCIL_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D11_DEPTH_STENCIL_VIEW_DESC_0 {
    pub Texture1D: D3D11_TEX1D_DSV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_DSV,
    pub Texture2D: D3D11_TEX2D_DSV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_DSV,
    pub Texture2DMS: D3D11_TEX2DMS_DSV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_DSV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_DEPTH_STENCIL_VIEW_DESC_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_DEPTH_STENCIL_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_DEPTH_WRITE_MASK_ZERO: i32 = 0i32;
pub const D3D11_DEPTH_WRITE_MASK_ALL: i32 = 1i32;
pub const D3D11_DEVICE_CONTEXT_IMMEDIATE: i32 = 0i32;
pub const D3D11_DEVICE_CONTEXT_DEFERRED: i32 = 1i32;
#[repr(C)]
pub struct D3D11_DOMAIN_SHADER_TRACE_DESC {
    pub Invocation: u64,
}
impl ::core::marker::Copy for D3D11_DOMAIN_SHADER_TRACE_DESC {}
impl ::core::clone::Clone for D3D11_DOMAIN_SHADER_TRACE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_DRAW_INDEXED_INSTANCED_INDIRECT_ARGS {
    pub IndexCountPerInstance: u32,
    pub InstanceCount: u32,
    pub StartIndexLocation: u32,
    pub BaseVertexLocation: i32,
    pub StartInstanceLocation: u32,
}
impl ::core::marker::Copy for D3D11_DRAW_INDEXED_INSTANCED_INDIRECT_ARGS {}
impl ::core::clone::Clone for D3D11_DRAW_INDEXED_INSTANCED_INDIRECT_ARGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_DRAW_INSTANCED_INDIRECT_ARGS {
    pub VertexCountPerInstance: u32,
    pub InstanceCount: u32,
    pub StartVertexLocation: u32,
    pub StartInstanceLocation: u32,
}
impl ::core::marker::Copy for D3D11_DRAW_INSTANCED_INDIRECT_ARGS {}
impl ::core::clone::Clone for D3D11_DRAW_INSTANCED_INDIRECT_ARGS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_DSV_DIMENSION_UNKNOWN: i32 = 0i32;
pub const D3D11_DSV_DIMENSION_TEXTURE1D: i32 = 1i32;
pub const D3D11_DSV_DIMENSION_TEXTURE1DARRAY: i32 = 2i32;
pub const D3D11_DSV_DIMENSION_TEXTURE2D: i32 = 3i32;
pub const D3D11_DSV_DIMENSION_TEXTURE2DARRAY: i32 = 4i32;
pub const D3D11_DSV_DIMENSION_TEXTURE2DMS: i32 = 5i32;
pub const D3D11_DSV_DIMENSION_TEXTURE2DMSARRAY: i32 = 6i32;
pub const D3D11_DSV_READ_ONLY_DEPTH: i32 = 1i32;
pub const D3D11_DSV_READ_ONLY_STENCIL: i32 = 2i32;
pub const D3D11_DS_INPUT_CONTROL_POINTS_MAX_TOTAL_SCALARS: u32 = 3968u32;
pub const D3D11_DS_INPUT_CONTROL_POINT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_DS_INPUT_CONTROL_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_DS_INPUT_CONTROL_POINT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_DS_INPUT_CONTROL_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_DS_INPUT_CONTROL_POINT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_DS_INPUT_DOMAIN_POINT_REGISTER_COMPONENTS: u32 = 3u32;
pub const D3D11_DS_INPUT_DOMAIN_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_DS_INPUT_DOMAIN_POINT_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_DS_INPUT_DOMAIN_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_DS_INPUT_DOMAIN_POINT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_DS_INPUT_PATCH_CONSTANT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_DS_INPUT_PATCH_CONSTANT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_DS_INPUT_PATCH_CONSTANT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_DS_INPUT_PATCH_CONSTANT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_DS_INPUT_PATCH_CONSTANT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_DS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_DS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_DS_INPUT_PRIMITIVE_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_DS_INPUT_PRIMITIVE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_DS_INPUT_PRIMITIVE_ID_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_DS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_DS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_DS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
#[repr(C)]
pub struct D3D11_ENCRYPTED_BLOCK_INFO {
    pub NumEncryptedBytesAtBeginning: u32,
    pub NumBytesInSkipPattern: u32,
    pub NumBytesInEncryptPattern: u32,
}
impl ::core::marker::Copy for D3D11_ENCRYPTED_BLOCK_INFO {}
impl ::core::clone::Clone for D3D11_ENCRYPTED_BLOCK_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_FEATURE_THREADING: i32 = 0i32;
pub const D3D11_FEATURE_DOUBLES: i32 = 1i32;
pub const D3D11_FEATURE_FORMAT_SUPPORT: i32 = 2i32;
pub const D3D11_FEATURE_FORMAT_SUPPORT2: i32 = 3i32;
pub const D3D11_FEATURE_D3D10_X_HARDWARE_OPTIONS: i32 = 4i32;
pub const D3D11_FEATURE_D3D11_OPTIONS: i32 = 5i32;
pub const D3D11_FEATURE_ARCHITECTURE_INFO: i32 = 6i32;
pub const D3D11_FEATURE_D3D9_OPTIONS: i32 = 7i32;
pub const D3D11_FEATURE_SHADER_MIN_PRECISION_SUPPORT: i32 = 8i32;
pub const D3D11_FEATURE_D3D9_SHADOW_SUPPORT: i32 = 9i32;
pub const D3D11_FEATURE_D3D11_OPTIONS1: i32 = 10i32;
pub const D3D11_FEATURE_D3D9_SIMPLE_INSTANCING_SUPPORT: i32 = 11i32;
pub const D3D11_FEATURE_MARKER_SUPPORT: i32 = 12i32;
pub const D3D11_FEATURE_D3D9_OPTIONS1: i32 = 13i32;
pub const D3D11_FEATURE_D3D11_OPTIONS2: i32 = 14i32;
pub const D3D11_FEATURE_D3D11_OPTIONS3: i32 = 15i32;
pub const D3D11_FEATURE_GPU_VIRTUAL_ADDRESS_SUPPORT: i32 = 16i32;
pub const D3D11_FEATURE_D3D11_OPTIONS4: i32 = 17i32;
pub const D3D11_FEATURE_SHADER_CACHE: i32 = 18i32;
pub const D3D11_FEATURE_D3D11_OPTIONS5: i32 = 19i32;
pub const D3D11_FEATURE_DISPLAYABLE: i32 = 20i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_FEATURE_DATA_ARCHITECTURE_INFO {
    pub TileBasedDeferredRenderer: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_FEATURE_DATA_ARCHITECTURE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_FEATURE_DATA_ARCHITECTURE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    pub ComputeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS {
    pub OutputMergerLogicOp: super::super::Foundation::BOOL,
    pub UAVOnlyRenderingForcedSampleCount: super::super::Foundation::BOOL,
    pub DiscardAPIsSeenByDriver: super::super::Foundation::BOOL,
    pub FlagsForUpdateAndCopySeenByDriver: super::super::Foundation::BOOL,
    pub ClearView: super::super::Foundation::BOOL,
    pub CopyWithOverlap: super::super::Foundation::BOOL,
    pub ConstantBufferPartialUpdate: super::super::Foundation::BOOL,
    pub ConstantBufferOffsetting: super::super::Foundation::BOOL,
    pub MapNoOverwriteOnDynamicConstantBuffer: super::super::Foundation::BOOL,
    pub MapNoOverwriteOnDynamicBufferSRV: super::super::Foundation::BOOL,
    pub MultisampleRTVWithForcedSampleCountOne: super::super::Foundation::BOOL,
    pub SAD4ShaderInstructions: super::super::Foundation::BOOL,
    pub ExtendedDoublesShaderInstructions: super::super::Foundation::BOOL,
    pub ExtendedResourceSharing: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_FEATURE_DATA_D3D11_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_FEATURE_DATA_D3D11_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS1 {
    pub TiledResourcesTier: D3D11_TILED_RESOURCES_TIER,
    pub MinMaxFiltering: super::super::Foundation::BOOL,
    pub ClearViewAlsoSupportsDepthOnlyFormats: super::super::Foundation::BOOL,
    pub MapOnDefaultBuffers: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_FEATURE_DATA_D3D11_OPTIONS1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_FEATURE_DATA_D3D11_OPTIONS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS2 {
    pub PSSpecifiedStencilRefSupported: super::super::Foundation::BOOL,
    pub TypedUAVLoadAdditionalFormats: super::super::Foundation::BOOL,
    pub ROVsSupported: super::super::Foundation::BOOL,
    pub ConservativeRasterizationTier: D3D11_CONSERVATIVE_RASTERIZATION_TIER,
    pub TiledResourcesTier: D3D11_TILED_RESOURCES_TIER,
    pub MapOnDefaultTextures: super::super::Foundation::BOOL,
    pub StandardSwizzle: super::super::Foundation::BOOL,
    pub UnifiedMemoryArchitecture: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_FEATURE_DATA_D3D11_OPTIONS2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_FEATURE_DATA_D3D11_OPTIONS2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS3 {
    pub VPAndRTArrayIndexFromAnyShaderFeedingRasterizer: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_FEATURE_DATA_D3D11_OPTIONS3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_FEATURE_DATA_D3D11_OPTIONS3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS4 {
    pub ExtendedNV12SharedTextureSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_FEATURE_DATA_D3D11_OPTIONS4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_FEATURE_DATA_D3D11_OPTIONS4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS5 {
    pub SharedResourceTier: D3D11_SHARED_RESOURCE_TIER,
}
impl ::core::marker::Copy for D3D11_FEATURE_DATA_D3D11_OPTIONS5 {}
impl ::core::clone::Clone for D3D11_FEATURE_DATA_D3D11_OPTIONS5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_FEATURE_DATA_D3D9_OPTIONS {
    pub FullNonPow2TextureSupport: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_FEATURE_DATA_D3D9_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_FEATURE_DATA_D3D9_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_FEATURE_DATA_D3D9_OPTIONS1 {
    pub FullNonPow2TextureSupported: super::super::Foundation::BOOL,
    pub DepthAsTextureWithLessEqualComparisonFilterSupported: super::super::Foundation::BOOL,
    pub SimpleInstancingSupported: super::super::Foundation::BOOL,
    pub TextureCubeFaceRenderTargetWithNonCubeDepthStencilSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_FEATURE_DATA_D3D9_OPTIONS1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_FEATURE_DATA_D3D9_OPTIONS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT {
    pub SupportsDepthAsTextureWithLessEqualComparisonFilter: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT {
    pub SimpleInstancingSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_FEATURE_DATA_DISPLAYABLE {
    pub DisplayableTexture: super::super::Foundation::BOOL,
    pub SharedResourceTier: D3D11_SHARED_RESOURCE_TIER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_FEATURE_DATA_DISPLAYABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_FEATURE_DATA_DISPLAYABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_FEATURE_DATA_DOUBLES {
    pub DoublePrecisionFloatShaderOps: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_FEATURE_DATA_DOUBLES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_FEATURE_DATA_DOUBLES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D11_FEATURE_DATA_FORMAT_SUPPORT {
    pub InFormat: super::Dxgi::Common::DXGI_FORMAT,
    pub OutFormatSupport: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_FEATURE_DATA_FORMAT_SUPPORT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_FEATURE_DATA_FORMAT_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D11_FEATURE_DATA_FORMAT_SUPPORT2 {
    pub InFormat: super::Dxgi::Common::DXGI_FORMAT,
    pub OutFormatSupport2: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_FEATURE_DATA_FORMAT_SUPPORT2 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_FEATURE_DATA_FORMAT_SUPPORT2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    pub MaxGPUVirtualAddressBitsPerResource: u32,
    pub MaxGPUVirtualAddressBitsPerProcess: u32,
}
impl ::core::marker::Copy for D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {}
impl ::core::clone::Clone for D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_FEATURE_DATA_MARKER_SUPPORT {
    pub Profile: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_FEATURE_DATA_MARKER_SUPPORT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_FEATURE_DATA_MARKER_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_FEATURE_DATA_SHADER_CACHE {
    pub SupportFlags: u32,
}
impl ::core::marker::Copy for D3D11_FEATURE_DATA_SHADER_CACHE {}
impl ::core::clone::Clone for D3D11_FEATURE_DATA_SHADER_CACHE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT {
    pub PixelShaderMinPrecision: u32,
    pub AllOtherShaderStagesMinPrecision: u32,
}
impl ::core::marker::Copy for D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT {}
impl ::core::clone::Clone for D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_FEATURE_DATA_THREADING {
    pub DriverConcurrentCreates: super::super::Foundation::BOOL,
    pub DriverCommandLists: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_FEATURE_DATA_THREADING {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_FEATURE_DATA_THREADING {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D11_FEATURE_DATA_VIDEO_DECODER_HISTOGRAM {
    pub DecoderDesc: D3D11_VIDEO_DECODER_DESC,
    pub Components: D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAGS,
    pub BinCount: u32,
    pub CounterBitDepth: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_FEATURE_DATA_VIDEO_DECODER_HISTOGRAM {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_FEATURE_DATA_VIDEO_DECODER_HISTOGRAM {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_FEATURE_VIDEO_DECODER_HISTOGRAM: i32 = 0i32;
pub const D3D11_FENCE_FLAG_NONE: u32 = 0u32;
pub const D3D11_FENCE_FLAG_SHARED: u32 = 2u32;
pub const D3D11_FENCE_FLAG_SHARED_CROSS_ADAPTER: u32 = 4u32;
pub const D3D11_FENCE_FLAG_NON_MONITORED: u32 = 8u32;
pub const D3D11_FILL_WIREFRAME: i32 = 2i32;
pub const D3D11_FILL_SOLID: i32 = 3i32;
pub const D3D11_FILTER_MIN_MAG_MIP_POINT: i32 = 0i32;
pub const D3D11_FILTER_MIN_MAG_POINT_MIP_LINEAR: i32 = 1i32;
pub const D3D11_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT: i32 = 4i32;
pub const D3D11_FILTER_MIN_POINT_MAG_MIP_LINEAR: i32 = 5i32;
pub const D3D11_FILTER_MIN_LINEAR_MAG_MIP_POINT: i32 = 16i32;
pub const D3D11_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR: i32 = 17i32;
pub const D3D11_FILTER_MIN_MAG_LINEAR_MIP_POINT: i32 = 20i32;
pub const D3D11_FILTER_MIN_MAG_MIP_LINEAR: i32 = 21i32;
pub const D3D11_FILTER_ANISOTROPIC: i32 = 85i32;
pub const D3D11_FILTER_COMPARISON_MIN_MAG_MIP_POINT: i32 = 128i32;
pub const D3D11_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR: i32 = 129i32;
pub const D3D11_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT: i32 = 132i32;
pub const D3D11_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR: i32 = 133i32;
pub const D3D11_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT: i32 = 144i32;
pub const D3D11_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR: i32 = 145i32;
pub const D3D11_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT: i32 = 148i32;
pub const D3D11_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR: i32 = 149i32;
pub const D3D11_FILTER_COMPARISON_ANISOTROPIC: i32 = 213i32;
pub const D3D11_FILTER_MINIMUM_MIN_MAG_MIP_POINT: i32 = 256i32;
pub const D3D11_FILTER_MINIMUM_MIN_MAG_POINT_MIP_LINEAR: i32 = 257i32;
pub const D3D11_FILTER_MINIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT: i32 = 260i32;
pub const D3D11_FILTER_MINIMUM_MIN_POINT_MAG_MIP_LINEAR: i32 = 261i32;
pub const D3D11_FILTER_MINIMUM_MIN_LINEAR_MAG_MIP_POINT: i32 = 272i32;
pub const D3D11_FILTER_MINIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR: i32 = 273i32;
pub const D3D11_FILTER_MINIMUM_MIN_MAG_LINEAR_MIP_POINT: i32 = 276i32;
pub const D3D11_FILTER_MINIMUM_MIN_MAG_MIP_LINEAR: i32 = 277i32;
pub const D3D11_FILTER_MINIMUM_ANISOTROPIC: i32 = 341i32;
pub const D3D11_FILTER_MAXIMUM_MIN_MAG_MIP_POINT: i32 = 384i32;
pub const D3D11_FILTER_MAXIMUM_MIN_MAG_POINT_MIP_LINEAR: i32 = 385i32;
pub const D3D11_FILTER_MAXIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT: i32 = 388i32;
pub const D3D11_FILTER_MAXIMUM_MIN_POINT_MAG_MIP_LINEAR: i32 = 389i32;
pub const D3D11_FILTER_MAXIMUM_MIN_LINEAR_MAG_MIP_POINT: i32 = 400i32;
pub const D3D11_FILTER_MAXIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR: i32 = 401i32;
pub const D3D11_FILTER_MAXIMUM_MIN_MAG_LINEAR_MIP_POINT: i32 = 404i32;
pub const D3D11_FILTER_MAXIMUM_MIN_MAG_MIP_LINEAR: i32 = 405i32;
pub const D3D11_FILTER_MAXIMUM_ANISOTROPIC: i32 = 469i32;
pub const D3D11_FILTER_REDUCTION_TYPE_STANDARD: i32 = 0i32;
pub const D3D11_FILTER_REDUCTION_TYPE_COMPARISON: i32 = 1i32;
pub const D3D11_FILTER_REDUCTION_TYPE_MINIMUM: i32 = 2i32;
pub const D3D11_FILTER_REDUCTION_TYPE_MAXIMUM: i32 = 3i32;
pub const D3D11_FILTER_REDUCTION_TYPE_MASK: u32 = 3u32;
pub const D3D11_FILTER_REDUCTION_TYPE_SHIFT: u32 = 7u32;
pub const D3D11_FILTER_TYPE_POINT: i32 = 0i32;
pub const D3D11_FILTER_TYPE_LINEAR: i32 = 1i32;
pub const D3D11_FILTER_TYPE_MASK: u32 = 3u32;
pub const D3D11_FLOAT16_FUSED_TOLERANCE_IN_ULP: f64 = 0.6f64;
pub const D3D11_FLOAT32_MAX: f32 = 340282350000000000000000000000000000000f32;
pub const D3D11_FLOAT32_TO_INTEGER_TOLERANCE_IN_ULP: f32 = 0.6f32;
pub const D3D11_FLOAT_TO_SRGB_EXPONENT_DENOMINATOR: f32 = 2.4f32;
pub const D3D11_FLOAT_TO_SRGB_EXPONENT_NUMERATOR: f32 = 1f32;
pub const D3D11_FLOAT_TO_SRGB_OFFSET: f32 = 0.055f32;
pub const D3D11_FLOAT_TO_SRGB_SCALE_1: f32 = 12.92f32;
pub const D3D11_FLOAT_TO_SRGB_SCALE_2: f32 = 1.055f32;
pub const D3D11_FLOAT_TO_SRGB_THRESHOLD: f32 = 0.0031308f32;
pub const D3D11_FORMAT_SUPPORT_BUFFER: i32 = 1i32;
pub const D3D11_FORMAT_SUPPORT_IA_VERTEX_BUFFER: i32 = 2i32;
pub const D3D11_FORMAT_SUPPORT_IA_INDEX_BUFFER: i32 = 4i32;
pub const D3D11_FORMAT_SUPPORT_SO_BUFFER: i32 = 8i32;
pub const D3D11_FORMAT_SUPPORT_TEXTURE1D: i32 = 16i32;
pub const D3D11_FORMAT_SUPPORT_TEXTURE2D: i32 = 32i32;
pub const D3D11_FORMAT_SUPPORT_TEXTURE3D: i32 = 64i32;
pub const D3D11_FORMAT_SUPPORT_TEXTURECUBE: i32 = 128i32;
pub const D3D11_FORMAT_SUPPORT_SHADER_LOAD: i32 = 256i32;
pub const D3D11_FORMAT_SUPPORT_SHADER_SAMPLE: i32 = 512i32;
pub const D3D11_FORMAT_SUPPORT_SHADER_SAMPLE_COMPARISON: i32 = 1024i32;
pub const D3D11_FORMAT_SUPPORT_SHADER_SAMPLE_MONO_TEXT: i32 = 2048i32;
pub const D3D11_FORMAT_SUPPORT_MIP: i32 = 4096i32;
pub const D3D11_FORMAT_SUPPORT_MIP_AUTOGEN: i32 = 8192i32;
pub const D3D11_FORMAT_SUPPORT_RENDER_TARGET: i32 = 16384i32;
pub const D3D11_FORMAT_SUPPORT_BLENDABLE: i32 = 32768i32;
pub const D3D11_FORMAT_SUPPORT_DEPTH_STENCIL: i32 = 65536i32;
pub const D3D11_FORMAT_SUPPORT_CPU_LOCKABLE: i32 = 131072i32;
pub const D3D11_FORMAT_SUPPORT_MULTISAMPLE_RESOLVE: i32 = 262144i32;
pub const D3D11_FORMAT_SUPPORT_DISPLAY: i32 = 524288i32;
pub const D3D11_FORMAT_SUPPORT_CAST_WITHIN_BIT_LAYOUT: i32 = 1048576i32;
pub const D3D11_FORMAT_SUPPORT_MULTISAMPLE_RENDERTARGET: i32 = 2097152i32;
pub const D3D11_FORMAT_SUPPORT_MULTISAMPLE_LOAD: i32 = 4194304i32;
pub const D3D11_FORMAT_SUPPORT_SHADER_GATHER: i32 = 8388608i32;
pub const D3D11_FORMAT_SUPPORT_BACK_BUFFER_CAST: i32 = 16777216i32;
pub const D3D11_FORMAT_SUPPORT_TYPED_UNORDERED_ACCESS_VIEW: i32 = 33554432i32;
pub const D3D11_FORMAT_SUPPORT_SHADER_GATHER_COMPARISON: i32 = 67108864i32;
pub const D3D11_FORMAT_SUPPORT_DECODER_OUTPUT: i32 = 134217728i32;
pub const D3D11_FORMAT_SUPPORT_VIDEO_PROCESSOR_OUTPUT: i32 = 268435456i32;
pub const D3D11_FORMAT_SUPPORT_VIDEO_PROCESSOR_INPUT: i32 = 536870912i32;
pub const D3D11_FORMAT_SUPPORT_VIDEO_ENCODER: i32 = 1073741824i32;
pub const D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_ADD: i32 = 1i32;
pub const D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_BITWISE_OPS: i32 = 2i32;
pub const D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_COMPARE_STORE_OR_COMPARE_EXCHANGE: i32 = 4i32;
pub const D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_EXCHANGE: i32 = 8i32;
pub const D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_SIGNED_MIN_OR_MAX: i32 = 16i32;
pub const D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_UNSIGNED_MIN_OR_MAX: i32 = 32i32;
pub const D3D11_FORMAT_SUPPORT2_UAV_TYPED_LOAD: i32 = 64i32;
pub const D3D11_FORMAT_SUPPORT2_UAV_TYPED_STORE: i32 = 128i32;
pub const D3D11_FORMAT_SUPPORT2_OUTPUT_MERGER_LOGIC_OP: i32 = 256i32;
pub const D3D11_FORMAT_SUPPORT2_TILED: i32 = 512i32;
pub const D3D11_FORMAT_SUPPORT2_SHAREABLE: i32 = 1024i32;
pub const D3D11_FORMAT_SUPPORT2_MULTIPLANE_OVERLAY: i32 = 16384i32;
pub const D3D11_FTOI_INSTRUCTION_MAX_INPUT: f32 = 2147483600f32;
pub const D3D11_FTOI_INSTRUCTION_MIN_INPUT: f32 = -2147483600f32;
pub const D3D11_FTOU_INSTRUCTION_MAX_INPUT: f32 = 4294967300f32;
pub const D3D11_FTOU_INSTRUCTION_MIN_INPUT: f32 = 0f32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D11_FUNCTION_DESC {
    pub Version: u32,
    pub Creator: super::super::Foundation::PSTR,
    pub Flags: u32,
    pub ConstantBuffers: u32,
    pub BoundResources: u32,
    pub InstructionCount: u32,
    pub TempRegisterCount: u32,
    pub TempArrayCount: u32,
    pub DefCount: u32,
    pub DclCount: u32,
    pub TextureNormalInstructions: u32,
    pub TextureLoadInstructions: u32,
    pub TextureCompInstructions: u32,
    pub TextureBiasInstructions: u32,
    pub TextureGradientInstructions: u32,
    pub FloatInstructionCount: u32,
    pub IntInstructionCount: u32,
    pub UintInstructionCount: u32,
    pub StaticFlowControlCount: u32,
    pub DynamicFlowControlCount: u32,
    pub MacroInstructionCount: u32,
    pub ArrayInstructionCount: u32,
    pub MovInstructionCount: u32,
    pub MovcInstructionCount: u32,
    pub ConversionInstructionCount: u32,
    pub BitwiseInstructionCount: u32,
    pub MinFeatureLevel: super::Direct3D::D3D_FEATURE_LEVEL,
    pub RequiredFeatureFlags: u64,
    pub Name: super::super::Foundation::PSTR,
    pub FunctionParameterCount: i32,
    pub HasReturn: super::super::Foundation::BOOL,
    pub Has10Level9VertexShader: super::super::Foundation::BOOL,
    pub Has10Level9PixelShader: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D11_FUNCTION_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D11_FUNCTION_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_GEOMETRY_SHADER_TRACE_DESC {
    pub Invocation: u64,
}
impl ::core::marker::Copy for D3D11_GEOMETRY_SHADER_TRACE_DESC {}
impl ::core::clone::Clone for D3D11_GEOMETRY_SHADER_TRACE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_GS_INPUT_INSTANCE_ID_READS_PER_INST: u32 = 2u32;
pub const D3D11_GS_INPUT_INSTANCE_ID_READ_PORTS: u32 = 1u32;
pub const D3D11_GS_INPUT_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_GS_INPUT_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_GS_INPUT_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_GS_INPUT_PRIM_CONST_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_GS_INPUT_PRIM_CONST_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_GS_INPUT_PRIM_CONST_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_GS_INPUT_PRIM_CONST_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_GS_INPUT_PRIM_CONST_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_GS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_GS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_GS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_GS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_GS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_GS_INPUT_REGISTER_VERTICES: u32 = 32u32;
pub const D3D11_GS_MAX_INSTANCE_COUNT: u32 = 32u32;
pub const D3D11_GS_MAX_OUTPUT_VERTEX_COUNT_ACROSS_INSTANCES: u32 = 1024u32;
pub const D3D11_GS_OUTPUT_ELEMENTS: u32 = 32u32;
pub const D3D11_GS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_GS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_GS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_HS_CONTROL_POINT_PHASE_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_HS_CONTROL_POINT_PHASE_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_HS_CONTROL_POINT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_HS_CONTROL_POINT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_HS_CONTROL_POINT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_HS_CONTROL_POINT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_HS_FORK_PHASE_INSTANCE_COUNT_UPPER_BOUND: u32 = 4294967295u32;
pub const D3D11_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_HS_INPUT_FORK_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_HS_INPUT_FORK_INSTANCE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_HS_INPUT_FORK_INSTANCE_ID_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_HS_INPUT_JOIN_INSTANCE_ID_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_HS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_HS_INPUT_PRIMITIVE_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_HS_INPUT_PRIMITIVE_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_HS_INPUT_PRIMITIVE_ID_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_HS_INPUT_PRIMITIVE_ID_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_HS_JOIN_PHASE_INSTANCE_COUNT_UPPER_BOUND: u32 = 4294967295u32;
pub const D3D11_HS_MAXTESSFACTOR_LOWER_BOUND: f32 = 1f32;
pub const D3D11_HS_MAXTESSFACTOR_UPPER_BOUND: f32 = 64f32;
pub const D3D11_HS_OUTPUT_CONTROL_POINTS_MAX_TOTAL_SCALARS: u32 = 3968u32;
pub const D3D11_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_HS_OUTPUT_CONTROL_POINT_ID_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_HS_OUTPUT_PATCH_CONSTANT_REGISTER_SCALAR_COMPONENTS: u32 = 128u32;
#[repr(C)]
pub struct D3D11_HULL_SHADER_TRACE_DESC {
    pub Invocation: u64,
}
impl ::core::marker::Copy for D3D11_HULL_SHADER_TRACE_DESC {}
impl ::core::clone::Clone for D3D11_HULL_SHADER_TRACE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_IA_DEFAULT_INDEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
pub const D3D11_IA_DEFAULT_PRIMITIVE_TOPOLOGY: u32 = 0u32;
pub const D3D11_IA_DEFAULT_VERTEX_BUFFER_OFFSET_IN_BYTES: u32 = 0u32;
pub const D3D11_IA_INDEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 1u32;
pub const D3D11_IA_INSTANCE_ID_BIT_COUNT: u32 = 32u32;
pub const D3D11_IA_INTEGER_ARITHMETIC_BIT_COUNT: u32 = 32u32;
pub const D3D11_IA_PATCH_MAX_CONTROL_POINT_COUNT: u32 = 32u32;
pub const D3D11_IA_PRIMITIVE_ID_BIT_COUNT: u32 = 32u32;
pub const D3D11_IA_VERTEX_ID_BIT_COUNT: u32 = 32u32;
pub const D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: u32 = 32u32;
pub const D3D11_IA_VERTEX_INPUT_STRUCTURE_ELEMENTS_COMPONENTS: u32 = 128u32;
pub const D3D11_IA_VERTEX_INPUT_STRUCTURE_ELEMENT_COUNT: u32 = 32u32;
pub const D3D11_INFO_QUEUE_DEFAULT_MESSAGE_COUNT_LIMIT: u32 = 1024u32;
#[repr(C)]
pub struct D3D11_INFO_QUEUE_FILTER {
    pub AllowList: D3D11_INFO_QUEUE_FILTER_DESC,
    pub DenyList: D3D11_INFO_QUEUE_FILTER_DESC,
}
impl ::core::marker::Copy for D3D11_INFO_QUEUE_FILTER {}
impl ::core::clone::Clone for D3D11_INFO_QUEUE_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_INFO_QUEUE_FILTER_DESC {
    pub NumCategories: u32,
    pub pCategoryList: *mut D3D11_MESSAGE_CATEGORY,
    pub NumSeverities: u32,
    pub pSeverityList: *mut D3D11_MESSAGE_SEVERITY,
    pub NumIDs: u32,
    pub pIDList: *mut D3D11_MESSAGE_ID,
}
impl ::core::marker::Copy for D3D11_INFO_QUEUE_FILTER_DESC {}
impl ::core::clone::Clone for D3D11_INFO_QUEUE_FILTER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_INPUT_PER_VERTEX_DATA: i32 = 0i32;
pub const D3D11_INPUT_PER_INSTANCE_DATA: i32 = 1i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D11_INPUT_ELEMENT_DESC {
    pub SemanticName: super::super::Foundation::PSTR,
    pub SemanticIndex: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub InputSlot: u32,
    pub AlignedByteOffset: u32,
    pub InputSlotClass: D3D11_INPUT_CLASSIFICATION,
    pub InstanceDataStepRate: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D11_INPUT_ELEMENT_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D11_INPUT_ELEMENT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_INTEGER_DIVIDE_BY_ZERO_QUOTIENT: u32 = 4294967295u32;
pub const D3D11_INTEGER_DIVIDE_BY_ZERO_REMAINDER: u32 = 4294967295u32;
pub const D3D11_KEEP_RENDER_TARGETS_AND_DEPTH_STENCIL: u32 = 4294967295u32;
pub const D3D11_KEEP_UNORDERED_ACCESS_VIEWS: u32 = 4294967295u32;
pub const D3D11_KEY_EXCHANGE_HW_PROTECTION: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2971078026,
    data2: 25229,
    data3: 19875,
    data4: [173, 59, 130, 221, 176, 139, 73, 112],
};
#[repr(C)]
pub struct D3D11_KEY_EXCHANGE_HW_PROTECTION_DATA {
    pub HWProtectionFunctionID: u32,
    pub pInputData: *mut D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA,
    pub pOutputData: *mut D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA,
    pub Status: ::windows_sys::core::HRESULT,
}
impl ::core::marker::Copy for D3D11_KEY_EXCHANGE_HW_PROTECTION_DATA {}
impl ::core::clone::Clone for D3D11_KEY_EXCHANGE_HW_PROTECTION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA {
    pub PrivateDataSize: u32,
    pub HWProtectionDataSize: u32,
    pub pbInput: [u8; 4],
}
impl ::core::marker::Copy for D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA {}
impl ::core::clone::Clone for D3D11_KEY_EXCHANGE_HW_PROTECTION_INPUT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA {
    pub PrivateDataSize: u32,
    pub MaxHWProtectionDataSize: u32,
    pub HWProtectionDataSize: u32,
    pub TransportTime: u64,
    pub ExecutionTime: u64,
    pub pbOutput: [u8; 4],
}
impl ::core::marker::Copy for D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA {}
impl ::core::clone::Clone for D3D11_KEY_EXCHANGE_HW_PROTECTION_OUTPUT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_KEY_EXCHANGE_RSAES_OAEP: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3247741077, data2: 55082, data3: 18973, data4: [142, 93, 237, 133, 125, 23, 21, 32] };
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_LIBRARY_DESC {
    pub Creator: super::super::Foundation::PSTR,
    pub Flags: u32,
    pub FunctionCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_LIBRARY_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_LIBRARY_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_LINEAR_GAMMA: f32 = 1f32;
pub const D3D11_LOGIC_OP_CLEAR: i32 = 0i32;
pub const D3D11_LOGIC_OP_SET: i32 = 1i32;
pub const D3D11_LOGIC_OP_COPY: i32 = 2i32;
pub const D3D11_LOGIC_OP_COPY_INVERTED: i32 = 3i32;
pub const D3D11_LOGIC_OP_NOOP: i32 = 4i32;
pub const D3D11_LOGIC_OP_INVERT: i32 = 5i32;
pub const D3D11_LOGIC_OP_AND: i32 = 6i32;
pub const D3D11_LOGIC_OP_NAND: i32 = 7i32;
pub const D3D11_LOGIC_OP_OR: i32 = 8i32;
pub const D3D11_LOGIC_OP_NOR: i32 = 9i32;
pub const D3D11_LOGIC_OP_XOR: i32 = 10i32;
pub const D3D11_LOGIC_OP_EQUIV: i32 = 11i32;
pub const D3D11_LOGIC_OP_AND_REVERSE: i32 = 12i32;
pub const D3D11_LOGIC_OP_AND_INVERTED: i32 = 13i32;
pub const D3D11_LOGIC_OP_OR_REVERSE: i32 = 14i32;
pub const D3D11_LOGIC_OP_OR_INVERTED: i32 = 15i32;
pub const D3D11_MAG_FILTER_SHIFT: u32 = 2u32;
pub const D3D11_MAJOR_VERSION: u32 = 11u32;
pub const D3D11_MAP_READ: i32 = 1i32;
pub const D3D11_MAP_WRITE: i32 = 2i32;
pub const D3D11_MAP_READ_WRITE: i32 = 3i32;
pub const D3D11_MAP_WRITE_DISCARD: i32 = 4i32;
pub const D3D11_MAP_WRITE_NO_OVERWRITE: i32 = 5i32;
#[repr(C)]
pub struct D3D11_MAPPED_SUBRESOURCE {
    pub pData: *mut ::core::ffi::c_void,
    pub RowPitch: u32,
    pub DepthPitch: u32,
}
impl ::core::marker::Copy for D3D11_MAPPED_SUBRESOURCE {}
impl ::core::clone::Clone for D3D11_MAPPED_SUBRESOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_MAP_FLAG_DO_NOT_WAIT: i32 = 1048576i32;
pub const D3D11_MAX_BORDER_COLOR_COMPONENT: f32 = 1f32;
pub const D3D11_MAX_DEPTH: f32 = 1f32;
pub const D3D11_MAX_MAXANISOTROPY: u32 = 16u32;
pub const D3D11_MAX_MULTISAMPLE_SAMPLE_COUNT: u32 = 32u32;
pub const D3D11_MAX_POSITION_VALUE: f32 = 34028236000000000000000000000000000f32;
pub const D3D11_MAX_TEXTURE_DIMENSION_2_TO_EXP: u32 = 17u32;
#[repr(C)]
pub struct D3D11_MESSAGE {
    pub Category: D3D11_MESSAGE_CATEGORY,
    pub Severity: D3D11_MESSAGE_SEVERITY,
    pub ID: D3D11_MESSAGE_ID,
    pub pDescription: *mut u8,
    pub DescriptionByteLength: usize,
}
impl ::core::marker::Copy for D3D11_MESSAGE {}
impl ::core::clone::Clone for D3D11_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_MESSAGE_CATEGORY_APPLICATION_DEFINED: i32 = 0i32;
pub const D3D11_MESSAGE_CATEGORY_MISCELLANEOUS: i32 = 1i32;
pub const D3D11_MESSAGE_CATEGORY_INITIALIZATION: i32 = 2i32;
pub const D3D11_MESSAGE_CATEGORY_CLEANUP: i32 = 3i32;
pub const D3D11_MESSAGE_CATEGORY_COMPILATION: i32 = 4i32;
pub const D3D11_MESSAGE_CATEGORY_STATE_CREATION: i32 = 5i32;
pub const D3D11_MESSAGE_CATEGORY_STATE_SETTING: i32 = 6i32;
pub const D3D11_MESSAGE_CATEGORY_STATE_GETTING: i32 = 7i32;
pub const D3D11_MESSAGE_CATEGORY_RESOURCE_MANIPULATION: i32 = 8i32;
pub const D3D11_MESSAGE_CATEGORY_EXECUTION: i32 = 9i32;
pub const D3D11_MESSAGE_CATEGORY_SHADER: i32 = 10i32;
pub const D3D11_MESSAGE_ID_UNKNOWN: i32 = 0i32;
pub const D3D11_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_HAZARD: i32 = 1i32;
pub const D3D11_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_HAZARD: i32 = 2i32;
pub const D3D11_MESSAGE_ID_DEVICE_VSSETSHADERRESOURCES_HAZARD: i32 = 3i32;
pub const D3D11_MESSAGE_ID_DEVICE_VSSETCONSTANTBUFFERS_HAZARD: i32 = 4i32;
pub const D3D11_MESSAGE_ID_DEVICE_GSSETSHADERRESOURCES_HAZARD: i32 = 5i32;
pub const D3D11_MESSAGE_ID_DEVICE_GSSETCONSTANTBUFFERS_HAZARD: i32 = 6i32;
pub const D3D11_MESSAGE_ID_DEVICE_PSSETSHADERRESOURCES_HAZARD: i32 = 7i32;
pub const D3D11_MESSAGE_ID_DEVICE_PSSETCONSTANTBUFFERS_HAZARD: i32 = 8i32;
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETS_HAZARD: i32 = 9i32;
pub const D3D11_MESSAGE_ID_DEVICE_SOSETTARGETS_HAZARD: i32 = 10i32;
pub const D3D11_MESSAGE_ID_STRING_FROM_APPLICATION: i32 = 11i32;
pub const D3D11_MESSAGE_ID_CORRUPTED_THIS: i32 = 12i32;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER1: i32 = 13i32;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER2: i32 = 14i32;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER3: i32 = 15i32;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER4: i32 = 16i32;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER5: i32 = 17i32;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER6: i32 = 18i32;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER7: i32 = 19i32;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER8: i32 = 20i32;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER9: i32 = 21i32;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER10: i32 = 22i32;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER11: i32 = 23i32;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER12: i32 = 24i32;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER13: i32 = 25i32;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER14: i32 = 26i32;
pub const D3D11_MESSAGE_ID_CORRUPTED_PARAMETER15: i32 = 27i32;
pub const D3D11_MESSAGE_ID_CORRUPTED_MULTITHREADING: i32 = 28i32;
pub const D3D11_MESSAGE_ID_MESSAGE_REPORTING_OUTOFMEMORY: i32 = 29i32;
pub const D3D11_MESSAGE_ID_IASETINPUTLAYOUT_UNBINDDELETINGOBJECT: i32 = 30i32;
pub const D3D11_MESSAGE_ID_IASETVERTEXBUFFERS_UNBINDDELETINGOBJECT: i32 = 31i32;
pub const D3D11_MESSAGE_ID_IASETINDEXBUFFER_UNBINDDELETINGOBJECT: i32 = 32i32;
pub const D3D11_MESSAGE_ID_VSSETSHADER_UNBINDDELETINGOBJECT: i32 = 33i32;
pub const D3D11_MESSAGE_ID_VSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: i32 = 34i32;
pub const D3D11_MESSAGE_ID_VSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: i32 = 35i32;
pub const D3D11_MESSAGE_ID_VSSETSAMPLERS_UNBINDDELETINGOBJECT: i32 = 36i32;
pub const D3D11_MESSAGE_ID_GSSETSHADER_UNBINDDELETINGOBJECT: i32 = 37i32;
pub const D3D11_MESSAGE_ID_GSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: i32 = 38i32;
pub const D3D11_MESSAGE_ID_GSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: i32 = 39i32;
pub const D3D11_MESSAGE_ID_GSSETSAMPLERS_UNBINDDELETINGOBJECT: i32 = 40i32;
pub const D3D11_MESSAGE_ID_SOSETTARGETS_UNBINDDELETINGOBJECT: i32 = 41i32;
pub const D3D11_MESSAGE_ID_PSSETSHADER_UNBINDDELETINGOBJECT: i32 = 42i32;
pub const D3D11_MESSAGE_ID_PSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: i32 = 43i32;
pub const D3D11_MESSAGE_ID_PSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: i32 = 44i32;
pub const D3D11_MESSAGE_ID_PSSETSAMPLERS_UNBINDDELETINGOBJECT: i32 = 45i32;
pub const D3D11_MESSAGE_ID_RSSETSTATE_UNBINDDELETINGOBJECT: i32 = 46i32;
pub const D3D11_MESSAGE_ID_OMSETBLENDSTATE_UNBINDDELETINGOBJECT: i32 = 47i32;
pub const D3D11_MESSAGE_ID_OMSETDEPTHSTENCILSTATE_UNBINDDELETINGOBJECT: i32 = 48i32;
pub const D3D11_MESSAGE_ID_OMSETRENDERTARGETS_UNBINDDELETINGOBJECT: i32 = 49i32;
pub const D3D11_MESSAGE_ID_SETPREDICATION_UNBINDDELETINGOBJECT: i32 = 50i32;
pub const D3D11_MESSAGE_ID_GETPRIVATEDATA_MOREDATA: i32 = 51i32;
pub const D3D11_MESSAGE_ID_SETPRIVATEDATA_INVALIDFREEDATA: i32 = 52i32;
pub const D3D11_MESSAGE_ID_SETPRIVATEDATA_INVALIDIUNKNOWN: i32 = 53i32;
pub const D3D11_MESSAGE_ID_SETPRIVATEDATA_INVALIDFLAGS: i32 = 54i32;
pub const D3D11_MESSAGE_ID_SETPRIVATEDATA_CHANGINGPARAMS: i32 = 55i32;
pub const D3D11_MESSAGE_ID_SETPRIVATEDATA_OUTOFMEMORY: i32 = 56i32;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDFORMAT: i32 = 57i32;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDSAMPLES: i32 = 58i32;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDUSAGE: i32 = 59i32;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDBINDFLAGS: i32 = 60i32;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDCPUACCESSFLAGS: i32 = 61i32;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_UNRECOGNIZEDMISCFLAGS: i32 = 62i32;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDCPUACCESSFLAGS: i32 = 63i32;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDBINDFLAGS: i32 = 64i32;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDINITIALDATA: i32 = 65i32;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDDIMENSIONS: i32 = 66i32;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDMIPLEVELS: i32 = 67i32;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDMISCFLAGS: i32 = 68i32;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDARG_RETURN: i32 = 69i32;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_OUTOFMEMORY_RETURN: i32 = 70i32;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_NULLDESC: i32 = 71i32;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDCONSTANTBUFFERBINDINGS: i32 = 72i32;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_LARGEALLOCATION: i32 = 73i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDFORMAT: i32 = 74i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_UNSUPPORTEDFORMAT: i32 = 75i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDSAMPLES: i32 = 76i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDUSAGE: i32 = 77i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDBINDFLAGS: i32 = 78i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDCPUACCESSFLAGS: i32 = 79i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_UNRECOGNIZEDMISCFLAGS: i32 = 80i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDCPUACCESSFLAGS: i32 = 81i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDBINDFLAGS: i32 = 82i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDINITIALDATA: i32 = 83i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDDIMENSIONS: i32 = 84i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDMIPLEVELS: i32 = 85i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDMISCFLAGS: i32 = 86i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDARG_RETURN: i32 = 87i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_OUTOFMEMORY_RETURN: i32 = 88i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_NULLDESC: i32 = 89i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_LARGEALLOCATION: i32 = 90i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDFORMAT: i32 = 91i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_UNSUPPORTEDFORMAT: i32 = 92i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDSAMPLES: i32 = 93i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDUSAGE: i32 = 94i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDBINDFLAGS: i32 = 95i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDCPUACCESSFLAGS: i32 = 96i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_UNRECOGNIZEDMISCFLAGS: i32 = 97i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDCPUACCESSFLAGS: i32 = 98i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDBINDFLAGS: i32 = 99i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDINITIALDATA: i32 = 100i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDDIMENSIONS: i32 = 101i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDMIPLEVELS: i32 = 102i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDMISCFLAGS: i32 = 103i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDARG_RETURN: i32 = 104i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_OUTOFMEMORY_RETURN: i32 = 105i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_NULLDESC: i32 = 106i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_LARGEALLOCATION: i32 = 107i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDFORMAT: i32 = 108i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_UNSUPPORTEDFORMAT: i32 = 109i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDSAMPLES: i32 = 110i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDUSAGE: i32 = 111i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDBINDFLAGS: i32 = 112i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDCPUACCESSFLAGS: i32 = 113i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_UNRECOGNIZEDMISCFLAGS: i32 = 114i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDCPUACCESSFLAGS: i32 = 115i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDBINDFLAGS: i32 = 116i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDINITIALDATA: i32 = 117i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDDIMENSIONS: i32 = 118i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDMIPLEVELS: i32 = 119i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDMISCFLAGS: i32 = 120i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_INVALIDARG_RETURN: i32 = 121i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_OUTOFMEMORY_RETURN: i32 = 122i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_NULLDESC: i32 = 123i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE3D_LARGEALLOCATION: i32 = 124i32;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_UNRECOGNIZEDFORMAT: i32 = 125i32;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDESC: i32 = 126i32;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDFORMAT: i32 = 127i32;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDIMENSIONS: i32 = 128i32;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDRESOURCE: i32 = 129i32;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_TOOMANYOBJECTS: i32 = 130i32;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDARG_RETURN: i32 = 131i32;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_OUTOFMEMORY_RETURN: i32 = 132i32;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_UNRECOGNIZEDFORMAT: i32 = 133i32;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_UNSUPPORTEDFORMAT: i32 = 134i32;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDESC: i32 = 135i32;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDFORMAT: i32 = 136i32;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDIMENSIONS: i32 = 137i32;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDRESOURCE: i32 = 138i32;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_TOOMANYOBJECTS: i32 = 139i32;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDARG_RETURN: i32 = 140i32;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_OUTOFMEMORY_RETURN: i32 = 141i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_UNRECOGNIZEDFORMAT: i32 = 142i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDESC: i32 = 143i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFORMAT: i32 = 144i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDDIMENSIONS: i32 = 145i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDRESOURCE: i32 = 146i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_TOOMANYOBJECTS: i32 = 147i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDARG_RETURN: i32 = 148i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_OUTOFMEMORY_RETURN: i32 = 149i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_OUTOFMEMORY: i32 = 150i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_TOOMANYELEMENTS: i32 = 151i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDFORMAT: i32 = 152i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INCOMPATIBLEFORMAT: i32 = 153i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOT: i32 = 154i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDINPUTSLOTCLASS: i32 = 155i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_STEPRATESLOTCLASSMISMATCH: i32 = 156i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSLOTCLASSCHANGE: i32 = 157i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDSTEPRATECHANGE: i32 = 158i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_INVALIDALIGNMENT: i32 = 159i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_DUPLICATESEMANTIC: i32 = 160i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_UNPARSEABLEINPUTSIGNATURE: i32 = 161i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_NULLSEMANTIC: i32 = 162i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_MISSINGELEMENT: i32 = 163i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_NULLDESC: i32 = 164i32;
pub const D3D11_MESSAGE_ID_CREATEVERTEXSHADER_OUTOFMEMORY: i32 = 165i32;
pub const D3D11_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERBYTECODE: i32 = 166i32;
pub const D3D11_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDSHADERTYPE: i32 = 167i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADER_OUTOFMEMORY: i32 = 168i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERBYTECODE: i32 = 169i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDSHADERTYPE: i32 = 170i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTOFMEMORY: i32 = 171i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERBYTECODE: i32 = 172i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSHADERTYPE: i32 = 173i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMENTRIES: i32 = 174i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSTREAMSTRIDEUNUSED: i32 = 175i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDDECL: i32 = 176i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_EXPECTEDDECL: i32 = 177i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_OUTPUTSLOT0EXPECTED: i32 = 178i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSLOT: i32 = 179i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_ONLYONEELEMENTPERSLOT: i32 = 180i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDCOMPONENTCOUNT: i32 = 181i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTARTCOMPONENTANDCOMPONENTCOUNT: i32 = 182i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDGAPDEFINITION: i32 = 183i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_REPEATEDOUTPUT: i32 = 184i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDOUTPUTSTREAMSTRIDE: i32 = 185i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGSEMANTIC: i32 = 186i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MASKMISMATCH: i32 = 187i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_CANTHAVEONLYGAPS: i32 = 188i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DECLTOOCOMPLEX: i32 = 189i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_MISSINGOUTPUTSIGNATURE: i32 = 190i32;
pub const D3D11_MESSAGE_ID_CREATEPIXELSHADER_OUTOFMEMORY: i32 = 191i32;
pub const D3D11_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERBYTECODE: i32 = 192i32;
pub const D3D11_MESSAGE_ID_CREATEPIXELSHADER_INVALIDSHADERTYPE: i32 = 193i32;
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFILLMODE: i32 = 194i32;
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDCULLMODE: i32 = 195i32;
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDDEPTHBIASCLAMP: i32 = 196i32;
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDSLOPESCALEDDEPTHBIAS: i32 = 197i32;
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_TOOMANYOBJECTS: i32 = 198i32;
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_NULLDESC: i32 = 199i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHWRITEMASK: i32 = 200i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDDEPTHFUNC: i32 = 201i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFAILOP: i32 = 202i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILZFAILOP: i32 = 203i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILPASSOP: i32 = 204i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDFRONTFACESTENCILFUNC: i32 = 205i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFAILOP: i32 = 206i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILZFAILOP: i32 = 207i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILPASSOP: i32 = 208i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_INVALIDBACKFACESTENCILFUNC: i32 = 209i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_TOOMANYOBJECTS: i32 = 210i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_NULLDESC: i32 = 211i32;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLEND: i32 = 212i32;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLEND: i32 = 213i32;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOP: i32 = 214i32;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDSRCBLENDALPHA: i32 = 215i32;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDDESTBLENDALPHA: i32 = 216i32;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDBLENDOPALPHA: i32 = 217i32;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDRENDERTARGETWRITEMASK: i32 = 218i32;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_TOOMANYOBJECTS: i32 = 219i32;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_NULLDESC: i32 = 220i32;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDFILTER: i32 = 221i32;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSU: i32 = 222i32;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSV: i32 = 223i32;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDADDRESSW: i32 = 224i32;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMIPLODBIAS: i32 = 225i32;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMAXANISOTROPY: i32 = 226i32;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDCOMPARISONFUNC: i32 = 227i32;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMINLOD: i32 = 228i32;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_INVALIDMAXLOD: i32 = 229i32;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_TOOMANYOBJECTS: i32 = 230i32;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_NULLDESC: i32 = 231i32;
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_INVALIDQUERY: i32 = 232i32;
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_INVALIDMISCFLAGS: i32 = 233i32;
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_UNEXPECTEDMISCFLAG: i32 = 234i32;
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_NULLDESC: i32 = 235i32;
pub const D3D11_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_TOPOLOGY_UNRECOGNIZED: i32 = 236i32;
pub const D3D11_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_TOPOLOGY_UNDEFINED: i32 = 237i32;
pub const D3D11_MESSAGE_ID_IASETVERTEXBUFFERS_INVALIDBUFFER: i32 = 238i32;
pub const D3D11_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_OFFSET_TOO_LARGE: i32 = 239i32;
pub const D3D11_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_BUFFERS_EMPTY: i32 = 240i32;
pub const D3D11_MESSAGE_ID_IASETINDEXBUFFER_INVALIDBUFFER: i32 = 241i32;
pub const D3D11_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_FORMAT_INVALID: i32 = 242i32;
pub const D3D11_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_OFFSET_TOO_LARGE: i32 = 243i32;
pub const D3D11_MESSAGE_ID_DEVICE_IASETINDEXBUFFER_OFFSET_UNALIGNED: i32 = 244i32;
pub const D3D11_MESSAGE_ID_DEVICE_VSSETSHADERRESOURCES_VIEWS_EMPTY: i32 = 245i32;
pub const D3D11_MESSAGE_ID_VSSETCONSTANTBUFFERS_INVALIDBUFFER: i32 = 246i32;
pub const D3D11_MESSAGE_ID_DEVICE_VSSETCONSTANTBUFFERS_BUFFERS_EMPTY: i32 = 247i32;
pub const D3D11_MESSAGE_ID_DEVICE_VSSETSAMPLERS_SAMPLERS_EMPTY: i32 = 248i32;
pub const D3D11_MESSAGE_ID_DEVICE_GSSETSHADERRESOURCES_VIEWS_EMPTY: i32 = 249i32;
pub const D3D11_MESSAGE_ID_GSSETCONSTANTBUFFERS_INVALIDBUFFER: i32 = 250i32;
pub const D3D11_MESSAGE_ID_DEVICE_GSSETCONSTANTBUFFERS_BUFFERS_EMPTY: i32 = 251i32;
pub const D3D11_MESSAGE_ID_DEVICE_GSSETSAMPLERS_SAMPLERS_EMPTY: i32 = 252i32;
pub const D3D11_MESSAGE_ID_SOSETTARGETS_INVALIDBUFFER: i32 = 253i32;
pub const D3D11_MESSAGE_ID_DEVICE_SOSETTARGETS_OFFSET_UNALIGNED: i32 = 254i32;
pub const D3D11_MESSAGE_ID_DEVICE_PSSETSHADERRESOURCES_VIEWS_EMPTY: i32 = 255i32;
pub const D3D11_MESSAGE_ID_PSSETCONSTANTBUFFERS_INVALIDBUFFER: i32 = 256i32;
pub const D3D11_MESSAGE_ID_DEVICE_PSSETCONSTANTBUFFERS_BUFFERS_EMPTY: i32 = 257i32;
pub const D3D11_MESSAGE_ID_DEVICE_PSSETSAMPLERS_SAMPLERS_EMPTY: i32 = 258i32;
pub const D3D11_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_INVALIDVIEWPORT: i32 = 259i32;
pub const D3D11_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_INVALIDSCISSOR: i32 = 260i32;
pub const D3D11_MESSAGE_ID_CLEARRENDERTARGETVIEW_DENORMFLUSH: i32 = 261i32;
pub const D3D11_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_DENORMFLUSH: i32 = 262i32;
pub const D3D11_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_INVALID: i32 = 263i32;
pub const D3D11_MESSAGE_ID_DEVICE_IAGETVERTEXBUFFERS_BUFFERS_EMPTY: i32 = 264i32;
pub const D3D11_MESSAGE_ID_DEVICE_VSGETSHADERRESOURCES_VIEWS_EMPTY: i32 = 265i32;
pub const D3D11_MESSAGE_ID_DEVICE_VSGETCONSTANTBUFFERS_BUFFERS_EMPTY: i32 = 266i32;
pub const D3D11_MESSAGE_ID_DEVICE_VSGETSAMPLERS_SAMPLERS_EMPTY: i32 = 267i32;
pub const D3D11_MESSAGE_ID_DEVICE_GSGETSHADERRESOURCES_VIEWS_EMPTY: i32 = 268i32;
pub const D3D11_MESSAGE_ID_DEVICE_GSGETCONSTANTBUFFERS_BUFFERS_EMPTY: i32 = 269i32;
pub const D3D11_MESSAGE_ID_DEVICE_GSGETSAMPLERS_SAMPLERS_EMPTY: i32 = 270i32;
pub const D3D11_MESSAGE_ID_DEVICE_SOGETTARGETS_BUFFERS_EMPTY: i32 = 271i32;
pub const D3D11_MESSAGE_ID_DEVICE_PSGETSHADERRESOURCES_VIEWS_EMPTY: i32 = 272i32;
pub const D3D11_MESSAGE_ID_DEVICE_PSGETCONSTANTBUFFERS_BUFFERS_EMPTY: i32 = 273i32;
pub const D3D11_MESSAGE_ID_DEVICE_PSGETSAMPLERS_SAMPLERS_EMPTY: i32 = 274i32;
pub const D3D11_MESSAGE_ID_DEVICE_RSGETVIEWPORTS_VIEWPORTS_EMPTY: i32 = 275i32;
pub const D3D11_MESSAGE_ID_DEVICE_RSGETSCISSORRECTS_RECTS_EMPTY: i32 = 276i32;
pub const D3D11_MESSAGE_ID_DEVICE_GENERATEMIPS_RESOURCE_INVALID: i32 = 277i32;
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDDESTINATIONSUBRESOURCE: i32 = 278i32;
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCESUBRESOURCE: i32 = 279i32;
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCEBOX: i32 = 280i32;
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCE: i32 = 281i32;
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDDESTINATIONSTATE: i32 = 282i32;
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_INVALIDSOURCESTATE: i32 = 283i32;
pub const D3D11_MESSAGE_ID_COPYRESOURCE_INVALIDSOURCE: i32 = 284i32;
pub const D3D11_MESSAGE_ID_COPYRESOURCE_INVALIDDESTINATIONSTATE: i32 = 285i32;
pub const D3D11_MESSAGE_ID_COPYRESOURCE_INVALIDSOURCESTATE: i32 = 286i32;
pub const D3D11_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONSUBRESOURCE: i32 = 287i32;
pub const D3D11_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONBOX: i32 = 288i32;
pub const D3D11_MESSAGE_ID_UPDATESUBRESOURCE_INVALIDDESTINATIONSTATE: i32 = 289i32;
pub const D3D11_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_DESTINATION_INVALID: i32 = 290i32;
pub const D3D11_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_DESTINATION_SUBRESOURCE_INVALID: i32 = 291i32;
pub const D3D11_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_SOURCE_INVALID: i32 = 292i32;
pub const D3D11_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_SOURCE_SUBRESOURCE_INVALID: i32 = 293i32;
pub const D3D11_MESSAGE_ID_DEVICE_RESOLVESUBRESOURCE_FORMAT_INVALID: i32 = 294i32;
pub const D3D11_MESSAGE_ID_BUFFER_MAP_INVALIDMAPTYPE: i32 = 295i32;
pub const D3D11_MESSAGE_ID_BUFFER_MAP_INVALIDFLAGS: i32 = 296i32;
pub const D3D11_MESSAGE_ID_BUFFER_MAP_ALREADYMAPPED: i32 = 297i32;
pub const D3D11_MESSAGE_ID_BUFFER_MAP_DEVICEREMOVED_RETURN: i32 = 298i32;
pub const D3D11_MESSAGE_ID_BUFFER_UNMAP_NOTMAPPED: i32 = 299i32;
pub const D3D11_MESSAGE_ID_TEXTURE1D_MAP_INVALIDMAPTYPE: i32 = 300i32;
pub const D3D11_MESSAGE_ID_TEXTURE1D_MAP_INVALIDSUBRESOURCE: i32 = 301i32;
pub const D3D11_MESSAGE_ID_TEXTURE1D_MAP_INVALIDFLAGS: i32 = 302i32;
pub const D3D11_MESSAGE_ID_TEXTURE1D_MAP_ALREADYMAPPED: i32 = 303i32;
pub const D3D11_MESSAGE_ID_TEXTURE1D_MAP_DEVICEREMOVED_RETURN: i32 = 304i32;
pub const D3D11_MESSAGE_ID_TEXTURE1D_UNMAP_INVALIDSUBRESOURCE: i32 = 305i32;
pub const D3D11_MESSAGE_ID_TEXTURE1D_UNMAP_NOTMAPPED: i32 = 306i32;
pub const D3D11_MESSAGE_ID_TEXTURE2D_MAP_INVALIDMAPTYPE: i32 = 307i32;
pub const D3D11_MESSAGE_ID_TEXTURE2D_MAP_INVALIDSUBRESOURCE: i32 = 308i32;
pub const D3D11_MESSAGE_ID_TEXTURE2D_MAP_INVALIDFLAGS: i32 = 309i32;
pub const D3D11_MESSAGE_ID_TEXTURE2D_MAP_ALREADYMAPPED: i32 = 310i32;
pub const D3D11_MESSAGE_ID_TEXTURE2D_MAP_DEVICEREMOVED_RETURN: i32 = 311i32;
pub const D3D11_MESSAGE_ID_TEXTURE2D_UNMAP_INVALIDSUBRESOURCE: i32 = 312i32;
pub const D3D11_MESSAGE_ID_TEXTURE2D_UNMAP_NOTMAPPED: i32 = 313i32;
pub const D3D11_MESSAGE_ID_TEXTURE3D_MAP_INVALIDMAPTYPE: i32 = 314i32;
pub const D3D11_MESSAGE_ID_TEXTURE3D_MAP_INVALIDSUBRESOURCE: i32 = 315i32;
pub const D3D11_MESSAGE_ID_TEXTURE3D_MAP_INVALIDFLAGS: i32 = 316i32;
pub const D3D11_MESSAGE_ID_TEXTURE3D_MAP_ALREADYMAPPED: i32 = 317i32;
pub const D3D11_MESSAGE_ID_TEXTURE3D_MAP_DEVICEREMOVED_RETURN: i32 = 318i32;
pub const D3D11_MESSAGE_ID_TEXTURE3D_UNMAP_INVALIDSUBRESOURCE: i32 = 319i32;
pub const D3D11_MESSAGE_ID_TEXTURE3D_UNMAP_NOTMAPPED: i32 = 320i32;
pub const D3D11_MESSAGE_ID_CHECKFORMATSUPPORT_FORMAT_DEPRECATED: i32 = 321i32;
pub const D3D11_MESSAGE_ID_CHECKMULTISAMPLEQUALITYLEVELS_FORMAT_DEPRECATED: i32 = 322i32;
pub const D3D11_MESSAGE_ID_SETEXCEPTIONMODE_UNRECOGNIZEDFLAGS: i32 = 323i32;
pub const D3D11_MESSAGE_ID_SETEXCEPTIONMODE_INVALIDARG_RETURN: i32 = 324i32;
pub const D3D11_MESSAGE_ID_SETEXCEPTIONMODE_DEVICEREMOVED_RETURN: i32 = 325i32;
pub const D3D11_MESSAGE_ID_REF_SIMULATING_INFINITELY_FAST_HARDWARE: i32 = 326i32;
pub const D3D11_MESSAGE_ID_REF_THREADING_MODE: i32 = 327i32;
pub const D3D11_MESSAGE_ID_REF_UMDRIVER_EXCEPTION: i32 = 328i32;
pub const D3D11_MESSAGE_ID_REF_KMDRIVER_EXCEPTION: i32 = 329i32;
pub const D3D11_MESSAGE_ID_REF_HARDWARE_EXCEPTION: i32 = 330i32;
pub const D3D11_MESSAGE_ID_REF_ACCESSING_INDEXABLE_TEMP_OUT_OF_RANGE: i32 = 331i32;
pub const D3D11_MESSAGE_ID_REF_PROBLEM_PARSING_SHADER: i32 = 332i32;
pub const D3D11_MESSAGE_ID_REF_OUT_OF_MEMORY: i32 = 333i32;
pub const D3D11_MESSAGE_ID_REF_INFO: i32 = 334i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEXPOS_OVERFLOW: i32 = 335i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINDEXED_INDEXPOS_OVERFLOW: i32 = 336i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINSTANCED_VERTEXPOS_OVERFLOW: i32 = 337i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINSTANCED_INSTANCEPOS_OVERFLOW: i32 = 338i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINDEXEDINSTANCED_INSTANCEPOS_OVERFLOW: i32 = 339i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINDEXEDINSTANCED_INDEXPOS_OVERFLOW: i32 = 340i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEX_SHADER_NOT_SET: i32 = 341i32;
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_SEMANTICNAME_NOT_FOUND: i32 = 342i32;
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_REGISTERINDEX: i32 = 343i32;
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_COMPONENTTYPE: i32 = 344i32;
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_REGISTERMASK: i32 = 345i32;
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_SYSTEMVALUE: i32 = 346i32;
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_NEVERWRITTEN_ALWAYSREADS: i32 = 347i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_NOT_SET: i32 = 348i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INPUTLAYOUT_NOT_SET: i32 = 349i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_CONSTANT_BUFFER_NOT_SET: i32 = 350i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_CONSTANT_BUFFER_TOO_SMALL: i32 = 351i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_SAMPLER_NOT_SET: i32 = 352i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_SHADERRESOURCEVIEW_NOT_SET: i32 = 353i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VIEW_DIMENSION_MISMATCH: i32 = 354i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_STRIDE_TOO_SMALL: i32 = 355i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEX_BUFFER_TOO_SMALL: i32 = 356i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_NOT_SET: i32 = 357i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_FORMAT_INVALID: i32 = 358i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INDEX_BUFFER_TOO_SMALL: i32 = 359i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_GS_INPUT_PRIMITIVE_MISMATCH: i32 = 360i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_RETURN_TYPE_MISMATCH: i32 = 361i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_POSITION_NOT_PRESENT: i32 = 362i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_OUTPUT_STREAM_NOT_SET: i32 = 363i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_BOUND_RESOURCE_MAPPED: i32 = 364i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INVALID_PRIMITIVETOPOLOGY: i32 = 365i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEX_OFFSET_UNALIGNED: i32 = 366i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VERTEX_STRIDE_UNALIGNED: i32 = 367i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INDEX_OFFSET_UNALIGNED: i32 = 368i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_OUTPUT_STREAM_OFFSET_UNALIGNED: i32 = 369i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_LD_UNSUPPORTED: i32 = 370i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_SAMPLE_UNSUPPORTED: i32 = 371i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_SAMPLE_C_UNSUPPORTED: i32 = 372i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_MULTISAMPLE_UNSUPPORTED: i32 = 373i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_SO_TARGETS_BOUND_WITHOUT_SOURCE: i32 = 374i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_SO_STRIDE_LARGER_THAN_BUFFER: i32 = 375i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_OM_RENDER_TARGET_DOES_NOT_SUPPORT_BLENDING: i32 = 376i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_OM_DUAL_SOURCE_BLENDING_CAN_ONLY_HAVE_RENDER_TARGET_0: i32 = 377i32;
pub const D3D11_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_AT_FAULT: i32 = 378i32;
pub const D3D11_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_POSSIBLY_AT_FAULT: i32 = 379i32;
pub const D3D11_MESSAGE_ID_DEVICE_REMOVAL_PROCESS_NOT_AT_FAULT: i32 = 380i32;
pub const D3D11_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_INVALIDARG_RETURN: i32 = 381i32;
pub const D3D11_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_OUTOFMEMORY_RETURN: i32 = 382i32;
pub const D3D11_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_BADINTERFACE_RETURN: i32 = 383i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_VIEWPORT_NOT_SET: i32 = 384i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_TRAILING_DIGIT_IN_SEMANTIC: i32 = 385i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_TRAILING_DIGIT_IN_SEMANTIC: i32 = 386i32;
pub const D3D11_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_DENORMFLUSH: i32 = 387i32;
pub const D3D11_MESSAGE_ID_OMSETRENDERTARGETS_INVALIDVIEW: i32 = 388i32;
pub const D3D11_MESSAGE_ID_DEVICE_SETTEXTFILTERSIZE_INVALIDDIMENSIONS: i32 = 389i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_SAMPLER_MISMATCH: i32 = 390i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_TYPE_MISMATCH: i32 = 391i32;
pub const D3D11_MESSAGE_ID_BLENDSTATE_GETDESC_LEGACY: i32 = 392i32;
pub const D3D11_MESSAGE_ID_SHADERRESOURCEVIEW_GETDESC_LEGACY: i32 = 393i32;
pub const D3D11_MESSAGE_ID_CREATEQUERY_OUTOFMEMORY_RETURN: i32 = 394i32;
pub const D3D11_MESSAGE_ID_CREATEPREDICATE_OUTOFMEMORY_RETURN: i32 = 395i32;
pub const D3D11_MESSAGE_ID_CREATECOUNTER_OUTOFRANGE_COUNTER: i32 = 396i32;
pub const D3D11_MESSAGE_ID_CREATECOUNTER_SIMULTANEOUS_ACTIVE_COUNTERS_EXHAUSTED: i32 = 397i32;
pub const D3D11_MESSAGE_ID_CREATECOUNTER_UNSUPPORTED_WELLKNOWN_COUNTER: i32 = 398i32;
pub const D3D11_MESSAGE_ID_CREATECOUNTER_OUTOFMEMORY_RETURN: i32 = 399i32;
pub const D3D11_MESSAGE_ID_CREATECOUNTER_NONEXCLUSIVE_RETURN: i32 = 400i32;
pub const D3D11_MESSAGE_ID_CREATECOUNTER_NULLDESC: i32 = 401i32;
pub const D3D11_MESSAGE_ID_CHECKCOUNTER_OUTOFRANGE_COUNTER: i32 = 402i32;
pub const D3D11_MESSAGE_ID_CHECKCOUNTER_UNSUPPORTED_WELLKNOWN_COUNTER: i32 = 403i32;
pub const D3D11_MESSAGE_ID_SETPREDICATION_INVALID_PREDICATE_STATE: i32 = 404i32;
pub const D3D11_MESSAGE_ID_QUERY_BEGIN_UNSUPPORTED: i32 = 405i32;
pub const D3D11_MESSAGE_ID_PREDICATE_BEGIN_DURING_PREDICATION: i32 = 406i32;
pub const D3D11_MESSAGE_ID_QUERY_BEGIN_DUPLICATE: i32 = 407i32;
pub const D3D11_MESSAGE_ID_QUERY_BEGIN_ABANDONING_PREVIOUS_RESULTS: i32 = 408i32;
pub const D3D11_MESSAGE_ID_PREDICATE_END_DURING_PREDICATION: i32 = 409i32;
pub const D3D11_MESSAGE_ID_QUERY_END_ABANDONING_PREVIOUS_RESULTS: i32 = 410i32;
pub const D3D11_MESSAGE_ID_QUERY_END_WITHOUT_BEGIN: i32 = 411i32;
pub const D3D11_MESSAGE_ID_QUERY_GETDATA_INVALID_DATASIZE: i32 = 412i32;
pub const D3D11_MESSAGE_ID_QUERY_GETDATA_INVALID_FLAGS: i32 = 413i32;
pub const D3D11_MESSAGE_ID_QUERY_GETDATA_INVALID_CALL: i32 = 414i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_PS_OUTPUT_TYPE_MISMATCH: i32 = 415i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_FORMAT_GATHER_UNSUPPORTED: i32 = 416i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INVALID_USE_OF_CENTER_MULTISAMPLE_PATTERN: i32 = 417i32;
pub const D3D11_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_STRIDE_TOO_LARGE: i32 = 418i32;
pub const D3D11_MESSAGE_ID_DEVICE_IASETVERTEXBUFFERS_INVALIDRANGE: i32 = 419i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_EMPTY_LAYOUT: i32 = 420i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RESOURCE_SAMPLE_COUNT_MISMATCH: i32 = 421i32;
pub const D3D11_MESSAGE_ID_LIVE_OBJECT_SUMMARY: i32 = 422i32;
pub const D3D11_MESSAGE_ID_LIVE_BUFFER: i32 = 423i32;
pub const D3D11_MESSAGE_ID_LIVE_TEXTURE1D: i32 = 424i32;
pub const D3D11_MESSAGE_ID_LIVE_TEXTURE2D: i32 = 425i32;
pub const D3D11_MESSAGE_ID_LIVE_TEXTURE3D: i32 = 426i32;
pub const D3D11_MESSAGE_ID_LIVE_SHADERRESOURCEVIEW: i32 = 427i32;
pub const D3D11_MESSAGE_ID_LIVE_RENDERTARGETVIEW: i32 = 428i32;
pub const D3D11_MESSAGE_ID_LIVE_DEPTHSTENCILVIEW: i32 = 429i32;
pub const D3D11_MESSAGE_ID_LIVE_VERTEXSHADER: i32 = 430i32;
pub const D3D11_MESSAGE_ID_LIVE_GEOMETRYSHADER: i32 = 431i32;
pub const D3D11_MESSAGE_ID_LIVE_PIXELSHADER: i32 = 432i32;
pub const D3D11_MESSAGE_ID_LIVE_INPUTLAYOUT: i32 = 433i32;
pub const D3D11_MESSAGE_ID_LIVE_SAMPLER: i32 = 434i32;
pub const D3D11_MESSAGE_ID_LIVE_BLENDSTATE: i32 = 435i32;
pub const D3D11_MESSAGE_ID_LIVE_DEPTHSTENCILSTATE: i32 = 436i32;
pub const D3D11_MESSAGE_ID_LIVE_RASTERIZERSTATE: i32 = 437i32;
pub const D3D11_MESSAGE_ID_LIVE_QUERY: i32 = 438i32;
pub const D3D11_MESSAGE_ID_LIVE_PREDICATE: i32 = 439i32;
pub const D3D11_MESSAGE_ID_LIVE_COUNTER: i32 = 440i32;
pub const D3D11_MESSAGE_ID_LIVE_DEVICE: i32 = 441i32;
pub const D3D11_MESSAGE_ID_LIVE_SWAPCHAIN: i32 = 442i32;
pub const D3D11_MESSAGE_ID_D3D10_MESSAGES_END: i32 = 443i32;
pub const D3D11_MESSAGE_ID_D3D10L9_MESSAGES_START: i32 = 1048576i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILSTATE_STENCIL_NO_TWO_SIDED: i32 = 1048577i32;
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_DepthBiasClamp_NOT_SUPPORTED: i32 = 1048578i32;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_NO_COMPARISON_SUPPORT: i32 = 1048579i32;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_EXCESSIVE_ANISOTROPY: i32 = 1048580i32;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_BORDER_OUT_OF_RANGE: i32 = 1048581i32;
pub const D3D11_MESSAGE_ID_VSSETSAMPLERS_NOT_SUPPORTED: i32 = 1048582i32;
pub const D3D11_MESSAGE_ID_VSSETSAMPLERS_TOO_MANY_SAMPLERS: i32 = 1048583i32;
pub const D3D11_MESSAGE_ID_PSSETSAMPLERS_TOO_MANY_SAMPLERS: i32 = 1048584i32;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NO_ARRAYS: i32 = 1048585i32;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NO_VB_AND_IB_BIND: i32 = 1048586i32;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NO_TEXTURE_1D: i32 = 1048587i32;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_DIMENSION_OUT_OF_RANGE: i32 = 1048588i32;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NOT_BINDABLE_AS_SHADER_RESOURCE: i32 = 1048589i32;
pub const D3D11_MESSAGE_ID_OMSETRENDERTARGETS_TOO_MANY_RENDER_TARGETS: i32 = 1048590i32;
pub const D3D11_MESSAGE_ID_OMSETRENDERTARGETS_NO_DIFFERING_BIT_DEPTHS: i32 = 1048591i32;
pub const D3D11_MESSAGE_ID_IASETVERTEXBUFFERS_BAD_BUFFER_INDEX: i32 = 1048592i32;
pub const D3D11_MESSAGE_ID_DEVICE_RSSETVIEWPORTS_TOO_MANY_VIEWPORTS: i32 = 1048593i32;
pub const D3D11_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_ADJACENCY_UNSUPPORTED: i32 = 1048594i32;
pub const D3D11_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_TOO_MANY_SCISSORS: i32 = 1048595i32;
pub const D3D11_MESSAGE_ID_COPYRESOURCE_ONLY_TEXTURE_2D_WITHIN_GPU_MEMORY: i32 = 1048596i32;
pub const D3D11_MESSAGE_ID_COPYRESOURCE_NO_TEXTURE_3D_READBACK: i32 = 1048597i32;
pub const D3D11_MESSAGE_ID_COPYRESOURCE_NO_TEXTURE_ONLY_READBACK: i32 = 1048598i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_UNSUPPORTED_FORMAT: i32 = 1048599i32;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_NO_ALPHA_TO_COVERAGE: i32 = 1048600i32;
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_DepthClipEnable_MUST_BE_TRUE: i32 = 1048601i32;
pub const D3D11_MESSAGE_ID_DRAWINDEXED_STARTINDEXLOCATION_MUST_BE_POSITIVE: i32 = 1048602i32;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_MUST_USE_LOWEST_LOD: i32 = 1048603i32;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_MINLOD_MUST_NOT_BE_FRACTIONAL: i32 = 1048604i32;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_MAXLOD_MUST_BE_FLT_MAX: i32 = 1048605i32;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_FIRSTARRAYSLICE_MUST_BE_ZERO: i32 = 1048606i32;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_CUBES_MUST_HAVE_6_SIDES: i32 = 1048607i32;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NOT_BINDABLE_AS_RENDER_TARGET: i32 = 1048608i32;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NO_DWORD_INDEX_BUFFER: i32 = 1048609i32;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_MSAA_PRECLUDES_SHADER_RESOURCE: i32 = 1048610i32;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_PRESENTATION_PRECLUDES_SHADER_RESOURCE: i32 = 1048611i32;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_NO_INDEPENDENT_BLEND_ENABLE: i32 = 1048612i32;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_NO_INDEPENDENT_WRITE_MASKS: i32 = 1048613i32;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NO_STREAM_OUT: i32 = 1048614i32;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_ONLY_VB_IB_FOR_BUFFERS: i32 = 1048615i32;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NO_AUTOGEN_FOR_VOLUMES: i32 = 1048616i32;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_DXGI_FORMAT_R8G8B8A8_CANNOT_BE_SHARED: i32 = 1048617i32;
pub const D3D11_MESSAGE_ID_VSSHADERRESOURCES_NOT_SUPPORTED: i32 = 1048618i32;
pub const D3D11_MESSAGE_ID_GEOMETRY_SHADER_NOT_SUPPORTED: i32 = 1048619i32;
pub const D3D11_MESSAGE_ID_STREAM_OUT_NOT_SUPPORTED: i32 = 1048620i32;
pub const D3D11_MESSAGE_ID_TEXT_FILTER_NOT_SUPPORTED: i32 = 1048621i32;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_NO_SEPARATE_ALPHA_BLEND: i32 = 1048622i32;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_NO_MRT_BLEND: i32 = 1048623i32;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_OPERATION_NOT_SUPPORTED: i32 = 1048624i32;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_NO_MIRRORONCE: i32 = 1048625i32;
pub const D3D11_MESSAGE_ID_DRAWINSTANCED_NOT_SUPPORTED: i32 = 1048626i32;
pub const D3D11_MESSAGE_ID_DRAWINDEXEDINSTANCED_NOT_SUPPORTED_BELOW_9_3: i32 = 1048627i32;
pub const D3D11_MESSAGE_ID_DRAWINDEXED_POINTLIST_UNSUPPORTED: i32 = 1048628i32;
pub const D3D11_MESSAGE_ID_SETBLENDSTATE_SAMPLE_MASK_CANNOT_BE_ZERO: i32 = 1048629i32;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_DIMENSION_EXCEEDS_FEATURE_LEVEL_DEFINITION: i32 = 1048630i32;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_ONLY_SINGLE_MIP_LEVEL_DEPTH_STENCIL_SUPPORTED: i32 = 1048631i32;
pub const D3D11_MESSAGE_ID_DEVICE_RSSETSCISSORRECTS_NEGATIVESCISSOR: i32 = 1048632i32;
pub const D3D11_MESSAGE_ID_SLOT_ZERO_MUST_BE_D3D10_INPUT_PER_VERTEX_DATA: i32 = 1048633i32;
pub const D3D11_MESSAGE_ID_CREATERESOURCE_NON_POW_2_MIPMAP: i32 = 1048634i32;
pub const D3D11_MESSAGE_ID_CREATESAMPLERSTATE_BORDER_NOT_SUPPORTED: i32 = 1048635i32;
pub const D3D11_MESSAGE_ID_OMSETRENDERTARGETS_NO_SRGB_MRT: i32 = 1048636i32;
pub const D3D11_MESSAGE_ID_COPYRESOURCE_NO_3D_MISMATCHED_UPDATES: i32 = 1048637i32;
pub const D3D11_MESSAGE_ID_D3D10L9_MESSAGES_END: i32 = 1048638i32;
pub const D3D11_MESSAGE_ID_D3D11_MESSAGES_START: i32 = 2097152i32;
pub const D3D11_MESSAGE_ID_CREATEDEPTHSTENCILVIEW_INVALIDFLAGS: i32 = 2097153i32;
pub const D3D11_MESSAGE_ID_CREATEVERTEXSHADER_INVALIDCLASSLINKAGE: i32 = 2097154i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADER_INVALIDCLASSLINKAGE: i32 = 2097155i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMSTREAMS: i32 = 2097156i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTREAMTORASTERIZER: i32 = 2097157i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDSTREAMS: i32 = 2097158i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDCLASSLINKAGE: i32 = 2097159i32;
pub const D3D11_MESSAGE_ID_CREATEPIXELSHADER_INVALIDCLASSLINKAGE: i32 = 2097160i32;
pub const D3D11_MESSAGE_ID_CREATEDEFERREDCONTEXT_INVALID_COMMANDLISTFLAGS: i32 = 2097161i32;
pub const D3D11_MESSAGE_ID_CREATEDEFERREDCONTEXT_SINGLETHREADED: i32 = 2097162i32;
pub const D3D11_MESSAGE_ID_CREATEDEFERREDCONTEXT_INVALIDARG_RETURN: i32 = 2097163i32;
pub const D3D11_MESSAGE_ID_CREATEDEFERREDCONTEXT_INVALID_CALL_RETURN: i32 = 2097164i32;
pub const D3D11_MESSAGE_ID_CREATEDEFERREDCONTEXT_OUTOFMEMORY_RETURN: i32 = 2097165i32;
pub const D3D11_MESSAGE_ID_FINISHDISPLAYLIST_ONIMMEDIATECONTEXT: i32 = 2097166i32;
pub const D3D11_MESSAGE_ID_FINISHDISPLAYLIST_OUTOFMEMORY_RETURN: i32 = 2097167i32;
pub const D3D11_MESSAGE_ID_FINISHDISPLAYLIST_INVALID_CALL_RETURN: i32 = 2097168i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDSTREAM: i32 = 2097169i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDENTRIES: i32 = 2097170i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UNEXPECTEDSTRIDES: i32 = 2097171i32;
pub const D3D11_MESSAGE_ID_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_INVALIDNUMSTRIDES: i32 = 2097172i32;
pub const D3D11_MESSAGE_ID_DEVICE_HSSETSHADERRESOURCES_HAZARD: i32 = 2097173i32;
pub const D3D11_MESSAGE_ID_DEVICE_HSSETCONSTANTBUFFERS_HAZARD: i32 = 2097174i32;
pub const D3D11_MESSAGE_ID_HSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: i32 = 2097175i32;
pub const D3D11_MESSAGE_ID_HSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: i32 = 2097176i32;
pub const D3D11_MESSAGE_ID_CREATEHULLSHADER_INVALIDCALL: i32 = 2097177i32;
pub const D3D11_MESSAGE_ID_CREATEHULLSHADER_OUTOFMEMORY: i32 = 2097178i32;
pub const D3D11_MESSAGE_ID_CREATEHULLSHADER_INVALIDSHADERBYTECODE: i32 = 2097179i32;
pub const D3D11_MESSAGE_ID_CREATEHULLSHADER_INVALIDSHADERTYPE: i32 = 2097180i32;
pub const D3D11_MESSAGE_ID_CREATEHULLSHADER_INVALIDCLASSLINKAGE: i32 = 2097181i32;
pub const D3D11_MESSAGE_ID_DEVICE_HSSETSHADERRESOURCES_VIEWS_EMPTY: i32 = 2097182i32;
pub const D3D11_MESSAGE_ID_HSSETCONSTANTBUFFERS_INVALIDBUFFER: i32 = 2097183i32;
pub const D3D11_MESSAGE_ID_DEVICE_HSSETCONSTANTBUFFERS_BUFFERS_EMPTY: i32 = 2097184i32;
pub const D3D11_MESSAGE_ID_DEVICE_HSSETSAMPLERS_SAMPLERS_EMPTY: i32 = 2097185i32;
pub const D3D11_MESSAGE_ID_DEVICE_HSGETSHADERRESOURCES_VIEWS_EMPTY: i32 = 2097186i32;
pub const D3D11_MESSAGE_ID_DEVICE_HSGETCONSTANTBUFFERS_BUFFERS_EMPTY: i32 = 2097187i32;
pub const D3D11_MESSAGE_ID_DEVICE_HSGETSAMPLERS_SAMPLERS_EMPTY: i32 = 2097188i32;
pub const D3D11_MESSAGE_ID_DEVICE_DSSETSHADERRESOURCES_HAZARD: i32 = 2097189i32;
pub const D3D11_MESSAGE_ID_DEVICE_DSSETCONSTANTBUFFERS_HAZARD: i32 = 2097190i32;
pub const D3D11_MESSAGE_ID_DSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: i32 = 2097191i32;
pub const D3D11_MESSAGE_ID_DSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: i32 = 2097192i32;
pub const D3D11_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDCALL: i32 = 2097193i32;
pub const D3D11_MESSAGE_ID_CREATEDOMAINSHADER_OUTOFMEMORY: i32 = 2097194i32;
pub const D3D11_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDSHADERBYTECODE: i32 = 2097195i32;
pub const D3D11_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDSHADERTYPE: i32 = 2097196i32;
pub const D3D11_MESSAGE_ID_CREATEDOMAINSHADER_INVALIDCLASSLINKAGE: i32 = 2097197i32;
pub const D3D11_MESSAGE_ID_DEVICE_DSSETSHADERRESOURCES_VIEWS_EMPTY: i32 = 2097198i32;
pub const D3D11_MESSAGE_ID_DSSETCONSTANTBUFFERS_INVALIDBUFFER: i32 = 2097199i32;
pub const D3D11_MESSAGE_ID_DEVICE_DSSETCONSTANTBUFFERS_BUFFERS_EMPTY: i32 = 2097200i32;
pub const D3D11_MESSAGE_ID_DEVICE_DSSETSAMPLERS_SAMPLERS_EMPTY: i32 = 2097201i32;
pub const D3D11_MESSAGE_ID_DEVICE_DSGETSHADERRESOURCES_VIEWS_EMPTY: i32 = 2097202i32;
pub const D3D11_MESSAGE_ID_DEVICE_DSGETCONSTANTBUFFERS_BUFFERS_EMPTY: i32 = 2097203i32;
pub const D3D11_MESSAGE_ID_DEVICE_DSGETSAMPLERS_SAMPLERS_EMPTY: i32 = 2097204i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_HS_XOR_DS_MISMATCH: i32 = 2097205i32;
pub const D3D11_MESSAGE_ID_DEFERRED_CONTEXT_REMOVAL_PROCESS_AT_FAULT: i32 = 2097206i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINDIRECT_INVALID_ARG_BUFFER: i32 = 2097207i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINDIRECT_OFFSET_UNALIGNED: i32 = 2097208i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAWINDIRECT_OFFSET_OVERFLOW: i32 = 2097209i32;
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_INVALIDMAPTYPE: i32 = 2097210i32;
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_INVALIDSUBRESOURCE: i32 = 2097211i32;
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_INVALIDFLAGS: i32 = 2097212i32;
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_ALREADYMAPPED: i32 = 2097213i32;
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_DEVICEREMOVED_RETURN: i32 = 2097214i32;
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_OUTOFMEMORY_RETURN: i32 = 2097215i32;
pub const D3D11_MESSAGE_ID_RESOURCE_MAP_WITHOUT_INITIAL_DISCARD: i32 = 2097216i32;
pub const D3D11_MESSAGE_ID_RESOURCE_UNMAP_INVALIDSUBRESOURCE: i32 = 2097217i32;
pub const D3D11_MESSAGE_ID_RESOURCE_UNMAP_NOTMAPPED: i32 = 2097218i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RASTERIZING_CONTROL_POINTS: i32 = 2097219i32;
pub const D3D11_MESSAGE_ID_DEVICE_IASETPRIMITIVETOPOLOGY_TOPOLOGY_UNSUPPORTED: i32 = 2097220i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_HS_DS_SIGNATURE_MISMATCH: i32 = 2097221i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_HULL_SHADER_INPUT_TOPOLOGY_MISMATCH: i32 = 2097222i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_HS_DS_CONTROL_POINT_COUNT_MISMATCH: i32 = 2097223i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_HS_DS_TESSELLATOR_DOMAIN_MISMATCH: i32 = 2097224i32;
pub const D3D11_MESSAGE_ID_CREATE_CONTEXT: i32 = 2097225i32;
pub const D3D11_MESSAGE_ID_LIVE_CONTEXT: i32 = 2097226i32;
pub const D3D11_MESSAGE_ID_DESTROY_CONTEXT: i32 = 2097227i32;
pub const D3D11_MESSAGE_ID_CREATE_BUFFER: i32 = 2097228i32;
pub const D3D11_MESSAGE_ID_LIVE_BUFFER_WIN7: i32 = 2097229i32;
pub const D3D11_MESSAGE_ID_DESTROY_BUFFER: i32 = 2097230i32;
pub const D3D11_MESSAGE_ID_CREATE_TEXTURE1D: i32 = 2097231i32;
pub const D3D11_MESSAGE_ID_LIVE_TEXTURE1D_WIN7: i32 = 2097232i32;
pub const D3D11_MESSAGE_ID_DESTROY_TEXTURE1D: i32 = 2097233i32;
pub const D3D11_MESSAGE_ID_CREATE_TEXTURE2D: i32 = 2097234i32;
pub const D3D11_MESSAGE_ID_LIVE_TEXTURE2D_WIN7: i32 = 2097235i32;
pub const D3D11_MESSAGE_ID_DESTROY_TEXTURE2D: i32 = 2097236i32;
pub const D3D11_MESSAGE_ID_CREATE_TEXTURE3D: i32 = 2097237i32;
pub const D3D11_MESSAGE_ID_LIVE_TEXTURE3D_WIN7: i32 = 2097238i32;
pub const D3D11_MESSAGE_ID_DESTROY_TEXTURE3D: i32 = 2097239i32;
pub const D3D11_MESSAGE_ID_CREATE_SHADERRESOURCEVIEW: i32 = 2097240i32;
pub const D3D11_MESSAGE_ID_LIVE_SHADERRESOURCEVIEW_WIN7: i32 = 2097241i32;
pub const D3D11_MESSAGE_ID_DESTROY_SHADERRESOURCEVIEW: i32 = 2097242i32;
pub const D3D11_MESSAGE_ID_CREATE_RENDERTARGETVIEW: i32 = 2097243i32;
pub const D3D11_MESSAGE_ID_LIVE_RENDERTARGETVIEW_WIN7: i32 = 2097244i32;
pub const D3D11_MESSAGE_ID_DESTROY_RENDERTARGETVIEW: i32 = 2097245i32;
pub const D3D11_MESSAGE_ID_CREATE_DEPTHSTENCILVIEW: i32 = 2097246i32;
pub const D3D11_MESSAGE_ID_LIVE_DEPTHSTENCILVIEW_WIN7: i32 = 2097247i32;
pub const D3D11_MESSAGE_ID_DESTROY_DEPTHSTENCILVIEW: i32 = 2097248i32;
pub const D3D11_MESSAGE_ID_CREATE_VERTEXSHADER: i32 = 2097249i32;
pub const D3D11_MESSAGE_ID_LIVE_VERTEXSHADER_WIN7: i32 = 2097250i32;
pub const D3D11_MESSAGE_ID_DESTROY_VERTEXSHADER: i32 = 2097251i32;
pub const D3D11_MESSAGE_ID_CREATE_HULLSHADER: i32 = 2097252i32;
pub const D3D11_MESSAGE_ID_LIVE_HULLSHADER: i32 = 2097253i32;
pub const D3D11_MESSAGE_ID_DESTROY_HULLSHADER: i32 = 2097254i32;
pub const D3D11_MESSAGE_ID_CREATE_DOMAINSHADER: i32 = 2097255i32;
pub const D3D11_MESSAGE_ID_LIVE_DOMAINSHADER: i32 = 2097256i32;
pub const D3D11_MESSAGE_ID_DESTROY_DOMAINSHADER: i32 = 2097257i32;
pub const D3D11_MESSAGE_ID_CREATE_GEOMETRYSHADER: i32 = 2097258i32;
pub const D3D11_MESSAGE_ID_LIVE_GEOMETRYSHADER_WIN7: i32 = 2097259i32;
pub const D3D11_MESSAGE_ID_DESTROY_GEOMETRYSHADER: i32 = 2097260i32;
pub const D3D11_MESSAGE_ID_CREATE_PIXELSHADER: i32 = 2097261i32;
pub const D3D11_MESSAGE_ID_LIVE_PIXELSHADER_WIN7: i32 = 2097262i32;
pub const D3D11_MESSAGE_ID_DESTROY_PIXELSHADER: i32 = 2097263i32;
pub const D3D11_MESSAGE_ID_CREATE_INPUTLAYOUT: i32 = 2097264i32;
pub const D3D11_MESSAGE_ID_LIVE_INPUTLAYOUT_WIN7: i32 = 2097265i32;
pub const D3D11_MESSAGE_ID_DESTROY_INPUTLAYOUT: i32 = 2097266i32;
pub const D3D11_MESSAGE_ID_CREATE_SAMPLER: i32 = 2097267i32;
pub const D3D11_MESSAGE_ID_LIVE_SAMPLER_WIN7: i32 = 2097268i32;
pub const D3D11_MESSAGE_ID_DESTROY_SAMPLER: i32 = 2097269i32;
pub const D3D11_MESSAGE_ID_CREATE_BLENDSTATE: i32 = 2097270i32;
pub const D3D11_MESSAGE_ID_LIVE_BLENDSTATE_WIN7: i32 = 2097271i32;
pub const D3D11_MESSAGE_ID_DESTROY_BLENDSTATE: i32 = 2097272i32;
pub const D3D11_MESSAGE_ID_CREATE_DEPTHSTENCILSTATE: i32 = 2097273i32;
pub const D3D11_MESSAGE_ID_LIVE_DEPTHSTENCILSTATE_WIN7: i32 = 2097274i32;
pub const D3D11_MESSAGE_ID_DESTROY_DEPTHSTENCILSTATE: i32 = 2097275i32;
pub const D3D11_MESSAGE_ID_CREATE_RASTERIZERSTATE: i32 = 2097276i32;
pub const D3D11_MESSAGE_ID_LIVE_RASTERIZERSTATE_WIN7: i32 = 2097277i32;
pub const D3D11_MESSAGE_ID_DESTROY_RASTERIZERSTATE: i32 = 2097278i32;
pub const D3D11_MESSAGE_ID_CREATE_QUERY: i32 = 2097279i32;
pub const D3D11_MESSAGE_ID_LIVE_QUERY_WIN7: i32 = 2097280i32;
pub const D3D11_MESSAGE_ID_DESTROY_QUERY: i32 = 2097281i32;
pub const D3D11_MESSAGE_ID_CREATE_PREDICATE: i32 = 2097282i32;
pub const D3D11_MESSAGE_ID_LIVE_PREDICATE_WIN7: i32 = 2097283i32;
pub const D3D11_MESSAGE_ID_DESTROY_PREDICATE: i32 = 2097284i32;
pub const D3D11_MESSAGE_ID_CREATE_COUNTER: i32 = 2097285i32;
pub const D3D11_MESSAGE_ID_DESTROY_COUNTER: i32 = 2097286i32;
pub const D3D11_MESSAGE_ID_CREATE_COMMANDLIST: i32 = 2097287i32;
pub const D3D11_MESSAGE_ID_LIVE_COMMANDLIST: i32 = 2097288i32;
pub const D3D11_MESSAGE_ID_DESTROY_COMMANDLIST: i32 = 2097289i32;
pub const D3D11_MESSAGE_ID_CREATE_CLASSINSTANCE: i32 = 2097290i32;
pub const D3D11_MESSAGE_ID_LIVE_CLASSINSTANCE: i32 = 2097291i32;
pub const D3D11_MESSAGE_ID_DESTROY_CLASSINSTANCE: i32 = 2097292i32;
pub const D3D11_MESSAGE_ID_CREATE_CLASSLINKAGE: i32 = 2097293i32;
pub const D3D11_MESSAGE_ID_LIVE_CLASSLINKAGE: i32 = 2097294i32;
pub const D3D11_MESSAGE_ID_DESTROY_CLASSLINKAGE: i32 = 2097295i32;
pub const D3D11_MESSAGE_ID_LIVE_DEVICE_WIN7: i32 = 2097296i32;
pub const D3D11_MESSAGE_ID_LIVE_OBJECT_SUMMARY_WIN7: i32 = 2097297i32;
pub const D3D11_MESSAGE_ID_CREATE_COMPUTESHADER: i32 = 2097298i32;
pub const D3D11_MESSAGE_ID_LIVE_COMPUTESHADER: i32 = 2097299i32;
pub const D3D11_MESSAGE_ID_DESTROY_COMPUTESHADER: i32 = 2097300i32;
pub const D3D11_MESSAGE_ID_CREATE_UNORDEREDACCESSVIEW: i32 = 2097301i32;
pub const D3D11_MESSAGE_ID_LIVE_UNORDEREDACCESSVIEW: i32 = 2097302i32;
pub const D3D11_MESSAGE_ID_DESTROY_UNORDEREDACCESSVIEW: i32 = 2097303i32;
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INTERFACES_FEATURELEVEL: i32 = 2097304i32;
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INTERFACE_COUNT_MISMATCH: i32 = 2097305i32;
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INVALID_INSTANCE: i32 = 2097306i32;
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INVALID_INSTANCE_INDEX: i32 = 2097307i32;
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INVALID_INSTANCE_TYPE: i32 = 2097308i32;
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INVALID_INSTANCE_DATA: i32 = 2097309i32;
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_UNBOUND_INSTANCE_DATA: i32 = 2097310i32;
pub const D3D11_MESSAGE_ID_DEVICE_SETSHADER_INSTANCE_DATA_BINDINGS: i32 = 2097311i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATESHADER_CLASSLINKAGE_FULL: i32 = 2097312i32;
pub const D3D11_MESSAGE_ID_DEVICE_CHECKFEATURESUPPORT_UNRECOGNIZED_FEATURE: i32 = 2097313i32;
pub const D3D11_MESSAGE_ID_DEVICE_CHECKFEATURESUPPORT_MISMATCHED_DATA_SIZE: i32 = 2097314i32;
pub const D3D11_MESSAGE_ID_DEVICE_CHECKFEATURESUPPORT_INVALIDARG_RETURN: i32 = 2097315i32;
pub const D3D11_MESSAGE_ID_DEVICE_CSSETSHADERRESOURCES_HAZARD: i32 = 2097316i32;
pub const D3D11_MESSAGE_ID_DEVICE_CSSETCONSTANTBUFFERS_HAZARD: i32 = 2097317i32;
pub const D3D11_MESSAGE_ID_CSSETSHADERRESOURCES_UNBINDDELETINGOBJECT: i32 = 2097318i32;
pub const D3D11_MESSAGE_ID_CSSETCONSTANTBUFFERS_UNBINDDELETINGOBJECT: i32 = 2097319i32;
pub const D3D11_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDCALL: i32 = 2097320i32;
pub const D3D11_MESSAGE_ID_CREATECOMPUTESHADER_OUTOFMEMORY: i32 = 2097321i32;
pub const D3D11_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDSHADERBYTECODE: i32 = 2097322i32;
pub const D3D11_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDSHADERTYPE: i32 = 2097323i32;
pub const D3D11_MESSAGE_ID_CREATECOMPUTESHADER_INVALIDCLASSLINKAGE: i32 = 2097324i32;
pub const D3D11_MESSAGE_ID_DEVICE_CSSETSHADERRESOURCES_VIEWS_EMPTY: i32 = 2097325i32;
pub const D3D11_MESSAGE_ID_CSSETCONSTANTBUFFERS_INVALIDBUFFER: i32 = 2097326i32;
pub const D3D11_MESSAGE_ID_DEVICE_CSSETCONSTANTBUFFERS_BUFFERS_EMPTY: i32 = 2097327i32;
pub const D3D11_MESSAGE_ID_DEVICE_CSSETSAMPLERS_SAMPLERS_EMPTY: i32 = 2097328i32;
pub const D3D11_MESSAGE_ID_DEVICE_CSGETSHADERRESOURCES_VIEWS_EMPTY: i32 = 2097329i32;
pub const D3D11_MESSAGE_ID_DEVICE_CSGETCONSTANTBUFFERS_BUFFERS_EMPTY: i32 = 2097330i32;
pub const D3D11_MESSAGE_ID_DEVICE_CSGETSAMPLERS_SAMPLERS_EMPTY: i32 = 2097331i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_DOUBLEFLOATOPSNOTSUPPORTED: i32 = 2097332i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEHULLSHADER_DOUBLEFLOATOPSNOTSUPPORTED: i32 = 2097333i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_DOUBLEFLOATOPSNOTSUPPORTED: i32 = 2097334i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_DOUBLEFLOATOPSNOTSUPPORTED: i32 = 2097335i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DOUBLEFLOATOPSNOTSUPPORTED: i32 = 2097336i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_DOUBLEFLOATOPSNOTSUPPORTED: i32 = 2097337i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_DOUBLEFLOATOPSNOTSUPPORTED: i32 = 2097338i32;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDSTRUCTURESTRIDE: i32 = 2097339i32;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDFLAGS: i32 = 2097340i32;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDRESOURCE: i32 = 2097341i32;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDESC: i32 = 2097342i32;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDFORMAT: i32 = 2097343i32;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDIMENSIONS: i32 = 2097344i32;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_UNRECOGNIZEDFORMAT: i32 = 2097345i32;
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETSANDUNORDEREDACCESSVIEWS_HAZARD: i32 = 2097346i32;
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETSANDUNORDEREDACCESSVIEWS_OVERLAPPING_OLD_SLOTS: i32 = 2097347i32;
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETSANDUNORDEREDACCESSVIEWS_NO_OP: i32 = 2097348i32;
pub const D3D11_MESSAGE_ID_CSSETUNORDEREDACCESSVIEWS_UNBINDDELETINGOBJECT: i32 = 2097349i32;
pub const D3D11_MESSAGE_ID_PSSETUNORDEREDACCESSVIEWS_UNBINDDELETINGOBJECT: i32 = 2097350i32;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDARG_RETURN: i32 = 2097351i32;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_OUTOFMEMORY_RETURN: i32 = 2097352i32;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_TOOMANYOBJECTS: i32 = 2097353i32;
pub const D3D11_MESSAGE_ID_DEVICE_CSSETUNORDEREDACCESSVIEWS_HAZARD: i32 = 2097354i32;
pub const D3D11_MESSAGE_ID_CLEARUNORDEREDACCESSVIEW_DENORMFLUSH: i32 = 2097355i32;
pub const D3D11_MESSAGE_ID_DEVICE_CSSETUNORDEREDACCESSS_VIEWS_EMPTY: i32 = 2097356i32;
pub const D3D11_MESSAGE_ID_DEVICE_CSGETUNORDEREDACCESSS_VIEWS_EMPTY: i32 = 2097357i32;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDFLAGS: i32 = 2097358i32;
pub const D3D11_MESSAGE_ID_CREATESHADERRESESOURCEVIEW_TOOMANYOBJECTS: i32 = 2097359i32;
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCHINDIRECT_INVALID_ARG_BUFFER: i32 = 2097360i32;
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCHINDIRECT_OFFSET_UNALIGNED: i32 = 2097361i32;
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCHINDIRECT_OFFSET_OVERFLOW: i32 = 2097362i32;
pub const D3D11_MESSAGE_ID_DEVICE_SETRESOURCEMINLOD_INVALIDCONTEXT: i32 = 2097363i32;
pub const D3D11_MESSAGE_ID_DEVICE_SETRESOURCEMINLOD_INVALIDRESOURCE: i32 = 2097364i32;
pub const D3D11_MESSAGE_ID_DEVICE_SETRESOURCEMINLOD_INVALIDMINLOD: i32 = 2097365i32;
pub const D3D11_MESSAGE_ID_DEVICE_GETRESOURCEMINLOD_INVALIDCONTEXT: i32 = 2097366i32;
pub const D3D11_MESSAGE_ID_DEVICE_GETRESOURCEMINLOD_INVALIDRESOURCE: i32 = 2097367i32;
pub const D3D11_MESSAGE_ID_OMSETDEPTHSTENCIL_UNBINDDELETINGOBJECT: i32 = 2097368i32;
pub const D3D11_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_DEPTH_READONLY: i32 = 2097369i32;
pub const D3D11_MESSAGE_ID_CLEARDEPTHSTENCILVIEW_STENCIL_READONLY: i32 = 2097370i32;
pub const D3D11_MESSAGE_ID_CHECKFEATURESUPPORT_FORMAT_DEPRECATED: i32 = 2097371i32;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_RETURN_TYPE_MISMATCH: i32 = 2097372i32;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_NOT_SET: i32 = 2097373i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_UNORDEREDACCESSVIEW_RENDERTARGETVIEW_OVERLAP: i32 = 2097374i32;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_DIMENSION_MISMATCH: i32 = 2097375i32;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_APPEND_UNSUPPORTED: i32 = 2097376i32;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMICS_UNSUPPORTED: i32 = 2097377i32;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_STRUCTURE_STRIDE_MISMATCH: i32 = 2097378i32;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_BUFFER_TYPE_MISMATCH: i32 = 2097379i32;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_RAW_UNSUPPORTED: i32 = 2097380i32;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_FORMAT_LD_UNSUPPORTED: i32 = 2097381i32;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_FORMAT_STORE_UNSUPPORTED: i32 = 2097382i32;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMIC_ADD_UNSUPPORTED: i32 = 2097383i32;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMIC_BITWISE_OPS_UNSUPPORTED: i32 = 2097384i32;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMIC_CMPSTORE_CMPEXCHANGE_UNSUPPORTED: i32 = 2097385i32;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMIC_EXCHANGE_UNSUPPORTED: i32 = 2097386i32;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMIC_SIGNED_MINMAX_UNSUPPORTED: i32 = 2097387i32;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_ATOMIC_UNSIGNED_MINMAX_UNSUPPORTED: i32 = 2097388i32;
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCH_BOUND_RESOURCE_MAPPED: i32 = 2097389i32;
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCH_THREADGROUPCOUNT_OVERFLOW: i32 = 2097390i32;
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCH_THREADGROUPCOUNT_ZERO: i32 = 2097391i32;
pub const D3D11_MESSAGE_ID_DEVICE_SHADERRESOURCEVIEW_STRUCTURE_STRIDE_MISMATCH: i32 = 2097392i32;
pub const D3D11_MESSAGE_ID_DEVICE_SHADERRESOURCEVIEW_BUFFER_TYPE_MISMATCH: i32 = 2097393i32;
pub const D3D11_MESSAGE_ID_DEVICE_SHADERRESOURCEVIEW_RAW_UNSUPPORTED: i32 = 2097394i32;
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCH_UNSUPPORTED: i32 = 2097395i32;
pub const D3D11_MESSAGE_ID_DEVICE_DISPATCHINDIRECT_UNSUPPORTED: i32 = 2097396i32;
pub const D3D11_MESSAGE_ID_COPYSTRUCTURECOUNT_INVALIDOFFSET: i32 = 2097397i32;
pub const D3D11_MESSAGE_ID_COPYSTRUCTURECOUNT_LARGEOFFSET: i32 = 2097398i32;
pub const D3D11_MESSAGE_ID_COPYSTRUCTURECOUNT_INVALIDDESTINATIONSTATE: i32 = 2097399i32;
pub const D3D11_MESSAGE_ID_COPYSTRUCTURECOUNT_INVALIDSOURCESTATE: i32 = 2097400i32;
pub const D3D11_MESSAGE_ID_CHECKFORMATSUPPORT_FORMAT_NOT_SUPPORTED: i32 = 2097401i32;
pub const D3D11_MESSAGE_ID_DEVICE_CSSETUNORDEREDACCESSVIEWS_INVALIDVIEW: i32 = 2097402i32;
pub const D3D11_MESSAGE_ID_DEVICE_CSSETUNORDEREDACCESSVIEWS_INVALIDOFFSET: i32 = 2097403i32;
pub const D3D11_MESSAGE_ID_DEVICE_CSSETUNORDEREDACCESSVIEWS_TOOMANYVIEWS: i32 = 2097404i32;
pub const D3D11_MESSAGE_ID_CLEARUNORDEREDACCESSVIEWFLOAT_INVALIDFORMAT: i32 = 2097405i32;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_COUNTER_UNSUPPORTED: i32 = 2097406i32;
pub const D3D11_MESSAGE_ID_REF_WARNING: i32 = 2097407i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_PIXEL_SHADER_WITHOUT_RTV_OR_DSV: i32 = 2097408i32;
pub const D3D11_MESSAGE_ID_SHADER_ABORT: i32 = 2097409i32;
pub const D3D11_MESSAGE_ID_SHADER_MESSAGE: i32 = 2097410i32;
pub const D3D11_MESSAGE_ID_SHADER_ERROR: i32 = 2097411i32;
pub const D3D11_MESSAGE_ID_OFFERRESOURCES_INVALIDRESOURCE: i32 = 2097412i32;
pub const D3D11_MESSAGE_ID_HSSETSAMPLERS_UNBINDDELETINGOBJECT: i32 = 2097413i32;
pub const D3D11_MESSAGE_ID_DSSETSAMPLERS_UNBINDDELETINGOBJECT: i32 = 2097414i32;
pub const D3D11_MESSAGE_ID_CSSETSAMPLERS_UNBINDDELETINGOBJECT: i32 = 2097415i32;
pub const D3D11_MESSAGE_ID_HSSETSHADER_UNBINDDELETINGOBJECT: i32 = 2097416i32;
pub const D3D11_MESSAGE_ID_DSSETSHADER_UNBINDDELETINGOBJECT: i32 = 2097417i32;
pub const D3D11_MESSAGE_ID_CSSETSHADER_UNBINDDELETINGOBJECT: i32 = 2097418i32;
pub const D3D11_MESSAGE_ID_ENQUEUESETEVENT_INVALIDARG_RETURN: i32 = 2097419i32;
pub const D3D11_MESSAGE_ID_ENQUEUESETEVENT_OUTOFMEMORY_RETURN: i32 = 2097420i32;
pub const D3D11_MESSAGE_ID_ENQUEUESETEVENT_ACCESSDENIED_RETURN: i32 = 2097421i32;
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETSANDUNORDEREDACCESSVIEWS_NUMUAVS_INVALIDRANGE: i32 = 2097422i32;
pub const D3D11_MESSAGE_ID_USE_OF_ZERO_REFCOUNT_OBJECT: i32 = 2097423i32;
pub const D3D11_MESSAGE_ID_D3D11_MESSAGES_END: i32 = 2097424i32;
pub const D3D11_MESSAGE_ID_D3D11_1_MESSAGES_START: i32 = 3145728i32;
pub const D3D11_MESSAGE_ID_CREATE_VIDEODECODER: i32 = 3145729i32;
pub const D3D11_MESSAGE_ID_CREATE_VIDEOPROCESSORENUM: i32 = 3145730i32;
pub const D3D11_MESSAGE_ID_CREATE_VIDEOPROCESSOR: i32 = 3145731i32;
pub const D3D11_MESSAGE_ID_CREATE_DECODEROUTPUTVIEW: i32 = 3145732i32;
pub const D3D11_MESSAGE_ID_CREATE_PROCESSORINPUTVIEW: i32 = 3145733i32;
pub const D3D11_MESSAGE_ID_CREATE_PROCESSOROUTPUTVIEW: i32 = 3145734i32;
pub const D3D11_MESSAGE_ID_CREATE_DEVICECONTEXTSTATE: i32 = 3145735i32;
pub const D3D11_MESSAGE_ID_LIVE_VIDEODECODER: i32 = 3145736i32;
pub const D3D11_MESSAGE_ID_LIVE_VIDEOPROCESSORENUM: i32 = 3145737i32;
pub const D3D11_MESSAGE_ID_LIVE_VIDEOPROCESSOR: i32 = 3145738i32;
pub const D3D11_MESSAGE_ID_LIVE_DECODEROUTPUTVIEW: i32 = 3145739i32;
pub const D3D11_MESSAGE_ID_LIVE_PROCESSORINPUTVIEW: i32 = 3145740i32;
pub const D3D11_MESSAGE_ID_LIVE_PROCESSOROUTPUTVIEW: i32 = 3145741i32;
pub const D3D11_MESSAGE_ID_LIVE_DEVICECONTEXTSTATE: i32 = 3145742i32;
pub const D3D11_MESSAGE_ID_DESTROY_VIDEODECODER: i32 = 3145743i32;
pub const D3D11_MESSAGE_ID_DESTROY_VIDEOPROCESSORENUM: i32 = 3145744i32;
pub const D3D11_MESSAGE_ID_DESTROY_VIDEOPROCESSOR: i32 = 3145745i32;
pub const D3D11_MESSAGE_ID_DESTROY_DECODEROUTPUTVIEW: i32 = 3145746i32;
pub const D3D11_MESSAGE_ID_DESTROY_PROCESSORINPUTVIEW: i32 = 3145747i32;
pub const D3D11_MESSAGE_ID_DESTROY_PROCESSOROUTPUTVIEW: i32 = 3145748i32;
pub const D3D11_MESSAGE_ID_DESTROY_DEVICECONTEXTSTATE: i32 = 3145749i32;
pub const D3D11_MESSAGE_ID_CREATEDEVICECONTEXTSTATE_INVALIDFLAGS: i32 = 3145750i32;
pub const D3D11_MESSAGE_ID_CREATEDEVICECONTEXTSTATE_INVALIDFEATURELEVEL: i32 = 3145751i32;
pub const D3D11_MESSAGE_ID_CREATEDEVICECONTEXTSTATE_FEATURELEVELS_NOT_SUPPORTED: i32 = 3145752i32;
pub const D3D11_MESSAGE_ID_CREATEDEVICECONTEXTSTATE_INVALIDREFIID: i32 = 3145753i32;
pub const D3D11_MESSAGE_ID_DEVICE_DISCARDVIEW_INVALIDVIEW: i32 = 3145754i32;
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION1_INVALIDCOPYFLAGS: i32 = 3145755i32;
pub const D3D11_MESSAGE_ID_UPDATESUBRESOURCE1_INVALIDCOPYFLAGS: i32 = 3145756i32;
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_INVALIDFORCEDSAMPLECOUNT: i32 = 3145757i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODER_OUTOFMEMORY_RETURN: i32 = 3145758i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODER_NULLPARAM: i32 = 3145759i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODER_INVALIDFORMAT: i32 = 3145760i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODER_ZEROWIDTHHEIGHT: i32 = 3145761i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODER_DRIVER_INVALIDBUFFERSIZE: i32 = 3145762i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODER_DRIVER_INVALIDBUFFERUSAGE: i32 = 3145763i32;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERPROFILECOUNT_OUTOFMEMORY: i32 = 3145764i32;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERPROFILE_NULLPARAM: i32 = 3145765i32;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERPROFILE_INVALIDINDEX: i32 = 3145766i32;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERPROFILE_OUTOFMEMORY_RETURN: i32 = 3145767i32;
pub const D3D11_MESSAGE_ID_CHECKVIDEODECODERFORMAT_NULLPARAM: i32 = 3145768i32;
pub const D3D11_MESSAGE_ID_CHECKVIDEODECODERFORMAT_OUTOFMEMORY_RETURN: i32 = 3145769i32;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCONFIGCOUNT_NULLPARAM: i32 = 3145770i32;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCONFIGCOUNT_OUTOFMEMORY_RETURN: i32 = 3145771i32;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCONFIG_NULLPARAM: i32 = 3145772i32;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCONFIG_INVALIDINDEX: i32 = 3145773i32;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCONFIG_OUTOFMEMORY_RETURN: i32 = 3145774i32;
pub const D3D11_MESSAGE_ID_GETDECODERCREATIONPARAMS_NULLPARAM: i32 = 3145775i32;
pub const D3D11_MESSAGE_ID_GETDECODERDRIVERHANDLE_NULLPARAM: i32 = 3145776i32;
pub const D3D11_MESSAGE_ID_GETDECODERBUFFER_NULLPARAM: i32 = 3145777i32;
pub const D3D11_MESSAGE_ID_GETDECODERBUFFER_INVALIDBUFFER: i32 = 3145778i32;
pub const D3D11_MESSAGE_ID_GETDECODERBUFFER_INVALIDTYPE: i32 = 3145779i32;
pub const D3D11_MESSAGE_ID_GETDECODERBUFFER_LOCKED: i32 = 3145780i32;
pub const D3D11_MESSAGE_ID_RELEASEDECODERBUFFER_NULLPARAM: i32 = 3145781i32;
pub const D3D11_MESSAGE_ID_RELEASEDECODERBUFFER_INVALIDTYPE: i32 = 3145782i32;
pub const D3D11_MESSAGE_ID_RELEASEDECODERBUFFER_NOTLOCKED: i32 = 3145783i32;
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_NULLPARAM: i32 = 3145784i32;
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_HAZARD: i32 = 3145785i32;
pub const D3D11_MESSAGE_ID_DECODERENDFRAME_NULLPARAM: i32 = 3145786i32;
pub const D3D11_MESSAGE_ID_SUBMITDECODERBUFFERS_NULLPARAM: i32 = 3145787i32;
pub const D3D11_MESSAGE_ID_SUBMITDECODERBUFFERS_INVALIDTYPE: i32 = 3145788i32;
pub const D3D11_MESSAGE_ID_DECODEREXTENSION_NULLPARAM: i32 = 3145789i32;
pub const D3D11_MESSAGE_ID_DECODEREXTENSION_INVALIDRESOURCE: i32 = 3145790i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_OUTOFMEMORY_RETURN: i32 = 3145791i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_NULLPARAM: i32 = 3145792i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_INVALIDFRAMEFORMAT: i32 = 3145793i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_INVALIDUSAGE: i32 = 3145794i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_INVALIDINPUTFRAMERATE: i32 = 3145795i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_INVALIDOUTPUTFRAMERATE: i32 = 3145796i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORENUMERATOR_INVALIDWIDTHHEIGHT: i32 = 3145797i32;
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORCONTENTDESC_NULLPARAM: i32 = 3145798i32;
pub const D3D11_MESSAGE_ID_CHECKVIDEOPROCESSORFORMAT_NULLPARAM: i32 = 3145799i32;
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORCAPS_NULLPARAM: i32 = 3145800i32;
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORRATECONVERSIONCAPS_NULLPARAM: i32 = 3145801i32;
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORRATECONVERSIONCAPS_INVALIDINDEX: i32 = 3145802i32;
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORCUSTOMRATE_NULLPARAM: i32 = 3145803i32;
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORCUSTOMRATE_INVALIDINDEX: i32 = 3145804i32;
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORFILTERRANGE_NULLPARAM: i32 = 3145805i32;
pub const D3D11_MESSAGE_ID_GETVIDEOPROCESSORFILTERRANGE_UNSUPPORTED: i32 = 3145806i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOR_OUTOFMEMORY_RETURN: i32 = 3145807i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOR_NULLPARAM: i32 = 3145808i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTTARGETRECT_NULLPARAM: i32 = 3145809i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTBACKGROUNDCOLOR_NULLPARAM: i32 = 3145810i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTBACKGROUNDCOLOR_INVALIDALPHA: i32 = 3145811i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTCOLORSPACE_NULLPARAM: i32 = 3145812i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTALPHAFILLMODE_NULLPARAM: i32 = 3145813i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTALPHAFILLMODE_UNSUPPORTED: i32 = 3145814i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTALPHAFILLMODE_INVALIDSTREAM: i32 = 3145815i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTALPHAFILLMODE_INVALIDFILLMODE: i32 = 3145816i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTCONSTRICTION_NULLPARAM: i32 = 3145817i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTSTEREOMODE_NULLPARAM: i32 = 3145818i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTSTEREOMODE_UNSUPPORTED: i32 = 3145819i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTEXTENSION_NULLPARAM: i32 = 3145820i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTTARGETRECT_NULLPARAM: i32 = 3145821i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTBACKGROUNDCOLOR_NULLPARAM: i32 = 3145822i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTCOLORSPACE_NULLPARAM: i32 = 3145823i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTALPHAFILLMODE_NULLPARAM: i32 = 3145824i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTCONSTRICTION_NULLPARAM: i32 = 3145825i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTCONSTRICTION_UNSUPPORTED: i32 = 3145826i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTCONSTRICTION_INVALIDSIZE: i32 = 3145827i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTSTEREOMODE_NULLPARAM: i32 = 3145828i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTEXTENSION_NULLPARAM: i32 = 3145829i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFRAMEFORMAT_NULLPARAM: i32 = 3145830i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFRAMEFORMAT_INVALIDFORMAT: i32 = 3145831i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFRAMEFORMAT_INVALIDSTREAM: i32 = 3145832i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMCOLORSPACE_NULLPARAM: i32 = 3145833i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMCOLORSPACE_INVALIDSTREAM: i32 = 3145834i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMOUTPUTRATE_NULLPARAM: i32 = 3145835i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMOUTPUTRATE_INVALIDRATE: i32 = 3145836i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMOUTPUTRATE_INVALIDFLAG: i32 = 3145837i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMOUTPUTRATE_INVALIDSTREAM: i32 = 3145838i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSOURCERECT_NULLPARAM: i32 = 3145839i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSOURCERECT_INVALIDSTREAM: i32 = 3145840i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSOURCERECT_INVALIDRECT: i32 = 3145841i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMDESTRECT_NULLPARAM: i32 = 3145842i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMDESTRECT_INVALIDSTREAM: i32 = 3145843i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMDESTRECT_INVALIDRECT: i32 = 3145844i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMALPHA_NULLPARAM: i32 = 3145845i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMALPHA_INVALIDSTREAM: i32 = 3145846i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMALPHA_INVALIDALPHA: i32 = 3145847i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPALETTE_NULLPARAM: i32 = 3145848i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPALETTE_INVALIDSTREAM: i32 = 3145849i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPALETTE_INVALIDCOUNT: i32 = 3145850i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPALETTE_INVALIDALPHA: i32 = 3145851i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPIXELASPECTRATIO_NULLPARAM: i32 = 3145852i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPIXELASPECTRATIO_INVALIDSTREAM: i32 = 3145853i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPIXELASPECTRATIO_INVALIDRATIO: i32 = 3145854i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMLUMAKEY_NULLPARAM: i32 = 3145855i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMLUMAKEY_INVALIDSTREAM: i32 = 3145856i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMLUMAKEY_INVALIDRANGE: i32 = 3145857i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMLUMAKEY_UNSUPPORTED: i32 = 3145858i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_NULLPARAM: i32 = 3145859i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_INVALIDSTREAM: i32 = 3145860i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_UNSUPPORTED: i32 = 3145861i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_FLIPUNSUPPORTED: i32 = 3145862i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_MONOOFFSETUNSUPPORTED: i32 = 3145863i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_FORMATUNSUPPORTED: i32 = 3145864i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMSTEREOFORMAT_INVALIDFORMAT: i32 = 3145865i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMAUTOPROCESSINGMODE_NULLPARAM: i32 = 3145866i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMAUTOPROCESSINGMODE_INVALIDSTREAM: i32 = 3145867i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFILTER_NULLPARAM: i32 = 3145868i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFILTER_INVALIDSTREAM: i32 = 3145869i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFILTER_INVALIDFILTER: i32 = 3145870i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFILTER_UNSUPPORTED: i32 = 3145871i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMFILTER_INVALIDLEVEL: i32 = 3145872i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMEXTENSION_NULLPARAM: i32 = 3145873i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMEXTENSION_INVALIDSTREAM: i32 = 3145874i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMFRAMEFORMAT_NULLPARAM: i32 = 3145875i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMCOLORSPACE_NULLPARAM: i32 = 3145876i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMOUTPUTRATE_NULLPARAM: i32 = 3145877i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMSOURCERECT_NULLPARAM: i32 = 3145878i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMDESTRECT_NULLPARAM: i32 = 3145879i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMALPHA_NULLPARAM: i32 = 3145880i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMPALETTE_NULLPARAM: i32 = 3145881i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMPIXELASPECTRATIO_NULLPARAM: i32 = 3145882i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMLUMAKEY_NULLPARAM: i32 = 3145883i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMSTEREOFORMAT_NULLPARAM: i32 = 3145884i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMAUTOPROCESSINGMODE_NULLPARAM: i32 = 3145885i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMFILTER_NULLPARAM: i32 = 3145886i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMEXTENSION_NULLPARAM: i32 = 3145887i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMEXTENSION_INVALIDSTREAM: i32 = 3145888i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_NULLPARAM: i32 = 3145889i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDSTREAMCOUNT: i32 = 3145890i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_TARGETRECT: i32 = 3145891i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDOUTPUT: i32 = 3145892i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDPASTFRAMES: i32 = 3145893i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDFUTUREFRAMES: i32 = 3145894i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDSOURCERECT: i32 = 3145895i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDDESTRECT: i32 = 3145896i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDINPUTRESOURCE: i32 = 3145897i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDARRAYSIZE: i32 = 3145898i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDARRAY: i32 = 3145899i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_RIGHTEXPECTED: i32 = 3145900i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_RIGHTNOTEXPECTED: i32 = 3145901i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_STEREONOTENABLED: i32 = 3145902i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INVALIDRIGHTRESOURCE: i32 = 3145903i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_NOSTEREOSTREAMS: i32 = 3145904i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_INPUTHAZARD: i32 = 3145905i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORBLT_OUTPUTHAZARD: i32 = 3145906i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_OUTOFMEMORY_RETURN: i32 = 3145907i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_NULLPARAM: i32 = 3145908i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_INVALIDTYPE: i32 = 3145909i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_INVALIDBIND: i32 = 3145910i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_UNSUPPORTEDFORMAT: i32 = 3145911i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_INVALIDMIP: i32 = 3145912i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_UNSUPPORTEMIP: i32 = 3145913i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_INVALIDARRAYSIZE: i32 = 3145914i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_INVALIDARRAY: i32 = 3145915i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEODECODEROUTPUTVIEW_INVALIDDIMENSION: i32 = 3145916i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_OUTOFMEMORY_RETURN: i32 = 3145917i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_NULLPARAM: i32 = 3145918i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDTYPE: i32 = 3145919i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDBIND: i32 = 3145920i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDMISC: i32 = 3145921i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDUSAGE: i32 = 3145922i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDFORMAT: i32 = 3145923i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDFOURCC: i32 = 3145924i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDMIP: i32 = 3145925i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_UNSUPPORTEDMIP: i32 = 3145926i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDARRAYSIZE: i32 = 3145927i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDARRAY: i32 = 3145928i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDDIMENSION: i32 = 3145929i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_OUTOFMEMORY_RETURN: i32 = 3145930i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_NULLPARAM: i32 = 3145931i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDTYPE: i32 = 3145932i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDBIND: i32 = 3145933i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDFORMAT: i32 = 3145934i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDMIP: i32 = 3145935i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_UNSUPPORTEDMIP: i32 = 3145936i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_UNSUPPORTEDARRAY: i32 = 3145937i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDARRAY: i32 = 3145938i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDDIMENSION: i32 = 3145939i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INVALID_USE_OF_FORCED_SAMPLE_COUNT: i32 = 3145940i32;
pub const D3D11_MESSAGE_ID_CREATEBLENDSTATE_INVALIDLOGICOPS: i32 = 3145941i32;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDDARRAYWITHDECODER: i32 = 3145942i32;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDDARRAYWITHDECODER: i32 = 3145943i32;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDDARRAYWITHDECODER: i32 = 3145944i32;
pub const D3D11_MESSAGE_ID_DEVICE_LOCKEDOUT_INTERFACE: i32 = 3145945i32;
pub const D3D11_MESSAGE_ID_REF_WARNING_ATOMIC_INCONSISTENT: i32 = 3145946i32;
pub const D3D11_MESSAGE_ID_REF_WARNING_READING_UNINITIALIZED_RESOURCE: i32 = 3145947i32;
pub const D3D11_MESSAGE_ID_REF_WARNING_RAW_HAZARD: i32 = 3145948i32;
pub const D3D11_MESSAGE_ID_REF_WARNING_WAR_HAZARD: i32 = 3145949i32;
pub const D3D11_MESSAGE_ID_REF_WARNING_WAW_HAZARD: i32 = 3145950i32;
pub const D3D11_MESSAGE_ID_CREATECRYPTOSESSION_NULLPARAM: i32 = 3145951i32;
pub const D3D11_MESSAGE_ID_CREATECRYPTOSESSION_OUTOFMEMORY_RETURN: i32 = 3145952i32;
pub const D3D11_MESSAGE_ID_GETCRYPTOTYPE_NULLPARAM: i32 = 3145953i32;
pub const D3D11_MESSAGE_ID_GETDECODERPROFILE_NULLPARAM: i32 = 3145954i32;
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONCERTIFICATESIZE_NULLPARAM: i32 = 3145955i32;
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONCERTIFICATE_NULLPARAM: i32 = 3145956i32;
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONCERTIFICATE_WRONGSIZE: i32 = 3145957i32;
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONHANDLE_WRONGSIZE: i32 = 3145958i32;
pub const D3D11_MESSAGE_ID_NEGOTIATECRPYTOSESSIONKEYEXCHANGE_NULLPARAM: i32 = 3145959i32;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_UNSUPPORTED: i32 = 3145960i32;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_NULLPARAM: i32 = 3145961i32;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_SRC_WRONGDEVICE: i32 = 3145962i32;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_DST_WRONGDEVICE: i32 = 3145963i32;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_FORMAT_MISMATCH: i32 = 3145964i32;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_SIZE_MISMATCH: i32 = 3145965i32;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_SRC_MULTISAMPLED: i32 = 3145966i32;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_DST_NOT_STAGING: i32 = 3145967i32;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_SRC_MAPPED: i32 = 3145968i32;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_DST_MAPPED: i32 = 3145969i32;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_SRC_OFFERED: i32 = 3145970i32;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_DST_OFFERED: i32 = 3145971i32;
pub const D3D11_MESSAGE_ID_ENCRYPTIONBLT_SRC_CONTENT_UNDEFINED: i32 = 3145972i32;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_UNSUPPORTED: i32 = 3145973i32;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_NULLPARAM: i32 = 3145974i32;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_SRC_WRONGDEVICE: i32 = 3145975i32;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_DST_WRONGDEVICE: i32 = 3145976i32;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_FORMAT_MISMATCH: i32 = 3145977i32;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_SIZE_MISMATCH: i32 = 3145978i32;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_DST_MULTISAMPLED: i32 = 3145979i32;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_SRC_NOT_STAGING: i32 = 3145980i32;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_DST_NOT_RENDER_TARGET: i32 = 3145981i32;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_SRC_MAPPED: i32 = 3145982i32;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_DST_MAPPED: i32 = 3145983i32;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_SRC_OFFERED: i32 = 3145984i32;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_DST_OFFERED: i32 = 3145985i32;
pub const D3D11_MESSAGE_ID_DECRYPTIONBLT_SRC_CONTENT_UNDEFINED: i32 = 3145986i32;
pub const D3D11_MESSAGE_ID_STARTSESSIONKEYREFRESH_NULLPARAM: i32 = 3145987i32;
pub const D3D11_MESSAGE_ID_STARTSESSIONKEYREFRESH_INVALIDSIZE: i32 = 3145988i32;
pub const D3D11_MESSAGE_ID_FINISHSESSIONKEYREFRESH_NULLPARAM: i32 = 3145989i32;
pub const D3D11_MESSAGE_ID_GETENCRYPTIONBLTKEY_NULLPARAM: i32 = 3145990i32;
pub const D3D11_MESSAGE_ID_GETENCRYPTIONBLTKEY_INVALIDSIZE: i32 = 3145991i32;
pub const D3D11_MESSAGE_ID_GETCONTENTPROTECTIONCAPS_NULLPARAM: i32 = 3145992i32;
pub const D3D11_MESSAGE_ID_CHECKCRYPTOKEYEXCHANGE_NULLPARAM: i32 = 3145993i32;
pub const D3D11_MESSAGE_ID_CHECKCRYPTOKEYEXCHANGE_INVALIDINDEX: i32 = 3145994i32;
pub const D3D11_MESSAGE_ID_CREATEAUTHENTICATEDCHANNEL_NULLPARAM: i32 = 3145995i32;
pub const D3D11_MESSAGE_ID_CREATEAUTHENTICATEDCHANNEL_UNSUPPORTED: i32 = 3145996i32;
pub const D3D11_MESSAGE_ID_CREATEAUTHENTICATEDCHANNEL_INVALIDTYPE: i32 = 3145997i32;
pub const D3D11_MESSAGE_ID_CREATEAUTHENTICATEDCHANNEL_OUTOFMEMORY_RETURN: i32 = 3145998i32;
pub const D3D11_MESSAGE_ID_GETAUTHENTICATEDCHANNELCERTIFICATESIZE_INVALIDCHANNEL: i32 = 3145999i32;
pub const D3D11_MESSAGE_ID_GETAUTHENTICATEDCHANNELCERTIFICATESIZE_NULLPARAM: i32 = 3146000i32;
pub const D3D11_MESSAGE_ID_GETAUTHENTICATEDCHANNELCERTIFICATE_INVALIDCHANNEL: i32 = 3146001i32;
pub const D3D11_MESSAGE_ID_GETAUTHENTICATEDCHANNELCERTIFICATE_NULLPARAM: i32 = 3146002i32;
pub const D3D11_MESSAGE_ID_GETAUTHENTICATEDCHANNELCERTIFICATE_WRONGSIZE: i32 = 3146003i32;
pub const D3D11_MESSAGE_ID_NEGOTIATEAUTHENTICATEDCHANNELKEYEXCHANGE_INVALIDCHANNEL: i32 = 3146004i32;
pub const D3D11_MESSAGE_ID_NEGOTIATEAUTHENTICATEDCHANNELKEYEXCHANGE_NULLPARAM: i32 = 3146005i32;
pub const D3D11_MESSAGE_ID_QUERYAUTHENTICATEDCHANNEL_NULLPARAM: i32 = 3146006i32;
pub const D3D11_MESSAGE_ID_QUERYAUTHENTICATEDCHANNEL_WRONGCHANNEL: i32 = 3146007i32;
pub const D3D11_MESSAGE_ID_QUERYAUTHENTICATEDCHANNEL_UNSUPPORTEDQUERY: i32 = 3146008i32;
pub const D3D11_MESSAGE_ID_QUERYAUTHENTICATEDCHANNEL_WRONGSIZE: i32 = 3146009i32;
pub const D3D11_MESSAGE_ID_QUERYAUTHENTICATEDCHANNEL_INVALIDPROCESSINDEX: i32 = 3146010i32;
pub const D3D11_MESSAGE_ID_CONFIGUREAUTHENTICATEDCHANNEL_NULLPARAM: i32 = 3146011i32;
pub const D3D11_MESSAGE_ID_CONFIGUREAUTHENTICATEDCHANNEL_WRONGCHANNEL: i32 = 3146012i32;
pub const D3D11_MESSAGE_ID_CONFIGUREAUTHENTICATEDCHANNEL_UNSUPPORTEDCONFIGURE: i32 = 3146013i32;
pub const D3D11_MESSAGE_ID_CONFIGUREAUTHENTICATEDCHANNEL_WRONGSIZE: i32 = 3146014i32;
pub const D3D11_MESSAGE_ID_CONFIGUREAUTHENTICATEDCHANNEL_INVALIDPROCESSIDTYPE: i32 = 3146015i32;
pub const D3D11_MESSAGE_ID_VSSETCONSTANTBUFFERS_INVALIDBUFFEROFFSETORCOUNT: i32 = 3146016i32;
pub const D3D11_MESSAGE_ID_DSSETCONSTANTBUFFERS_INVALIDBUFFEROFFSETORCOUNT: i32 = 3146017i32;
pub const D3D11_MESSAGE_ID_HSSETCONSTANTBUFFERS_INVALIDBUFFEROFFSETORCOUNT: i32 = 3146018i32;
pub const D3D11_MESSAGE_ID_GSSETCONSTANTBUFFERS_INVALIDBUFFEROFFSETORCOUNT: i32 = 3146019i32;
pub const D3D11_MESSAGE_ID_PSSETCONSTANTBUFFERS_INVALIDBUFFEROFFSETORCOUNT: i32 = 3146020i32;
pub const D3D11_MESSAGE_ID_CSSETCONSTANTBUFFERS_INVALIDBUFFEROFFSETORCOUNT: i32 = 3146021i32;
pub const D3D11_MESSAGE_ID_NEGOTIATECRPYTOSESSIONKEYEXCHANGE_INVALIDSIZE: i32 = 3146022i32;
pub const D3D11_MESSAGE_ID_NEGOTIATEAUTHENTICATEDCHANNELKEYEXCHANGE_INVALIDSIZE: i32 = 3146023i32;
pub const D3D11_MESSAGE_ID_OFFERRESOURCES_INVALIDPRIORITY: i32 = 3146024i32;
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONHANDLE_OUTOFMEMORY: i32 = 3146025i32;
pub const D3D11_MESSAGE_ID_ACQUIREHANDLEFORCAPTURE_NULLPARAM: i32 = 3146026i32;
pub const D3D11_MESSAGE_ID_ACQUIREHANDLEFORCAPTURE_INVALIDTYPE: i32 = 3146027i32;
pub const D3D11_MESSAGE_ID_ACQUIREHANDLEFORCAPTURE_INVALIDBIND: i32 = 3146028i32;
pub const D3D11_MESSAGE_ID_ACQUIREHANDLEFORCAPTURE_INVALIDARRAY: i32 = 3146029i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMROTATION_NULLPARAM: i32 = 3146030i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMROTATION_INVALIDSTREAM: i32 = 3146031i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMROTATION_INVALID: i32 = 3146032i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMROTATION_UNSUPPORTED: i32 = 3146033i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMROTATION_NULLPARAM: i32 = 3146034i32;
pub const D3D11_MESSAGE_ID_DEVICE_CLEARVIEW_INVALIDVIEW: i32 = 3146035i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: i32 = 3146036i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_SHADEREXTENSIONSNOTSUPPORTED: i32 = 3146037i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEHULLSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: i32 = 3146038i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEHULLSHADER_SHADEREXTENSIONSNOTSUPPORTED: i32 = 3146039i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: i32 = 3146040i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_SHADEREXTENSIONSNOTSUPPORTED: i32 = 3146041i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: i32 = 3146042i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_SHADEREXTENSIONSNOTSUPPORTED: i32 = 3146043i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_DOUBLEEXTENSIONSNOTSUPPORTED: i32 = 3146044i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_SHADEREXTENSIONSNOTSUPPORTED: i32 = 3146045i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_DOUBLEEXTENSIONSNOTSUPPORTED: i32 = 3146046i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_SHADEREXTENSIONSNOTSUPPORTED: i32 = 3146047i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_DOUBLEEXTENSIONSNOTSUPPORTED: i32 = 3146048i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_SHADEREXTENSIONSNOTSUPPORTED: i32 = 3146049i32;
pub const D3D11_MESSAGE_ID_DEVICE_SHADER_LINKAGE_MINPRECISION: i32 = 3146050i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMALPHA_UNSUPPORTED: i32 = 3146051i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMPIXELASPECTRATIO_UNSUPPORTED: i32 = 3146052i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEVERTEXSHADER_UAVSNOTSUPPORTED: i32 = 3146053i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEHULLSHADER_UAVSNOTSUPPORTED: i32 = 3146054i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEDOMAINSHADER_UAVSNOTSUPPORTED: i32 = 3146055i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADER_UAVSNOTSUPPORTED: i32 = 3146056i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEGEOMETRYSHADERWITHSTREAMOUTPUT_UAVSNOTSUPPORTED: i32 = 3146057i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATEPIXELSHADER_UAVSNOTSUPPORTED: i32 = 3146058i32;
pub const D3D11_MESSAGE_ID_DEVICE_CREATECOMPUTESHADER_UAVSNOTSUPPORTED: i32 = 3146059i32;
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETSANDUNORDEREDACCESSVIEWS_INVALIDOFFSET: i32 = 3146060i32;
pub const D3D11_MESSAGE_ID_DEVICE_OMSETRENDERTARGETSANDUNORDEREDACCESSVIEWS_TOOMANYVIEWS: i32 = 3146061i32;
pub const D3D11_MESSAGE_ID_DEVICE_CLEARVIEW_NOTSUPPORTED: i32 = 3146062i32;
pub const D3D11_MESSAGE_ID_SWAPDEVICECONTEXTSTATE_NOTSUPPORTED: i32 = 3146063i32;
pub const D3D11_MESSAGE_ID_UPDATESUBRESOURCE_PREFERUPDATESUBRESOURCE1: i32 = 3146064i32;
pub const D3D11_MESSAGE_ID_GETDC_INACCESSIBLE: i32 = 3146065i32;
pub const D3D11_MESSAGE_ID_DEVICE_CLEARVIEW_INVALIDRECT: i32 = 3146066i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_SAMPLE_MASK_IGNORED_ON_FL9: i32 = 3146067i32;
pub const D3D11_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE1_NOT_SUPPORTED: i32 = 3146068i32;
pub const D3D11_MESSAGE_ID_DEVICE_OPEN_SHARED_RESOURCE_BY_NAME_NOT_SUPPORTED: i32 = 3146069i32;
pub const D3D11_MESSAGE_ID_ENQUEUESETEVENT_NOT_SUPPORTED: i32 = 3146070i32;
pub const D3D11_MESSAGE_ID_OFFERRELEASE_NOT_SUPPORTED: i32 = 3146071i32;
pub const D3D11_MESSAGE_ID_OFFERRESOURCES_INACCESSIBLE: i32 = 3146072i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSORINPUTVIEW_INVALIDMSAA: i32 = 3146073i32;
pub const D3D11_MESSAGE_ID_CREATEVIDEOPROCESSOROUTPUTVIEW_INVALIDMSAA: i32 = 3146074i32;
pub const D3D11_MESSAGE_ID_DEVICE_CLEARVIEW_INVALIDSOURCERECT: i32 = 3146075i32;
pub const D3D11_MESSAGE_ID_DEVICE_CLEARVIEW_EMPTYRECT: i32 = 3146076i32;
pub const D3D11_MESSAGE_ID_UPDATESUBRESOURCE_EMPTYDESTBOX: i32 = 3146077i32;
pub const D3D11_MESSAGE_ID_COPYSUBRESOURCEREGION_EMPTYSOURCEBOX: i32 = 3146078i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_OM_RENDER_TARGET_DOES_NOT_SUPPORT_LOGIC_OPS: i32 = 3146079i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_DEPTHSTENCILVIEW_NOT_SET: i32 = 3146080i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RENDERTARGETVIEW_NOT_SET: i32 = 3146081i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_RENDERTARGETVIEW_NOT_SET_DUE_TO_FLIP_PRESENT: i32 = 3146082i32;
pub const D3D11_MESSAGE_ID_DEVICE_UNORDEREDACCESSVIEW_NOT_SET_DUE_TO_FLIP_PRESENT: i32 = 3146083i32;
pub const D3D11_MESSAGE_ID_GETDATAFORNEWHARDWAREKEY_NULLPARAM: i32 = 3146084i32;
pub const D3D11_MESSAGE_ID_CHECKCRYPTOSESSIONSTATUS_NULLPARAM: i32 = 3146085i32;
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONPRIVATEDATASIZE_NULLPARAM: i32 = 3146086i32;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCAPS_NULLPARAM: i32 = 3146087i32;
pub const D3D11_MESSAGE_ID_GETVIDEODECODERCAPS_ZEROWIDTHHEIGHT: i32 = 3146088i32;
pub const D3D11_MESSAGE_ID_CHECKVIDEODECODERDOWNSAMPLING_NULLPARAM: i32 = 3146089i32;
pub const D3D11_MESSAGE_ID_CHECKVIDEODECODERDOWNSAMPLING_INVALIDCOLORSPACE: i32 = 3146090i32;
pub const D3D11_MESSAGE_ID_CHECKVIDEODECODERDOWNSAMPLING_ZEROWIDTHHEIGHT: i32 = 3146091i32;
pub const D3D11_MESSAGE_ID_VIDEODECODERENABLEDOWNSAMPLING_NULLPARAM: i32 = 3146092i32;
pub const D3D11_MESSAGE_ID_VIDEODECODERENABLEDOWNSAMPLING_UNSUPPORTED: i32 = 3146093i32;
pub const D3D11_MESSAGE_ID_VIDEODECODERUPDATEDOWNSAMPLING_NULLPARAM: i32 = 3146094i32;
pub const D3D11_MESSAGE_ID_VIDEODECODERUPDATEDOWNSAMPLING_UNSUPPORTED: i32 = 3146095i32;
pub const D3D11_MESSAGE_ID_CHECKVIDEOPROCESSORFORMATCONVERSION_NULLPARAM: i32 = 3146096i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTCOLORSPACE1_NULLPARAM: i32 = 3146097i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTCOLORSPACE1_NULLPARAM: i32 = 3146098i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMCOLORSPACE1_NULLPARAM: i32 = 3146099i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMCOLORSPACE1_INVALIDSTREAM: i32 = 3146100i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMMIRROR_NULLPARAM: i32 = 3146101i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMMIRROR_INVALIDSTREAM: i32 = 3146102i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMMIRROR_UNSUPPORTED: i32 = 3146103i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMCOLORSPACE1_NULLPARAM: i32 = 3146104i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMMIRROR_NULLPARAM: i32 = 3146105i32;
pub const D3D11_MESSAGE_ID_RECOMMENDVIDEODECODERDOWNSAMPLING_NULLPARAM: i32 = 3146106i32;
pub const D3D11_MESSAGE_ID_RECOMMENDVIDEODECODERDOWNSAMPLING_INVALIDCOLORSPACE: i32 = 3146107i32;
pub const D3D11_MESSAGE_ID_RECOMMENDVIDEODECODERDOWNSAMPLING_ZEROWIDTHHEIGHT: i32 = 3146108i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTSHADERUSAGE_NULLPARAM: i32 = 3146109i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTSHADERUSAGE_NULLPARAM: i32 = 3146110i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETBEHAVIORHINTS_NULLPARAM: i32 = 3146111i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETBEHAVIORHINTS_INVALIDSTREAMCOUNT: i32 = 3146112i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETBEHAVIORHINTS_TARGETRECT: i32 = 3146113i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETBEHAVIORHINTS_INVALIDSOURCERECT: i32 = 3146114i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETBEHAVIORHINTS_INVALIDDESTRECT: i32 = 3146115i32;
pub const D3D11_MESSAGE_ID_GETCRYPTOSESSIONPRIVATEDATASIZE_INVALID_KEY_EXCHANGE_TYPE: i32 = 3146116i32;
pub const D3D11_MESSAGE_ID_D3D11_1_MESSAGES_END: i32 = 3146117i32;
pub const D3D11_MESSAGE_ID_D3D11_2_MESSAGES_START: i32 = 3146118i32;
pub const D3D11_MESSAGE_ID_CREATEBUFFER_INVALIDUSAGE: i32 = 3146119i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE1D_INVALIDUSAGE: i32 = 3146120i32;
pub const D3D11_MESSAGE_ID_CREATETEXTURE2D_INVALIDUSAGE: i32 = 3146121i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_LEVEL9_STEPRATE_NOT_1: i32 = 3146122i32;
pub const D3D11_MESSAGE_ID_CREATEINPUTLAYOUT_LEVEL9_INSTANCING_NOT_SUPPORTED: i32 = 3146123i32;
pub const D3D11_MESSAGE_ID_UPDATETILEMAPPINGS_INVALID_PARAMETER: i32 = 3146124i32;
pub const D3D11_MESSAGE_ID_COPYTILEMAPPINGS_INVALID_PARAMETER: i32 = 3146125i32;
pub const D3D11_MESSAGE_ID_COPYTILES_INVALID_PARAMETER: i32 = 3146126i32;
pub const D3D11_MESSAGE_ID_UPDATETILES_INVALID_PARAMETER: i32 = 3146127i32;
pub const D3D11_MESSAGE_ID_RESIZETILEPOOL_INVALID_PARAMETER: i32 = 3146128i32;
pub const D3D11_MESSAGE_ID_TILEDRESOURCEBARRIER_INVALID_PARAMETER: i32 = 3146129i32;
pub const D3D11_MESSAGE_ID_NULL_TILE_MAPPING_ACCESS_WARNING: i32 = 3146130i32;
pub const D3D11_MESSAGE_ID_NULL_TILE_MAPPING_ACCESS_ERROR: i32 = 3146131i32;
pub const D3D11_MESSAGE_ID_DIRTY_TILE_MAPPING_ACCESS: i32 = 3146132i32;
pub const D3D11_MESSAGE_ID_DUPLICATE_TILE_MAPPINGS_IN_COVERED_AREA: i32 = 3146133i32;
pub const D3D11_MESSAGE_ID_TILE_MAPPINGS_IN_COVERED_AREA_DUPLICATED_OUTSIDE: i32 = 3146134i32;
pub const D3D11_MESSAGE_ID_TILE_MAPPINGS_SHARED_BETWEEN_INCOMPATIBLE_RESOURCES: i32 = 3146135i32;
pub const D3D11_MESSAGE_ID_TILE_MAPPINGS_SHARED_BETWEEN_INPUT_AND_OUTPUT: i32 = 3146136i32;
pub const D3D11_MESSAGE_ID_CHECKMULTISAMPLEQUALITYLEVELS_INVALIDFLAGS: i32 = 3146137i32;
pub const D3D11_MESSAGE_ID_GETRESOURCETILING_NONTILED_RESOURCE: i32 = 3146138i32;
pub const D3D11_MESSAGE_ID_RESIZETILEPOOL_SHRINK_WITH_MAPPINGS_STILL_DEFINED_PAST_END: i32 = 3146139i32;
pub const D3D11_MESSAGE_ID_NEED_TO_CALL_TILEDRESOURCEBARRIER: i32 = 3146140i32;
pub const D3D11_MESSAGE_ID_CREATEDEVICE_INVALIDARGS: i32 = 3146141i32;
pub const D3D11_MESSAGE_ID_CREATEDEVICE_WARNING: i32 = 3146142i32;
pub const D3D11_MESSAGE_ID_CLEARUNORDEREDACCESSVIEWUINT_HAZARD: i32 = 3146143i32;
pub const D3D11_MESSAGE_ID_CLEARUNORDEREDACCESSVIEWFLOAT_HAZARD: i32 = 3146144i32;
pub const D3D11_MESSAGE_ID_TILED_RESOURCE_TIER_1_BUFFER_TEXTURE_MISMATCH: i32 = 3146145i32;
pub const D3D11_MESSAGE_ID_CREATE_CRYPTOSESSION: i32 = 3146146i32;
pub const D3D11_MESSAGE_ID_CREATE_AUTHENTICATEDCHANNEL: i32 = 3146147i32;
pub const D3D11_MESSAGE_ID_LIVE_CRYPTOSESSION: i32 = 3146148i32;
pub const D3D11_MESSAGE_ID_LIVE_AUTHENTICATEDCHANNEL: i32 = 3146149i32;
pub const D3D11_MESSAGE_ID_DESTROY_CRYPTOSESSION: i32 = 3146150i32;
pub const D3D11_MESSAGE_ID_DESTROY_AUTHENTICATEDCHANNEL: i32 = 3146151i32;
pub const D3D11_MESSAGE_ID_D3D11_2_MESSAGES_END: i32 = 3146152i32;
pub const D3D11_MESSAGE_ID_D3D11_3_MESSAGES_START: i32 = 3146153i32;
pub const D3D11_MESSAGE_ID_CREATERASTERIZERSTATE_INVALID_CONSERVATIVERASTERMODE: i32 = 3146154i32;
pub const D3D11_MESSAGE_ID_DEVICE_DRAW_INVALID_SYSTEMVALUE: i32 = 3146155i32;
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_INVALIDCONTEXTTYPE: i32 = 3146156i32;
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_DECODENOTSUPPORTED: i32 = 3146157i32;
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_ENCODENOTSUPPORTED: i32 = 3146158i32;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDPLANEINDEX: i32 = 3146159i32;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_INVALIDVIDEOPLANEINDEX: i32 = 3146160i32;
pub const D3D11_MESSAGE_ID_CREATESHADERRESOURCEVIEW_AMBIGUOUSVIDEOPLANEINDEX: i32 = 3146161i32;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDPLANEINDEX: i32 = 3146162i32;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_INVALIDVIDEOPLANEINDEX: i32 = 3146163i32;
pub const D3D11_MESSAGE_ID_CREATERENDERTARGETVIEW_AMBIGUOUSVIDEOPLANEINDEX: i32 = 3146164i32;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDPLANEINDEX: i32 = 3146165i32;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_INVALIDVIDEOPLANEINDEX: i32 = 3146166i32;
pub const D3D11_MESSAGE_ID_CREATEUNORDEREDACCESSVIEW_AMBIGUOUSVIDEOPLANEINDEX: i32 = 3146167i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDSCANDATAOFFSET: i32 = 3146168i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_NOTSUPPORTED: i32 = 3146169i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_DIMENSIONSTOOLARGE: i32 = 3146170i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDCOMPONENTS: i32 = 3146171i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_DESTINATIONNOT2D: i32 = 3146172i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_TILEDRESOURCESUNSUPPORTED: i32 = 3146173i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_GUARDRECTSUNSUPPORTED: i32 = 3146174i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_FORMATUNSUPPORTED: i32 = 3146175i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDSUBRESOURCE: i32 = 3146176i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDMIPLEVEL: i32 = 3146177i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_EMPTYDESTBOX: i32 = 3146178i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_DESTBOXNOT2D: i32 = 3146179i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_DESTBOXNOTSUB: i32 = 3146180i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_DESTBOXESINTERSECT: i32 = 3146181i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_XSUBSAMPLEMISMATCH: i32 = 3146182i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_YSUBSAMPLEMISMATCH: i32 = 3146183i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_XSUBSAMPLEODD: i32 = 3146184i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_YSUBSAMPLEODD: i32 = 3146185i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_OUTPUTDIMENSIONSTOOLARGE: i32 = 3146186i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_NONPOW2SCALEUNSUPPORTED: i32 = 3146187i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_FRACTIONALDOWNSCALETOLARGE: i32 = 3146188i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_CHROMASIZEMISMATCH: i32 = 3146189i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_LUMACHROMASIZEMISMATCH: i32 = 3146190i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDNUMDESTINATIONS: i32 = 3146191i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_SUBBOXUNSUPPORTED: i32 = 3146192i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_1DESTUNSUPPORTEDFORMAT: i32 = 3146193i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_3DESTUNSUPPORTEDFORMAT: i32 = 3146194i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_SCALEUNSUPPORTED: i32 = 3146195i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDSOURCESIZE: i32 = 3146196i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_INVALIDCOPYFLAGS: i32 = 3146197i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_HAZARD: i32 = 3146198i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_UNSUPPORTEDSRCBUFFERUSAGE: i32 = 3146199i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_UNSUPPORTEDSRCBUFFERMISCFLAGS: i32 = 3146200i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_UNSUPPORTEDDSTTEXTUREUSAGE: i32 = 3146201i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_BACKBUFFERNOTSUPPORTED: i32 = 3146202i32;
pub const D3D11_MESSAGE_ID_JPEGDECODE_UNSUPPRTEDCOPYFLAGS: i32 = 3146203i32;
pub const D3D11_MESSAGE_ID_JPEGENCODE_NOTSUPPORTED: i32 = 3146204i32;
pub const D3D11_MESSAGE_ID_JPEGENCODE_INVALIDSCANDATAOFFSET: i32 = 3146205i32;
pub const D3D11_MESSAGE_ID_JPEGENCODE_INVALIDCOMPONENTS: i32 = 3146206i32;
pub const D3D11_MESSAGE_ID_JPEGENCODE_SOURCENOT2D: i32 = 3146207i32;
pub const D3D11_MESSAGE_ID_JPEGENCODE_TILEDRESOURCESUNSUPPORTED: i32 = 3146208i32;
pub const D3D11_MESSAGE_ID_JPEGENCODE_GUARDRECTSUNSUPPORTED: i32 = 3146209i32;
pub const D3D11_MESSAGE_ID_JPEGENCODE_XSUBSAMPLEMISMATCH: i32 = 3146210i32;
pub const D3D11_MESSAGE_ID_JPEGENCODE_YSUBSAMPLEMISMATCH: i32 = 3146211i32;
pub const D3D11_MESSAGE_ID_JPEGENCODE_FORMATUNSUPPORTED: i32 = 3146212i32;
pub const D3D11_MESSAGE_ID_JPEGENCODE_INVALIDSUBRESOURCE: i32 = 3146213i32;
pub const D3D11_MESSAGE_ID_JPEGENCODE_INVALIDMIPLEVEL: i32 = 3146214i32;
pub const D3D11_MESSAGE_ID_JPEGENCODE_DIMENSIONSTOOLARGE: i32 = 3146215i32;
pub const D3D11_MESSAGE_ID_JPEGENCODE_HAZARD: i32 = 3146216i32;
pub const D3D11_MESSAGE_ID_JPEGENCODE_UNSUPPORTEDDSTBUFFERUSAGE: i32 = 3146217i32;
pub const D3D11_MESSAGE_ID_JPEGENCODE_UNSUPPORTEDDSTBUFFERMISCFLAGS: i32 = 3146218i32;
pub const D3D11_MESSAGE_ID_JPEGENCODE_UNSUPPORTEDSRCTEXTUREUSAGE: i32 = 3146219i32;
pub const D3D11_MESSAGE_ID_JPEGENCODE_BACKBUFFERNOTSUPPORTED: i32 = 3146220i32;
pub const D3D11_MESSAGE_ID_CREATEQUERYORPREDICATE_UNSUPPORTEDCONTEXTTTYPEFORQUERY: i32 = 3146221i32;
pub const D3D11_MESSAGE_ID_FLUSH1_INVALIDCONTEXTTYPE: i32 = 3146222i32;
pub const D3D11_MESSAGE_ID_DEVICE_SETHARDWAREPROTECTION_INVALIDCONTEXT: i32 = 3146223i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTHDRMETADATA_NULLPARAM: i32 = 3146224i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETOUTPUTHDRMETADATA_INVALIDSIZE: i32 = 3146225i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTHDRMETADATA_NULLPARAM: i32 = 3146226i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETOUTPUTHDRMETADATA_INVALIDSIZE: i32 = 3146227i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMHDRMETADATA_NULLPARAM: i32 = 3146228i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMHDRMETADATA_INVALIDSTREAM: i32 = 3146229i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORSETSTREAMHDRMETADATA_INVALIDSIZE: i32 = 3146230i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMHDRMETADATA_NULLPARAM: i32 = 3146231i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMHDRMETADATA_INVALIDSTREAM: i32 = 3146232i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMHDRMETADATA_INVALIDSIZE: i32 = 3146233i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMFRAMEFORMAT_INVALIDSTREAM: i32 = 3146234i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMCOLORSPACE_INVALIDSTREAM: i32 = 3146235i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMOUTPUTRATE_INVALIDSTREAM: i32 = 3146236i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMSOURCERECT_INVALIDSTREAM: i32 = 3146237i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMDESTRECT_INVALIDSTREAM: i32 = 3146238i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMALPHA_INVALIDSTREAM: i32 = 3146239i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMPALETTE_INVALIDSTREAM: i32 = 3146240i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMPIXELASPECTRATIO_INVALIDSTREAM: i32 = 3146241i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMLUMAKEY_INVALIDSTREAM: i32 = 3146242i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMSTEREOFORMAT_INVALIDSTREAM: i32 = 3146243i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMAUTOPROCESSINGMODE_INVALIDSTREAM: i32 = 3146244i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMFILTER_INVALIDSTREAM: i32 = 3146245i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMROTATION_INVALIDSTREAM: i32 = 3146246i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMCOLORSPACE1_INVALIDSTREAM: i32 = 3146247i32;
pub const D3D11_MESSAGE_ID_VIDEOPROCESSORGETSTREAMMIRROR_INVALIDSTREAM: i32 = 3146248i32;
pub const D3D11_MESSAGE_ID_CREATE_FENCE: i32 = 3146249i32;
pub const D3D11_MESSAGE_ID_LIVE_FENCE: i32 = 3146250i32;
pub const D3D11_MESSAGE_ID_DESTROY_FENCE: i32 = 3146251i32;
pub const D3D11_MESSAGE_ID_CREATE_SYNCHRONIZEDCHANNEL: i32 = 3146252i32;
pub const D3D11_MESSAGE_ID_LIVE_SYNCHRONIZEDCHANNEL: i32 = 3146253i32;
pub const D3D11_MESSAGE_ID_DESTROY_SYNCHRONIZEDCHANNEL: i32 = 3146254i32;
pub const D3D11_MESSAGE_ID_CREATEFENCE_INVALIDFLAGS: i32 = 3146255i32;
pub const D3D11_MESSAGE_ID_D3D11_3_MESSAGES_END: i32 = 3146256i32;
pub const D3D11_MESSAGE_ID_D3D11_5_MESSAGES_START: i32 = 3146257i32;
pub const D3D11_MESSAGE_ID_NEGOTIATECRYPTOSESSIONKEYEXCHANGEMT_INVALIDKEYEXCHANGETYPE: i32 = 3146258i32;
pub const D3D11_MESSAGE_ID_NEGOTIATECRYPTOSESSIONKEYEXCHANGEMT_NOT_SUPPORTED: i32 = 3146259i32;
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_INVALID_HISTOGRAM_COMPONENT_COUNT: i32 = 3146260i32;
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_INVALID_HISTOGRAM_COMPONENT: i32 = 3146261i32;
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_INVALID_HISTOGRAM_BUFFER_SIZE: i32 = 3146262i32;
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_INVALID_HISTOGRAM_BUFFER_USAGE: i32 = 3146263i32;
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_INVALID_HISTOGRAM_BUFFER_MISC_FLAGS: i32 = 3146264i32;
pub const D3D11_MESSAGE_ID_DECODERBEGINFRAME_INVALID_HISTOGRAM_BUFFER_OFFSET: i32 = 3146265i32;
pub const D3D11_MESSAGE_ID_CREATE_TRACKEDWORKLOAD: i32 = 3146266i32;
pub const D3D11_MESSAGE_ID_LIVE_TRACKEDWORKLOAD: i32 = 3146267i32;
pub const D3D11_MESSAGE_ID_DESTROY_TRACKEDWORKLOAD: i32 = 3146268i32;
pub const D3D11_MESSAGE_ID_CREATE_TRACKED_WORKLOAD_NULLPARAM: i32 = 3146269i32;
pub const D3D11_MESSAGE_ID_CREATE_TRACKED_WORKLOAD_INVALID_MAX_INSTANCES: i32 = 3146270i32;
pub const D3D11_MESSAGE_ID_CREATE_TRACKED_WORKLOAD_INVALID_DEADLINE_TYPE: i32 = 3146271i32;
pub const D3D11_MESSAGE_ID_CREATE_TRACKED_WORKLOAD_INVALID_ENGINE_TYPE: i32 = 3146272i32;
pub const D3D11_MESSAGE_ID_MULTIPLE_TRACKED_WORKLOADS: i32 = 3146273i32;
pub const D3D11_MESSAGE_ID_MULTIPLE_TRACKED_WORKLOAD_PAIRS: i32 = 3146274i32;
pub const D3D11_MESSAGE_ID_INCOMPLETE_TRACKED_WORKLOAD_PAIR: i32 = 3146275i32;
pub const D3D11_MESSAGE_ID_OUT_OF_ORDER_TRACKED_WORKLOAD_PAIR: i32 = 3146276i32;
pub const D3D11_MESSAGE_ID_CANNOT_ADD_TRACKED_WORKLOAD: i32 = 3146277i32;
pub const D3D11_MESSAGE_ID_TRACKED_WORKLOAD_NOT_SUPPORTED: i32 = 3146278i32;
pub const D3D11_MESSAGE_ID_TRACKED_WORKLOAD_ENGINE_TYPE_NOT_FOUND: i32 = 3146279i32;
pub const D3D11_MESSAGE_ID_NO_TRACKED_WORKLOAD_SLOT_AVAILABLE: i32 = 3146280i32;
pub const D3D11_MESSAGE_ID_END_TRACKED_WORKLOAD_INVALID_ARG: i32 = 3146281i32;
pub const D3D11_MESSAGE_ID_TRACKED_WORKLOAD_DISJOINT_FAILURE: i32 = 3146282i32;
pub const D3D11_MESSAGE_ID_D3D11_5_MESSAGES_END: i32 = 3146283i32;
pub const D3D11_MESSAGE_SEVERITY_CORRUPTION: i32 = 0i32;
pub const D3D11_MESSAGE_SEVERITY_ERROR: i32 = 1i32;
pub const D3D11_MESSAGE_SEVERITY_WARNING: i32 = 2i32;
pub const D3D11_MESSAGE_SEVERITY_INFO: i32 = 3i32;
pub const D3D11_MESSAGE_SEVERITY_MESSAGE: i32 = 4i32;
pub const D3D11_MINOR_VERSION: u32 = 0u32;
pub const D3D11_MIN_BORDER_COLOR_COMPONENT: f32 = 0f32;
pub const D3D11_MIN_DEPTH: f32 = 0f32;
pub const D3D11_MIN_FILTER_SHIFT: u32 = 4u32;
pub const D3D11_MIN_MAXANISOTROPY: u32 = 0u32;
pub const D3D11_MIP_FILTER_SHIFT: u32 = 0u32;
pub const D3D11_MIP_LOD_BIAS_MAX: f32 = 15.99f32;
pub const D3D11_MIP_LOD_BIAS_MIN: f32 = -16f32;
pub const D3D11_MIP_LOD_FRACTIONAL_BIT_COUNT: u32 = 8u32;
pub const D3D11_MIP_LOD_RANGE_BIT_COUNT: u32 = 8u32;
pub const D3D11_MULTISAMPLE_ANTIALIAS_LINE_WIDTH: f32 = 1.4f32;
pub const D3D11_NONSAMPLE_FETCH_OUT_OF_RANGE_ACCESS_RESULT: u32 = 0u32;
#[repr(C)]
pub struct D3D11_OMAC {
    pub Omac: [u8; 16],
}
impl ::core::marker::Copy for D3D11_OMAC {}
impl ::core::clone::Clone for D3D11_OMAC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_PACKED_MIP_DESC {
    pub NumStandardMips: u8,
    pub NumPackedMips: u8,
    pub NumTilesForPackedMips: u32,
    pub StartTileIndexInOverallResource: u32,
}
impl ::core::marker::Copy for D3D11_PACKED_MIP_DESC {}
impl ::core::clone::Clone for D3D11_PACKED_MIP_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_PACKED_TILE: u32 = 4294967295u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D11_PARAMETER_DESC {
    pub Name: super::super::Foundation::PSTR,
    pub SemanticName: super::super::Foundation::PSTR,
    pub Type: super::Direct3D::D3D_SHADER_VARIABLE_TYPE,
    pub Class: super::Direct3D::D3D_SHADER_VARIABLE_CLASS,
    pub Rows: u32,
    pub Columns: u32,
    pub InterpolationMode: super::Direct3D::D3D_INTERPOLATION_MODE,
    pub Flags: super::Direct3D::D3D_PARAMETER_FLAGS,
    pub FirstInRegister: u32,
    pub FirstInComponent: u32,
    pub FirstOutRegister: u32,
    pub FirstOutComponent: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D11_PARAMETER_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D11_PARAMETER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 15u32;
#[repr(C)]
pub struct D3D11_PIXEL_SHADER_TRACE_DESC {
    pub Invocation: u64,
    pub X: i32,
    pub Y: i32,
    pub SampleMask: u64,
}
impl ::core::marker::Copy for D3D11_PIXEL_SHADER_TRACE_DESC {}
impl ::core::clone::Clone for D3D11_PIXEL_SHADER_TRACE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_PRE_SCISSOR_PIXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 16u32;
pub const D3D11_PS_CS_UAV_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_PS_CS_UAV_REGISTER_COUNT: u32 = 8u32;
pub const D3D11_PS_CS_UAV_REGISTER_READS_PER_INST: u32 = 1u32;
pub const D3D11_PS_CS_UAV_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_PS_FRONTFACING_DEFAULT_VALUE: u32 = 4294967295u32;
pub const D3D11_PS_FRONTFACING_FALSE_VALUE: u32 = 0u32;
pub const D3D11_PS_FRONTFACING_TRUE_VALUE: u32 = 4294967295u32;
pub const D3D11_PS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_PS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_PS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_PS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_PS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_PS_LEGACY_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0f32;
pub const D3D11_PS_OUTPUT_DEPTH_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_PS_OUTPUT_DEPTH_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_PS_OUTPUT_DEPTH_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_PS_OUTPUT_MASK_REGISTER_COMPONENTS: u32 = 1u32;
pub const D3D11_PS_OUTPUT_MASK_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_PS_OUTPUT_MASK_REGISTER_COUNT: u32 = 1u32;
pub const D3D11_PS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_PS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_PS_OUTPUT_REGISTER_COUNT: u32 = 8u32;
pub const D3D11_PS_PIXEL_CENTER_FRACTIONAL_COMPONENT: f32 = 0.5f32;
pub const D3D11_QUERY_EVENT: i32 = 0i32;
pub const D3D11_QUERY_OCCLUSION: i32 = 1i32;
pub const D3D11_QUERY_TIMESTAMP: i32 = 2i32;
pub const D3D11_QUERY_TIMESTAMP_DISJOINT: i32 = 3i32;
pub const D3D11_QUERY_PIPELINE_STATISTICS: i32 = 4i32;
pub const D3D11_QUERY_OCCLUSION_PREDICATE: i32 = 5i32;
pub const D3D11_QUERY_SO_STATISTICS: i32 = 6i32;
pub const D3D11_QUERY_SO_OVERFLOW_PREDICATE: i32 = 7i32;
pub const D3D11_QUERY_SO_STATISTICS_STREAM0: i32 = 8i32;
pub const D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM0: i32 = 9i32;
pub const D3D11_QUERY_SO_STATISTICS_STREAM1: i32 = 10i32;
pub const D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM1: i32 = 11i32;
pub const D3D11_QUERY_SO_STATISTICS_STREAM2: i32 = 12i32;
pub const D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM2: i32 = 13i32;
pub const D3D11_QUERY_SO_STATISTICS_STREAM3: i32 = 14i32;
pub const D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM3: i32 = 15i32;
#[repr(C)]
pub struct D3D11_QUERY_DATA_PIPELINE_STATISTICS {
    pub IAVertices: u64,
    pub IAPrimitives: u64,
    pub VSInvocations: u64,
    pub GSInvocations: u64,
    pub GSPrimitives: u64,
    pub CInvocations: u64,
    pub CPrimitives: u64,
    pub PSInvocations: u64,
    pub HSInvocations: u64,
    pub DSInvocations: u64,
    pub CSInvocations: u64,
}
impl ::core::marker::Copy for D3D11_QUERY_DATA_PIPELINE_STATISTICS {}
impl ::core::clone::Clone for D3D11_QUERY_DATA_PIPELINE_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_QUERY_DATA_SO_STATISTICS {
    pub NumPrimitivesWritten: u64,
    pub PrimitivesStorageNeeded: u64,
}
impl ::core::marker::Copy for D3D11_QUERY_DATA_SO_STATISTICS {}
impl ::core::clone::Clone for D3D11_QUERY_DATA_SO_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_QUERY_DATA_TIMESTAMP_DISJOINT {
    pub Frequency: u64,
    pub Disjoint: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_QUERY_DATA_TIMESTAMP_DISJOINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_QUERY_DATA_TIMESTAMP_DISJOINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_QUERY_DESC {
    pub Query: D3D11_QUERY,
    pub MiscFlags: u32,
}
impl ::core::marker::Copy for D3D11_QUERY_DESC {}
impl ::core::clone::Clone for D3D11_QUERY_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_QUERY_DESC1 {
    pub Query: D3D11_QUERY,
    pub MiscFlags: u32,
    pub ContextType: D3D11_CONTEXT_TYPE,
}
impl ::core::marker::Copy for D3D11_QUERY_DESC1 {}
impl ::core::clone::Clone for D3D11_QUERY_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_QUERY_MISC_PREDICATEHINT: i32 = 1i32;
pub const D3D11_RAISE_FLAG_DRIVER_INTERNAL_ERROR: i32 = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_RASTERIZER_DESC {
    pub FillMode: D3D11_FILL_MODE,
    pub CullMode: D3D11_CULL_MODE,
    pub FrontCounterClockwise: super::super::Foundation::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: super::super::Foundation::BOOL,
    pub ScissorEnable: super::super::Foundation::BOOL,
    pub MultisampleEnable: super::super::Foundation::BOOL,
    pub AntialiasedLineEnable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_RASTERIZER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_RASTERIZER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_RASTERIZER_DESC1 {
    pub FillMode: D3D11_FILL_MODE,
    pub CullMode: D3D11_CULL_MODE,
    pub FrontCounterClockwise: super::super::Foundation::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: super::super::Foundation::BOOL,
    pub ScissorEnable: super::super::Foundation::BOOL,
    pub MultisampleEnable: super::super::Foundation::BOOL,
    pub AntialiasedLineEnable: super::super::Foundation::BOOL,
    pub ForcedSampleCount: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_RASTERIZER_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_RASTERIZER_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_RASTERIZER_DESC2 {
    pub FillMode: D3D11_FILL_MODE,
    pub CullMode: D3D11_CULL_MODE,
    pub FrontCounterClockwise: super::super::Foundation::BOOL,
    pub DepthBias: i32,
    pub DepthBiasClamp: f32,
    pub SlopeScaledDepthBias: f32,
    pub DepthClipEnable: super::super::Foundation::BOOL,
    pub ScissorEnable: super::super::Foundation::BOOL,
    pub MultisampleEnable: super::super::Foundation::BOOL,
    pub AntialiasedLineEnable: super::super::Foundation::BOOL,
    pub ForcedSampleCount: u32,
    pub ConservativeRaster: D3D11_CONSERVATIVE_RASTERIZATION_MODE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_RASTERIZER_DESC2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_RASTERIZER_DESC2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_RAW_UAV_SRV_BYTE_ALIGNMENT: u32 = 16u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_RENDER_TARGET_BLEND_DESC {
    pub BlendEnable: super::super::Foundation::BOOL,
    pub SrcBlend: D3D11_BLEND,
    pub DestBlend: D3D11_BLEND,
    pub BlendOp: D3D11_BLEND_OP,
    pub SrcBlendAlpha: D3D11_BLEND,
    pub DestBlendAlpha: D3D11_BLEND,
    pub BlendOpAlpha: D3D11_BLEND_OP,
    pub RenderTargetWriteMask: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_RENDER_TARGET_BLEND_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_RENDER_TARGET_BLEND_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_RENDER_TARGET_BLEND_DESC1 {
    pub BlendEnable: super::super::Foundation::BOOL,
    pub LogicOpEnable: super::super::Foundation::BOOL,
    pub SrcBlend: D3D11_BLEND,
    pub DestBlend: D3D11_BLEND,
    pub BlendOp: D3D11_BLEND_OP,
    pub SrcBlendAlpha: D3D11_BLEND,
    pub DestBlendAlpha: D3D11_BLEND,
    pub BlendOpAlpha: D3D11_BLEND_OP,
    pub LogicOp: D3D11_LOGIC_OP,
    pub RenderTargetWriteMask: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_RENDER_TARGET_BLEND_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_RENDER_TARGET_BLEND_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D11_RENDER_TARGET_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D11_RTV_DIMENSION,
    pub Anonymous: D3D11_RENDER_TARGET_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_RENDER_TARGET_VIEW_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_RENDER_TARGET_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D11_RENDER_TARGET_VIEW_DESC_0 {
    pub Buffer: D3D11_BUFFER_RTV,
    pub Texture1D: D3D11_TEX1D_RTV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_RTV,
    pub Texture2D: D3D11_TEX2D_RTV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_RTV,
    pub Texture2DMS: D3D11_TEX2DMS_RTV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_RTV,
    pub Texture3D: D3D11_TEX3D_RTV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_RENDER_TARGET_VIEW_DESC_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_RENDER_TARGET_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D11_RENDER_TARGET_VIEW_DESC1 {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D11_RTV_DIMENSION,
    pub Anonymous: D3D11_RENDER_TARGET_VIEW_DESC1_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_RENDER_TARGET_VIEW_DESC1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_RENDER_TARGET_VIEW_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D11_RENDER_TARGET_VIEW_DESC1_0 {
    pub Buffer: D3D11_BUFFER_RTV,
    pub Texture1D: D3D11_TEX1D_RTV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_RTV,
    pub Texture2D: D3D11_TEX2D_RTV1,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_RTV1,
    pub Texture2DMS: D3D11_TEX2DMS_RTV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_RTV,
    pub Texture3D: D3D11_TEX3D_RTV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_RENDER_TARGET_VIEW_DESC1_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_RENDER_TARGET_VIEW_DESC1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_REQ_BLEND_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
pub const D3D11_REQ_BUFFER_RESOURCE_TEXEL_COUNT_2_TO_EXP: u32 = 27u32;
pub const D3D11_REQ_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
pub const D3D11_REQ_DEPTH_STENCIL_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
pub const D3D11_REQ_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 32u32;
pub const D3D11_REQ_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 32u32;
pub const D3D11_REQ_FILTERING_HW_ADDRESSABLE_RESOURCE_DIMENSION: u32 = 16384u32;
pub const D3D11_REQ_GS_INVOCATION_32BIT_OUTPUT_COMPONENT_LIMIT: u32 = 1024u32;
pub const D3D11_REQ_IMMEDIATE_CONSTANT_BUFFER_ELEMENT_COUNT: u32 = 4096u32;
pub const D3D11_REQ_MAXANISOTROPY: u32 = 16u32;
pub const D3D11_REQ_MIP_LEVELS: u32 = 15u32;
pub const D3D11_REQ_MULTI_ELEMENT_STRUCTURE_SIZE_IN_BYTES: u32 = 2048u32;
pub const D3D11_REQ_RASTERIZER_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
pub const D3D11_REQ_RENDER_TO_BUFFER_WINDOW_WIDTH: u32 = 16384u32;
pub const D3D11_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_A_TERM: u32 = 128u32;
pub const D3D11_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_B_TERM: f32 = 0.25f32;
pub const D3D11_REQ_RESOURCE_SIZE_IN_MEGABYTES_EXPRESSION_C_TERM: u32 = 2048u32;
pub const D3D11_REQ_RESOURCE_VIEW_COUNT_PER_DEVICE_2_TO_EXP: u32 = 20u32;
pub const D3D11_REQ_SAMPLER_OBJECT_COUNT_PER_DEVICE: u32 = 4096u32;
pub const D3D11_REQ_TEXTURE1D_ARRAY_AXIS_DIMENSION: u32 = 2048u32;
pub const D3D11_REQ_TEXTURE1D_U_DIMENSION: u32 = 16384u32;
pub const D3D11_REQ_TEXTURE2D_ARRAY_AXIS_DIMENSION: u32 = 2048u32;
pub const D3D11_REQ_TEXTURE2D_U_OR_V_DIMENSION: u32 = 16384u32;
pub const D3D11_REQ_TEXTURE3D_U_V_OR_W_DIMENSION: u32 = 2048u32;
pub const D3D11_REQ_TEXTURECUBE_DIMENSION: u32 = 16384u32;
pub const D3D11_RESINFO_INSTRUCTION_MISSING_COMPONENT_RETVAL: u32 = 0u32;
pub const D3D11_RESOURCE_DIMENSION_UNKNOWN: i32 = 0i32;
pub const D3D11_RESOURCE_DIMENSION_BUFFER: i32 = 1i32;
pub const D3D11_RESOURCE_DIMENSION_TEXTURE1D: i32 = 2i32;
pub const D3D11_RESOURCE_DIMENSION_TEXTURE2D: i32 = 3i32;
pub const D3D11_RESOURCE_DIMENSION_TEXTURE3D: i32 = 4i32;
pub const D3D11_RESOURCE_MISC_GENERATE_MIPS: u32 = 1u32;
pub const D3D11_RESOURCE_MISC_SHARED: u32 = 2u32;
pub const D3D11_RESOURCE_MISC_TEXTURECUBE: u32 = 4u32;
pub const D3D11_RESOURCE_MISC_DRAWINDIRECT_ARGS: u32 = 16u32;
pub const D3D11_RESOURCE_MISC_BUFFER_ALLOW_RAW_VIEWS: u32 = 32u32;
pub const D3D11_RESOURCE_MISC_BUFFER_STRUCTURED: u32 = 64u32;
pub const D3D11_RESOURCE_MISC_RESOURCE_CLAMP: u32 = 128u32;
pub const D3D11_RESOURCE_MISC_SHARED_KEYEDMUTEX: u32 = 256u32;
pub const D3D11_RESOURCE_MISC_GDI_COMPATIBLE: u32 = 512u32;
pub const D3D11_RESOURCE_MISC_SHARED_NTHANDLE: u32 = 2048u32;
pub const D3D11_RESOURCE_MISC_RESTRICTED_CONTENT: u32 = 4096u32;
pub const D3D11_RESOURCE_MISC_RESTRICT_SHARED_RESOURCE: u32 = 8192u32;
pub const D3D11_RESOURCE_MISC_RESTRICT_SHARED_RESOURCE_DRIVER: u32 = 16384u32;
pub const D3D11_RESOURCE_MISC_GUARDED: u32 = 32768u32;
pub const D3D11_RESOURCE_MISC_TILE_POOL: u32 = 131072u32;
pub const D3D11_RESOURCE_MISC_TILED: u32 = 262144u32;
pub const D3D11_RESOURCE_MISC_HW_PROTECTED: u32 = 524288u32;
pub const D3D11_RESOURCE_MISC_SHARED_DISPLAYABLE: u32 = 1048576u32;
pub const D3D11_RESOURCE_MISC_SHARED_EXCLUSIVE_WRITER: u32 = 2097152u32;
pub const D3D11_RLDO_SUMMARY: i32 = 1i32;
pub const D3D11_RLDO_DETAIL: i32 = 2i32;
pub const D3D11_RLDO_IGNORE_INTERNAL: i32 = 4i32;
pub const D3D11_RTV_DIMENSION_UNKNOWN: i32 = 0i32;
pub const D3D11_RTV_DIMENSION_BUFFER: i32 = 1i32;
pub const D3D11_RTV_DIMENSION_TEXTURE1D: i32 = 2i32;
pub const D3D11_RTV_DIMENSION_TEXTURE1DARRAY: i32 = 3i32;
pub const D3D11_RTV_DIMENSION_TEXTURE2D: i32 = 4i32;
pub const D3D11_RTV_DIMENSION_TEXTURE2DARRAY: i32 = 5i32;
pub const D3D11_RTV_DIMENSION_TEXTURE2DMS: i32 = 6i32;
pub const D3D11_RTV_DIMENSION_TEXTURE2DMSARRAY: i32 = 7i32;
pub const D3D11_RTV_DIMENSION_TEXTURE3D: i32 = 8i32;
#[repr(C)]
pub struct D3D11_SAMPLER_DESC {
    pub Filter: D3D11_FILTER,
    pub AddressU: D3D11_TEXTURE_ADDRESS_MODE,
    pub AddressV: D3D11_TEXTURE_ADDRESS_MODE,
    pub AddressW: D3D11_TEXTURE_ADDRESS_MODE,
    pub MipLODBias: f32,
    pub MaxAnisotropy: u32,
    pub ComparisonFunc: D3D11_COMPARISON_FUNC,
    pub BorderColor: [f32; 4],
    pub MinLOD: f32,
    pub MaxLOD: f32,
}
impl ::core::marker::Copy for D3D11_SAMPLER_DESC {}
impl ::core::clone::Clone for D3D11_SAMPLER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_SDK_LAYERS_VERSION: u32 = 1u32;
pub const D3D11_SDK_VERSION: u32 = 7u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D11_SHADER_BUFFER_DESC {
    pub Name: super::super::Foundation::PSTR,
    pub Type: super::Direct3D::D3D_CBUFFER_TYPE,
    pub Variables: u32,
    pub Size: u32,
    pub uFlags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D11_SHADER_BUFFER_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D11_SHADER_BUFFER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_SHADER_CACHE_SUPPORT_NONE: i32 = 0i32;
pub const D3D11_SHADER_CACHE_SUPPORT_AUTOMATIC_INPROC_CACHE: i32 = 1i32;
pub const D3D11_SHADER_CACHE_SUPPORT_AUTOMATIC_DISK_CACHE: i32 = 2i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D11_SHADER_DESC {
    pub Version: u32,
    pub Creator: super::super::Foundation::PSTR,
    pub Flags: u32,
    pub ConstantBuffers: u32,
    pub BoundResources: u32,
    pub InputParameters: u32,
    pub OutputParameters: u32,
    pub InstructionCount: u32,
    pub TempRegisterCount: u32,
    pub TempArrayCount: u32,
    pub DefCount: u32,
    pub DclCount: u32,
    pub TextureNormalInstructions: u32,
    pub TextureLoadInstructions: u32,
    pub TextureCompInstructions: u32,
    pub TextureBiasInstructions: u32,
    pub TextureGradientInstructions: u32,
    pub FloatInstructionCount: u32,
    pub IntInstructionCount: u32,
    pub UintInstructionCount: u32,
    pub StaticFlowControlCount: u32,
    pub DynamicFlowControlCount: u32,
    pub MacroInstructionCount: u32,
    pub ArrayInstructionCount: u32,
    pub CutInstructionCount: u32,
    pub EmitInstructionCount: u32,
    pub GSOutputTopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY,
    pub GSMaxOutputVertexCount: u32,
    pub InputPrimitive: super::Direct3D::D3D_PRIMITIVE,
    pub PatchConstantParameters: u32,
    pub cGSInstanceCount: u32,
    pub cControlPoints: u32,
    pub HSOutputPrimitive: super::Direct3D::D3D_TESSELLATOR_OUTPUT_PRIMITIVE,
    pub HSPartitioning: super::Direct3D::D3D_TESSELLATOR_PARTITIONING,
    pub TessellatorDomain: super::Direct3D::D3D_TESSELLATOR_DOMAIN,
    pub cBarrierInstructions: u32,
    pub cInterlockedInstructions: u32,
    pub cTextureStoreInstructions: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D11_SHADER_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D11_SHADER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D11_SHADER_INPUT_BIND_DESC {
    pub Name: super::super::Foundation::PSTR,
    pub Type: super::Direct3D::D3D_SHADER_INPUT_TYPE,
    pub BindPoint: u32,
    pub BindCount: u32,
    pub uFlags: u32,
    pub ReturnType: super::Direct3D::D3D_RESOURCE_RETURN_TYPE,
    pub Dimension: super::Direct3D::D3D_SRV_DIMENSION,
    pub NumSamples: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D11_SHADER_INPUT_BIND_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D11_SHADER_INPUT_BIND_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_SHADER_MAJOR_VERSION: u32 = 5u32;
pub const D3D11_SHADER_MAX_INSTANCES: u32 = 65535u32;
pub const D3D11_SHADER_MAX_INTERFACES: u32 = 253u32;
pub const D3D11_SHADER_MAX_INTERFACE_CALL_SITES: u32 = 4096u32;
pub const D3D11_SHADER_MAX_TYPES: u32 = 65535u32;
pub const D3D11_SHADER_MINOR_VERSION: u32 = 0u32;
pub const D3D11_SHADER_MIN_PRECISION_10_BIT: i32 = 1i32;
pub const D3D11_SHADER_MIN_PRECISION_16_BIT: i32 = 2i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: super::Direct3D::D3D_SRV_DIMENSION,
    pub Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC_0,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D11_SHADER_RESOURCE_VIEW_DESC {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D11_SHADER_RESOURCE_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub union D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
    pub Buffer: D3D11_BUFFER_SRV,
    pub Texture1D: D3D11_TEX1D_SRV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_SRV,
    pub Texture2D: D3D11_TEX2D_SRV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_SRV,
    pub Texture2DMS: D3D11_TEX2DMS_SRV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_SRV,
    pub Texture3D: D3D11_TEX3D_SRV,
    pub TextureCube: D3D11_TEXCUBE_SRV,
    pub TextureCubeArray: D3D11_TEXCUBE_ARRAY_SRV,
    pub BufferEx: D3D11_BUFFEREX_SRV,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D11_SHADER_RESOURCE_VIEW_DESC_0 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC1 {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: super::Direct3D::D3D_SRV_DIMENSION,
    pub Anonymous: D3D11_SHADER_RESOURCE_VIEW_DESC1_0,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D11_SHADER_RESOURCE_VIEW_DESC1 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D11_SHADER_RESOURCE_VIEW_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
pub union D3D11_SHADER_RESOURCE_VIEW_DESC1_0 {
    pub Buffer: D3D11_BUFFER_SRV,
    pub Texture1D: D3D11_TEX1D_SRV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_SRV,
    pub Texture2D: D3D11_TEX2D_SRV1,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_SRV1,
    pub Texture2DMS: D3D11_TEX2DMS_SRV,
    pub Texture2DMSArray: D3D11_TEX2DMS_ARRAY_SRV,
    pub Texture3D: D3D11_TEX3D_SRV,
    pub TextureCube: D3D11_TEXCUBE_SRV,
    pub TextureCubeArray: D3D11_TEXCUBE_ARRAY_SRV,
    pub BufferEx: D3D11_BUFFEREX_SRV,
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D11_SHADER_RESOURCE_VIEW_DESC1_0 {}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D11_SHADER_RESOURCE_VIEW_DESC1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_SHADER_TRACE_DESC {
    pub Type: D3D11_SHADER_TYPE,
    pub Flags: u32,
    pub Anonymous: D3D11_SHADER_TRACE_DESC_0,
}
impl ::core::marker::Copy for D3D11_SHADER_TRACE_DESC {}
impl ::core::clone::Clone for D3D11_SHADER_TRACE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D3D11_SHADER_TRACE_DESC_0 {
    pub VertexShaderTraceDesc: D3D11_VERTEX_SHADER_TRACE_DESC,
    pub HullShaderTraceDesc: D3D11_HULL_SHADER_TRACE_DESC,
    pub DomainShaderTraceDesc: D3D11_DOMAIN_SHADER_TRACE_DESC,
    pub GeometryShaderTraceDesc: D3D11_GEOMETRY_SHADER_TRACE_DESC,
    pub PixelShaderTraceDesc: D3D11_PIXEL_SHADER_TRACE_DESC,
    pub ComputeShaderTraceDesc: D3D11_COMPUTE_SHADER_TRACE_DESC,
}
impl ::core::marker::Copy for D3D11_SHADER_TRACE_DESC_0 {}
impl ::core::clone::Clone for D3D11_SHADER_TRACE_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_SHADER_TRACE_FLAG_RECORD_REGISTER_READS: u32 = 2u32;
pub const D3D11_SHADER_TRACE_FLAG_RECORD_REGISTER_WRITES: u32 = 1u32;
pub const D3D11_SHADER_TRACKING_OPTION_IGNORE: i32 = 0i32;
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_UNINITIALIZED: i32 = 1i32;
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_RAW: i32 = 2i32;
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_WAR: i32 = 4i32;
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_WAW: i32 = 8i32;
pub const D3D11_SHADER_TRACKING_OPTION_ALLOW_SAME: i32 = 16i32;
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_ATOMIC_CONSISTENCY: i32 = 32i32;
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_RAW_ACROSS_THREADGROUPS: i32 = 64i32;
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_WAR_ACROSS_THREADGROUPS: i32 = 128i32;
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_WAW_ACROSS_THREADGROUPS: i32 = 256i32;
pub const D3D11_SHADER_TRACKING_OPTION_TRACK_ATOMIC_CONSISTENCY_ACROSS_THREADGROUPS: i32 = 512i32;
pub const D3D11_SHADER_TRACKING_OPTION_UAV_SPECIFIC_FLAGS: i32 = 960i32;
pub const D3D11_SHADER_TRACKING_OPTION_ALL_HAZARDS: i32 = 1006i32;
pub const D3D11_SHADER_TRACKING_OPTION_ALL_HAZARDS_ALLOWING_SAME: i32 = 1022i32;
pub const D3D11_SHADER_TRACKING_OPTION_ALL_OPTIONS: i32 = 1023i32;
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_NONE: i32 = 0i32;
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_UAV_DEVICEMEMORY: i32 = 1i32;
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_NON_UAV_DEVICEMEMORY: i32 = 2i32;
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_ALL_DEVICEMEMORY: i32 = 3i32;
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_GROUPSHARED_MEMORY: i32 = 4i32;
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_ALL_SHARED_MEMORY: i32 = 5i32;
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_GROUPSHARED_NON_UAV: i32 = 6i32;
pub const D3D11_SHADER_TRACKING_RESOURCE_TYPE_ALL: i32 = 7i32;
pub const D3D11_VERTEX_SHADER: i32 = 1i32;
pub const D3D11_HULL_SHADER: i32 = 2i32;
pub const D3D11_DOMAIN_SHADER: i32 = 3i32;
pub const D3D11_GEOMETRY_SHADER: i32 = 4i32;
pub const D3D11_PIXEL_SHADER: i32 = 5i32;
pub const D3D11_COMPUTE_SHADER: i32 = 6i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D11_SHADER_TYPE_DESC {
    pub Class: super::Direct3D::D3D_SHADER_VARIABLE_CLASS,
    pub Type: super::Direct3D::D3D_SHADER_VARIABLE_TYPE,
    pub Rows: u32,
    pub Columns: u32,
    pub Elements: u32,
    pub Members: u32,
    pub Offset: u32,
    pub Name: super::super::Foundation::PSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D11_SHADER_TYPE_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D11_SHADER_TYPE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_SHADER_VARIABLE_DESC {
    pub Name: super::super::Foundation::PSTR,
    pub StartOffset: u32,
    pub Size: u32,
    pub uFlags: u32,
    pub DefaultValue: *mut ::core::ffi::c_void,
    pub StartTexture: u32,
    pub TextureSize: u32,
    pub StartSampler: u32,
    pub SamplerSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_SHADER_VARIABLE_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_SHADER_VARIABLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_SHVER_PIXEL_SHADER: i32 = 0i32;
pub const D3D11_SHVER_VERTEX_SHADER: i32 = 1i32;
pub const D3D11_SHVER_GEOMETRY_SHADER: i32 = 2i32;
pub const D3D11_SHVER_HULL_SHADER: i32 = 3i32;
pub const D3D11_SHVER_DOMAIN_SHADER: i32 = 4i32;
pub const D3D11_SHVER_COMPUTE_SHADER: i32 = 5i32;
pub const D3D11_SHVER_RESERVED0: i32 = 65520i32;
pub const D3D11_SHARED_RESOURCE_TIER_0: i32 = 0i32;
pub const D3D11_SHARED_RESOURCE_TIER_1: i32 = 1i32;
pub const D3D11_SHARED_RESOURCE_TIER_2: i32 = 2i32;
pub const D3D11_SHARED_RESOURCE_TIER_3: i32 = 3i32;
pub const D3D11_SHIFT_INSTRUCTION_PAD_VALUE: u32 = 0u32;
pub const D3D11_SHIFT_INSTRUCTION_SHIFT_VALUE_BIT_COUNT: u32 = 5u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
pub struct D3D11_SIGNATURE_PARAMETER_DESC {
    pub SemanticName: super::super::Foundation::PSTR,
    pub SemanticIndex: u32,
    pub Register: u32,
    pub SystemValueType: super::Direct3D::D3D_NAME,
    pub ComponentType: super::Direct3D::D3D_REGISTER_COMPONENT_TYPE,
    pub Mask: u8,
    pub ReadWriteMask: u8,
    pub Stream: u32,
    pub MinPrecision: super::Direct3D::D3D_MIN_PRECISION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::marker::Copy for D3D11_SIGNATURE_PARAMETER_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::clone::Clone for D3D11_SIGNATURE_PARAMETER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT: u32 = 8u32;
pub const D3D11_SO_BUFFER_MAX_STRIDE_IN_BYTES: u32 = 2048u32;
pub const D3D11_SO_BUFFER_MAX_WRITE_WINDOW_IN_BYTES: u32 = 512u32;
pub const D3D11_SO_BUFFER_SLOT_COUNT: u32 = 4u32;
pub const D3D11_SO_DDI_REGISTER_INDEX_DENOTING_GAP: u32 = 4294967295u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_SO_DECLARATION_ENTRY {
    pub Stream: u32,
    pub SemanticName: super::super::Foundation::PSTR,
    pub SemanticIndex: u32,
    pub StartComponent: u8,
    pub ComponentCount: u8,
    pub OutputSlot: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_SO_DECLARATION_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_SO_DECLARATION_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_SO_NO_RASTERIZED_STREAM: u32 = 4294967295u32;
pub const D3D11_SO_OUTPUT_COMPONENT_COUNT: u32 = 128u32;
pub const D3D11_SO_STREAM_COUNT: u32 = 4u32;
pub const D3D11_SPEC_DATE_DAY: u32 = 16u32;
pub const D3D11_SPEC_DATE_MONTH: u32 = 5u32;
pub const D3D11_SPEC_DATE_YEAR: u32 = 2011u32;
pub const D3D11_SPEC_VERSION: f64 = 1.07f64;
pub const D3D11_SRGB_GAMMA: f32 = 2.2f32;
pub const D3D11_SRGB_TO_FLOAT_DENOMINATOR_1: f32 = 12.92f32;
pub const D3D11_SRGB_TO_FLOAT_DENOMINATOR_2: f32 = 1.055f32;
pub const D3D11_SRGB_TO_FLOAT_EXPONENT: f32 = 2.4f32;
pub const D3D11_SRGB_TO_FLOAT_OFFSET: f32 = 0.055f32;
pub const D3D11_SRGB_TO_FLOAT_THRESHOLD: f32 = 0.04045f32;
pub const D3D11_SRGB_TO_FLOAT_TOLERANCE_IN_ULP: f32 = 0.5f32;
pub const D3D11_STANDARD_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_STANDARD_COMPONENT_BIT_COUNT_DOUBLED: u32 = 64u32;
pub const D3D11_STANDARD_MAXIMUM_ELEMENT_ALIGNMENT_BYTE_MULTIPLE: u32 = 4u32;
pub const D3D11_STANDARD_MULTISAMPLE_PATTERN: i32 = -1i32;
pub const D3D11_CENTER_MULTISAMPLE_PATTERN: i32 = -2i32;
pub const D3D11_STANDARD_PIXEL_COMPONENT_COUNT: u32 = 128u32;
pub const D3D11_STANDARD_PIXEL_ELEMENT_COUNT: u32 = 32u32;
pub const D3D11_STANDARD_VECTOR_SIZE: u32 = 4u32;
pub const D3D11_STANDARD_VERTEX_ELEMENT_COUNT: u32 = 32u32;
pub const D3D11_STANDARD_VERTEX_TOTAL_COMPONENT_COUNT: u32 = 64u32;
pub const D3D11_STENCIL_OP_KEEP: i32 = 1i32;
pub const D3D11_STENCIL_OP_ZERO: i32 = 2i32;
pub const D3D11_STENCIL_OP_REPLACE: i32 = 3i32;
pub const D3D11_STENCIL_OP_INCR_SAT: i32 = 4i32;
pub const D3D11_STENCIL_OP_DECR_SAT: i32 = 5i32;
pub const D3D11_STENCIL_OP_INVERT: i32 = 6i32;
pub const D3D11_STENCIL_OP_INCR: i32 = 7i32;
pub const D3D11_STENCIL_OP_DECR: i32 = 8i32;
pub const D3D11_SUBPIXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
#[repr(C)]
pub struct D3D11_SUBRESOURCE_DATA {
    pub pSysMem: *mut ::core::ffi::c_void,
    pub SysMemPitch: u32,
    pub SysMemSlicePitch: u32,
}
impl ::core::marker::Copy for D3D11_SUBRESOURCE_DATA {}
impl ::core::clone::Clone for D3D11_SUBRESOURCE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_SUBRESOURCE_TILING {
    pub WidthInTiles: u32,
    pub HeightInTiles: u16,
    pub DepthInTiles: u16,
    pub StartTileIndexInOverallResource: u32,
}
impl ::core::marker::Copy for D3D11_SUBRESOURCE_TILING {}
impl ::core::clone::Clone for D3D11_SUBRESOURCE_TILING {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_SUBTEXEL_FRACTIONAL_BIT_COUNT: u32 = 8u32;
pub const D3D11_TESSELLATOR_MAX_EVEN_TESSELLATION_FACTOR: u32 = 64u32;
pub const D3D11_TESSELLATOR_MAX_ISOLINE_DENSITY_TESSELLATION_FACTOR: u32 = 64u32;
pub const D3D11_TESSELLATOR_MAX_ODD_TESSELLATION_FACTOR: u32 = 63u32;
pub const D3D11_TESSELLATOR_MAX_TESSELLATION_FACTOR: u32 = 64u32;
pub const D3D11_TESSELLATOR_MIN_EVEN_TESSELLATION_FACTOR: u32 = 2u32;
pub const D3D11_TESSELLATOR_MIN_ISOLINE_DENSITY_TESSELLATION_FACTOR: u32 = 1u32;
pub const D3D11_TESSELLATOR_MIN_ODD_TESSELLATION_FACTOR: u32 = 1u32;
#[repr(C)]
pub struct D3D11_TEX1D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D11_TEX1D_ARRAY_DSV {}
impl ::core::clone::Clone for D3D11_TEX1D_ARRAY_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX1D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D11_TEX1D_ARRAY_RTV {}
impl ::core::clone::Clone for D3D11_TEX1D_ARRAY_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX1D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D11_TEX1D_ARRAY_SRV {}
impl ::core::clone::Clone for D3D11_TEX1D_ARRAY_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX1D_ARRAY_UAV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D11_TEX1D_ARRAY_UAV {}
impl ::core::clone::Clone for D3D11_TEX1D_ARRAY_UAV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX1D_DSV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D11_TEX1D_DSV {}
impl ::core::clone::Clone for D3D11_TEX1D_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX1D_RTV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D11_TEX1D_RTV {}
impl ::core::clone::Clone for D3D11_TEX1D_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX1D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
impl ::core::marker::Copy for D3D11_TEX1D_SRV {}
impl ::core::clone::Clone for D3D11_TEX1D_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX1D_UAV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D11_TEX1D_UAV {}
impl ::core::clone::Clone for D3D11_TEX1D_UAV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2DMS_ARRAY_DSV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D11_TEX2DMS_ARRAY_DSV {}
impl ::core::clone::Clone for D3D11_TEX2DMS_ARRAY_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2DMS_ARRAY_RTV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D11_TEX2DMS_ARRAY_RTV {}
impl ::core::clone::Clone for D3D11_TEX2DMS_ARRAY_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2DMS_ARRAY_SRV {
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D11_TEX2DMS_ARRAY_SRV {}
impl ::core::clone::Clone for D3D11_TEX2DMS_ARRAY_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2DMS_DSV {
    pub UnusedField_NothingToDefine: u32,
}
impl ::core::marker::Copy for D3D11_TEX2DMS_DSV {}
impl ::core::clone::Clone for D3D11_TEX2DMS_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2DMS_RTV {
    pub UnusedField_NothingToDefine: u32,
}
impl ::core::marker::Copy for D3D11_TEX2DMS_RTV {}
impl ::core::clone::Clone for D3D11_TEX2DMS_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2DMS_SRV {
    pub UnusedField_NothingToDefine: u32,
}
impl ::core::marker::Copy for D3D11_TEX2DMS_SRV {}
impl ::core::clone::Clone for D3D11_TEX2DMS_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2D_ARRAY_DSV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D11_TEX2D_ARRAY_DSV {}
impl ::core::clone::Clone for D3D11_TEX2D_ARRAY_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2D_ARRAY_RTV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D11_TEX2D_ARRAY_RTV {}
impl ::core::clone::Clone for D3D11_TEX2D_ARRAY_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2D_ARRAY_RTV1 {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
}
impl ::core::marker::Copy for D3D11_TEX2D_ARRAY_RTV1 {}
impl ::core::clone::Clone for D3D11_TEX2D_ARRAY_RTV1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2D_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D11_TEX2D_ARRAY_SRV {}
impl ::core::clone::Clone for D3D11_TEX2D_ARRAY_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2D_ARRAY_SRV1 {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
}
impl ::core::marker::Copy for D3D11_TEX2D_ARRAY_SRV1 {}
impl ::core::clone::Clone for D3D11_TEX2D_ARRAY_SRV1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2D_ARRAY_UAV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D11_TEX2D_ARRAY_UAV {}
impl ::core::clone::Clone for D3D11_TEX2D_ARRAY_UAV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2D_ARRAY_UAV1 {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
    pub PlaneSlice: u32,
}
impl ::core::marker::Copy for D3D11_TEX2D_ARRAY_UAV1 {}
impl ::core::clone::Clone for D3D11_TEX2D_ARRAY_UAV1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2D_ARRAY_VPOV {
    pub MipSlice: u32,
    pub FirstArraySlice: u32,
    pub ArraySize: u32,
}
impl ::core::marker::Copy for D3D11_TEX2D_ARRAY_VPOV {}
impl ::core::clone::Clone for D3D11_TEX2D_ARRAY_VPOV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2D_DSV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D11_TEX2D_DSV {}
impl ::core::clone::Clone for D3D11_TEX2D_DSV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2D_RTV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D11_TEX2D_RTV {}
impl ::core::clone::Clone for D3D11_TEX2D_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2D_RTV1 {
    pub MipSlice: u32,
    pub PlaneSlice: u32,
}
impl ::core::marker::Copy for D3D11_TEX2D_RTV1 {}
impl ::core::clone::Clone for D3D11_TEX2D_RTV1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
impl ::core::marker::Copy for D3D11_TEX2D_SRV {}
impl ::core::clone::Clone for D3D11_TEX2D_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2D_SRV1 {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub PlaneSlice: u32,
}
impl ::core::marker::Copy for D3D11_TEX2D_SRV1 {}
impl ::core::clone::Clone for D3D11_TEX2D_SRV1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2D_UAV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D11_TEX2D_UAV {}
impl ::core::clone::Clone for D3D11_TEX2D_UAV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2D_UAV1 {
    pub MipSlice: u32,
    pub PlaneSlice: u32,
}
impl ::core::marker::Copy for D3D11_TEX2D_UAV1 {}
impl ::core::clone::Clone for D3D11_TEX2D_UAV1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2D_VDOV {
    pub ArraySlice: u32,
}
impl ::core::marker::Copy for D3D11_TEX2D_VDOV {}
impl ::core::clone::Clone for D3D11_TEX2D_VDOV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2D_VPIV {
    pub MipSlice: u32,
    pub ArraySlice: u32,
}
impl ::core::marker::Copy for D3D11_TEX2D_VPIV {}
impl ::core::clone::Clone for D3D11_TEX2D_VPIV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX2D_VPOV {
    pub MipSlice: u32,
}
impl ::core::marker::Copy for D3D11_TEX2D_VPOV {}
impl ::core::clone::Clone for D3D11_TEX2D_VPOV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX3D_RTV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
impl ::core::marker::Copy for D3D11_TEX3D_RTV {}
impl ::core::clone::Clone for D3D11_TEX3D_RTV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX3D_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
impl ::core::marker::Copy for D3D11_TEX3D_SRV {}
impl ::core::clone::Clone for D3D11_TEX3D_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEX3D_UAV {
    pub MipSlice: u32,
    pub FirstWSlice: u32,
    pub WSize: u32,
}
impl ::core::marker::Copy for D3D11_TEX3D_UAV {}
impl ::core::clone::Clone for D3D11_TEX3D_UAV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEXCUBE_ARRAY_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
    pub First2DArrayFace: u32,
    pub NumCubes: u32,
}
impl ::core::marker::Copy for D3D11_TEXCUBE_ARRAY_SRV {}
impl ::core::clone::Clone for D3D11_TEXCUBE_ARRAY_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TEXCUBE_SRV {
    pub MostDetailedMip: u32,
    pub MipLevels: u32,
}
impl ::core::marker::Copy for D3D11_TEXCUBE_SRV {}
impl ::core::clone::Clone for D3D11_TEXCUBE_SRV {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_TEXEL_ADDRESS_RANGE_BIT_COUNT: u32 = 16u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D11_TEXTURE1D_DESC {
    pub Width: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_TEXTURE1D_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_TEXTURE1D_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D11_TEXTURE2D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub SampleDesc: super::Dxgi::Common::DXGI_SAMPLE_DESC,
    pub Usage: D3D11_USAGE,
    pub BindFlags: D3D11_BIND_FLAG,
    pub CPUAccessFlags: D3D11_CPU_ACCESS_FLAG,
    pub MiscFlags: D3D11_RESOURCE_MISC_FLAG,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_TEXTURE2D_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_TEXTURE2D_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D11_TEXTURE2D_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub SampleDesc: super::Dxgi::Common::DXGI_SAMPLE_DESC,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
    pub TextureLayout: D3D11_TEXTURE_LAYOUT,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_TEXTURE2D_DESC1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_TEXTURE2D_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D11_TEXTURE3D_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_TEXTURE3D_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_TEXTURE3D_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D11_TEXTURE3D_DESC1 {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub Usage: D3D11_USAGE,
    pub BindFlags: u32,
    pub CPUAccessFlags: u32,
    pub MiscFlags: u32,
    pub TextureLayout: D3D11_TEXTURE_LAYOUT,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_TEXTURE3D_DESC1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_TEXTURE3D_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_TEXTURECUBE_FACE_POSITIVE_X: i32 = 0i32;
pub const D3D11_TEXTURECUBE_FACE_NEGATIVE_X: i32 = 1i32;
pub const D3D11_TEXTURECUBE_FACE_POSITIVE_Y: i32 = 2i32;
pub const D3D11_TEXTURECUBE_FACE_NEGATIVE_Y: i32 = 3i32;
pub const D3D11_TEXTURECUBE_FACE_POSITIVE_Z: i32 = 4i32;
pub const D3D11_TEXTURECUBE_FACE_NEGATIVE_Z: i32 = 5i32;
pub const D3D11_TEXTURE_ADDRESS_WRAP: i32 = 1i32;
pub const D3D11_TEXTURE_ADDRESS_MIRROR: i32 = 2i32;
pub const D3D11_TEXTURE_ADDRESS_CLAMP: i32 = 3i32;
pub const D3D11_TEXTURE_ADDRESS_BORDER: i32 = 4i32;
pub const D3D11_TEXTURE_ADDRESS_MIRROR_ONCE: i32 = 5i32;
pub const D3D11_TEXTURE_LAYOUT_UNDEFINED: i32 = 0i32;
pub const D3D11_TEXTURE_LAYOUT_ROW_MAJOR: i32 = 1i32;
pub const D3D11_TEXTURE_LAYOUT_64K_STANDARD_SWIZZLE: i32 = 2i32;
pub const D3D11_TILED_RESOURCES_NOT_SUPPORTED: i32 = 0i32;
pub const D3D11_TILED_RESOURCES_TIER_1: i32 = 1i32;
pub const D3D11_TILED_RESOURCES_TIER_2: i32 = 2i32;
pub const D3D11_TILED_RESOURCES_TIER_3: i32 = 3i32;
#[repr(C)]
pub struct D3D11_TILED_RESOURCE_COORDINATE {
    pub X: u32,
    pub Y: u32,
    pub Z: u32,
    pub Subresource: u32,
}
impl ::core::marker::Copy for D3D11_TILED_RESOURCE_COORDINATE {}
impl ::core::clone::Clone for D3D11_TILED_RESOURCE_COORDINATE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_TILE_COPY_NO_OVERWRITE: i32 = 1i32;
pub const D3D11_TILE_COPY_LINEAR_BUFFER_TO_SWIZZLED_TILED_RESOURCE: i32 = 2i32;
pub const D3D11_TILE_COPY_SWIZZLED_TILED_RESOURCE_TO_LINEAR_BUFFER: i32 = 4i32;
pub const D3D11_TILE_MAPPING_NO_OVERWRITE: i32 = 1i32;
pub const D3D11_TILE_RANGE_NULL: i32 = 1i32;
pub const D3D11_TILE_RANGE_SKIP: i32 = 2i32;
pub const D3D11_TILE_RANGE_REUSE_SINGLE_TILE: i32 = 4i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_TILE_REGION_SIZE {
    pub NumTiles: u32,
    pub bUseBox: super::super::Foundation::BOOL,
    pub Width: u32,
    pub Height: u16,
    pub Depth: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_TILE_REGION_SIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_TILE_REGION_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TILE_SHAPE {
    pub WidthInTexels: u32,
    pub HeightInTexels: u32,
    pub DepthInTexels: u32,
}
impl ::core::marker::Copy for D3D11_TILE_SHAPE {}
impl ::core::clone::Clone for D3D11_TILE_SHAPE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_TRACE_COMPONENT_W: u32 = 8u32;
pub const D3D11_TRACE_COMPONENT_X: u32 = 1u32;
pub const D3D11_TRACE_COMPONENT_Y: u32 = 2u32;
pub const D3D11_TRACE_COMPONENT_Z: u32 = 4u32;
pub const D3D11_TRACE_GS_INPUT_PRIMITIVE_UNDEFINED: i32 = 0i32;
pub const D3D11_TRACE_GS_INPUT_PRIMITIVE_POINT: i32 = 1i32;
pub const D3D11_TRACE_GS_INPUT_PRIMITIVE_LINE: i32 = 2i32;
pub const D3D11_TRACE_GS_INPUT_PRIMITIVE_TRIANGLE: i32 = 3i32;
pub const D3D11_TRACE_GS_INPUT_PRIMITIVE_LINE_ADJ: i32 = 6i32;
pub const D3D11_TRACE_GS_INPUT_PRIMITIVE_TRIANGLE_ADJ: i32 = 7i32;
pub const D3D11_TRACE_MISC_GS_CUT: u32 = 2u32;
pub const D3D11_TRACE_MISC_GS_CUT_STREAM: u32 = 16u32;
pub const D3D11_TRACE_MISC_GS_EMIT: u32 = 1u32;
pub const D3D11_TRACE_MISC_GS_EMIT_STREAM: u32 = 8u32;
pub const D3D11_TRACE_MISC_HALT: u32 = 32u32;
pub const D3D11_TRACE_MISC_MESSAGE: u32 = 64u32;
pub const D3D11_TRACE_MISC_PS_DISCARD: u32 = 4u32;
#[repr(C)]
pub struct D3D11_TRACE_REGISTER {
    pub RegType: D3D11_TRACE_REGISTER_TYPE,
    pub Anonymous: D3D11_TRACE_REGISTER_0,
    pub OperandIndex: u8,
    pub Flags: u8,
}
impl ::core::marker::Copy for D3D11_TRACE_REGISTER {}
impl ::core::clone::Clone for D3D11_TRACE_REGISTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D3D11_TRACE_REGISTER_0 {
    pub Index1D: u16,
    pub Index2D: [u16; 2],
}
impl ::core::marker::Copy for D3D11_TRACE_REGISTER_0 {}
impl ::core::clone::Clone for D3D11_TRACE_REGISTER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_TRACE_REGISTER_FLAGS_RELATIVE_INDEXING: u32 = 1u32;
pub const D3D11_TRACE_OUTPUT_NULL_REGISTER: i32 = 0i32;
pub const D3D11_TRACE_INPUT_REGISTER: i32 = 1i32;
pub const D3D11_TRACE_INPUT_PRIMITIVE_ID_REGISTER: i32 = 2i32;
pub const D3D11_TRACE_IMMEDIATE_CONSTANT_BUFFER: i32 = 3i32;
pub const D3D11_TRACE_TEMP_REGISTER: i32 = 4i32;
pub const D3D11_TRACE_INDEXABLE_TEMP_REGISTER: i32 = 5i32;
pub const D3D11_TRACE_OUTPUT_REGISTER: i32 = 6i32;
pub const D3D11_TRACE_OUTPUT_DEPTH_REGISTER: i32 = 7i32;
pub const D3D11_TRACE_CONSTANT_BUFFER: i32 = 8i32;
pub const D3D11_TRACE_IMMEDIATE32: i32 = 9i32;
pub const D3D11_TRACE_SAMPLER: i32 = 10i32;
pub const D3D11_TRACE_RESOURCE: i32 = 11i32;
pub const D3D11_TRACE_RASTERIZER: i32 = 12i32;
pub const D3D11_TRACE_OUTPUT_COVERAGE_MASK: i32 = 13i32;
pub const D3D11_TRACE_STREAM: i32 = 14i32;
pub const D3D11_TRACE_THIS_POINTER: i32 = 15i32;
pub const D3D11_TRACE_OUTPUT_CONTROL_POINT_ID_REGISTER: i32 = 16i32;
pub const D3D11_TRACE_INPUT_FORK_INSTANCE_ID_REGISTER: i32 = 17i32;
pub const D3D11_TRACE_INPUT_JOIN_INSTANCE_ID_REGISTER: i32 = 18i32;
pub const D3D11_TRACE_INPUT_CONTROL_POINT_REGISTER: i32 = 19i32;
pub const D3D11_TRACE_OUTPUT_CONTROL_POINT_REGISTER: i32 = 20i32;
pub const D3D11_TRACE_INPUT_PATCH_CONSTANT_REGISTER: i32 = 21i32;
pub const D3D11_TRACE_INPUT_DOMAIN_POINT_REGISTER: i32 = 22i32;
pub const D3D11_TRACE_UNORDERED_ACCESS_VIEW: i32 = 23i32;
pub const D3D11_TRACE_THREAD_GROUP_SHARED_MEMORY: i32 = 24i32;
pub const D3D11_TRACE_INPUT_THREAD_ID_REGISTER: i32 = 25i32;
pub const D3D11_TRACE_INPUT_THREAD_GROUP_ID_REGISTER: i32 = 26i32;
pub const D3D11_TRACE_INPUT_THREAD_ID_IN_GROUP_REGISTER: i32 = 27i32;
pub const D3D11_TRACE_INPUT_COVERAGE_MASK_REGISTER: i32 = 28i32;
pub const D3D11_TRACE_INPUT_THREAD_ID_IN_GROUP_FLATTENED_REGISTER: i32 = 29i32;
pub const D3D11_TRACE_INPUT_GS_INSTANCE_ID_REGISTER: i32 = 30i32;
pub const D3D11_TRACE_OUTPUT_DEPTH_GREATER_EQUAL_REGISTER: i32 = 31i32;
pub const D3D11_TRACE_OUTPUT_DEPTH_LESS_EQUAL_REGISTER: i32 = 32i32;
pub const D3D11_TRACE_IMMEDIATE64: i32 = 33i32;
pub const D3D11_TRACE_INPUT_CYCLE_COUNTER_REGISTER: i32 = 34i32;
pub const D3D11_TRACE_INTERFACE_POINTER: i32 = 35i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_TRACE_STATS {
    pub TraceDesc: D3D11_SHADER_TRACE_DESC,
    pub NumInvocationsInStamp: u8,
    pub TargetStampIndex: u8,
    pub NumTraceSteps: u32,
    pub InputMask: [u8; 32],
    pub OutputMask: [u8; 32],
    pub NumTemps: u16,
    pub MaxIndexableTempIndex: u16,
    pub IndexableTempSize: [u16; 4096],
    pub ImmediateConstantBufferSize: u16,
    pub PixelPosition: [u32; 8],
    pub PixelCoverageMask: [u64; 4],
    pub PixelDiscardedMask: [u64; 4],
    pub PixelCoverageMaskAfterShader: [u64; 4],
    pub PixelCoverageMaskAfterA2CSampleMask: [u64; 4],
    pub PixelCoverageMaskAfterA2CSampleMaskDepth: [u64; 4],
    pub PixelCoverageMaskAfterA2CSampleMaskDepthStencil: [u64; 4],
    pub PSOutputsDepth: super::super::Foundation::BOOL,
    pub PSOutputsMask: super::super::Foundation::BOOL,
    pub GSInputPrimitive: D3D11_TRACE_GS_INPUT_PRIMITIVE,
    pub GSInputsPrimitiveID: super::super::Foundation::BOOL,
    pub HSOutputPatchConstantMask: [u8; 32],
    pub DSInputPatchConstantMask: [u8; 32],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_TRACE_STATS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_TRACE_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_TRACE_STEP {
    pub ID: u32,
    pub InstructionActive: super::super::Foundation::BOOL,
    pub NumRegistersWritten: u8,
    pub NumRegistersRead: u8,
    pub MiscOperations: u16,
    pub OpcodeType: u32,
    pub CurrentGlobalCycle: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_TRACE_STEP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_TRACE_STEP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_TRACE_VALUE {
    pub Bits: [u32; 4],
    pub ValidMask: u8,
}
impl ::core::marker::Copy for D3D11_TRACE_VALUE {}
impl ::core::clone::Clone for D3D11_TRACE_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_UAV_DIMENSION_UNKNOWN: i32 = 0i32;
pub const D3D11_UAV_DIMENSION_BUFFER: i32 = 1i32;
pub const D3D11_UAV_DIMENSION_TEXTURE1D: i32 = 2i32;
pub const D3D11_UAV_DIMENSION_TEXTURE1DARRAY: i32 = 3i32;
pub const D3D11_UAV_DIMENSION_TEXTURE2D: i32 = 4i32;
pub const D3D11_UAV_DIMENSION_TEXTURE2DARRAY: i32 = 5i32;
pub const D3D11_UAV_DIMENSION_TEXTURE3D: i32 = 8i32;
pub const D3D11_UNBOUND_MEMORY_ACCESS_RESULT: u32 = 0u32;
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D11_UAV_DIMENSION,
    pub Anonymous: D3D11_UNORDERED_ACCESS_VIEW_DESC_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_UNORDERED_ACCESS_VIEW_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_UNORDERED_ACCESS_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D11_UNORDERED_ACCESS_VIEW_DESC_0 {
    pub Buffer: D3D11_BUFFER_UAV,
    pub Texture1D: D3D11_TEX1D_UAV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_UAV,
    pub Texture2D: D3D11_TEX2D_UAV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_UAV,
    pub Texture3D: D3D11_TEX3D_UAV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_UNORDERED_ACCESS_VIEW_DESC_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_UNORDERED_ACCESS_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC1 {
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ViewDimension: D3D11_UAV_DIMENSION,
    pub Anonymous: D3D11_UNORDERED_ACCESS_VIEW_DESC1_0,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_UNORDERED_ACCESS_VIEW_DESC1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_UNORDERED_ACCESS_VIEW_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub union D3D11_UNORDERED_ACCESS_VIEW_DESC1_0 {
    pub Buffer: D3D11_BUFFER_UAV,
    pub Texture1D: D3D11_TEX1D_UAV,
    pub Texture1DArray: D3D11_TEX1D_ARRAY_UAV,
    pub Texture2D: D3D11_TEX2D_UAV1,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_UAV1,
    pub Texture3D: D3D11_TEX3D_UAV,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_UNORDERED_ACCESS_VIEW_DESC1_0 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_UNORDERED_ACCESS_VIEW_DESC1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_USAGE_DEFAULT: i32 = 0i32;
pub const D3D11_USAGE_IMMUTABLE: i32 = 1i32;
pub const D3D11_USAGE_DYNAMIC: i32 = 2i32;
pub const D3D11_USAGE_STAGING: i32 = 3i32;
pub const D3D11_VDOV_DIMENSION_UNKNOWN: i32 = 0i32;
pub const D3D11_VDOV_DIMENSION_TEXTURE2D: i32 = 1i32;
#[repr(C)]
pub struct D3D11_VERTEX_SHADER_TRACE_DESC {
    pub Invocation: u64,
}
impl ::core::marker::Copy for D3D11_VERTEX_SHADER_TRACE_DESC {}
impl ::core::clone::Clone for D3D11_VERTEX_SHADER_TRACE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_VIDEO_COLOR {
    pub Anonymous: D3D11_VIDEO_COLOR_0,
}
impl ::core::marker::Copy for D3D11_VIDEO_COLOR {}
impl ::core::clone::Clone for D3D11_VIDEO_COLOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D3D11_VIDEO_COLOR_0 {
    pub YCbCr: D3D11_VIDEO_COLOR_YCbCrA,
    pub RGBA: D3D11_VIDEO_COLOR_RGBA,
}
impl ::core::marker::Copy for D3D11_VIDEO_COLOR_0 {}
impl ::core::clone::Clone for D3D11_VIDEO_COLOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_VIDEO_COLOR_RGBA {
    pub R: f32,
    pub G: f32,
    pub B: f32,
    pub A: f32,
}
impl ::core::marker::Copy for D3D11_VIDEO_COLOR_RGBA {}
impl ::core::clone::Clone for D3D11_VIDEO_COLOR_RGBA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_VIDEO_COLOR_YCbCrA {
    pub Y: f32,
    pub Cb: f32,
    pub Cr: f32,
    pub A: f32,
}
impl ::core::marker::Copy for D3D11_VIDEO_COLOR_YCbCrA {}
impl ::core::clone::Clone for D3D11_VIDEO_COLOR_YCbCrA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_VIDEO_CONTENT_PROTECTION_CAPS {
    pub Caps: u32,
    pub KeyExchangeTypeCount: u32,
    pub BlockAlignmentSize: u32,
    pub ProtectedMemorySize: u64,
}
impl ::core::marker::Copy for D3D11_VIDEO_CONTENT_PROTECTION_CAPS {}
impl ::core::clone::Clone for D3D11_VIDEO_CONTENT_PROTECTION_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION {
    pub pCryptoSession: ID3D11CryptoSession,
    pub BlobSize: u32,
    pub pBlob: *mut ::core::ffi::c_void,
    pub pKeyInfoId: *mut ::windows_sys::core::GUID,
    pub PrivateDataSize: u32,
    pub pPrivateData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION {}
impl ::core::clone::Clone for D3D11_VIDEO_DECODER_BEGIN_FRAME_CRYPTO_SESSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_VIDEO_DECODER_BUFFER_DESC {
    pub BufferType: D3D11_VIDEO_DECODER_BUFFER_TYPE,
    pub BufferIndex: u32,
    pub DataOffset: u32,
    pub DataSize: u32,
    pub FirstMBaddress: u32,
    pub NumMBsInBuffer: u32,
    pub Width: u32,
    pub Height: u32,
    pub Stride: u32,
    pub ReservedBits: u32,
    pub pIV: *mut ::core::ffi::c_void,
    pub IVSize: u32,
    pub PartialEncryption: super::super::Foundation::BOOL,
    pub EncryptedBlockInfo: D3D11_ENCRYPTED_BLOCK_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_VIDEO_DECODER_BUFFER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_VIDEO_DECODER_BUFFER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_VIDEO_DECODER_BUFFER_DESC1 {
    pub BufferType: D3D11_VIDEO_DECODER_BUFFER_TYPE,
    pub DataOffset: u32,
    pub DataSize: u32,
    pub pIV: *mut ::core::ffi::c_void,
    pub IVSize: u32,
    pub pSubSampleMappingBlock: *mut D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK,
    pub SubSampleMappingCount: u32,
}
impl ::core::marker::Copy for D3D11_VIDEO_DECODER_BUFFER_DESC1 {}
impl ::core::clone::Clone for D3D11_VIDEO_DECODER_BUFFER_DESC1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_VIDEO_DECODER_BUFFER_DESC2 {
    pub BufferType: D3D11_VIDEO_DECODER_BUFFER_TYPE,
    pub DataOffset: u32,
    pub DataSize: u32,
    pub pIV: *mut ::core::ffi::c_void,
    pub IVSize: u32,
    pub pSubSampleMappingBlock: *mut D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK,
    pub SubSampleMappingCount: u32,
    pub cBlocksStripeEncrypted: u32,
    pub cBlocksStripeClear: u32,
}
impl ::core::marker::Copy for D3D11_VIDEO_DECODER_BUFFER_DESC2 {}
impl ::core::clone::Clone for D3D11_VIDEO_DECODER_BUFFER_DESC2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_VIDEO_DECODER_BUFFER_PICTURE_PARAMETERS: i32 = 0i32;
pub const D3D11_VIDEO_DECODER_BUFFER_MACROBLOCK_CONTROL: i32 = 1i32;
pub const D3D11_VIDEO_DECODER_BUFFER_RESIDUAL_DIFFERENCE: i32 = 2i32;
pub const D3D11_VIDEO_DECODER_BUFFER_DEBLOCKING_CONTROL: i32 = 3i32;
pub const D3D11_VIDEO_DECODER_BUFFER_INVERSE_QUANTIZATION_MATRIX: i32 = 4i32;
pub const D3D11_VIDEO_DECODER_BUFFER_SLICE_CONTROL: i32 = 5i32;
pub const D3D11_VIDEO_DECODER_BUFFER_BITSTREAM: i32 = 6i32;
pub const D3D11_VIDEO_DECODER_BUFFER_MOTION_VECTOR: i32 = 7i32;
pub const D3D11_VIDEO_DECODER_BUFFER_FILM_GRAIN: i32 = 8i32;
pub const D3D11_VIDEO_DECODER_CAPS_DOWNSAMPLE: i32 = 1i32;
pub const D3D11_VIDEO_DECODER_CAPS_NON_REAL_TIME: i32 = 2i32;
pub const D3D11_VIDEO_DECODER_CAPS_DOWNSAMPLE_DYNAMIC: i32 = 4i32;
pub const D3D11_VIDEO_DECODER_CAPS_DOWNSAMPLE_REQUIRED: i32 = 8i32;
pub const D3D11_VIDEO_DECODER_CAPS_UNSUPPORTED: i32 = 16i32;
#[repr(C)]
pub struct D3D11_VIDEO_DECODER_CONFIG {
    pub guidConfigBitstreamEncryption: ::windows_sys::core::GUID,
    pub guidConfigMBcontrolEncryption: ::windows_sys::core::GUID,
    pub guidConfigResidDiffEncryption: ::windows_sys::core::GUID,
    pub ConfigBitstreamRaw: u32,
    pub ConfigMBcontrolRasterOrder: u32,
    pub ConfigResidDiffHost: u32,
    pub ConfigSpatialResid8: u32,
    pub ConfigResid8Subtraction: u32,
    pub ConfigSpatialHost8or9Clipping: u32,
    pub ConfigSpatialResidInterleaved: u32,
    pub ConfigIntraResidUnsigned: u32,
    pub ConfigResidDiffAccelerator: u32,
    pub ConfigHostInverseScan: u32,
    pub ConfigSpecificIDCT: u32,
    pub Config4GroupedCoefs: u32,
    pub ConfigMinRenderTargetBuffCount: u16,
    pub ConfigDecoderSpecific: u16,
}
impl ::core::marker::Copy for D3D11_VIDEO_DECODER_CONFIG {}
impl ::core::clone::Clone for D3D11_VIDEO_DECODER_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D11_VIDEO_DECODER_DESC {
    pub Guid: ::windows_sys::core::GUID,
    pub SampleWidth: u32,
    pub SampleHeight: u32,
    pub OutputFormat: super::Dxgi::Common::DXGI_FORMAT,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_VIDEO_DECODER_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_VIDEO_DECODER_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_VIDEO_DECODER_EXTENSION {
    pub Function: u32,
    pub pPrivateInputData: *mut ::core::ffi::c_void,
    pub PrivateInputDataSize: u32,
    pub pPrivateOutputData: *mut ::core::ffi::c_void,
    pub PrivateOutputDataSize: u32,
    pub ResourceCount: u32,
    pub ppResourceList: *mut ID3D11Resource,
}
impl ::core::marker::Copy for D3D11_VIDEO_DECODER_EXTENSION {}
impl ::core::clone::Clone for D3D11_VIDEO_DECODER_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_Y: i32 = 0i32;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_U: i32 = 1i32;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_V: i32 = 2i32;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_R: i32 = 0i32;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_G: i32 = 1i32;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_B: i32 = 2i32;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_A: i32 = 3i32;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_NONE: u32 = 0u32;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_Y: u32 = 1u32;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_U: u32 = 2u32;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_V: u32 = 4u32;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_R: u32 = 1u32;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_G: u32 = 2u32;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_B: u32 = 4u32;
pub const D3D11_VIDEO_DECODER_HISTOGRAM_COMPONENT_FLAG_A: u32 = 8u32;
#[repr(C)]
pub struct D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC {
    pub DecodeProfile: ::windows_sys::core::GUID,
    pub ViewDimension: D3D11_VDOV_DIMENSION,
    pub Anonymous: D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC_0,
}
impl ::core::marker::Copy for D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC {}
impl ::core::clone::Clone for D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC_0 {
    pub Texture2D: D3D11_TEX2D_VDOV,
}
impl ::core::marker::Copy for D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC_0 {}
impl ::core::clone::Clone for D3D11_VIDEO_DECODER_OUTPUT_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK {
    pub ClearSize: u32,
    pub EncryptedSize: u32,
}
impl ::core::marker::Copy for D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK {}
impl ::core::clone::Clone for D3D11_VIDEO_DECODER_SUB_SAMPLE_MAPPING_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_VIDEO_FRAME_FORMAT_PROGRESSIVE: i32 = 0i32;
pub const D3D11_VIDEO_FRAME_FORMAT_INTERLACED_TOP_FIELD_FIRST: i32 = 1i32;
pub const D3D11_VIDEO_FRAME_FORMAT_INTERLACED_BOTTOM_FIELD_FIRST: i32 = 2i32;
pub const D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE_OPAQUE: i32 = 0i32;
pub const D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE_BACKGROUND: i32 = 1i32;
pub const D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE_DESTINATION: i32 = 2i32;
pub const D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE_SOURCE_STREAM: i32 = 3i32;
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_DENOISE: i32 = 1i32;
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_DERINGING: i32 = 2i32;
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_EDGE_ENHANCEMENT: i32 = 4i32;
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_COLOR_CORRECTION: i32 = 8i32;
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_FLESH_TONE_MAPPING: i32 = 16i32;
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_IMAGE_STABILIZATION: i32 = 32i32;
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_SUPER_RESOLUTION: i32 = 64i32;
pub const D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_ANAMORPHIC_SCALING: i32 = 128i32;
pub const D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_ROTATION: i32 = 1i32;
pub const D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_RESIZE: i32 = 2i32;
pub const D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_MULTIPLANE_OVERLAY_COLOR_SPACE_CONVERSION: i32 = 4i32;
pub const D3D11_VIDEO_PROCESSOR_BEHAVIOR_HINT_TRIPLE_BUFFER_OUTPUT: i32 = 8i32;
#[repr(C)]
pub struct D3D11_VIDEO_PROCESSOR_CAPS {
    pub DeviceCaps: u32,
    pub FeatureCaps: u32,
    pub FilterCaps: u32,
    pub InputFormatCaps: u32,
    pub AutoStreamCaps: u32,
    pub StereoCaps: u32,
    pub RateConversionCapsCount: u32,
    pub MaxInputStreams: u32,
    pub MaxStreamStates: u32,
}
impl ::core::marker::Copy for D3D11_VIDEO_PROCESSOR_CAPS {}
impl ::core::clone::Clone for D3D11_VIDEO_PROCESSOR_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct D3D11_VIDEO_PROCESSOR_COLOR_SPACE {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for D3D11_VIDEO_PROCESSOR_COLOR_SPACE {}
impl ::core::clone::Clone for D3D11_VIDEO_PROCESSOR_COLOR_SPACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D11_VIDEO_PROCESSOR_CONTENT_DESC {
    pub InputFrameFormat: D3D11_VIDEO_FRAME_FORMAT,
    pub InputFrameRate: super::Dxgi::Common::DXGI_RATIONAL,
    pub InputWidth: u32,
    pub InputHeight: u32,
    pub OutputFrameRate: super::Dxgi::Common::DXGI_RATIONAL,
    pub OutputWidth: u32,
    pub OutputHeight: u32,
    pub Usage: D3D11_VIDEO_USAGE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_VIDEO_PROCESSOR_CONTENT_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_VIDEO_PROCESSOR_CONTENT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D11_VIDEO_PROCESSOR_CUSTOM_RATE {
    pub CustomRate: super::Dxgi::Common::DXGI_RATIONAL,
    pub OutputFrames: u32,
    pub InputInterlaced: super::super::Foundation::BOOL,
    pub InputFramesOrFields: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D11_VIDEO_PROCESSOR_CUSTOM_RATE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D11_VIDEO_PROCESSOR_CUSTOM_RATE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_VIDEO_PROCESSOR_DEVICE_CAPS_LINEAR_SPACE: i32 = 1i32;
pub const D3D11_VIDEO_PROCESSOR_DEVICE_CAPS_xvYCC: i32 = 2i32;
pub const D3D11_VIDEO_PROCESSOR_DEVICE_CAPS_RGB_RANGE_CONVERSION: i32 = 4i32;
pub const D3D11_VIDEO_PROCESSOR_DEVICE_CAPS_YCbCr_MATRIX_CONVERSION: i32 = 8i32;
pub const D3D11_VIDEO_PROCESSOR_DEVICE_CAPS_NOMINAL_RANGE: i32 = 16i32;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_ALPHA_FILL: i32 = 1i32;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_CONSTRICTION: i32 = 2i32;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_LUMA_KEY: i32 = 4i32;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_ALPHA_PALETTE: i32 = 8i32;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_LEGACY: i32 = 16i32;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_STEREO: i32 = 32i32;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_ROTATION: i32 = 64i32;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_ALPHA_STREAM: i32 = 128i32;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_PIXEL_ASPECT_RATIO: i32 = 256i32;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_MIRROR: i32 = 512i32;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_SHADER_USAGE: i32 = 1024i32;
pub const D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_METADATA_HDR10: i32 = 2048i32;
pub const D3D11_VIDEO_PROCESSOR_FILTER_BRIGHTNESS: i32 = 0i32;
pub const D3D11_VIDEO_PROCESSOR_FILTER_CONTRAST: i32 = 1i32;
pub const D3D11_VIDEO_PROCESSOR_FILTER_HUE: i32 = 2i32;
pub const D3D11_VIDEO_PROCESSOR_FILTER_SATURATION: i32 = 3i32;
pub const D3D11_VIDEO_PROCESSOR_FILTER_NOISE_REDUCTION: i32 = 4i32;
pub const D3D11_VIDEO_PROCESSOR_FILTER_EDGE_ENHANCEMENT: i32 = 5i32;
pub const D3D11_VIDEO_PROCESSOR_FILTER_ANAMORPHIC_SCALING: i32 = 6i32;
pub const D3D11_VIDEO_PROCESSOR_FILTER_STEREO_ADJUSTMENT: i32 = 7i32;
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_BRIGHTNESS: i32 = 1i32;
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_CONTRAST: i32 = 2i32;
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_HUE: i32 = 4i32;
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_SATURATION: i32 = 8i32;
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_NOISE_REDUCTION: i32 = 16i32;
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_EDGE_ENHANCEMENT: i32 = 32i32;
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_ANAMORPHIC_SCALING: i32 = 64i32;
pub const D3D11_VIDEO_PROCESSOR_FILTER_CAPS_STEREO_ADJUSTMENT: i32 = 128i32;
#[repr(C)]
pub struct D3D11_VIDEO_PROCESSOR_FILTER_RANGE {
    pub Minimum: i32,
    pub Maximum: i32,
    pub Default: i32,
    pub Multiplier: f32,
}
impl ::core::marker::Copy for D3D11_VIDEO_PROCESSOR_FILTER_RANGE {}
impl ::core::clone::Clone for D3D11_VIDEO_PROCESSOR_FILTER_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_VIDEO_PROCESSOR_FORMAT_CAPS_RGB_INTERLACED: i32 = 1i32;
pub const D3D11_VIDEO_PROCESSOR_FORMAT_CAPS_RGB_PROCAMP: i32 = 2i32;
pub const D3D11_VIDEO_PROCESSOR_FORMAT_CAPS_RGB_LUMA_KEY: i32 = 4i32;
pub const D3D11_VIDEO_PROCESSOR_FORMAT_CAPS_PALETTE_INTERLACED: i32 = 8i32;
pub const D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT_INPUT: i32 = 1i32;
pub const D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT_OUTPUT: i32 = 2i32;
#[repr(C)]
pub struct D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC {
    pub FourCC: u32,
    pub ViewDimension: D3D11_VPIV_DIMENSION,
    pub Anonymous: D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC_0,
}
impl ::core::marker::Copy for D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC {}
impl ::core::clone::Clone for D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC_0 {
    pub Texture2D: D3D11_TEX2D_VPIV,
}
impl ::core::marker::Copy for D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC_0 {}
impl ::core::clone::Clone for D3D11_VIDEO_PROCESSOR_INPUT_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_32: i32 = 1i32;
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_22: i32 = 2i32;
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_2224: i32 = 4i32;
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_2332: i32 = 8i32;
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_32322: i32 = 16i32;
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_55: i32 = 32i32;
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_64: i32 = 64i32;
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_87: i32 = 128i32;
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_222222222223: i32 = 256i32;
pub const D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_OTHER: i32 = -2147483648i32;
pub const D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE_UNDEFINED: i32 = 0i32;
pub const D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE_16_235: i32 = 1i32;
pub const D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE_0_255: i32 = 2i32;
pub const D3D11_VIDEO_PROCESSOR_OUTPUT_RATE_NORMAL: i32 = 0i32;
pub const D3D11_VIDEO_PROCESSOR_OUTPUT_RATE_HALF: i32 = 1i32;
pub const D3D11_VIDEO_PROCESSOR_OUTPUT_RATE_CUSTOM: i32 = 2i32;
#[repr(C)]
pub struct D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC {
    pub ViewDimension: D3D11_VPOV_DIMENSION,
    pub Anonymous: D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC_0,
}
impl ::core::marker::Copy for D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC {}
impl ::core::clone::Clone for D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC_0 {
    pub Texture2D: D3D11_TEX2D_VPOV,
    pub Texture2DArray: D3D11_TEX2D_ARRAY_VPOV,
}
impl ::core::marker::Copy for D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC_0 {}
impl ::core::clone::Clone for D3D11_VIDEO_PROCESSOR_OUTPUT_VIEW_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_DEINTERLACE_BLEND: i32 = 1i32;
pub const D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_DEINTERLACE_BOB: i32 = 2i32;
pub const D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_DEINTERLACE_ADAPTIVE: i32 = 4i32;
pub const D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_DEINTERLACE_MOTION_COMPENSATION: i32 = 8i32;
pub const D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_INVERSE_TELECINE: i32 = 16i32;
pub const D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_FRAME_RATE_CONVERSION: i32 = 32i32;
#[repr(C)]
pub struct D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS {
    pub PastFrames: u32,
    pub FutureFrames: u32,
    pub ProcessorCaps: u32,
    pub ITelecineCaps: u32,
    pub CustomRateCount: u32,
}
impl ::core::marker::Copy for D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS {}
impl ::core::clone::Clone for D3D11_VIDEO_PROCESSOR_RATE_CONVERSION_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_VIDEO_PROCESSOR_ROTATION_IDENTITY: i32 = 0i32;
pub const D3D11_VIDEO_PROCESSOR_ROTATION_90: i32 = 1i32;
pub const D3D11_VIDEO_PROCESSOR_ROTATION_180: i32 = 2i32;
pub const D3D11_VIDEO_PROCESSOR_ROTATION_270: i32 = 3i32;
pub const D3D11_VIDEO_PROCESSOR_STEREO_CAPS_MONO_OFFSET: i32 = 1i32;
pub const D3D11_VIDEO_PROCESSOR_STEREO_CAPS_ROW_INTERLEAVED: i32 = 2i32;
pub const D3D11_VIDEO_PROCESSOR_STEREO_CAPS_COLUMN_INTERLEAVED: i32 = 4i32;
pub const D3D11_VIDEO_PROCESSOR_STEREO_CAPS_CHECKERBOARD: i32 = 8i32;
pub const D3D11_VIDEO_PROCESSOR_STEREO_CAPS_FLIP_MODE: i32 = 16i32;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FLIP_NONE: i32 = 0i32;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FLIP_FRAME0: i32 = 1i32;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FLIP_FRAME1: i32 = 2i32;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_MONO: i32 = 0i32;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_HORIZONTAL: i32 = 1i32;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_VERTICAL: i32 = 2i32;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_SEPARATE: i32 = 3i32;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_MONO_OFFSET: i32 = 4i32;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_ROW_INTERLEAVED: i32 = 5i32;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_COLUMN_INTERLEAVED: i32 = 6i32;
pub const D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_CHECKERBOARD: i32 = 7i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct D3D11_VIDEO_PROCESSOR_STREAM {
    pub Enable: super::super::Foundation::BOOL,
    pub OutputIndex: u32,
    pub InputFrameOrField: u32,
    pub PastFrames: u32,
    pub FutureFrames: u32,
    pub ppPastSurfaces: *mut ID3D11VideoProcessorInputView,
    pub pInputSurface: ID3D11VideoProcessorInputView,
    pub ppFutureSurfaces: *mut ID3D11VideoProcessorInputView,
    pub ppPastSurfacesRight: *mut ID3D11VideoProcessorInputView,
    pub pInputSurfaceRight: ID3D11VideoProcessorInputView,
    pub ppFutureSurfacesRight: *mut ID3D11VideoProcessorInputView,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for D3D11_VIDEO_PROCESSOR_STREAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for D3D11_VIDEO_PROCESSOR_STREAM {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT {
    pub Enable: super::super::Foundation::BOOL,
    pub Width: u32,
    pub Height: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for D3D11_VIDEO_PROCESSOR_STREAM_BEHAVIOR_HINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct D3D11_VIDEO_SAMPLE_DESC {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::Dxgi::Common::DXGI_FORMAT,
    pub ColorSpace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for D3D11_VIDEO_SAMPLE_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for D3D11_VIDEO_SAMPLE_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_VIDEO_USAGE_PLAYBACK_NORMAL: i32 = 0i32;
pub const D3D11_VIDEO_USAGE_OPTIMAL_SPEED: i32 = 1i32;
pub const D3D11_VIDEO_USAGE_OPTIMAL_QUALITY: i32 = 2i32;
#[repr(C)]
pub struct D3D11_VIEWPORT {
    pub TopLeftX: f32,
    pub TopLeftY: f32,
    pub Width: f32,
    pub Height: f32,
    pub MinDepth: f32,
    pub MaxDepth: f32,
}
impl ::core::marker::Copy for D3D11_VIEWPORT {}
impl ::core::clone::Clone for D3D11_VIEWPORT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3D11_VIEWPORT_AND_SCISSORRECT_MAX_INDEX: u32 = 15u32;
pub const D3D11_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE: u32 = 16u32;
pub const D3D11_VIEWPORT_BOUNDS_MAX: u32 = 32767u32;
pub const D3D11_VIEWPORT_BOUNDS_MIN: i32 = -32768i32;
pub const D3D11_VPIV_DIMENSION_UNKNOWN: i32 = 0i32;
pub const D3D11_VPIV_DIMENSION_TEXTURE2D: i32 = 1i32;
pub const D3D11_VPOV_DIMENSION_UNKNOWN: i32 = 0i32;
pub const D3D11_VPOV_DIMENSION_TEXTURE2D: i32 = 1i32;
pub const D3D11_VPOV_DIMENSION_TEXTURE2DARRAY: i32 = 2i32;
pub const D3D11_VS_INPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_VS_INPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_VS_INPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_VS_INPUT_REGISTER_READS_PER_INST: u32 = 2u32;
pub const D3D11_VS_INPUT_REGISTER_READ_PORTS: u32 = 1u32;
pub const D3D11_VS_OUTPUT_REGISTER_COMPONENTS: u32 = 4u32;
pub const D3D11_VS_OUTPUT_REGISTER_COMPONENT_BIT_COUNT: u32 = 32u32;
pub const D3D11_VS_OUTPUT_REGISTER_COUNT: u32 = 32u32;
pub const D3D11_WHQL_CONTEXT_COUNT_FOR_RESOURCE_LIMIT: u32 = 10u32;
pub const D3D11_WHQL_DRAWINDEXED_INDEX_COUNT_2_TO_EXP: u32 = 25u32;
pub const D3D11_WHQL_DRAW_VERTEX_COUNT_2_TO_EXP: u32 = 25u32;
#[repr(C)]
pub struct D3DX11_FFT_BUFFER_INFO {
    pub NumTempBufferSizes: u32,
    pub TempBufferFloatSizes: [u32; 4],
    pub NumPrecomputeBufferSizes: u32,
    pub PrecomputeBufferFloatSizes: [u32; 4],
}
impl ::core::marker::Copy for D3DX11_FFT_BUFFER_INFO {}
impl ::core::clone::Clone for D3DX11_FFT_BUFFER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3DX11_FFT_CREATE_FLAG_NO_PRECOMPUTE_BUFFERS: i32 = 1i32;
pub const D3DX11_FFT_DATA_TYPE_REAL: i32 = 0i32;
pub const D3DX11_FFT_DATA_TYPE_COMPLEX: i32 = 1i32;
#[repr(C)]
pub struct D3DX11_FFT_DESC {
    pub NumDimensions: u32,
    pub ElementLengths: [u32; 32],
    pub DimensionMask: u32,
    pub Type: D3DX11_FFT_DATA_TYPE,
}
impl ::core::marker::Copy for D3DX11_FFT_DESC {}
impl ::core::clone::Clone for D3DX11_FFT_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const D3DX11_FFT_DIM_MASK_1D: i32 = 1i32;
pub const D3DX11_FFT_DIM_MASK_2D: i32 = 3i32;
pub const D3DX11_FFT_DIM_MASK_3D: i32 = 7i32;
pub const D3DX11_FFT_MAX_DIMENSIONS: u32 = 32u32;
pub const D3DX11_FFT_MAX_PRECOMPUTE_BUFFERS: u32 = 4u32;
pub const D3DX11_FFT_MAX_TEMP_BUFFERS: u32 = 4u32;
pub const D3DX11_SCAN_DATA_TYPE_FLOAT: i32 = 1i32;
pub const D3DX11_SCAN_DATA_TYPE_INT: i32 = 2i32;
pub const D3DX11_SCAN_DATA_TYPE_UINT: i32 = 3i32;
pub const D3DX11_SCAN_DIRECTION_FORWARD: i32 = 1i32;
pub const D3DX11_SCAN_DIRECTION_BACKWARD: i32 = 2i32;
pub const D3DX11_SCAN_OPCODE_ADD: i32 = 1i32;
pub const D3DX11_SCAN_OPCODE_MIN: i32 = 2i32;
pub const D3DX11_SCAN_OPCODE_MAX: i32 = 3i32;
pub const D3DX11_SCAN_OPCODE_MUL: i32 = 4i32;
pub const D3DX11_SCAN_OPCODE_AND: i32 = 5i32;
pub const D3DX11_SCAN_OPCODE_OR: i32 = 6i32;
pub const D3DX11_SCAN_OPCODE_XOR: i32 = 7i32;
pub const D3D_RETURN_PARAMETER_INDEX: i32 = -1i32;
pub const D3D_SHADER_REQUIRES_11_1_DOUBLE_EXTENSIONS: u32 = 32u32;
pub const D3D_SHADER_REQUIRES_11_1_SHADER_EXTENSIONS: u32 = 64u32;
pub const D3D_SHADER_REQUIRES_64_UAVS: u32 = 8u32;
pub const D3D_SHADER_REQUIRES_DOUBLES: u32 = 1u32;
pub const D3D_SHADER_REQUIRES_EARLY_DEPTH_STENCIL: u32 = 2u32;
pub const D3D_SHADER_REQUIRES_LEVEL_9_COMPARISON_FILTERING: u32 = 128u32;
pub const D3D_SHADER_REQUIRES_MINIMUM_PRECISION: u32 = 16u32;
pub const D3D_SHADER_REQUIRES_TILED_RESOURCES: u32 = 256u32;
pub const D3D_SHADER_REQUIRES_UAVS_AT_EVERY_STAGE: u32 = 4u32;
pub const DXGI_DEBUG_D3D11: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1268330875,
    data2: 44089,
    data3: 19110,
    data4: [187, 11, 186, 160, 71, 132, 121, 143],
};
#[repr(transparent)]
pub struct ID3D11Asynchronous(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Asynchronous {}
impl ::core::clone::Clone for ID3D11Asynchronous {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11AuthenticatedChannel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11AuthenticatedChannel {}
impl ::core::clone::Clone for ID3D11AuthenticatedChannel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11BlendState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11BlendState {}
impl ::core::clone::Clone for ID3D11BlendState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11BlendState1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11BlendState1 {}
impl ::core::clone::Clone for ID3D11BlendState1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Buffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Buffer {}
impl ::core::clone::Clone for ID3D11Buffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11ClassInstance(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11ClassInstance {}
impl ::core::clone::Clone for ID3D11ClassInstance {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11ClassLinkage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11ClassLinkage {}
impl ::core::clone::Clone for ID3D11ClassLinkage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11CommandList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11CommandList {}
impl ::core::clone::Clone for ID3D11CommandList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11ComputeShader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11ComputeShader {}
impl ::core::clone::Clone for ID3D11ComputeShader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Counter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Counter {}
impl ::core::clone::Clone for ID3D11Counter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11CryptoSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11CryptoSession {}
impl ::core::clone::Clone for ID3D11CryptoSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Debug(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Debug {}
impl ::core::clone::Clone for ID3D11Debug {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11DepthStencilState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11DepthStencilState {}
impl ::core::clone::Clone for ID3D11DepthStencilState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11DepthStencilView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11DepthStencilView {}
impl ::core::clone::Clone for ID3D11DepthStencilView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Device(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Device {}
impl ::core::clone::Clone for ID3D11Device {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Device1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Device1 {}
impl ::core::clone::Clone for ID3D11Device1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Device2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Device2 {}
impl ::core::clone::Clone for ID3D11Device2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Device3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Device3 {}
impl ::core::clone::Clone for ID3D11Device3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Device4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Device4 {}
impl ::core::clone::Clone for ID3D11Device4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Device5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Device5 {}
impl ::core::clone::Clone for ID3D11Device5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11DeviceChild(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11DeviceChild {}
impl ::core::clone::Clone for ID3D11DeviceChild {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11DeviceContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11DeviceContext {}
impl ::core::clone::Clone for ID3D11DeviceContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11DeviceContext1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11DeviceContext1 {}
impl ::core::clone::Clone for ID3D11DeviceContext1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11DeviceContext2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11DeviceContext2 {}
impl ::core::clone::Clone for ID3D11DeviceContext2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11DeviceContext3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11DeviceContext3 {}
impl ::core::clone::Clone for ID3D11DeviceContext3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11DeviceContext4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11DeviceContext4 {}
impl ::core::clone::Clone for ID3D11DeviceContext4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11DomainShader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11DomainShader {}
impl ::core::clone::Clone for ID3D11DomainShader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Fence(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Fence {}
impl ::core::clone::Clone for ID3D11Fence {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11FunctionLinkingGraph(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11FunctionLinkingGraph {}
impl ::core::clone::Clone for ID3D11FunctionLinkingGraph {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11FunctionParameterReflection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11FunctionParameterReflection {}
impl ::core::clone::Clone for ID3D11FunctionParameterReflection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11FunctionReflection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11FunctionReflection {}
impl ::core::clone::Clone for ID3D11FunctionReflection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11GeometryShader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11GeometryShader {}
impl ::core::clone::Clone for ID3D11GeometryShader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11HullShader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11HullShader {}
impl ::core::clone::Clone for ID3D11HullShader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11InfoQueue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11InfoQueue {}
impl ::core::clone::Clone for ID3D11InfoQueue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11InputLayout(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11InputLayout {}
impl ::core::clone::Clone for ID3D11InputLayout {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11LibraryReflection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11LibraryReflection {}
impl ::core::clone::Clone for ID3D11LibraryReflection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Linker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Linker {}
impl ::core::clone::Clone for ID3D11Linker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11LinkingNode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11LinkingNode {}
impl ::core::clone::Clone for ID3D11LinkingNode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Module(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Module {}
impl ::core::clone::Clone for ID3D11Module {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11ModuleInstance(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11ModuleInstance {}
impl ::core::clone::Clone for ID3D11ModuleInstance {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Multithread(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Multithread {}
impl ::core::clone::Clone for ID3D11Multithread {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11PixelShader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11PixelShader {}
impl ::core::clone::Clone for ID3D11PixelShader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Predicate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Predicate {}
impl ::core::clone::Clone for ID3D11Predicate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Query(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Query {}
impl ::core::clone::Clone for ID3D11Query {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Query1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Query1 {}
impl ::core::clone::Clone for ID3D11Query1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11RasterizerState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11RasterizerState {}
impl ::core::clone::Clone for ID3D11RasterizerState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11RasterizerState1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11RasterizerState1 {}
impl ::core::clone::Clone for ID3D11RasterizerState1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11RasterizerState2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11RasterizerState2 {}
impl ::core::clone::Clone for ID3D11RasterizerState2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11RefDefaultTrackingOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11RefDefaultTrackingOptions {}
impl ::core::clone::Clone for ID3D11RefDefaultTrackingOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11RefTrackingOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11RefTrackingOptions {}
impl ::core::clone::Clone for ID3D11RefTrackingOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11RenderTargetView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11RenderTargetView {}
impl ::core::clone::Clone for ID3D11RenderTargetView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11RenderTargetView1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11RenderTargetView1 {}
impl ::core::clone::Clone for ID3D11RenderTargetView1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Resource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Resource {}
impl ::core::clone::Clone for ID3D11Resource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11SamplerState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11SamplerState {}
impl ::core::clone::Clone for ID3D11SamplerState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11ShaderReflection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11ShaderReflection {}
impl ::core::clone::Clone for ID3D11ShaderReflection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11ShaderReflectionConstantBuffer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11ShaderReflectionConstantBuffer {}
impl ::core::clone::Clone for ID3D11ShaderReflectionConstantBuffer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11ShaderReflectionType(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11ShaderReflectionType {}
impl ::core::clone::Clone for ID3D11ShaderReflectionType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11ShaderReflectionVariable(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11ShaderReflectionVariable {}
impl ::core::clone::Clone for ID3D11ShaderReflectionVariable {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11ShaderResourceView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11ShaderResourceView {}
impl ::core::clone::Clone for ID3D11ShaderResourceView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11ShaderResourceView1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11ShaderResourceView1 {}
impl ::core::clone::Clone for ID3D11ShaderResourceView1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11ShaderTrace(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11ShaderTrace {}
impl ::core::clone::Clone for ID3D11ShaderTrace {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11ShaderTraceFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11ShaderTraceFactory {}
impl ::core::clone::Clone for ID3D11ShaderTraceFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11SwitchToRef(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11SwitchToRef {}
impl ::core::clone::Clone for ID3D11SwitchToRef {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Texture1D(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Texture1D {}
impl ::core::clone::Clone for ID3D11Texture1D {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Texture2D(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Texture2D {}
impl ::core::clone::Clone for ID3D11Texture2D {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Texture2D1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Texture2D1 {}
impl ::core::clone::Clone for ID3D11Texture2D1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Texture3D(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Texture3D {}
impl ::core::clone::Clone for ID3D11Texture3D {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11Texture3D1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11Texture3D1 {}
impl ::core::clone::Clone for ID3D11Texture3D1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11TracingDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11TracingDevice {}
impl ::core::clone::Clone for ID3D11TracingDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11UnorderedAccessView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11UnorderedAccessView {}
impl ::core::clone::Clone for ID3D11UnorderedAccessView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11UnorderedAccessView1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11UnorderedAccessView1 {}
impl ::core::clone::Clone for ID3D11UnorderedAccessView1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11VertexShader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11VertexShader {}
impl ::core::clone::Clone for ID3D11VertexShader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11VideoContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11VideoContext {}
impl ::core::clone::Clone for ID3D11VideoContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11VideoContext1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11VideoContext1 {}
impl ::core::clone::Clone for ID3D11VideoContext1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11VideoContext2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11VideoContext2 {}
impl ::core::clone::Clone for ID3D11VideoContext2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11VideoContext3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11VideoContext3 {}
impl ::core::clone::Clone for ID3D11VideoContext3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11VideoDecoder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11VideoDecoder {}
impl ::core::clone::Clone for ID3D11VideoDecoder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11VideoDecoderOutputView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11VideoDecoderOutputView {}
impl ::core::clone::Clone for ID3D11VideoDecoderOutputView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11VideoDevice(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11VideoDevice {}
impl ::core::clone::Clone for ID3D11VideoDevice {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11VideoDevice1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11VideoDevice1 {}
impl ::core::clone::Clone for ID3D11VideoDevice1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11VideoDevice2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11VideoDevice2 {}
impl ::core::clone::Clone for ID3D11VideoDevice2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11VideoProcessor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11VideoProcessor {}
impl ::core::clone::Clone for ID3D11VideoProcessor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11VideoProcessorEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11VideoProcessorEnumerator {}
impl ::core::clone::Clone for ID3D11VideoProcessorEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11VideoProcessorEnumerator1(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11VideoProcessorEnumerator1 {}
impl ::core::clone::Clone for ID3D11VideoProcessorEnumerator1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11VideoProcessorInputView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11VideoProcessorInputView {}
impl ::core::clone::Clone for ID3D11VideoProcessorInputView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11VideoProcessorOutputView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11VideoProcessorOutputView {}
impl ::core::clone::Clone for ID3D11VideoProcessorOutputView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3D11View(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3D11View {}
impl ::core::clone::Clone for ID3D11View {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3DDeviceContextState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3DDeviceContextState {}
impl ::core::clone::Clone for ID3DDeviceContextState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3DUserDefinedAnnotation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3DUserDefinedAnnotation {}
impl ::core::clone::Clone for ID3DUserDefinedAnnotation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3DX11FFT(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3DX11FFT {}
impl ::core::clone::Clone for ID3DX11FFT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3DX11Scan(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3DX11Scan {}
impl ::core::clone::Clone for ID3DX11Scan {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ID3DX11SegmentedScan(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ID3DX11SegmentedScan {}
impl ::core::clone::Clone for ID3DX11SegmentedScan {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi"))]
pub type PFN_D3D11_CREATE_DEVICE = unsafe extern "system" fn(param0: super::Dxgi::IDXGIAdapter, param1: super::Direct3D::D3D_DRIVER_TYPE, param2: super::super::Foundation::HINSTANCE, param3: u32, param4: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, param6: u32, param7: *mut ID3D11Device, param8: *mut super::Direct3D::D3D_FEATURE_LEVEL, param9: *mut ID3D11DeviceContext) -> ::windows_sys::core::HRESULT;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi", feature = "Win32_Graphics_Dxgi_Common"))]
pub type PFN_D3D11_CREATE_DEVICE_AND_SWAP_CHAIN =
    unsafe extern "system" fn(param0: super::Dxgi::IDXGIAdapter, param1: super::Direct3D::D3D_DRIVER_TYPE, param2: super::super::Foundation::HINSTANCE, param3: u32, param4: *const super::Direct3D::D3D_FEATURE_LEVEL, featurelevels: u32, param6: u32, param7: *const super::Dxgi::DXGI_SWAP_CHAIN_DESC, param8: *mut super::Dxgi::IDXGISwapChain, param9: *mut ID3D11Device, param10: *mut super::Direct3D::D3D_FEATURE_LEVEL, param11: *mut ID3D11DeviceContext) -> ::windows_sys::core::HRESULT;
pub const _FACD3D11: u32 = 2172u32;
