#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapBuffer(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBitmapBuffer {
    type Vtable = IBitmapBuffer_Vtbl;
}
unsafe impl ::windows::core::Interface for IBitmapBuffer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa53e04c4_399c_438c_b28f_a63a6b83d1a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapBuffer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetPlaneCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub GetPlaneDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, result__: *mut BitmapPlaneDescription) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapCodecInformation(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBitmapCodecInformation {
    type Vtable = IBitmapCodecInformation_Vtbl;
}
unsafe impl ::windows::core::Interface for IBitmapCodecInformation {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x400caaf2_c4b0_4392_a3b0_6f6f9ba95cb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapCodecInformation_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CodecId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FileExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FileExtensions: usize,
    pub FriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MimeTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MimeTypes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapDecoder(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBitmapDecoder {
    type Vtable = IBitmapDecoder_Vtbl;
}
unsafe impl ::windows::core::Interface for IBitmapDecoder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xacef22ba_1d74_4c91_9dfc_9620745233e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapDecoder_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BitmapContainerProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DecoderInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FrameCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetPreviewAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetPreviewAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frameindex: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFrameAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapDecoderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBitmapDecoderStatics {
    type Vtable = IBitmapDecoderStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IBitmapDecoderStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x438ccb26_bcef_4e95_bad6_23a822e58d01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapDecoderStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BmpDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub JpegDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PngDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub TiffDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GifDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub JpegXRDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub IcoDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetDecoderInformationEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetDecoderInformationEnumerator: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateWithIdAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, decoderid: ::windows::core::GUID, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateWithIdAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapDecoderStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBitmapDecoderStatics2 {
    type Vtable = IBitmapDecoderStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IBitmapDecoderStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ba68ea_99a1_40c4_80d9_aef0dafa6c3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapDecoderStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub HeifDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub WebpDecoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapEncoder(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBitmapEncoder {
    type Vtable = IBitmapEncoder_Vtbl;
}
unsafe impl ::windows::core::Interface for IBitmapEncoder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bc468e3_e1f8_4b54_95e8_32919551ce62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapEncoder_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub EncoderInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BitmapProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BitmapContainerProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsThumbnailGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsThumbnailGenerated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub GeneratedThumbnailWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetGeneratedThumbnailWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub GeneratedThumbnailHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetGeneratedThumbnailHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub BitmapTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPixelData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, width: u32, height: u32, dpix: f64, dpiy: f64, pixels_array_size: u32, pixels: *const u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GoToNextFrameAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GoToNextFrameAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GoToNextFrameWithEncodingOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encodingoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GoToNextFrameWithEncodingOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FlushAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FlushAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapEncoderStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBitmapEncoderStatics {
    type Vtable = IBitmapEncoderStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IBitmapEncoderStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa74356a7_a4e4_4eb9_8e40_564de7e1ccb2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapEncoderStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BmpEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub JpegEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub PngEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub TiffEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GifEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub JpegXREncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetEncoderInformationEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetEncoderInformationEnumerator: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encoderid: ::windows::core::GUID, stream: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateAsync: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub CreateWithEncodingOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, encoderid: ::windows::core::GUID, stream: *mut ::core::ffi::c_void, encodingoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_Streams")))]
    CreateWithEncodingOptionsAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub CreateForTranscodingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, bitmapdecoder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    CreateForTranscodingAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateForInPlacePropertyEncodingAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmapdecoder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateForInPlacePropertyEncodingAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapEncoderStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBitmapEncoderStatics2 {
    type Vtable = IBitmapEncoderStatics2_Vtbl;
}
unsafe impl ::windows::core::Interface for IBitmapEncoderStatics2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33cbc259_fe31_41b1_b812_086d21e87e16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapEncoderStatics2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub HeifEncoderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapEncoderWithSoftwareBitmap(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBitmapEncoderWithSoftwareBitmap {
    type Vtable = IBitmapEncoderWithSoftwareBitmap_Vtbl;
}
unsafe impl ::windows::core::Interface for IBitmapEncoderWithSoftwareBitmap {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x686cd241_4330_4c77_ace4_0334968b1768);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapEncoderWithSoftwareBitmap_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetSoftwareBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IBitmapFrame(::windows::core::IUnknown);
impl IBitmapFrame {
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetThumbnailAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BitmapProperties(&self) -> ::windows::core::Result<BitmapPropertiesView> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitmapProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BitmapPixelFormat(&self) -> ::windows::core::Result<BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitmapPixelFormat)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> ::windows::core::Result<BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitmapAlphaMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DpiX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DpiX)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DpiY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DpiY)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PixelWidth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PixelHeight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OrientedPixelWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OrientedPixelWidth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OrientedPixelHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OrientedPixelHeight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPixelDataAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataTransformedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: &BitmapTransform, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPixelDataTransformedAsync)(::windows::core::Vtable::as_raw(this), pixelformat, alphamode, ::core::mem::transmute_copy(transform), exiforientationmode, colormanagementmode, result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IBitmapFrame, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IBitmapFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IBitmapFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{72a49a1c-8081-438d-91bc-94ecfc8185c6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IBitmapFrame {
    type Vtable = IBitmapFrame_Vtbl;
}
unsafe impl ::windows::core::Interface for IBitmapFrame {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72a49a1c_8081_438d_91bc_94ecfc8185c6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapFrame_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetThumbnailAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetThumbnailAsync: usize,
    pub BitmapProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub BitmapPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapPixelFormat) -> ::windows::core::HRESULT,
    pub BitmapAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapAlphaMode) -> ::windows::core::HRESULT,
    pub DpiX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub DpiY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub PixelWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub PixelHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub OrientedPixelWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub OrientedPixelHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetPixelDataAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPixelDataAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetPixelDataTransformedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: *mut ::core::ffi::c_void, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPixelDataTransformedAsync: usize,
}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IBitmapFrameWithSoftwareBitmap(::windows::core::IUnknown);
impl IBitmapFrameWithSoftwareBitmap {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSoftwareBitmapAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapConvertedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSoftwareBitmapConvertedAsync)(::windows::core::Vtable::as_raw(this), pixelformat, alphamode, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapTransformedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: &BitmapTransform, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSoftwareBitmapTransformedAsync)(::windows::core::Vtable::as_raw(this), pixelformat, alphamode, ::core::mem::transmute_copy(transform), exiforientationmode, colormanagementmode, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageStream>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetThumbnailAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BitmapProperties(&self) -> ::windows::core::Result<BitmapPropertiesView> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitmapProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BitmapPixelFormat(&self) -> ::windows::core::Result<BitmapPixelFormat> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitmapPixelFormat)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> ::windows::core::Result<BitmapAlphaMode> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitmapAlphaMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DpiX(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DpiX)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DpiY(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DpiY)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PixelWidth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PixelHeight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OrientedPixelWidth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OrientedPixelWidth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OrientedPixelHeight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OrientedPixelHeight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPixelDataAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataTransformedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: &BitmapTransform, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPixelDataTransformedAsync)(::windows::core::Vtable::as_raw(this), pixelformat, alphamode, ::core::mem::transmute_copy(transform), exiforientationmode, colormanagementmode, result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IBitmapFrameWithSoftwareBitmap, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<IBitmapFrameWithSoftwareBitmap> for IBitmapFrame {
    type Error = ::windows::core::Error;
    fn try_from(value: IBitmapFrameWithSoftwareBitmap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IBitmapFrameWithSoftwareBitmap> for IBitmapFrame {
    type Error = ::windows::core::Error;
    fn try_from(value: &IBitmapFrameWithSoftwareBitmap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&IBitmapFrameWithSoftwareBitmap> for ::windows::core::InParam<IBitmapFrame> {
    type Error = ::windows::core::Error;
    fn try_from(value: &IBitmapFrameWithSoftwareBitmap) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::clone::Clone for IBitmapFrameWithSoftwareBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IBitmapFrameWithSoftwareBitmap {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fe287c9a-420c-4963-87ad-691436e08383}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IBitmapFrameWithSoftwareBitmap {
    type Vtable = IBitmapFrameWithSoftwareBitmap_Vtbl;
}
unsafe impl ::windows::core::Interface for IBitmapFrameWithSoftwareBitmap {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe287c9a_420c_4963_87ad_691436e08383);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapFrameWithSoftwareBitmap_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub GetSoftwareBitmapAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSoftwareBitmapAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetSoftwareBitmapConvertedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSoftwareBitmapConvertedAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetSoftwareBitmapTransformedAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: *mut ::core::ffi::c_void, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetSoftwareBitmapTransformedAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBitmapProperties {
    type Vtable = IBitmapProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for IBitmapProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea9f4f1b_b505_4450_a4d1_e8ca94529d8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapProperties_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SetPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiestoset: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetPropertiesAsync: usize,
}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IBitmapPropertiesView(::windows::core::IUnknown);
impl IBitmapPropertiesView {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPropertiesAsync<P0, E0>(&self, propertiestoretrieve: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapPropertySet>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPropertiesAsync)(::windows::core::Vtable::as_raw(this), propertiestoretrieve.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
::windows::core::interface_hierarchy!(IBitmapPropertiesView, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IBitmapPropertiesView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IBitmapPropertiesView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{7e0fe87a-3a70-48f8-9c55-196cf5a545f5}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IBitmapPropertiesView {
    type Vtable = IBitmapPropertiesView_Vtbl;
}
unsafe impl ::windows::core::Interface for IBitmapPropertiesView {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e0fe87a_3a70_48f8_9c55_196cf5a545f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapPropertiesView_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertiestoretrieve: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPropertiesAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapTransform(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBitmapTransform {
    type Vtable = IBitmapTransform_Vtbl;
}
unsafe impl ::windows::core::Interface for IBitmapTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae755344_e268_4d35_adcf_e995d31a8d34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapTransform_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub ScaledWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetScaledWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub ScaledHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetScaledHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub InterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapInterpolationMode) -> ::windows::core::HRESULT,
    pub SetInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BitmapInterpolationMode) -> ::windows::core::HRESULT,
    pub Flip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapFlip) -> ::windows::core::HRESULT,
    pub SetFlip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BitmapFlip) -> ::windows::core::HRESULT,
    pub Rotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapRotation) -> ::windows::core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BitmapRotation) -> ::windows::core::HRESULT,
    pub Bounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapBounds) -> ::windows::core::HRESULT,
    pub SetBounds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BitmapBounds) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapTypedValue(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBitmapTypedValue {
    type Vtable = IBitmapTypedValue_Vtbl;
}
unsafe impl ::windows::core::Interface for IBitmapTypedValue {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd8044a9_2443_4000_b0cd_79316c56f589);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapTypedValue_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::PropertyType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Type: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBitmapTypedValueFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IBitmapTypedValueFactory {
    type Vtable = IBitmapTypedValueFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IBitmapTypedValueFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92dbb599_ce13_46bb_9545_cb3a3f63eb8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapTypedValueFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, r#type: super::super::Foundation::PropertyType, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPixelDataProvider(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPixelDataProvider {
    type Vtable = IPixelDataProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for IPixelDataProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd831f25_185c_4595_9fb9_ccbe6ec18a6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPixelDataProvider_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DetachPixelData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISoftwareBitmap(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISoftwareBitmap {
    type Vtable = ISoftwareBitmap_Vtbl;
}
unsafe impl ::windows::core::Interface for ISoftwareBitmap {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x689e0708_7eef_483f_963f_da938818e073);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmap_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BitmapPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapPixelFormat) -> ::windows::core::HRESULT,
    pub BitmapAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BitmapAlphaMode) -> ::windows::core::HRESULT,
    pub PixelWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub PixelHeight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetDpiX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub DpiX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetDpiY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub DpiY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub LockBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: BitmapBufferAccessMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CopyTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CopyFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CopyFromBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CopyToBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CopyToBuffer: usize,
    pub GetReadOnlyView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISoftwareBitmapFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISoftwareBitmapFactory {
    type Vtable = ISoftwareBitmapFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for ISoftwareBitmapFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc99feb69_2d62_4d47_a6b3_4fdb6a07fdf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: BitmapPixelFormat, width: i32, height: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithAlpha: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISoftwareBitmapStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISoftwareBitmapStatics {
    type Vtable = ISoftwareBitmapStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ISoftwareBitmapStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf0385db_672f_4a9d_806e_c2442f343e86);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Copy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Convert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, format: BitmapPixelFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ConvertWithAlpha: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, format: BitmapPixelFormat, alpha: BitmapAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCopyFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, format: BitmapPixelFormat, width: i32, height: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCopyFromBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub CreateCopyWithAlphaFromBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    CreateCopyWithAlphaFromBuffer: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub CreateCopyFromSurfaceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, surface: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))]
    CreateCopyFromSurfaceAsync: usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub CreateCopyWithAlphaFromSurfaceAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, surface: *mut ::core::ffi::c_void, alpha: BitmapAlphaMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))]
    CreateCopyWithAlphaFromSurfaceAsync: usize,
}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct BitmapBuffer(::windows::core::IUnknown);
impl BitmapBuffer {
    pub fn GetPlaneCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPlaneCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn GetPlaneDescription(&self, index: i32) -> ::windows::core::Result<BitmapPlaneDescription> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPlaneDescription)(::windows::core::Vtable::as_raw(this), index, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateReference)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for BitmapBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for BitmapBuffer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapBuffer;{a53e04c4-399c-438c-b28f-a63a6b83d1a1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BitmapBuffer {
    type Vtable = IBitmapBuffer_Vtbl;
}
unsafe impl ::windows::core::Interface for BitmapBuffer {
    const IID: ::windows::core::GUID = <IBitmapBuffer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BitmapBuffer {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapBuffer";
}
::windows::core::interface_hierarchy!(BitmapBuffer, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<BitmapBuffer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: BitmapBuffer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&BitmapBuffer> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &BitmapBuffer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&BitmapBuffer> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BitmapBuffer) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<BitmapBuffer> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: BitmapBuffer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&BitmapBuffer> for super::super::Foundation::IMemoryBuffer {
    type Error = ::windows::core::Error;
    fn try_from(value: &BitmapBuffer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&BitmapBuffer> for ::windows::core::InParam<super::super::Foundation::IMemoryBuffer> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BitmapBuffer) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for BitmapBuffer {}
unsafe impl ::core::marker::Sync for BitmapBuffer {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct BitmapCodecInformation(::windows::core::IUnknown);
impl BitmapCodecInformation {
    pub fn CodecId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CodecId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FileExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FileExtensions)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FriendlyName)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MimeTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MimeTypes)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for BitmapCodecInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for BitmapCodecInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapCodecInformation;{400caaf2-c4b0-4392-a3b0-6f6f9ba95cb4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BitmapCodecInformation {
    type Vtable = IBitmapCodecInformation_Vtbl;
}
unsafe impl ::windows::core::Interface for BitmapCodecInformation {
    const IID: ::windows::core::GUID = <IBitmapCodecInformation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BitmapCodecInformation {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapCodecInformation";
}
::windows::core::interface_hierarchy!(BitmapCodecInformation, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BitmapCodecInformation {}
unsafe impl ::core::marker::Sync for BitmapCodecInformation {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct BitmapDecoder(::windows::core::IUnknown);
impl BitmapDecoder {
    pub fn BitmapContainerProperties(&self) -> ::windows::core::Result<BitmapPropertiesView> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitmapContainerProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DecoderInformation(&self) -> ::windows::core::Result<BitmapCodecInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DecoderInformation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn FrameCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FrameCount)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetPreviewAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPreviewAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFrameAsync(&self, frameindex: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapFrame>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetFrameAsync)(::windows::core::Vtable::as_raw(this), frameindex, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BmpDecoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BmpDecoderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn JpegDecoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).JpegDecoderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn PngDecoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PngDecoderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TiffDecoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TiffDecoderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GifDecoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GifDecoderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn JpegXRDecoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).JpegXRDecoderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn IcoDecoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IcoDecoderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDecoderInformationEnumerator() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<BitmapCodecInformation>> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetDecoderInformationEnumerator)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateAsync<P0, E0>(stream: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapDecoder>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateAsync)(::windows::core::Vtable::as_raw(this), stream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateWithIdAsync<P0, E0>(decoderid: ::windows::core::GUID, stream: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapDecoder>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithIdAsync)(::windows::core::Vtable::as_raw(this), decoderid, stream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn HeifDecoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapDecoderStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HeifDecoderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn WebpDecoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapDecoderStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WebpDecoderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageStream>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetThumbnailAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BitmapProperties(&self) -> ::windows::core::Result<BitmapPropertiesView> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitmapProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BitmapPixelFormat(&self) -> ::windows::core::Result<BitmapPixelFormat> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitmapPixelFormat)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> ::windows::core::Result<BitmapAlphaMode> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitmapAlphaMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DpiX(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DpiX)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DpiY(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DpiY)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PixelWidth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PixelHeight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OrientedPixelWidth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OrientedPixelWidth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OrientedPixelHeight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OrientedPixelHeight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPixelDataAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataTransformedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: &BitmapTransform, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPixelDataTransformedAsync)(::windows::core::Vtable::as_raw(this), pixelformat, alphamode, ::core::mem::transmute_copy(transform), exiforientationmode, colormanagementmode, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSoftwareBitmapAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapConvertedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSoftwareBitmapConvertedAsync)(::windows::core::Vtable::as_raw(this), pixelformat, alphamode, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapTransformedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: &BitmapTransform, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSoftwareBitmapTransformedAsync)(::windows::core::Vtable::as_raw(this), pixelformat, alphamode, ::core::mem::transmute_copy(transform), exiforientationmode, colormanagementmode, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn IBitmapDecoderStatics<R, F: FnOnce(&IBitmapDecoderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BitmapDecoder, IBitmapDecoderStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBitmapDecoderStatics2<R, F: FnOnce(&IBitmapDecoderStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BitmapDecoder, IBitmapDecoderStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for BitmapDecoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for BitmapDecoder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapDecoder;{acef22ba-1d74-4c91-9dfc-9620745233e6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BitmapDecoder {
    type Vtable = IBitmapDecoder_Vtbl;
}
unsafe impl ::windows::core::Interface for BitmapDecoder {
    const IID: ::windows::core::GUID = <IBitmapDecoder as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BitmapDecoder {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapDecoder";
}
::windows::core::interface_hierarchy!(BitmapDecoder, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<BitmapDecoder> for IBitmapFrame {
    type Error = ::windows::core::Error;
    fn try_from(value: BitmapDecoder) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BitmapDecoder> for IBitmapFrame {
    type Error = ::windows::core::Error;
    fn try_from(value: &BitmapDecoder) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&BitmapDecoder> for ::windows::core::InParam<IBitmapFrame> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BitmapDecoder) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<BitmapDecoder> for IBitmapFrameWithSoftwareBitmap {
    type Error = ::windows::core::Error;
    fn try_from(value: BitmapDecoder) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BitmapDecoder> for IBitmapFrameWithSoftwareBitmap {
    type Error = ::windows::core::Error;
    fn try_from(value: &BitmapDecoder) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&BitmapDecoder> for ::windows::core::InParam<IBitmapFrameWithSoftwareBitmap> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BitmapDecoder) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for BitmapDecoder {}
unsafe impl ::core::marker::Sync for BitmapDecoder {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct BitmapEncoder(::windows::core::IUnknown);
impl BitmapEncoder {
    pub fn EncoderInformation(&self) -> ::windows::core::Result<BitmapCodecInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).EncoderInformation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BitmapProperties(&self) -> ::windows::core::Result<BitmapProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitmapProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BitmapContainerProperties(&self) -> ::windows::core::Result<BitmapProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitmapContainerProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsThumbnailGenerated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsThumbnailGenerated)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsThumbnailGenerated(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsThumbnailGenerated)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn GeneratedThumbnailWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GeneratedThumbnailWidth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetGeneratedThumbnailWidth(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetGeneratedThumbnailWidth)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn GeneratedThumbnailHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GeneratedThumbnailHeight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetGeneratedThumbnailHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetGeneratedThumbnailHeight)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn BitmapTransform(&self) -> ::windows::core::Result<BitmapTransform> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitmapTransform)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetPixelData(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, width: u32, height: u32, dpix: f64, dpiy: f64, pixels: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetPixelData)(::windows::core::Vtable::as_raw(this), pixelformat, alphamode, width, height, dpix, dpiy, pixels.len() as u32, pixels.as_ptr()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GoToNextFrameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GoToNextFrameAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GoToNextFrameWithEncodingOptionsAsync<P0, E0>(&self, encodingoptions: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GoToNextFrameWithEncodingOptionsAsync)(::windows::core::Vtable::as_raw(this), encodingoptions.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FlushAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BmpEncoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BmpEncoderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn JpegEncoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).JpegEncoderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn PngEncoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PngEncoderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn TiffEncoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).TiffEncoderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn GifEncoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GifEncoderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn JpegXREncoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).JpegXREncoderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetEncoderInformationEnumerator() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<BitmapCodecInformation>> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetEncoderInformationEnumerator)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateAsync<P0, E0>(encoderid: ::windows::core::GUID, stream: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateAsync)(::windows::core::Vtable::as_raw(this), encoderid, stream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn CreateWithEncodingOptionsAsync<P0, E0, P1, E1>(encoderid: ::windows::core::GUID, stream: P0, encodingoptions: P1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
        P1: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithEncodingOptionsAsync)(::windows::core::Vtable::as_raw(this), encoderid, stream.try_into().map_err(|e| e.into())?.abi(), encodingoptions.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateForTranscodingAsync<P0, E0>(stream: P0, bitmapdecoder: &BitmapDecoder) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateForTranscodingAsync)(::windows::core::Vtable::as_raw(this), stream.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(bitmapdecoder), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateForInPlacePropertyEncodingAsync(bitmapdecoder: &BitmapDecoder) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateForInPlacePropertyEncodingAsync)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmapdecoder), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn HeifEncoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapEncoderStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HeifEncoderId)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn SetSoftwareBitmap(&self, bitmap: &SoftwareBitmap) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBitmapEncoderWithSoftwareBitmap>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetSoftwareBitmap)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap)).ok() }
    }
    #[doc(hidden)]
    pub fn IBitmapEncoderStatics<R, F: FnOnce(&IBitmapEncoderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BitmapEncoder, IBitmapEncoderStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IBitmapEncoderStatics2<R, F: FnOnce(&IBitmapEncoderStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BitmapEncoder, IBitmapEncoderStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for BitmapEncoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for BitmapEncoder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapEncoder;{2bc468e3-e1f8-4b54-95e8-32919551ce62})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BitmapEncoder {
    type Vtable = IBitmapEncoder_Vtbl;
}
unsafe impl ::windows::core::Interface for BitmapEncoder {
    const IID: ::windows::core::GUID = <IBitmapEncoder as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BitmapEncoder {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapEncoder";
}
::windows::core::interface_hierarchy!(BitmapEncoder, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BitmapEncoder {}
unsafe impl ::core::marker::Sync for BitmapEncoder {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct BitmapFrame(::windows::core::IUnknown);
impl BitmapFrame {
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetThumbnailAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BitmapProperties(&self) -> ::windows::core::Result<BitmapPropertiesView> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitmapProperties)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BitmapPixelFormat(&self) -> ::windows::core::Result<BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitmapPixelFormat)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> ::windows::core::Result<BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitmapAlphaMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DpiX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DpiX)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn DpiY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DpiY)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PixelWidth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PixelHeight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OrientedPixelWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OrientedPixelWidth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn OrientedPixelHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).OrientedPixelHeight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPixelDataAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataTransformedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: &BitmapTransform, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPixelDataTransformedAsync)(::windows::core::Vtable::as_raw(this), pixelformat, alphamode, ::core::mem::transmute_copy(transform), exiforientationmode, colormanagementmode, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSoftwareBitmapAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapConvertedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSoftwareBitmapConvertedAsync)(::windows::core::Vtable::as_raw(this), pixelformat, alphamode, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapTransformedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: &BitmapTransform, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetSoftwareBitmapTransformedAsync)(::windows::core::Vtable::as_raw(this), pixelformat, alphamode, ::core::mem::transmute_copy(transform), exiforientationmode, colormanagementmode, result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for BitmapFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for BitmapFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapFrame;{72a49a1c-8081-438d-91bc-94ecfc8185c6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BitmapFrame {
    type Vtable = IBitmapFrame_Vtbl;
}
unsafe impl ::windows::core::Interface for BitmapFrame {
    const IID: ::windows::core::GUID = <IBitmapFrame as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BitmapFrame {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapFrame";
}
::windows::core::interface_hierarchy!(BitmapFrame, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<BitmapFrame> for IBitmapFrame {
    type Error = ::windows::core::Error;
    fn try_from(value: BitmapFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BitmapFrame> for IBitmapFrame {
    type Error = ::windows::core::Error;
    fn try_from(value: &BitmapFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&BitmapFrame> for ::windows::core::InParam<IBitmapFrame> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BitmapFrame) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<BitmapFrame> for IBitmapFrameWithSoftwareBitmap {
    type Error = ::windows::core::Error;
    fn try_from(value: BitmapFrame) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BitmapFrame> for IBitmapFrameWithSoftwareBitmap {
    type Error = ::windows::core::Error;
    fn try_from(value: &BitmapFrame) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&BitmapFrame> for ::windows::core::InParam<IBitmapFrameWithSoftwareBitmap> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BitmapFrame) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for BitmapFrame {}
unsafe impl ::core::marker::Sync for BitmapFrame {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct BitmapProperties(::windows::core::IUnknown);
impl BitmapProperties {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetPropertiesAsync<P0, E0>(&self, propertiestoset: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SetPropertiesAsync)(::windows::core::Vtable::as_raw(this), propertiestoset.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPropertiesAsync<P0, E0>(&self, propertiestoretrieve: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapPropertySet>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<IBitmapPropertiesView>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPropertiesAsync)(::windows::core::Vtable::as_raw(this), propertiestoretrieve.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for BitmapProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for BitmapProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapProperties;{ea9f4f1b-b505-4450-a4d1-e8ca94529d8d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BitmapProperties {
    type Vtable = IBitmapProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for BitmapProperties {
    const IID: ::windows::core::GUID = <IBitmapProperties as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BitmapProperties {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapProperties";
}
::windows::core::interface_hierarchy!(BitmapProperties, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<BitmapProperties> for IBitmapPropertiesView {
    type Error = ::windows::core::Error;
    fn try_from(value: BitmapProperties) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BitmapProperties> for IBitmapPropertiesView {
    type Error = ::windows::core::Error;
    fn try_from(value: &BitmapProperties) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&BitmapProperties> for ::windows::core::InParam<IBitmapPropertiesView> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BitmapProperties) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for BitmapProperties {}
unsafe impl ::core::marker::Sync for BitmapProperties {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct BitmapPropertiesView(::windows::core::IUnknown);
impl BitmapPropertiesView {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPropertiesAsync<P0, E0>(&self, propertiestoretrieve: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapPropertySet>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetPropertiesAsync)(::windows::core::Vtable::as_raw(this), propertiestoretrieve.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for BitmapPropertiesView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for BitmapPropertiesView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapPropertiesView;{7e0fe87a-3a70-48f8-9c55-196cf5a545f5})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BitmapPropertiesView {
    type Vtable = IBitmapPropertiesView_Vtbl;
}
unsafe impl ::windows::core::Interface for BitmapPropertiesView {
    const IID: ::windows::core::GUID = <IBitmapPropertiesView as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BitmapPropertiesView {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapPropertiesView";
}
::windows::core::interface_hierarchy!(BitmapPropertiesView, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<BitmapPropertiesView> for IBitmapPropertiesView {
    type Error = ::windows::core::Error;
    fn try_from(value: BitmapPropertiesView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&BitmapPropertiesView> for IBitmapPropertiesView {
    type Error = ::windows::core::Error;
    fn try_from(value: &BitmapPropertiesView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&BitmapPropertiesView> for ::windows::core::InParam<IBitmapPropertiesView> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BitmapPropertiesView) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for BitmapPropertiesView {}
unsafe impl ::core::marker::Sync for BitmapPropertiesView {}
#[doc = "*Required features: `\"Graphics_Imaging\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct BitmapPropertySet(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl BitmapPropertySet {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BitmapPropertySet, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).First)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<BitmapTypedValue> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Lookup)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).HasKey)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, BitmapTypedValue>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert(&self, key: &::windows::core::HSTRING, value: &BitmapTypedValue) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Insert)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key), ::core::mem::transmute_copy(value), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove(&self, key: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Remove)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(key)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Clear)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for BitmapPropertySet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for BitmapPropertySet {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapPropertySet;pinterface({3c2925fe-8519-45c1-aa79-197b6718c1c1};string;rc(Windows.Graphics.Imaging.BitmapTypedValue;{cd8044a9-2443-4000-b0cd-79316c56f589})))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Vtable for BitmapPropertySet {
    type Vtable = super::super::Foundation::Collections::IMap_Vtbl<::windows::core::HSTRING, BitmapTypedValue>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for BitmapPropertySet {
    const IID: ::windows::core::GUID = <super::super::Foundation::Collections::IMap<::windows::core::HSTRING, BitmapTypedValue> as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for BitmapPropertySet {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapPropertySet";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for BitmapPropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &BitmapPropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows::core::interface_hierarchy!(BitmapPropertySet, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<BitmapPropertySet> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: BitmapPropertySet) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&BitmapPropertySet> for super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BitmapPropertySet) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&BitmapPropertySet> for ::windows::core::InParam<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BitmapPropertySet) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<BitmapPropertySet> for super::super::Foundation::Collections::IMap<::windows::core::HSTRING, BitmapTypedValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: BitmapPropertySet) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&BitmapPropertySet> for super::super::Foundation::Collections::IMap<::windows::core::HSTRING, BitmapTypedValue> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BitmapPropertySet) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&BitmapPropertySet> for ::windows::core::InParam<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, BitmapTypedValue>> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BitmapPropertySet) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for BitmapPropertySet {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for BitmapPropertySet {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct BitmapTransform(::windows::core::IUnknown);
impl BitmapTransform {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BitmapTransform, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn ScaledWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScaledWidth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetScaledWidth(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetScaledWidth)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ScaledHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ScaledHeight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetScaledHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetScaledHeight)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn InterpolationMode(&self) -> ::windows::core::Result<BitmapInterpolationMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).InterpolationMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetInterpolationMode(&self, value: BitmapInterpolationMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetInterpolationMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Flip(&self) -> ::windows::core::Result<BitmapFlip> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Flip)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetFlip(&self, value: BitmapFlip) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetFlip)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Rotation(&self) -> ::windows::core::Result<BitmapRotation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Rotation)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetRotation(&self, value: BitmapRotation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetRotation)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Bounds(&self) -> ::windows::core::Result<BitmapBounds> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Bounds)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetBounds(&self, value: BitmapBounds) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBounds)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
}
impl ::core::clone::Clone for BitmapTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for BitmapTransform {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapTransform;{ae755344-e268-4d35-adcf-e995d31a8d34})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BitmapTransform {
    type Vtable = IBitmapTransform_Vtbl;
}
unsafe impl ::windows::core::Interface for BitmapTransform {
    const IID: ::windows::core::GUID = <IBitmapTransform as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BitmapTransform {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapTransform";
}
::windows::core::interface_hierarchy!(BitmapTransform, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BitmapTransform {}
unsafe impl ::core::marker::Sync for BitmapTransform {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct BitmapTypedValue(::windows::core::IUnknown);
impl BitmapTypedValue {
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Value)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Type(&self) -> ::windows::core::Result<super::super::Foundation::PropertyType> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Type)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Create<P0>(value: P0, r#type: super::super::Foundation::PropertyType) -> ::windows::core::Result<BitmapTypedValue>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IInspectable>>,
    {
        Self::IBitmapTypedValueFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), value.into().abi(), r#type, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBitmapTypedValueFactory<R, F: FnOnce(&IBitmapTypedValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<BitmapTypedValue, IBitmapTypedValueFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for BitmapTypedValue {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for BitmapTypedValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapTypedValue;{cd8044a9-2443-4000-b0cd-79316c56f589})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for BitmapTypedValue {
    type Vtable = IBitmapTypedValue_Vtbl;
}
unsafe impl ::windows::core::Interface for BitmapTypedValue {
    const IID: ::windows::core::GUID = <IBitmapTypedValue as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BitmapTypedValue {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapTypedValue";
}
::windows::core::interface_hierarchy!(BitmapTypedValue, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for BitmapTypedValue {}
unsafe impl ::core::marker::Sync for BitmapTypedValue {}
#[doc = "*Required features: `\"Graphics_Imaging\"`, `\"Storage_Streams\"`*"]
#[cfg(feature = "Storage_Streams")]
#[repr(transparent)]
pub struct ImageStream(::windows::core::IUnknown);
#[cfg(feature = "Storage_Streams")]
impl ImageStream {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ContentType)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsync<P0, E0>(&self, buffer: P0, count: u32, options: super::super::Storage::Streams::InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ReadAsync)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), count, options, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteAsync<P0, E0>(&self, buffer: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).WriteAsync)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FlushAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).SetSize)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetInputStreamAt)(::windows::core::Vtable::as_raw(this), position, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetOutputStreamAt)(::windows::core::Vtable::as_raw(this), position, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Position(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Position)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Seek(&self, position: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Seek)(::windows::core::Vtable::as_raw(this), position).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CloneStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CloneStream)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanRead)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CanWrite)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::clone::Clone for ImageStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for ImageStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.ImageStream;{cc254827-4b3d-438f-9232-10c76bc7e038})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::Vtable for ImageStream {
    type Vtable = super::super::Storage::Streams::IRandomAccessStreamWithContentType_Vtbl;
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::Interface for ImageStream {
    const IID: ::windows::core::GUID = <super::super::Storage::Streams::IRandomAccessStreamWithContentType as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Storage_Streams")]
impl ::windows::core::RuntimeName for ImageStream {
    const NAME: &'static str = "Windows.Graphics.Imaging.ImageStream";
}
#[cfg(feature = "Storage_Streams")]
::windows::core::interface_hierarchy!(ImageStream, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::core::convert::TryFrom<ImageStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: ImageStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::core::convert::TryFrom<&ImageStream> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::core::convert::TryFrom<&ImageStream> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<ImageStream> for super::super::Storage::Streams::IContentTypeProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: ImageStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&ImageStream> for super::super::Storage::Streams::IContentTypeProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&ImageStream> for ::windows::core::InParam<super::super::Storage::Streams::IContentTypeProvider> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<ImageStream> for super::super::Storage::Streams::IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: ImageStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&ImageStream> for super::super::Storage::Streams::IInputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&ImageStream> for ::windows::core::InParam<super::super::Storage::Streams::IInputStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<ImageStream> for super::super::Storage::Streams::IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: ImageStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&ImageStream> for super::super::Storage::Streams::IOutputStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&ImageStream> for ::windows::core::InParam<super::super::Storage::Streams::IOutputStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<ImageStream> for super::super::Storage::Streams::IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: ImageStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&ImageStream> for super::super::Storage::Streams::IRandomAccessStream {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&ImageStream> for ::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStream> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<ImageStream> for super::super::Storage::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows::core::Error;
    fn try_from(value: ImageStream) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&ImageStream> for super::super::Storage::Streams::IRandomAccessStreamWithContentType {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageStream) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&ImageStream> for ::windows::core::InParam<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ImageStream) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::core::marker::Send for ImageStream {}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::core::marker::Sync for ImageStream {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct PixelDataProvider(::windows::core::IUnknown);
impl PixelDataProvider {
    pub fn DetachPixelData(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DetachPixelData)(::windows::core::Vtable::as_raw(this), ::windows::core::Array::<u8>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
impl ::core::clone::Clone for PixelDataProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for PixelDataProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.PixelDataProvider;{dd831f25-185c-4595-9fb9-ccbe6ec18a6f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for PixelDataProvider {
    type Vtable = IPixelDataProvider_Vtbl;
}
unsafe impl ::windows::core::Interface for PixelDataProvider {
    const IID: ::windows::core::GUID = <IPixelDataProvider as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PixelDataProvider {
    const NAME: &'static str = "Windows.Graphics.Imaging.PixelDataProvider";
}
::windows::core::interface_hierarchy!(PixelDataProvider, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PixelDataProvider {}
unsafe impl ::core::marker::Sync for PixelDataProvider {}
#[doc = "*Required features: `\"Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct SoftwareBitmap(::windows::core::IUnknown);
impl SoftwareBitmap {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Vtable::vtable(this).Close)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn BitmapPixelFormat(&self) -> ::windows::core::Result<BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitmapPixelFormat)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> ::windows::core::Result<BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BitmapAlphaMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PixelWidth)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PixelHeight)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsReadOnly)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDpiX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDpiX)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DpiX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DpiX)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetDpiY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetDpiY)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn DpiY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).DpiY)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn LockBuffer(&self, mode: BitmapBufferAccessMode) -> ::windows::core::Result<BitmapBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).LockBuffer)(::windows::core::Vtable::as_raw(this), mode, result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CopyTo(&self, bitmap: &SoftwareBitmap) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).CopyTo)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(bitmap)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CopyFromBuffer<P0, E0>(&self, buffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).CopyFromBuffer)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CopyToBuffer<P0, E0>(&self, buffer: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).CopyToBuffer)(::windows::core::Vtable::as_raw(this), buffer.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    pub fn GetReadOnlyView(&self) -> ::windows::core::Result<SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetReadOnlyView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn Create(format: BitmapPixelFormat, width: i32, height: i32) -> ::windows::core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), format, width, height, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn CreateWithAlpha(format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> ::windows::core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWithAlpha)(::windows::core::Vtable::as_raw(this), format, width, height, alpha, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Copy(source: &SoftwareBitmap) -> ::windows::core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Copy)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(source), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn Convert(source: &SoftwareBitmap, format: BitmapPixelFormat) -> ::windows::core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Convert)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(source), format, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn ConvertWithAlpha(source: &SoftwareBitmap, format: BitmapPixelFormat, alpha: BitmapAlphaMode) -> ::windows::core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ConvertWithAlpha)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(source), format, alpha, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCopyFromBuffer<P0, E0>(source: P0, format: BitmapPixelFormat, width: i32, height: i32) -> ::windows::core::Result<SoftwareBitmap>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateCopyFromBuffer)(::windows::core::Vtable::as_raw(this), source.try_into().map_err(|e| e.into())?.abi(), format, width, height, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCopyWithAlphaFromBuffer<P0, E0>(source: P0, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> ::windows::core::Result<SoftwareBitmap>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::super::Storage::Streams::IBuffer>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateCopyWithAlphaFromBuffer)(::windows::core::Vtable::as_raw(this), source.try_into().map_err(|e| e.into())?.abi(), format, width, height, alpha, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn CreateCopyFromSurfaceAsync<P0, E0>(surface: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::DirectX::Direct3D11::IDirect3DSurface>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateCopyFromSurfaceAsync)(::windows::core::Vtable::as_raw(this), surface.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Graphics_DirectX_Direct3D11\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn CreateCopyWithAlphaFromSurfaceAsync<P0, E0>(surface: P0, alpha: BitmapAlphaMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<super::DirectX::Direct3D11::IDirect3DSurface>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateCopyWithAlphaFromSurfaceAsync)(::windows::core::Vtable::as_raw(this), surface.try_into().map_err(|e| e.into())?.abi(), alpha, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISoftwareBitmapFactory<R, F: FnOnce(&ISoftwareBitmapFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SoftwareBitmap, ISoftwareBitmapFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn ISoftwareBitmapStatics<R, F: FnOnce(&ISoftwareBitmapStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SoftwareBitmap, ISoftwareBitmapStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for SoftwareBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for SoftwareBitmap {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.SoftwareBitmap;{689e0708-7eef-483f-963f-da938818e073})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for SoftwareBitmap {
    type Vtable = ISoftwareBitmap_Vtbl;
}
unsafe impl ::windows::core::Interface for SoftwareBitmap {
    const IID: ::windows::core::GUID = <ISoftwareBitmap as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SoftwareBitmap {
    const NAME: &'static str = "Windows.Graphics.Imaging.SoftwareBitmap";
}
::windows::core::interface_hierarchy!(SoftwareBitmap, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<SoftwareBitmap> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: SoftwareBitmap) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SoftwareBitmap> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &SoftwareBitmap) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&SoftwareBitmap> for ::windows::core::InParam<super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SoftwareBitmap) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
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
unsafe impl ::windows::core::Abi for BitmapAlphaMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for BitmapAlphaMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapAlphaMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BitmapAlphaMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapAlphaMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for BitmapBufferAccessMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for BitmapBufferAccessMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapBufferAccessMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BitmapBufferAccessMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapBufferAccessMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for BitmapFlip {
    type Abi = Self;
}
impl ::core::fmt::Debug for BitmapFlip {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapFlip").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BitmapFlip {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapFlip;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for BitmapInterpolationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for BitmapInterpolationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapInterpolationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BitmapInterpolationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapInterpolationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for BitmapPixelFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for BitmapPixelFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapPixelFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BitmapPixelFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapPixelFormat;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for BitmapRotation {
    type Abi = Self;
}
impl ::core::fmt::Debug for BitmapRotation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BitmapRotation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BitmapRotation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapRotation;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for ColorManagementMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ColorManagementMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ColorManagementMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ColorManagementMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.ColorManagementMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for ExifOrientationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ExifOrientationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExifOrientationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ExifOrientationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.ExifOrientationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for JpegSubsamplingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for JpegSubsamplingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JpegSubsamplingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for JpegSubsamplingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.JpegSubsamplingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for PngFilterMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for PngFilterMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PngFilterMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PngFilterMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.PngFilterMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for TiffCompressionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for TiffCompressionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TiffCompressionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for TiffCompressionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.TiffCompressionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for BitmapBounds {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BitmapBounds {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Imaging.BitmapBounds;u4;u4;u4;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for BitmapPlaneDescription {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BitmapPlaneDescription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Imaging.BitmapPlaneDescription;i4;i4;i4;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
unsafe impl ::windows::core::Abi for BitmapSize {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BitmapSize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Imaging.BitmapSize;u4;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
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
