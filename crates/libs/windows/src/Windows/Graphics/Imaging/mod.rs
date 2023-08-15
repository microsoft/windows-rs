#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapBuffer(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapBuffer {
    type Vtable = IBitmapBuffer_Vtbl;
}
impl ::core::clone::Clone for IBitmapBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBitmapBuffer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa53e04c4_399c_438c_b28f_a63a6b83d1a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapBuffer_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetPlaneCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub GetPlaneDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, result__: *mut BitmapPlaneDescription) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapCodecInformation(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapCodecInformation {
    type Vtable = IBitmapCodecInformation_Vtbl;
}
impl ::core::clone::Clone for IBitmapCodecInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBitmapCodecInformation {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x400caaf2_c4b0_4392_a3b0_6f6f9ba95cb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapCodecInformation_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CodecId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FileExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FileExtensions: usize,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MimeTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MimeTypes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapDecoder(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapDecoder {
    type Vtable = IBitmapDecoder_Vtbl;
}
impl ::core::clone::Clone for IBitmapDecoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBitmapDecoder {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xacef22ba_1d74_4c91_9dfc_9620745233e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapDecoder_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BitmapContainerProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DecoderInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub FrameCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetPreviewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetPreviewAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameindex: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFrameAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapDecoderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapDecoderStatics {
    type Vtable = IBitmapDecoderStatics_Vtbl;
}
impl ::core::clone::Clone for IBitmapDecoderStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBitmapDecoderStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x438ccb26_bcef_4e95_bad6_23a822e58d01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapDecoderStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BmpDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub JpegDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub PngDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub TiffDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GifDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub JpegXRDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub IcoDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDecoderInformationEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDecoderInformationEnumerator: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateWithIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, decoderid: ::windows_core::GUID, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateWithIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapDecoderStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapDecoderStatics2 {
    type Vtable = IBitmapDecoderStatics2_Vtbl;
}
impl ::core::clone::Clone for IBitmapDecoderStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBitmapDecoderStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x50ba68ea_99a1_40c4_80d9_aef0dafa6c3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapDecoderStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HeifDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub WebpDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapEncoder(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapEncoder {
    type Vtable = IBitmapEncoder_Vtbl;
}
impl ::core::clone::Clone for IBitmapEncoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBitmapEncoder {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2bc468e3_e1f8_4b54_95e8_32919551ce62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapEncoder_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub EncoderInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BitmapProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BitmapContainerProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsThumbnailGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsThumbnailGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub GeneratedThumbnailWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetGeneratedThumbnailWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub GeneratedThumbnailHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetGeneratedThumbnailHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub BitmapTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetPixelData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, width: u32, height: u32, dpix: f64, dpiy: f64, pixels_array_size: u32, pixels: *const u8) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GoToNextFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GoToNextFrameAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GoToNextFrameWithEncodingOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GoToNextFrameWithEncodingOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FlushAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FlushAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapEncoderStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapEncoderStatics {
    type Vtable = IBitmapEncoderStatics_Vtbl;
}
impl ::core::clone::Clone for IBitmapEncoderStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBitmapEncoderStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa74356a7_a4e4_4eb9_8e40_564de7e1ccb2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapEncoderStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BmpEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub JpegEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub PngEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub TiffEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GifEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub JpegXREncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetEncoderInformationEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetEncoderInformationEnumerator: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encoderid: ::windows_core::GUID, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub CreateWithEncodingOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encoderid: ::windows_core::GUID, stream: *mut ::core::ffi::c_void, encodingoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    CreateWithEncodingOptionsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateForTranscodingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, bitmapdecoder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateForTranscodingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateForInPlacePropertyEncodingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapdecoder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateForInPlacePropertyEncodingAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapEncoderStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapEncoderStatics2 {
    type Vtable = IBitmapEncoderStatics2_Vtbl;
}
impl ::core::clone::Clone for IBitmapEncoderStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBitmapEncoderStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x33cbc259_fe31_41b1_b812_086d21e87e16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapEncoderStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub HeifEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapEncoderWithSoftwareBitmap(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapEncoderWithSoftwareBitmap {
    type Vtable = IBitmapEncoderWithSoftwareBitmap_Vtbl;
}
impl ::core::clone::Clone for IBitmapEncoderWithSoftwareBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBitmapEncoderWithSoftwareBitmap {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x686cd241_4330_4c77_ace4_0334968b1768);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapEncoderWithSoftwareBitmap_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetSoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IBitmapFrame(::windows_core::IUnknown);
impl IBitmapFrame {
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ImageStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetThumbnailAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BitmapProperties(&self) -> ::windows_core::Result<BitmapPropertiesView> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BitmapPixelFormat(&self) -> ::windows_core::Result<BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapPixelFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> ::windows_core::Result<BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapAlphaMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DpiX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DpiX)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DpiY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DpiY)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelWidth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelHeight)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OrientedPixelWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OrientedPixelWidth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OrientedPixelHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OrientedPixelHeight)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPixelDataAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataTransformedAsync<P0>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: P0, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>>
    where
        P0: ::windows_core::IntoParam<BitmapTransform>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPixelDataTransformedAsync)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IBitmapFrame, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IBitmapFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBitmapFrame {}
impl ::core::fmt::Debug for IBitmapFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBitmapFrame").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IBitmapFrame {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{72a49a1c-8081-438d-91bc-94ecfc8185c6}");
}
unsafe impl ::windows_core::Interface for IBitmapFrame {
    type Vtable = IBitmapFrame_Vtbl;
}
impl ::core::clone::Clone for IBitmapFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBitmapFrame {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72a49a1c_8081_438d_91bc_94ecfc8185c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapFrame_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetThumbnailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetThumbnailAsync: usize,
    pub BitmapProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub BitmapPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapPixelFormat) -> ::windows_core::HRESULT,
    pub BitmapAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapAlphaMode) -> ::windows_core::HRESULT,
    pub DpiX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub DpiY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub PixelWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub PixelHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub OrientedPixelWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub OrientedPixelHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPixelDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPixelDataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetPixelDataTransformedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: *mut ::core::ffi::c_void, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPixelDataTransformedAsync: usize,
}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IBitmapFrameWithSoftwareBitmap(::windows_core::IUnknown);
impl IBitmapFrameWithSoftwareBitmap {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSoftwareBitmapAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapConvertedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSoftwareBitmapConvertedAsync)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapTransformedAsync<P0>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: P0, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>
    where
        P0: ::windows_core::IntoParam<BitmapTransform>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSoftwareBitmapTransformedAsync)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ImageStream>> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetThumbnailAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BitmapProperties(&self) -> ::windows_core::Result<BitmapPropertiesView> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BitmapPixelFormat(&self) -> ::windows_core::Result<BitmapPixelFormat> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapPixelFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> ::windows_core::Result<BitmapAlphaMode> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapAlphaMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DpiX(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DpiX)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DpiY(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DpiY)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelWidth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelHeight)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OrientedPixelWidth(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OrientedPixelWidth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OrientedPixelHeight(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OrientedPixelHeight)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPixelDataAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataTransformedAsync<P0>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: P0, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>>
    where
        P0: ::windows_core::IntoParam<BitmapTransform>,
    {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPixelDataTransformedAsync)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IBitmapFrameWithSoftwareBitmap, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBitmapFrame> for IBitmapFrameWithSoftwareBitmap {}
impl ::core::cmp::PartialEq for IBitmapFrameWithSoftwareBitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBitmapFrameWithSoftwareBitmap {}
impl ::core::fmt::Debug for IBitmapFrameWithSoftwareBitmap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBitmapFrameWithSoftwareBitmap").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IBitmapFrameWithSoftwareBitmap {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{fe287c9a-420c-4963-87ad-691436e08383}");
}
unsafe impl ::windows_core::Interface for IBitmapFrameWithSoftwareBitmap {
    type Vtable = IBitmapFrameWithSoftwareBitmap_Vtbl;
}
impl ::core::clone::Clone for IBitmapFrameWithSoftwareBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBitmapFrameWithSoftwareBitmap {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe287c9a_420c_4963_87ad_691436e08383);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapFrameWithSoftwareBitmap_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetSoftwareBitmapAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSoftwareBitmapAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetSoftwareBitmapConvertedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSoftwareBitmapConvertedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetSoftwareBitmapTransformedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: *mut ::core::ffi::c_void, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSoftwareBitmapTransformedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapProperties(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapProperties {
    type Vtable = IBitmapProperties_Vtbl;
}
impl ::core::clone::Clone for IBitmapProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBitmapProperties {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xea9f4f1b_b505_4450_a4d1_e8ca94529d8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapProperties_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiestoset: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPropertiesAsync: usize,
}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IBitmapPropertiesView(::windows_core::IUnknown);
impl IBitmapPropertiesView {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPropertiesAsync<P0>(&self, propertiestoretrieve: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<BitmapPropertySet>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPropertiesAsync)(::windows_core::Interface::as_raw(this), propertiestoretrieve.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IBitmapPropertiesView, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IBitmapPropertiesView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBitmapPropertiesView {}
impl ::core::fmt::Debug for IBitmapPropertiesView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBitmapPropertiesView").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IBitmapPropertiesView {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{7e0fe87a-3a70-48f8-9c55-196cf5a545f5}");
}
unsafe impl ::windows_core::Interface for IBitmapPropertiesView {
    type Vtable = IBitmapPropertiesView_Vtbl;
}
impl ::core::clone::Clone for IBitmapPropertiesView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBitmapPropertiesView {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e0fe87a_3a70_48f8_9c55_196cf5a545f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapPropertiesView_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiestoretrieve: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPropertiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapTransform(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapTransform {
    type Vtable = IBitmapTransform_Vtbl;
}
impl ::core::clone::Clone for IBitmapTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBitmapTransform {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xae755344_e268_4d35_adcf_e995d31a8d34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapTransform_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ScaledWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetScaledWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub ScaledHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
    pub SetScaledHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT,
    pub InterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapInterpolationMode) -> ::windows_core::HRESULT,
    pub SetInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BitmapInterpolationMode) -> ::windows_core::HRESULT,
    pub Flip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapFlip) -> ::windows_core::HRESULT,
    pub SetFlip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BitmapFlip) -> ::windows_core::HRESULT,
    pub Rotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapRotation) -> ::windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BitmapRotation) -> ::windows_core::HRESULT,
    pub Bounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapBounds) -> ::windows_core::HRESULT,
    pub SetBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BitmapBounds) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapTypedValue(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapTypedValue {
    type Vtable = IBitmapTypedValue_Vtbl;
}
impl ::core::clone::Clone for IBitmapTypedValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBitmapTypedValue {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xcd8044a9_2443_4000_b0cd_79316c56f589);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapTypedValue_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::PropertyType) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Type: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapTypedValueFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IBitmapTypedValueFactory {
    type Vtable = IBitmapTypedValueFactory_Vtbl;
}
impl ::core::clone::Clone for IBitmapTypedValueFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IBitmapTypedValueFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x92dbb599_ce13_46bb_9545_cb3a3f63eb8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapTypedValueFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, r#type: super::super::Foundation::PropertyType, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPixelDataProvider(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IPixelDataProvider {
    type Vtable = IPixelDataProvider_Vtbl;
}
impl ::core::clone::Clone for IPixelDataProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPixelDataProvider {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdd831f25_185c_4595_9fb9_ccbe6ec18a6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPixelDataProvider_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub DetachPixelData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISoftwareBitmap(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISoftwareBitmap {
    type Vtable = ISoftwareBitmap_Vtbl;
}
impl ::core::clone::Clone for ISoftwareBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISoftwareBitmap {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x689e0708_7eef_483f_963f_da938818e073);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmap_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub BitmapPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapPixelFormat) -> ::windows_core::HRESULT,
    pub BitmapAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapAlphaMode) -> ::windows_core::HRESULT,
    pub PixelWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub PixelHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetDpiX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub DpiX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub SetDpiY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows_core::HRESULT,
    pub DpiY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
    pub LockBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: BitmapBufferAccessMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CopyFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CopyFromBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CopyToBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CopyToBuffer: usize,
    pub GetReadOnlyView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISoftwareBitmapFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISoftwareBitmapFactory {
    type Vtable = ISoftwareBitmapFactory_Vtbl;
}
impl ::core::clone::Clone for ISoftwareBitmapFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISoftwareBitmapFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc99feb69_2d62_4d47_a6b3_4fdb6a07fdf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: BitmapPixelFormat, width: i32, height: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateWithAlpha: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISoftwareBitmapStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ISoftwareBitmapStatics {
    type Vtable = ISoftwareBitmapStatics_Vtbl;
}
impl ::core::clone::Clone for ISoftwareBitmapStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ISoftwareBitmapStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xdf0385db_672f_4a9d_806e_c2442f343e86);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Convert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, format: BitmapPixelFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ConvertWithAlpha: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, format: BitmapPixelFormat, alpha: BitmapAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCopyFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, format: BitmapPixelFormat, width: i32, height: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCopyFromBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCopyWithAlphaFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCopyWithAlphaFromBuffer: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub CreateCopyFromSurfaceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, surface: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))]
    CreateCopyFromSurfaceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub CreateCopyWithAlphaFromSurfaceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, surface: *mut ::core::ffi::c_void, alpha: BitmapAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))]
    CreateCopyWithAlphaFromSurfaceAsync: usize,
}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct BitmapBuffer(::windows_core::IUnknown);
impl BitmapBuffer {
    pub fn GetPlaneCount(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPlaneCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetPlaneDescription(&self, index: i32) -> ::windows_core::Result<BitmapPlaneDescription> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPlaneDescription)(::windows_core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows_core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateReference)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for BitmapBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapBuffer {}
impl ::core::fmt::Debug for BitmapBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapBuffer").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BitmapBuffer {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapBuffer;{a53e04c4-399c-438c-b28f-a63a6b83d1a1})");
}
impl ::core::clone::Clone for BitmapBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for BitmapBuffer {
    type Vtable = IBitmapBuffer_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BitmapBuffer {
    const IID: ::windows_core::GUID = <IBitmapBuffer as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BitmapBuffer {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapBuffer";
}
::windows_core::imp::interface_hierarchy!(BitmapBuffer, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for BitmapBuffer {}
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IMemoryBuffer> for BitmapBuffer {}
unsafe impl ::core::marker::Send for BitmapBuffer {}
unsafe impl ::core::marker::Sync for BitmapBuffer {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct BitmapCodecInformation(::windows_core::IUnknown);
impl BitmapCodecInformation {
    pub fn CodecId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CodecId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FileExtensions(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FileExtensions)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FriendlyName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FriendlyName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MimeTypes(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MimeTypes)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for BitmapCodecInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapCodecInformation {}
impl ::core::fmt::Debug for BitmapCodecInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapCodecInformation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BitmapCodecInformation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapCodecInformation;{400caaf2-c4b0-4392-a3b0-6f6f9ba95cb4})");
}
impl ::core::clone::Clone for BitmapCodecInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for BitmapCodecInformation {
    type Vtable = IBitmapCodecInformation_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BitmapCodecInformation {
    const IID: ::windows_core::GUID = <IBitmapCodecInformation as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BitmapCodecInformation {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapCodecInformation";
}
::windows_core::imp::interface_hierarchy!(BitmapCodecInformation, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BitmapCodecInformation {}
unsafe impl ::core::marker::Sync for BitmapCodecInformation {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct BitmapDecoder(::windows_core::IUnknown);
impl BitmapDecoder {
    pub fn BitmapContainerProperties(&self) -> ::windows_core::Result<BitmapPropertiesView> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapContainerProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DecoderInformation(&self) -> ::windows_core::Result<BitmapCodecInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DecoderInformation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn FrameCount(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FrameCount)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetPreviewAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ImageStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPreviewAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFrameAsync(&self, frameindex: u32) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<BitmapFrame>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFrameAsync)(::windows_core::Interface::as_raw(this), frameindex, &mut result__).from_abi(result__)
        }
    }
    pub fn BmpDecoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BmpDecoderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn JpegDecoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).JpegDecoderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn PngDecoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PngDecoderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TiffDecoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TiffDecoderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GifDecoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GifDecoderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn JpegXRDecoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).JpegXRDecoderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IcoDecoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IcoDecoderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDecoderInformationEnumerator() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<BitmapCodecInformation>> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetDecoderInformationEnumerator)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateAsync<P0>(stream: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<BitmapDecoder>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsync)(::windows_core::Interface::as_raw(this), stream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateWithIdAsync<P0>(decoderid: ::windows_core::GUID, stream: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<BitmapDecoder>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithIdAsync)(::windows_core::Interface::as_raw(this), decoderid, stream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn HeifDecoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapDecoderStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HeifDecoderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn WebpDecoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapDecoderStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WebpDecoderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ImageStream>> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetThumbnailAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BitmapProperties(&self) -> ::windows_core::Result<BitmapPropertiesView> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BitmapPixelFormat(&self) -> ::windows_core::Result<BitmapPixelFormat> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapPixelFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> ::windows_core::Result<BitmapAlphaMode> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapAlphaMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DpiX(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DpiX)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DpiY(&self) -> ::windows_core::Result<f64> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DpiY)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelWidth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelHeight)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OrientedPixelWidth(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OrientedPixelWidth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OrientedPixelHeight(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OrientedPixelHeight)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPixelDataAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataTransformedAsync<P0>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: P0, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>>
    where
        P0: ::windows_core::IntoParam<BitmapTransform>,
    {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPixelDataTransformedAsync)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSoftwareBitmapAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapConvertedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSoftwareBitmapConvertedAsync)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapTransformedAsync<P0>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: P0, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>
    where
        P0: ::windows_core::IntoParam<BitmapTransform>,
    {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSoftwareBitmapTransformedAsync)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IBitmapDecoderStatics<R, F: FnOnce(&IBitmapDecoderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BitmapDecoder, IBitmapDecoderStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBitmapDecoderStatics2<R, F: FnOnce(&IBitmapDecoderStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BitmapDecoder, IBitmapDecoderStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for BitmapDecoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapDecoder {}
impl ::core::fmt::Debug for BitmapDecoder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapDecoder").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BitmapDecoder {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapDecoder;{acef22ba-1d74-4c91-9dfc-9620745233e6})");
}
impl ::core::clone::Clone for BitmapDecoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for BitmapDecoder {
    type Vtable = IBitmapDecoder_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BitmapDecoder {
    const IID: ::windows_core::GUID = <IBitmapDecoder as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BitmapDecoder {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapDecoder";
}
::windows_core::imp::interface_hierarchy!(BitmapDecoder, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBitmapFrame> for BitmapDecoder {}
impl ::windows_core::CanTryInto<IBitmapFrameWithSoftwareBitmap> for BitmapDecoder {}
unsafe impl ::core::marker::Send for BitmapDecoder {}
unsafe impl ::core::marker::Sync for BitmapDecoder {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct BitmapEncoder(::windows_core::IUnknown);
impl BitmapEncoder {
    pub fn EncoderInformation(&self) -> ::windows_core::Result<BitmapCodecInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EncoderInformation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BitmapProperties(&self) -> ::windows_core::Result<BitmapProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BitmapContainerProperties(&self) -> ::windows_core::Result<BitmapProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapContainerProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsThumbnailGenerated(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsThumbnailGenerated)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsThumbnailGenerated(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsThumbnailGenerated)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GeneratedThumbnailWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GeneratedThumbnailWidth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetGeneratedThumbnailWidth(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGeneratedThumbnailWidth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GeneratedThumbnailHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GeneratedThumbnailHeight)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetGeneratedThumbnailHeight(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetGeneratedThumbnailHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn BitmapTransform(&self) -> ::windows_core::Result<BitmapTransform> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapTransform)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPixelData(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, width: u32, height: u32, dpix: f64, dpiy: f64, pixels: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetPixelData)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, width, height, dpix, dpiy, pixels.len() as u32, pixels.as_ptr()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GoToNextFrameAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GoToNextFrameAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GoToNextFrameWithEncodingOptionsAsync<P0>(&self, encodingoptions: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GoToNextFrameWithEncodingOptionsAsync)(::windows_core::Interface::as_raw(this), encodingoptions.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlushAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BmpEncoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BmpEncoderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn JpegEncoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).JpegEncoderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn PngEncoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PngEncoderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn TiffEncoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).TiffEncoderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn GifEncoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GifEncoderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn JpegXREncoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).JpegXREncoderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetEncoderInformationEnumerator() -> ::windows_core::Result<super::super::Foundation::Collections::IVectorView<BitmapCodecInformation>> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetEncoderInformationEnumerator)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateAsync<P0>(encoderid: ::windows_core::GUID, stream: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateAsync)(::windows_core::Interface::as_raw(this), encoderid, stream.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn CreateWithEncodingOptionsAsync<P0, P1>(encoderid: ::windows_core::GUID, stream: P0, encodingoptions: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
        P1: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>>>,
    {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithEncodingOptionsAsync)(::windows_core::Interface::as_raw(this), encoderid, stream.try_into_param()?.abi(), encodingoptions.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateForTranscodingAsync<P0, P1>(stream: P0, bitmapdecoder: P1) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
        P1: ::windows_core::IntoParam<BitmapDecoder>,
    {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateForTranscodingAsync)(::windows_core::Interface::as_raw(this), stream.try_into_param()?.abi(), bitmapdecoder.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateForInPlacePropertyEncodingAsync<P0>(bitmapdecoder: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>>
    where
        P0: ::windows_core::IntoParam<BitmapDecoder>,
    {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateForInPlacePropertyEncodingAsync)(::windows_core::Interface::as_raw(this), bitmapdecoder.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn HeifEncoderId() -> ::windows_core::Result<::windows_core::GUID> {
        Self::IBitmapEncoderStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HeifEncoderId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SetSoftwareBitmap<P0>(&self, bitmap: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SoftwareBitmap>,
    {
        let this = &::windows_core::ComInterface::cast::<IBitmapEncoderWithSoftwareBitmap>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSoftwareBitmap)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi()).ok() }
    }
    #[doc(hidden)]
    pub fn IBitmapEncoderStatics<R, F: FnOnce(&IBitmapEncoderStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BitmapEncoder, IBitmapEncoderStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBitmapEncoderStatics2<R, F: FnOnce(&IBitmapEncoderStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BitmapEncoder, IBitmapEncoderStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for BitmapEncoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapEncoder {}
impl ::core::fmt::Debug for BitmapEncoder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapEncoder").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BitmapEncoder {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapEncoder;{2bc468e3-e1f8-4b54-95e8-32919551ce62})");
}
impl ::core::clone::Clone for BitmapEncoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for BitmapEncoder {
    type Vtable = IBitmapEncoder_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BitmapEncoder {
    const IID: ::windows_core::GUID = <IBitmapEncoder as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BitmapEncoder {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapEncoder";
}
::windows_core::imp::interface_hierarchy!(BitmapEncoder, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BitmapEncoder {}
unsafe impl ::core::marker::Sync for BitmapEncoder {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct BitmapFrame(::windows_core::IUnknown);
impl BitmapFrame {
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<ImageStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetThumbnailAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BitmapProperties(&self) -> ::windows_core::Result<BitmapPropertiesView> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapProperties)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BitmapPixelFormat(&self) -> ::windows_core::Result<BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapPixelFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> ::windows_core::Result<BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapAlphaMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DpiX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DpiX)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DpiY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DpiY)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelWidth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelHeight)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OrientedPixelWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OrientedPixelWidth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn OrientedPixelHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).OrientedPixelHeight)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPixelDataAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataTransformedAsync<P0>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: P0, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>>
    where
        P0: ::windows_core::IntoParam<BitmapTransform>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPixelDataTransformedAsync)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSoftwareBitmapAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapConvertedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSoftwareBitmapConvertedAsync)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapTransformedAsync<P0>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: P0, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>
    where
        P0: ::windows_core::IntoParam<BitmapTransform>,
    {
        let this = &::windows_core::ComInterface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSoftwareBitmapTransformedAsync)(::windows_core::Interface::as_raw(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for BitmapFrame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapFrame {}
impl ::core::fmt::Debug for BitmapFrame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapFrame").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BitmapFrame {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapFrame;{72a49a1c-8081-438d-91bc-94ecfc8185c6})");
}
impl ::core::clone::Clone for BitmapFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for BitmapFrame {
    type Vtable = IBitmapFrame_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BitmapFrame {
    const IID: ::windows_core::GUID = <IBitmapFrame as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BitmapFrame {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapFrame";
}
::windows_core::imp::interface_hierarchy!(BitmapFrame, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBitmapFrame> for BitmapFrame {}
impl ::windows_core::CanTryInto<IBitmapFrameWithSoftwareBitmap> for BitmapFrame {}
unsafe impl ::core::marker::Send for BitmapFrame {}
unsafe impl ::core::marker::Sync for BitmapFrame {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct BitmapProperties(::windows_core::IUnknown);
impl BitmapProperties {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetPropertiesAsync<P0>(&self, propertiestoset: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SetPropertiesAsync)(::windows_core::Interface::as_raw(this), propertiestoset.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPropertiesAsync<P0>(&self, propertiestoretrieve: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<BitmapPropertySet>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = &::windows_core::ComInterface::cast::<IBitmapPropertiesView>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPropertiesAsync)(::windows_core::Interface::as_raw(this), propertiestoretrieve.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for BitmapProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapProperties {}
impl ::core::fmt::Debug for BitmapProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapProperties").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BitmapProperties {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapProperties;{ea9f4f1b-b505-4450-a4d1-e8ca94529d8d})");
}
impl ::core::clone::Clone for BitmapProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for BitmapProperties {
    type Vtable = IBitmapProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BitmapProperties {
    const IID: ::windows_core::GUID = <IBitmapProperties as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BitmapProperties {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapProperties";
}
::windows_core::imp::interface_hierarchy!(BitmapProperties, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBitmapPropertiesView> for BitmapProperties {}
unsafe impl ::core::marker::Send for BitmapProperties {}
unsafe impl ::core::marker::Sync for BitmapProperties {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct BitmapPropertiesView(::windows_core::IUnknown);
impl BitmapPropertiesView {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPropertiesAsync<P0>(&self, propertiestoretrieve: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<BitmapPropertySet>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetPropertiesAsync)(::windows_core::Interface::as_raw(this), propertiestoretrieve.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for BitmapPropertiesView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapPropertiesView {}
impl ::core::fmt::Debug for BitmapPropertiesView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapPropertiesView").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BitmapPropertiesView {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapPropertiesView;{7e0fe87a-3a70-48f8-9c55-196cf5a545f5})");
}
impl ::core::clone::Clone for BitmapPropertiesView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for BitmapPropertiesView {
    type Vtable = IBitmapPropertiesView_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BitmapPropertiesView {
    const IID: ::windows_core::GUID = <IBitmapPropertiesView as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BitmapPropertiesView {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapPropertiesView";
}
::windows_core::imp::interface_hierarchy!(BitmapPropertiesView, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IBitmapPropertiesView> for BitmapPropertiesView {}
unsafe impl ::core::marker::Send for BitmapPropertiesView {}
unsafe impl ::core::marker::Sync for BitmapPropertiesView {}
#[doc = "*Required features: `\"Graphics_Imaging\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct BitmapPropertySet(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl BitmapPropertySet {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BitmapPropertySet, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<BitmapTypedValue> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, BitmapTypedValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<P0>(&self, key: &::windows_core::HSTRING, value: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<BitmapTypedValue>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), value.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for BitmapPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for BitmapPropertySet {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for BitmapPropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapPropertySet").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for BitmapPropertySet {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapPropertySet;pinterface({3c2925fe-8519-45c1-aa79-197b6718c1c1};string;rc(Windows.Graphics.Imaging.BitmapTypedValue;{cd8044a9-2443-4000-b0cd-79316c56f589})))");
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for BitmapPropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for BitmapPropertySet {
    type Vtable = super::super::Foundation::Collections::IMap_Vtbl<::windows_core::HSTRING, BitmapTypedValue>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for BitmapPropertySet {
    const IID: ::windows_core::GUID = <super::super::Foundation::Collections::IMap<::windows_core::HSTRING, BitmapTypedValue> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for BitmapPropertySet {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapPropertySet";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for BitmapPropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &BitmapPropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(BitmapPropertySet, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, BitmapTypedValue>>> for BitmapPropertySet {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IMap<::windows_core::HSTRING, BitmapTypedValue>> for BitmapPropertySet {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for BitmapPropertySet {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for BitmapPropertySet {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct BitmapTransform(::windows_core::IUnknown);
impl BitmapTransform {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BitmapTransform, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ScaledWidth(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScaledWidth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetScaledWidth(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScaledWidth)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ScaledHeight(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ScaledHeight)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetScaledHeight(&self, value: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetScaledHeight)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InterpolationMode(&self) -> ::windows_core::Result<BitmapInterpolationMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).InterpolationMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInterpolationMode(&self, value: BitmapInterpolationMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetInterpolationMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Flip(&self) -> ::windows_core::Result<BitmapFlip> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Flip)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetFlip(&self, value: BitmapFlip) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetFlip)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Rotation(&self) -> ::windows_core::Result<BitmapRotation> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Rotation)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRotation(&self, value: BitmapRotation) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRotation)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Bounds(&self) -> ::windows_core::Result<BitmapBounds> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Bounds)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBounds(&self, value: BitmapBounds) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetBounds)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for BitmapTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapTransform {}
impl ::core::fmt::Debug for BitmapTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapTransform").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BitmapTransform {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapTransform;{ae755344-e268-4d35-adcf-e995d31a8d34})");
}
impl ::core::clone::Clone for BitmapTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for BitmapTransform {
    type Vtable = IBitmapTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BitmapTransform {
    const IID: ::windows_core::GUID = <IBitmapTransform as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BitmapTransform {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapTransform";
}
::windows_core::imp::interface_hierarchy!(BitmapTransform, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BitmapTransform {}
unsafe impl ::core::marker::Sync for BitmapTransform {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct BitmapTypedValue(::windows_core::IUnknown);
impl BitmapTypedValue {
    pub fn Value(&self) -> ::windows_core::Result<::windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Value)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Type(&self) -> ::windows_core::Result<super::super::Foundation::PropertyType> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Type)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Create<P0>(value: P0, r#type: super::super::Foundation::PropertyType) -> ::windows_core::Result<BitmapTypedValue>
    where
        P0: ::windows_core::IntoParam<::windows_core::IInspectable>,
    {
        Self::IBitmapTypedValueFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), value.into_param().abi(), r#type, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBitmapTypedValueFactory<R, F: FnOnce(&IBitmapTypedValueFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<BitmapTypedValue, IBitmapTypedValueFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for BitmapTypedValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BitmapTypedValue {}
impl ::core::fmt::Debug for BitmapTypedValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapTypedValue").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BitmapTypedValue {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapTypedValue;{cd8044a9-2443-4000-b0cd-79316c56f589})");
}
impl ::core::clone::Clone for BitmapTypedValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for BitmapTypedValue {
    type Vtable = IBitmapTypedValue_Vtbl;
}
unsafe impl ::windows_core::ComInterface for BitmapTypedValue {
    const IID: ::windows_core::GUID = <IBitmapTypedValue as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for BitmapTypedValue {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapTypedValue";
}
::windows_core::imp::interface_hierarchy!(BitmapTypedValue, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for BitmapTypedValue {}
unsafe impl ::core::marker::Sync for BitmapTypedValue {}
#[doc = "*Required features: `\"Graphics_Imaging\"`, `\"Storage_Streams\"`*"]
#[cfg(feature = "Storage_Streams")]
#[repr(transparent)]
pub struct ImageStream(::windows_core::IUnknown);
#[cfg(feature = "Storage_Streams")]
impl ImageStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentType(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContentType)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: super::super::Storage::Streams::InputStreamOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ReadAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), count, options, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteAsync<P0>(&self, buffer: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).WriteAsync)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn FlushAsync(&self) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FlushAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Size(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSize(&self, value: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetSize)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows_core::Result<super::super::Storage::Streams::IInputStream> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetInputStreamAt)(::windows_core::Interface::as_raw(this), position, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows_core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetOutputStreamAt)(::windows_core::Interface::as_raw(this), position, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Position(&self) -> ::windows_core::Result<u64> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Position)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Seek(&self, position: u64) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Seek)(::windows_core::Interface::as_raw(this), position).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CloneStream(&self) -> ::windows_core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CloneStream)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CanRead(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanRead)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CanWrite(&self) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CanWrite)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::cmp::PartialEq for ImageStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::cmp::Eq for ImageStream {}
#[cfg(feature = "Storage_Streams")]
impl ::core::fmt::Debug for ImageStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ImageStream").field(&self.0).finish()
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::RuntimeType for ImageStream {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.ImageStream;{cc254827-4b3d-438f-9232-10c76bc7e038})");
}
#[cfg(feature = "Storage_Streams")]
impl ::core::clone::Clone for ImageStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows_core::Interface for ImageStream {
    type Vtable = super::super::Storage::Streams::IRandomAccessStreamWithContentType_Vtbl;
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows_core::ComInterface for ImageStream {
    const IID: ::windows_core::GUID = <super::super::Storage::Streams::IRandomAccessStreamWithContentType as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::RuntimeName for ImageStream {
    const NAME: &'static str = "Windows.Graphics.Imaging.ImageStream";
}
#[cfg(feature = "Storage_Streams")]
::windows_core::imp::interface_hierarchy!(ImageStream, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for ImageStream {}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::CanTryInto<super::super::Storage::Streams::IContentTypeProvider> for ImageStream {}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::CanTryInto<super::super::Storage::Streams::IInputStream> for ImageStream {}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::CanTryInto<super::super::Storage::Streams::IOutputStream> for ImageStream {}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::CanTryInto<super::super::Storage::Streams::IRandomAccessStream> for ImageStream {}
#[cfg(feature = "Storage_Streams")]
impl ::windows_core::CanTryInto<super::super::Storage::Streams::IRandomAccessStreamWithContentType> for ImageStream {}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::core::marker::Send for ImageStream {}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::core::marker::Sync for ImageStream {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct PixelDataProvider(::windows_core::IUnknown);
impl PixelDataProvider {
    pub fn DetachPixelData(&self) -> ::windows_core::Result<::windows_core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows_core::Interface::vtable(this).DetachPixelData)(::windows_core::Interface::as_raw(this), ::windows_core::Array::<u8>::set_abi_len(::std::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
impl ::core::cmp::PartialEq for PixelDataProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PixelDataProvider {}
impl ::core::fmt::Debug for PixelDataProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PixelDataProvider").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PixelDataProvider {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.PixelDataProvider;{dd831f25-185c-4595-9fb9-ccbe6ec18a6f})");
}
impl ::core::clone::Clone for PixelDataProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for PixelDataProvider {
    type Vtable = IPixelDataProvider_Vtbl;
}
unsafe impl ::windows_core::ComInterface for PixelDataProvider {
    const IID: ::windows_core::GUID = <IPixelDataProvider as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for PixelDataProvider {
    const NAME: &'static str = "Windows.Graphics.Imaging.PixelDataProvider";
}
::windows_core::imp::interface_hierarchy!(PixelDataProvider, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for PixelDataProvider {}
unsafe impl ::core::marker::Sync for PixelDataProvider {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct SoftwareBitmap(::windows_core::IUnknown);
impl SoftwareBitmap {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn BitmapPixelFormat(&self) -> ::windows_core::Result<BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapPixelFormat)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> ::windows_core::Result<BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapAlphaMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelWidth)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PixelHeight)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsReadOnly(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsReadOnly)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDpiX(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDpiX)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DpiX(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DpiX)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDpiY(&self, value: f64) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDpiY)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn DpiY(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DpiY)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn LockBuffer(&self, mode: BitmapBufferAccessMode) -> ::windows_core::Result<BitmapBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).LockBuffer)(::windows_core::Interface::as_raw(this), mode, &mut result__).from_abi(result__)
        }
    }
    pub fn CopyTo<P0>(&self, bitmap: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<SoftwareBitmap>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CopyTo)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CopyFromBuffer<P0>(&self, buffer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CopyFromBuffer)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CopyToBuffer<P0>(&self, buffer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).CopyToBuffer)(::windows_core::Interface::as_raw(this), buffer.try_into_param()?.abi()).ok() }
    }
    pub fn GetReadOnlyView(&self) -> ::windows_core::Result<SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetReadOnlyView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(format: BitmapPixelFormat, width: i32, height: i32) -> ::windows_core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Create)(::windows_core::Interface::as_raw(this), format, width, height, &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithAlpha(format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> ::windows_core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateWithAlpha)(::windows_core::Interface::as_raw(this), format, width, height, alpha, &mut result__).from_abi(result__)
        })
    }
    pub fn Copy<P0>(source: P0) -> ::windows_core::Result<SoftwareBitmap>
    where
        P0: ::windows_core::IntoParam<SoftwareBitmap>,
    {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Copy)(::windows_core::Interface::as_raw(this), source.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn Convert<P0>(source: P0, format: BitmapPixelFormat) -> ::windows_core::Result<SoftwareBitmap>
    where
        P0: ::windows_core::IntoParam<SoftwareBitmap>,
    {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Convert)(::windows_core::Interface::as_raw(this), source.into_param().abi(), format, &mut result__).from_abi(result__)
        })
    }
    pub fn ConvertWithAlpha<P0>(source: P0, format: BitmapPixelFormat, alpha: BitmapAlphaMode) -> ::windows_core::Result<SoftwareBitmap>
    where
        P0: ::windows_core::IntoParam<SoftwareBitmap>,
    {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ConvertWithAlpha)(::windows_core::Interface::as_raw(this), source.into_param().abi(), format, alpha, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCopyFromBuffer<P0>(source: P0, format: BitmapPixelFormat, width: i32, height: i32) -> ::windows_core::Result<SoftwareBitmap>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCopyFromBuffer)(::windows_core::Interface::as_raw(this), source.try_into_param()?.abi(), format, width, height, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCopyWithAlphaFromBuffer<P0>(source: P0, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> ::windows_core::Result<SoftwareBitmap>
    where
        P0: ::windows_core::TryIntoParam<super::super::Storage::Streams::IBuffer>,
    {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCopyWithAlphaFromBuffer)(::windows_core::Interface::as_raw(this), source.try_into_param()?.abi(), format, width, height, alpha, &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn CreateCopyFromSurfaceAsync<P0>(surface: P0) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>
    where
        P0: ::windows_core::TryIntoParam<super::DirectX::Direct3D11::IDirect3DSurface>,
    {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCopyFromSurfaceAsync)(::windows_core::Interface::as_raw(this), surface.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn CreateCopyWithAlphaFromSurfaceAsync<P0>(surface: P0, alpha: BitmapAlphaMode) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>
    where
        P0: ::windows_core::TryIntoParam<super::DirectX::Direct3D11::IDirect3DSurface>,
    {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateCopyWithAlphaFromSurfaceAsync)(::windows_core::Interface::as_raw(this), surface.try_into_param()?.abi(), alpha, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISoftwareBitmapFactory<R, F: FnOnce(&ISoftwareBitmapFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SoftwareBitmap, ISoftwareBitmapFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISoftwareBitmapStatics<R, F: FnOnce(&ISoftwareBitmapStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<SoftwareBitmap, ISoftwareBitmapStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for SoftwareBitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SoftwareBitmap {}
impl ::core::fmt::Debug for SoftwareBitmap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SoftwareBitmap").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for SoftwareBitmap {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.SoftwareBitmap;{689e0708-7eef-483f-963f-da938818e073})");
}
impl ::core::clone::Clone for SoftwareBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for SoftwareBitmap {
    type Vtable = ISoftwareBitmap_Vtbl;
}
unsafe impl ::windows_core::ComInterface for SoftwareBitmap {
    const IID: ::windows_core::GUID = <ISoftwareBitmap as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for SoftwareBitmap {
    const NAME: &'static str = "Windows.Graphics.Imaging.SoftwareBitmap";
}
::windows_core::imp::interface_hierarchy!(SoftwareBitmap, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::Foundation::IClosable> for SoftwareBitmap {}
unsafe impl ::core::marker::Send for SoftwareBitmap {}
unsafe impl ::core::marker::Sync for SoftwareBitmap {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BitmapAlphaMode(pub i32);
impl BitmapAlphaMode {
    pub const Premultiplied: Self = Self(0i32);
    pub const Straight: Self = Self(1i32);
    pub const Ignore: Self = Self(2i32);
}
impl ::core::marker::Copy for BitmapAlphaMode {}
impl ::core::clone::Clone for BitmapAlphaMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BitmapAlphaMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for BitmapAlphaMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for BitmapAlphaMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapAlphaMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BitmapAlphaMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapAlphaMode;i4)");
}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BitmapBufferAccessMode(pub i32);
impl BitmapBufferAccessMode {
    pub const Read: Self = Self(0i32);
    pub const ReadWrite: Self = Self(1i32);
    pub const Write: Self = Self(2i32);
}
impl ::core::marker::Copy for BitmapBufferAccessMode {}
impl ::core::clone::Clone for BitmapBufferAccessMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BitmapBufferAccessMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for BitmapBufferAccessMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for BitmapBufferAccessMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapBufferAccessMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BitmapBufferAccessMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapBufferAccessMode;i4)");
}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BitmapFlip(pub i32);
impl BitmapFlip {
    pub const None: Self = Self(0i32);
    pub const Horizontal: Self = Self(1i32);
    pub const Vertical: Self = Self(2i32);
}
impl ::core::marker::Copy for BitmapFlip {}
impl ::core::clone::Clone for BitmapFlip {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BitmapFlip {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for BitmapFlip {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for BitmapFlip {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapFlip").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BitmapFlip {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapFlip;i4)");
}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BitmapInterpolationMode(pub i32);
impl BitmapInterpolationMode {
    pub const NearestNeighbor: Self = Self(0i32);
    pub const Linear: Self = Self(1i32);
    pub const Cubic: Self = Self(2i32);
    pub const Fant: Self = Self(3i32);
}
impl ::core::marker::Copy for BitmapInterpolationMode {}
impl ::core::clone::Clone for BitmapInterpolationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BitmapInterpolationMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for BitmapInterpolationMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for BitmapInterpolationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapInterpolationMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BitmapInterpolationMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapInterpolationMode;i4)");
}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BitmapPixelFormat(pub i32);
impl BitmapPixelFormat {
    pub const Unknown: Self = Self(0i32);
    pub const Rgba16: Self = Self(12i32);
    pub const Rgba8: Self = Self(30i32);
    pub const Gray16: Self = Self(57i32);
    pub const Gray8: Self = Self(62i32);
    pub const Bgra8: Self = Self(87i32);
    pub const Nv12: Self = Self(103i32);
    pub const P010: Self = Self(104i32);
    pub const Yuy2: Self = Self(107i32);
}
impl ::core::marker::Copy for BitmapPixelFormat {}
impl ::core::clone::Clone for BitmapPixelFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BitmapPixelFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for BitmapPixelFormat {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for BitmapPixelFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapPixelFormat").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BitmapPixelFormat {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapPixelFormat;i4)");
}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BitmapRotation(pub i32);
impl BitmapRotation {
    pub const None: Self = Self(0i32);
    pub const Clockwise90Degrees: Self = Self(1i32);
    pub const Clockwise180Degrees: Self = Self(2i32);
    pub const Clockwise270Degrees: Self = Self(3i32);
}
impl ::core::marker::Copy for BitmapRotation {}
impl ::core::clone::Clone for BitmapRotation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BitmapRotation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for BitmapRotation {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for BitmapRotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapRotation").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for BitmapRotation {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapRotation;i4)");
}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ColorManagementMode(pub i32);
impl ColorManagementMode {
    pub const DoNotColorManage: Self = Self(0i32);
    pub const ColorManageToSRgb: Self = Self(1i32);
}
impl ::core::marker::Copy for ColorManagementMode {}
impl ::core::clone::Clone for ColorManagementMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ColorManagementMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ColorManagementMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ColorManagementMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorManagementMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ColorManagementMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.ColorManagementMode;i4)");
}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ExifOrientationMode(pub i32);
impl ExifOrientationMode {
    pub const IgnoreExifOrientation: Self = Self(0i32);
    pub const RespectExifOrientation: Self = Self(1i32);
}
impl ::core::marker::Copy for ExifOrientationMode {}
impl ::core::clone::Clone for ExifOrientationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExifOrientationMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ExifOrientationMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ExifOrientationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExifOrientationMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ExifOrientationMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.ExifOrientationMode;i4)");
}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct JpegSubsamplingMode(pub i32);
impl JpegSubsamplingMode {
    pub const Default: Self = Self(0i32);
    pub const Y4Cb2Cr0: Self = Self(1i32);
    pub const Y4Cb2Cr2: Self = Self(2i32);
    pub const Y4Cb4Cr4: Self = Self(3i32);
}
impl ::core::marker::Copy for JpegSubsamplingMode {}
impl ::core::clone::Clone for JpegSubsamplingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JpegSubsamplingMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for JpegSubsamplingMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for JpegSubsamplingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JpegSubsamplingMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for JpegSubsamplingMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.JpegSubsamplingMode;i4)");
}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PngFilterMode(pub i32);
impl PngFilterMode {
    pub const Automatic: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Sub: Self = Self(2i32);
    pub const Up: Self = Self(3i32);
    pub const Average: Self = Self(4i32);
    pub const Paeth: Self = Self(5i32);
    pub const Adaptive: Self = Self(6i32);
}
impl ::core::marker::Copy for PngFilterMode {}
impl ::core::clone::Clone for PngFilterMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PngFilterMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PngFilterMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PngFilterMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PngFilterMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for PngFilterMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.PngFilterMode;i4)");
}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TiffCompressionMode(pub i32);
impl TiffCompressionMode {
    pub const Automatic: Self = Self(0i32);
    pub const None: Self = Self(1i32);
    pub const Ccitt3: Self = Self(2i32);
    pub const Ccitt4: Self = Self(3i32);
    pub const Lzw: Self = Self(4i32);
    pub const Rle: Self = Self(5i32);
    pub const Zip: Self = Self(6i32);
    pub const LzwhDifferencing: Self = Self(7i32);
}
impl ::core::marker::Copy for TiffCompressionMode {}
impl ::core::clone::Clone for TiffCompressionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TiffCompressionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for TiffCompressionMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for TiffCompressionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TiffCompressionMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for TiffCompressionMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.TiffCompressionMode;i4)");
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
pub struct BitmapBounds {
    pub X: u32,
    pub Y: u32,
    pub Width: u32,
    pub Height: u32,
}
impl ::core::marker::Copy for BitmapBounds {}
impl ::core::clone::Clone for BitmapBounds {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BitmapBounds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BitmapBounds").field("X", &self.X).field("Y", &self.Y).field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::windows_core::TypeKind for BitmapBounds {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for BitmapBounds {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.Imaging.BitmapBounds;u4;u4;u4;u4)");
}
impl ::core::cmp::PartialEq for BitmapBounds {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for BitmapBounds {}
impl ::core::default::Default for BitmapBounds {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
pub struct BitmapPlaneDescription {
    pub StartIndex: i32,
    pub Width: i32,
    pub Height: i32,
    pub Stride: i32,
}
impl ::core::marker::Copy for BitmapPlaneDescription {}
impl ::core::clone::Clone for BitmapPlaneDescription {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BitmapPlaneDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BitmapPlaneDescription").field("StartIndex", &self.StartIndex).field("Width", &self.Width).field("Height", &self.Height).field("Stride", &self.Stride).finish()
    }
}
impl ::windows_core::TypeKind for BitmapPlaneDescription {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for BitmapPlaneDescription {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.Imaging.BitmapPlaneDescription;i4;i4;i4;i4)");
}
impl ::core::cmp::PartialEq for BitmapPlaneDescription {
    fn eq(&self, other: &Self) -> bool {
        self.StartIndex == other.StartIndex && self.Width == other.Width && self.Height == other.Height && self.Stride == other.Stride
    }
}
impl ::core::cmp::Eq for BitmapPlaneDescription {}
impl ::core::default::Default for BitmapPlaneDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
pub struct BitmapSize {
    pub Width: u32,
    pub Height: u32,
}
impl ::core::marker::Copy for BitmapSize {}
impl ::core::clone::Clone for BitmapSize {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BitmapSize {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BitmapSize").field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::windows_core::TypeKind for BitmapSize {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for BitmapSize {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.Imaging.BitmapSize;u4;u4)");
}
impl ::core::cmp::PartialEq for BitmapSize {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for BitmapSize {}
impl ::core::default::Default for BitmapSize {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
