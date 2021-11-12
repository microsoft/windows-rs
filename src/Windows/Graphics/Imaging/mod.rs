#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BitmapAlphaMode(pub i32);
impl BitmapAlphaMode {
    pub const Premultiplied: BitmapAlphaMode = BitmapAlphaMode(0i32);
    pub const Straight: BitmapAlphaMode = BitmapAlphaMode(1i32);
    pub const Ignore: BitmapAlphaMode = BitmapAlphaMode(2i32);
}
impl ::core::convert::From<i32> for BitmapAlphaMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BitmapAlphaMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BitmapAlphaMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapAlphaMode;i4)");
}
impl ::windows::core::DefaultType for BitmapAlphaMode {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct BitmapBounds {
    pub X: u32,
    pub Y: u32,
    pub Width: u32,
    pub Height: u32,
}
impl BitmapBounds {}
impl ::core::default::Default for BitmapBounds {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BitmapBounds {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BitmapBounds").field("X", &self.X).field("Y", &self.Y).field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::core::cmp::PartialEq for BitmapBounds {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for BitmapBounds {}
unsafe impl ::windows::core::Abi for BitmapBounds {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BitmapBounds {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Imaging.BitmapBounds;u4;u4;u4;u4)");
}
impl ::windows::core::DefaultType for BitmapBounds {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BitmapBuffer(pub ::windows::core::IInspectable);
impl BitmapBuffer {
    pub fn GetPlaneCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn GetPlaneDescription(&self, index: i32) -> ::windows::core::Result<BitmapPlaneDescription> {
        let this = self;
        unsafe {
            let mut result__: BitmapPlaneDescription = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<BitmapPlaneDescription>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateReference(&self) -> ::windows::core::Result<super::super::Foundation::IMemoryBufferReference> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IMemoryBuffer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IMemoryBufferReference>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BitmapBuffer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapBuffer;{a53e04c4-399c-438c-b28f-a63a6b83d1a1})");
}
unsafe impl ::windows::core::Interface for BitmapBuffer {
    type Vtable = IBitmapBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa53e04c4_399c_438c_b28f_a63a6b83d1a1);
}
impl ::windows::core::RuntimeName for BitmapBuffer {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapBuffer";
}
impl ::core::convert::From<BitmapBuffer> for ::windows::core::IUnknown {
    fn from(value: BitmapBuffer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BitmapBuffer> for ::windows::core::IUnknown {
    fn from(value: &BitmapBuffer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BitmapBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BitmapBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BitmapBuffer> for ::windows::core::IInspectable {
    fn from(value: BitmapBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BitmapBuffer> for ::windows::core::IInspectable {
    fn from(value: &BitmapBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BitmapBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BitmapBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for BitmapBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &BitmapBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for BitmapBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IMemoryBuffer> for &BitmapBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IMemoryBuffer> {
        ::core::convert::TryInto::<super::super::Foundation::IMemoryBuffer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BitmapBuffer {}
unsafe impl ::core::marker::Sync for BitmapBuffer {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BitmapBufferAccessMode(pub i32);
impl BitmapBufferAccessMode {
    pub const Read: BitmapBufferAccessMode = BitmapBufferAccessMode(0i32);
    pub const ReadWrite: BitmapBufferAccessMode = BitmapBufferAccessMode(1i32);
    pub const Write: BitmapBufferAccessMode = BitmapBufferAccessMode(2i32);
}
impl ::core::convert::From<i32> for BitmapBufferAccessMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BitmapBufferAccessMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BitmapBufferAccessMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapBufferAccessMode;i4)");
}
impl ::windows::core::DefaultType for BitmapBufferAccessMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BitmapCodecInformation(pub ::windows::core::IInspectable);
impl BitmapCodecInformation {
    pub fn CodecId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FileExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn MimeTypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BitmapCodecInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapCodecInformation;{400caaf2-c4b0-4392-a3b0-6f6f9ba95cb4})");
}
unsafe impl ::windows::core::Interface for BitmapCodecInformation {
    type Vtable = IBitmapCodecInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x400caaf2_c4b0_4392_a3b0_6f6f9ba95cb4);
}
impl ::windows::core::RuntimeName for BitmapCodecInformation {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapCodecInformation";
}
impl ::core::convert::From<BitmapCodecInformation> for ::windows::core::IUnknown {
    fn from(value: BitmapCodecInformation) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BitmapCodecInformation> for ::windows::core::IUnknown {
    fn from(value: &BitmapCodecInformation) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BitmapCodecInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BitmapCodecInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BitmapCodecInformation> for ::windows::core::IInspectable {
    fn from(value: BitmapCodecInformation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BitmapCodecInformation> for ::windows::core::IInspectable {
    fn from(value: &BitmapCodecInformation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BitmapCodecInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BitmapCodecInformation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BitmapCodecInformation {}
unsafe impl ::core::marker::Sync for BitmapCodecInformation {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BitmapDecoder(pub ::windows::core::IInspectable);
impl BitmapDecoder {
    pub fn BitmapContainerProperties(&self) -> ::windows::core::Result<BitmapPropertiesView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapPropertiesView>(result__)
        }
    }
    pub fn DecoderInformation(&self) -> ::windows::core::Result<BitmapCodecInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapCodecInformation>(result__)
        }
    }
    pub fn FrameCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetPreviewAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ImageStream>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetFrameAsync(&self, frameindex: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapFrame>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), frameindex, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BitmapFrame>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageStream>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ImageStream>>(result__)
        }
    }
    pub fn BitmapProperties(&self) -> ::windows::core::Result<BitmapPropertiesView> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapPropertiesView>(result__)
        }
    }
    pub fn BitmapPixelFormat(&self) -> ::windows::core::Result<BitmapPixelFormat> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: BitmapPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapPixelFormat>(result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> ::windows::core::Result<BitmapAlphaMode> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: BitmapAlphaMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapAlphaMode>(result__)
        }
    }
    pub fn DpiX(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn DpiY(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn OrientedPixelWidth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn OrientedPixelHeight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PixelDataProvider>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataTransformedAsync<'a, Param2: ::windows::core::IntoParam<'a, BitmapTransform>>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: Param2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PixelDataProvider>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapConvertedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), pixelformat, alphamode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapTransformedAsync<'a, Param2: ::windows::core::IntoParam<'a, BitmapTransform>>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: Param2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        }
    }
    pub fn BmpDecoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn JpegDecoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn PngDecoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn TiffDecoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn GifDecoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn JpegXRDecoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn IcoDecoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetDecoderInformationEnumerator() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<BitmapCodecInformation>> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<BitmapCodecInformation>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(stream: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapDecoder>> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BitmapDecoder>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateWithIdAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(decoderid: Param0, stream: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapDecoder>> {
        Self::IBitmapDecoderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), decoderid.into_param().abi(), stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BitmapDecoder>>(result__)
        })
    }
    pub fn HeifDecoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapDecoderStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn WebpDecoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapDecoderStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn IBitmapDecoderStatics<R, F: FnOnce(&IBitmapDecoderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BitmapDecoder, IBitmapDecoderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBitmapDecoderStatics2<R, F: FnOnce(&IBitmapDecoderStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BitmapDecoder, IBitmapDecoderStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BitmapDecoder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapDecoder;{acef22ba-1d74-4c91-9dfc-9620745233e6})");
}
unsafe impl ::windows::core::Interface for BitmapDecoder {
    type Vtable = IBitmapDecoder_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xacef22ba_1d74_4c91_9dfc_9620745233e6);
}
impl ::windows::core::RuntimeName for BitmapDecoder {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapDecoder";
}
impl ::core::convert::From<BitmapDecoder> for ::windows::core::IUnknown {
    fn from(value: BitmapDecoder) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BitmapDecoder> for ::windows::core::IUnknown {
    fn from(value: &BitmapDecoder) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BitmapDecoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BitmapDecoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BitmapDecoder> for ::windows::core::IInspectable {
    fn from(value: BitmapDecoder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BitmapDecoder> for ::windows::core::IInspectable {
    fn from(value: &BitmapDecoder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BitmapDecoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BitmapDecoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IBitmapFrame> for BitmapDecoder {
    fn into_param(self) -> ::windows::core::Param<'a, IBitmapFrame> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBitmapFrame> for &BitmapDecoder {
    fn into_param(self) -> ::windows::core::Param<'a, IBitmapFrame> {
        ::core::convert::TryInto::<IBitmapFrame>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
impl<'a> ::windows::core::IntoParam<'a, IBitmapFrameWithSoftwareBitmap> for BitmapDecoder {
    fn into_param(self) -> ::windows::core::Param<'a, IBitmapFrameWithSoftwareBitmap> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBitmapFrameWithSoftwareBitmap> for &BitmapDecoder {
    fn into_param(self) -> ::windows::core::Param<'a, IBitmapFrameWithSoftwareBitmap> {
        ::core::convert::TryInto::<IBitmapFrameWithSoftwareBitmap>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BitmapDecoder {}
unsafe impl ::core::marker::Sync for BitmapDecoder {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BitmapEncoder(pub ::windows::core::IInspectable);
impl BitmapEncoder {
    pub fn EncoderInformation(&self) -> ::windows::core::Result<BitmapCodecInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapCodecInformation>(result__)
        }
    }
    pub fn BitmapProperties(&self) -> ::windows::core::Result<BitmapProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapProperties>(result__)
        }
    }
    pub fn BitmapContainerProperties(&self) -> ::windows::core::Result<BitmapProperties> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapProperties>(result__)
        }
    }
    pub fn IsThumbnailGenerated(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsThumbnailGenerated(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn GeneratedThumbnailWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetGeneratedThumbnailWidth(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn GeneratedThumbnailHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetGeneratedThumbnailHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn BitmapTransform(&self) -> ::windows::core::Result<BitmapTransform> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapTransform>(result__)
        }
    }
    pub fn SetPixelData(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, width: u32, height: u32, dpix: f64, dpiy: f64, pixels: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), pixelformat, alphamode, width, height, dpix, dpiy, pixels.len() as u32, ::core::mem::transmute(pixels.as_ptr())).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn GoToNextFrameAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GoToNextFrameWithEncodingOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>>>(&self, encodingoptions: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), encodingoptions.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    pub fn SetSoftwareBitmap<'a, Param0: ::windows::core::IntoParam<'a, SoftwareBitmap>>(&self, bitmap: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBitmapEncoderWithSoftwareBitmap>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), bitmap.into_param().abi()).ok() }
    }
    pub fn BmpEncoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn JpegEncoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn PngEncoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn TiffEncoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn GifEncoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn JpegXREncoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetEncoderInformationEnumerator() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<BitmapCodecInformation>> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<BitmapCodecInformation>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>>(encoderid: Param0, stream: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), encoderid.into_param().abi(), stream.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BitmapEncoder>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))]
    pub fn CreateWithEncodingOptionsAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>>>(
        encoderid: Param0,
        stream: Param1,
        encodingoptions: Param2,
    ) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), encoderid.into_param().abi(), stream.into_param().abi(), encodingoptions.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BitmapEncoder>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn CreateForTranscodingAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Param1: ::windows::core::IntoParam<'a, BitmapDecoder>>(stream: Param0, bitmapdecoder: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), stream.into_param().abi(), bitmapdecoder.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BitmapEncoder>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateForInPlacePropertyEncodingAsync<'a, Param0: ::windows::core::IntoParam<'a, BitmapDecoder>>(bitmapdecoder: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapEncoder>> {
        Self::IBitmapEncoderStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), bitmapdecoder.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BitmapEncoder>>(result__)
        })
    }
    pub fn HeifEncoderId() -> ::windows::core::Result<::windows::core::GUID> {
        Self::IBitmapEncoderStatics2(|this| unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        })
    }
    pub fn IBitmapEncoderStatics<R, F: FnOnce(&IBitmapEncoderStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BitmapEncoder, IBitmapEncoderStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IBitmapEncoderStatics2<R, F: FnOnce(&IBitmapEncoderStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BitmapEncoder, IBitmapEncoderStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BitmapEncoder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapEncoder;{2bc468e3-e1f8-4b54-95e8-32919551ce62})");
}
unsafe impl ::windows::core::Interface for BitmapEncoder {
    type Vtable = IBitmapEncoder_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bc468e3_e1f8_4b54_95e8_32919551ce62);
}
impl ::windows::core::RuntimeName for BitmapEncoder {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapEncoder";
}
impl ::core::convert::From<BitmapEncoder> for ::windows::core::IUnknown {
    fn from(value: BitmapEncoder) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BitmapEncoder> for ::windows::core::IUnknown {
    fn from(value: &BitmapEncoder) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BitmapEncoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BitmapEncoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BitmapEncoder> for ::windows::core::IInspectable {
    fn from(value: BitmapEncoder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BitmapEncoder> for ::windows::core::IInspectable {
    fn from(value: &BitmapEncoder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BitmapEncoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BitmapEncoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BitmapEncoder {}
unsafe impl ::core::marker::Sync for BitmapEncoder {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BitmapFlip(pub i32);
impl BitmapFlip {
    pub const None: BitmapFlip = BitmapFlip(0i32);
    pub const Horizontal: BitmapFlip = BitmapFlip(1i32);
    pub const Vertical: BitmapFlip = BitmapFlip(2i32);
}
impl ::core::convert::From<i32> for BitmapFlip {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BitmapFlip {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BitmapFlip {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapFlip;i4)");
}
impl ::windows::core::DefaultType for BitmapFlip {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BitmapFrame(pub ::windows::core::IInspectable);
impl BitmapFrame {
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ImageStream>>(result__)
        }
    }
    pub fn BitmapProperties(&self) -> ::windows::core::Result<BitmapPropertiesView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapPropertiesView>(result__)
        }
    }
    pub fn BitmapPixelFormat(&self) -> ::windows::core::Result<BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: BitmapPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapPixelFormat>(result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> ::windows::core::Result<BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__: BitmapAlphaMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapAlphaMode>(result__)
        }
    }
    pub fn DpiX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn DpiY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn OrientedPixelWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn OrientedPixelHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PixelDataProvider>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataTransformedAsync<'a, Param2: ::windows::core::IntoParam<'a, BitmapTransform>>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: Param2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PixelDataProvider>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapConvertedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), pixelformat, alphamode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapTransformedAsync<'a, Param2: ::windows::core::IntoParam<'a, BitmapTransform>>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: Param2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrameWithSoftwareBitmap>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BitmapFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapFrame;{72a49a1c-8081-438d-91bc-94ecfc8185c6})");
}
unsafe impl ::windows::core::Interface for BitmapFrame {
    type Vtable = IBitmapFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72a49a1c_8081_438d_91bc_94ecfc8185c6);
}
impl ::windows::core::RuntimeName for BitmapFrame {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapFrame";
}
impl ::core::convert::From<BitmapFrame> for ::windows::core::IUnknown {
    fn from(value: BitmapFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BitmapFrame> for ::windows::core::IUnknown {
    fn from(value: &BitmapFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BitmapFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BitmapFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BitmapFrame> for ::windows::core::IInspectable {
    fn from(value: BitmapFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BitmapFrame> for ::windows::core::IInspectable {
    fn from(value: &BitmapFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BitmapFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BitmapFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<BitmapFrame> for IBitmapFrame {
    fn from(value: BitmapFrame) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapFrame> for IBitmapFrame {
    fn from(value: &BitmapFrame) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBitmapFrame> for BitmapFrame {
    fn into_param(self) -> ::windows::core::Param<'a, IBitmapFrame> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBitmapFrame> for &BitmapFrame {
    fn into_param(self) -> ::windows::core::Param<'a, IBitmapFrame> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IBitmapFrameWithSoftwareBitmap> for BitmapFrame {
    fn into_param(self) -> ::windows::core::Param<'a, IBitmapFrameWithSoftwareBitmap> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBitmapFrameWithSoftwareBitmap> for &BitmapFrame {
    fn into_param(self) -> ::windows::core::Param<'a, IBitmapFrameWithSoftwareBitmap> {
        ::core::convert::TryInto::<IBitmapFrameWithSoftwareBitmap>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BitmapFrame {}
unsafe impl ::core::marker::Sync for BitmapFrame {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BitmapInterpolationMode(pub i32);
impl BitmapInterpolationMode {
    pub const NearestNeighbor: BitmapInterpolationMode = BitmapInterpolationMode(0i32);
    pub const Linear: BitmapInterpolationMode = BitmapInterpolationMode(1i32);
    pub const Cubic: BitmapInterpolationMode = BitmapInterpolationMode(2i32);
    pub const Fant: BitmapInterpolationMode = BitmapInterpolationMode(3i32);
}
impl ::core::convert::From<i32> for BitmapInterpolationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BitmapInterpolationMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BitmapInterpolationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapInterpolationMode;i4)");
}
impl ::windows::core::DefaultType for BitmapInterpolationMode {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BitmapPixelFormat(pub i32);
impl BitmapPixelFormat {
    pub const Unknown: BitmapPixelFormat = BitmapPixelFormat(0i32);
    pub const Rgba16: BitmapPixelFormat = BitmapPixelFormat(12i32);
    pub const Rgba8: BitmapPixelFormat = BitmapPixelFormat(30i32);
    pub const Gray16: BitmapPixelFormat = BitmapPixelFormat(57i32);
    pub const Gray8: BitmapPixelFormat = BitmapPixelFormat(62i32);
    pub const Bgra8: BitmapPixelFormat = BitmapPixelFormat(87i32);
    pub const Nv12: BitmapPixelFormat = BitmapPixelFormat(103i32);
    pub const P010: BitmapPixelFormat = BitmapPixelFormat(104i32);
    pub const Yuy2: BitmapPixelFormat = BitmapPixelFormat(107i32);
}
impl ::core::convert::From<i32> for BitmapPixelFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BitmapPixelFormat {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BitmapPixelFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapPixelFormat;i4)");
}
impl ::windows::core::DefaultType for BitmapPixelFormat {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct BitmapPlaneDescription {
    pub StartIndex: i32,
    pub Width: i32,
    pub Height: i32,
    pub Stride: i32,
}
impl BitmapPlaneDescription {}
impl ::core::default::Default for BitmapPlaneDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BitmapPlaneDescription {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BitmapPlaneDescription").field("StartIndex", &self.StartIndex).field("Width", &self.Width).field("Height", &self.Height).field("Stride", &self.Stride).finish()
    }
}
impl ::core::cmp::PartialEq for BitmapPlaneDescription {
    fn eq(&self, other: &Self) -> bool {
        self.StartIndex == other.StartIndex && self.Width == other.Width && self.Height == other.Height && self.Stride == other.Stride
    }
}
impl ::core::cmp::Eq for BitmapPlaneDescription {}
unsafe impl ::windows::core::Abi for BitmapPlaneDescription {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BitmapPlaneDescription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Imaging.BitmapPlaneDescription;i4;i4;i4;i4)");
}
impl ::windows::core::DefaultType for BitmapPlaneDescription {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BitmapProperties(pub ::windows::core::IInspectable);
impl BitmapProperties {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SetPropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>>>(&self, propertiestoset: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertiestoset.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetPropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, propertiestoretrieve: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapPropertySet>> {
        let this = &::windows::core::Interface::cast::<IBitmapPropertiesView>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertiestoretrieve.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BitmapPropertySet>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BitmapProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapProperties;{ea9f4f1b-b505-4450-a4d1-e8ca94529d8d})");
}
unsafe impl ::windows::core::Interface for BitmapProperties {
    type Vtable = IBitmapProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea9f4f1b_b505_4450_a4d1_e8ca94529d8d);
}
impl ::windows::core::RuntimeName for BitmapProperties {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapProperties";
}
impl ::core::convert::From<BitmapProperties> for ::windows::core::IUnknown {
    fn from(value: BitmapProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BitmapProperties> for ::windows::core::IUnknown {
    fn from(value: &BitmapProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BitmapProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BitmapProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BitmapProperties> for ::windows::core::IInspectable {
    fn from(value: BitmapProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BitmapProperties> for ::windows::core::IInspectable {
    fn from(value: &BitmapProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BitmapProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BitmapProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IBitmapPropertiesView> for BitmapProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IBitmapPropertiesView> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBitmapPropertiesView> for &BitmapProperties {
    fn into_param(self) -> ::windows::core::Param<'a, IBitmapPropertiesView> {
        ::core::convert::TryInto::<IBitmapPropertiesView>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for BitmapProperties {}
unsafe impl ::core::marker::Sync for BitmapProperties {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BitmapPropertiesView(pub ::windows::core::IInspectable);
impl BitmapPropertiesView {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetPropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, propertiestoretrieve: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapPropertySet>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertiestoretrieve.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BitmapPropertySet>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for BitmapPropertiesView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapPropertiesView;{7e0fe87a-3a70-48f8-9c55-196cf5a545f5})");
}
unsafe impl ::windows::core::Interface for BitmapPropertiesView {
    type Vtable = IBitmapPropertiesView_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e0fe87a_3a70_48f8_9c55_196cf5a545f5);
}
impl ::windows::core::RuntimeName for BitmapPropertiesView {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapPropertiesView";
}
impl ::core::convert::From<BitmapPropertiesView> for ::windows::core::IUnknown {
    fn from(value: BitmapPropertiesView) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BitmapPropertiesView> for ::windows::core::IUnknown {
    fn from(value: &BitmapPropertiesView) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BitmapPropertiesView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BitmapPropertiesView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BitmapPropertiesView> for ::windows::core::IInspectable {
    fn from(value: BitmapPropertiesView) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BitmapPropertiesView> for ::windows::core::IInspectable {
    fn from(value: &BitmapPropertiesView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BitmapPropertiesView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BitmapPropertiesView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<BitmapPropertiesView> for IBitmapPropertiesView {
    fn from(value: BitmapPropertiesView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BitmapPropertiesView> for IBitmapPropertiesView {
    fn from(value: &BitmapPropertiesView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBitmapPropertiesView> for BitmapPropertiesView {
    fn into_param(self) -> ::windows::core::Param<'a, IBitmapPropertiesView> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBitmapPropertiesView> for &BitmapPropertiesView {
    fn into_param(self) -> ::windows::core::Param<'a, IBitmapPropertiesView> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BitmapPropertiesView {}
unsafe impl ::core::marker::Sync for BitmapPropertiesView {}
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BitmapPropertySet(pub ::windows::core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl BitmapPropertySet {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BitmapPropertySet, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<BitmapTypedValue> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<BitmapTypedValue>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), key.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, BitmapTypedValue>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, BitmapTypedValue>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, BitmapTypedValue>>(&self, key: Param0, value: Param1) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), key.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, key: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), key.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>>(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::RuntimeType for BitmapPropertySet {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapPropertySet;pinterface({3c2925fe-8519-45c1-aa79-197b6718c1c1};string;rc(Windows.Graphics.Imaging.BitmapTypedValue;{cd8044a9-2443-4000-b0cd-79316c56f589})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for BitmapPropertySet {
    type Vtable = super::super::Foundation::Collections::IMap_abi<::windows::core::HSTRING, BitmapTypedValue>;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_signature(<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, BitmapTypedValue> as ::windows::core::RuntimeType>::SIGNATURE);
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for BitmapPropertySet {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapPropertySet";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<BitmapPropertySet> for ::windows::core::IUnknown {
    fn from(value: BitmapPropertySet) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&BitmapPropertySet> for ::windows::core::IUnknown {
    fn from(value: &BitmapPropertySet) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BitmapPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BitmapPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<BitmapPropertySet> for ::windows::core::IInspectable {
    fn from(value: BitmapPropertySet) -> Self {
        value.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&BitmapPropertySet> for ::windows::core::IInspectable {
    fn from(value: &BitmapPropertySet) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BitmapPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BitmapPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<BitmapPropertySet> for super::super::Foundation::Collections::IMap<::windows::core::HSTRING, BitmapTypedValue> {
    fn from(value: BitmapPropertySet) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&BitmapPropertySet> for super::super::Foundation::Collections::IMap<::windows::core::HSTRING, BitmapTypedValue> {
    fn from(value: &BitmapPropertySet) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IMap<::windows::core::HSTRING, BitmapTypedValue>> for BitmapPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IMap<::windows::core::HSTRING, BitmapTypedValue>> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IMap<::windows::core::HSTRING, BitmapTypedValue>> for &BitmapPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IMap<::windows::core::HSTRING, BitmapTypedValue>> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>> for BitmapPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>> for &BitmapPropertySet {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for BitmapPropertySet {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for BitmapPropertySet {}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for BitmapPropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(all(feature = "Foundation_Collections"))]
impl ::core::iter::IntoIterator for &BitmapPropertySet {
    type Item = super::super::Foundation::Collections::IKeyValuePair<::windows::core::HSTRING, BitmapTypedValue>;
    type IntoIter = super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct BitmapRotation(pub i32);
impl BitmapRotation {
    pub const None: BitmapRotation = BitmapRotation(0i32);
    pub const Clockwise90Degrees: BitmapRotation = BitmapRotation(1i32);
    pub const Clockwise180Degrees: BitmapRotation = BitmapRotation(2i32);
    pub const Clockwise270Degrees: BitmapRotation = BitmapRotation(3i32);
}
impl ::core::convert::From<i32> for BitmapRotation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for BitmapRotation {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BitmapRotation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.BitmapRotation;i4)");
}
impl ::windows::core::DefaultType for BitmapRotation {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct BitmapSize {
    pub Width: u32,
    pub Height: u32,
}
impl BitmapSize {}
impl ::core::default::Default for BitmapSize {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for BitmapSize {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("BitmapSize").field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::core::cmp::PartialEq for BitmapSize {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for BitmapSize {}
unsafe impl ::windows::core::Abi for BitmapSize {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for BitmapSize {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.Imaging.BitmapSize;u4;u4)");
}
impl ::windows::core::DefaultType for BitmapSize {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BitmapTransform(pub ::windows::core::IInspectable);
impl BitmapTransform {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BitmapTransform, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ScaledWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetScaledWidth(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ScaledHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn SetScaledHeight(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn InterpolationMode(&self) -> ::windows::core::Result<BitmapInterpolationMode> {
        let this = self;
        unsafe {
            let mut result__: BitmapInterpolationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapInterpolationMode>(result__)
        }
    }
    pub fn SetInterpolationMode(&self, value: BitmapInterpolationMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Flip(&self) -> ::windows::core::Result<BitmapFlip> {
        let this = self;
        unsafe {
            let mut result__: BitmapFlip = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapFlip>(result__)
        }
    }
    pub fn SetFlip(&self, value: BitmapFlip) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Rotation(&self) -> ::windows::core::Result<BitmapRotation> {
        let this = self;
        unsafe {
            let mut result__: BitmapRotation = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapRotation>(result__)
        }
    }
    pub fn SetRotation(&self, value: BitmapRotation) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Bounds(&self) -> ::windows::core::Result<BitmapBounds> {
        let this = self;
        unsafe {
            let mut result__: BitmapBounds = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapBounds>(result__)
        }
    }
    pub fn SetBounds<'a, Param0: ::windows::core::IntoParam<'a, BitmapBounds>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for BitmapTransform {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapTransform;{ae755344-e268-4d35-adcf-e995d31a8d34})");
}
unsafe impl ::windows::core::Interface for BitmapTransform {
    type Vtable = IBitmapTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae755344_e268_4d35_adcf_e995d31a8d34);
}
impl ::windows::core::RuntimeName for BitmapTransform {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapTransform";
}
impl ::core::convert::From<BitmapTransform> for ::windows::core::IUnknown {
    fn from(value: BitmapTransform) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BitmapTransform> for ::windows::core::IUnknown {
    fn from(value: &BitmapTransform) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BitmapTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BitmapTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BitmapTransform> for ::windows::core::IInspectable {
    fn from(value: BitmapTransform) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BitmapTransform> for ::windows::core::IInspectable {
    fn from(value: &BitmapTransform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BitmapTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BitmapTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BitmapTransform {}
unsafe impl ::core::marker::Sync for BitmapTransform {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct BitmapTypedValue(pub ::windows::core::IInspectable);
impl BitmapTypedValue {
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Type(&self) -> ::windows::core::Result<super::super::Foundation::PropertyType> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::PropertyType = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::PropertyType>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(value: Param0, r#type: super::super::Foundation::PropertyType) -> ::windows::core::Result<BitmapTypedValue> {
        Self::IBitmapTypedValueFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value.into_param().abi(), r#type, &mut result__).from_abi::<BitmapTypedValue>(result__)
        })
    }
    pub fn IBitmapTypedValueFactory<R, F: FnOnce(&IBitmapTypedValueFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BitmapTypedValue, IBitmapTypedValueFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for BitmapTypedValue {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.BitmapTypedValue;{cd8044a9-2443-4000-b0cd-79316c56f589})");
}
unsafe impl ::windows::core::Interface for BitmapTypedValue {
    type Vtable = IBitmapTypedValue_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd8044a9_2443_4000_b0cd_79316c56f589);
}
impl ::windows::core::RuntimeName for BitmapTypedValue {
    const NAME: &'static str = "Windows.Graphics.Imaging.BitmapTypedValue";
}
impl ::core::convert::From<BitmapTypedValue> for ::windows::core::IUnknown {
    fn from(value: BitmapTypedValue) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&BitmapTypedValue> for ::windows::core::IUnknown {
    fn from(value: &BitmapTypedValue) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BitmapTypedValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BitmapTypedValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<BitmapTypedValue> for ::windows::core::IInspectable {
    fn from(value: BitmapTypedValue) -> Self {
        value.0
    }
}
impl ::core::convert::From<&BitmapTypedValue> for ::windows::core::IInspectable {
    fn from(value: &BitmapTypedValue) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BitmapTypedValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BitmapTypedValue {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for BitmapTypedValue {}
unsafe impl ::core::marker::Sync for BitmapTypedValue {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ColorManagementMode(pub i32);
impl ColorManagementMode {
    pub const DoNotColorManage: ColorManagementMode = ColorManagementMode(0i32);
    pub const ColorManageToSRgb: ColorManagementMode = ColorManagementMode(1i32);
}
impl ::core::convert::From<i32> for ColorManagementMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ColorManagementMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ColorManagementMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.ColorManagementMode;i4)");
}
impl ::windows::core::DefaultType for ColorManagementMode {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ExifOrientationMode(pub i32);
impl ExifOrientationMode {
    pub const IgnoreExifOrientation: ExifOrientationMode = ExifOrientationMode(0i32);
    pub const RespectExifOrientation: ExifOrientationMode = ExifOrientationMode(1i32);
}
impl ::core::convert::From<i32> for ExifOrientationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ExifOrientationMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for ExifOrientationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.ExifOrientationMode;i4)");
}
impl ::windows::core::DefaultType for ExifOrientationMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapBuffer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapBuffer {
    type Vtable = IBitmapBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa53e04c4_399c_438c_b28f_a63a6b83d1a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: i32, result__: *mut BitmapPlaneDescription) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapCodecInformation(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapCodecInformation {
    type Vtable = IBitmapCodecInformation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x400caaf2_c4b0_4392_a3b0_6f6f9ba95cb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapCodecInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapDecoder(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapDecoder {
    type Vtable = IBitmapDecoder_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xacef22ba_1d74_4c91_9dfc_9620745233e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapDecoder_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, frameindex: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapDecoderStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapDecoderStatics {
    type Vtable = IBitmapDecoderStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x438ccb26_bcef_4e95_bad6_23a822e58d01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapDecoderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, decoderid: ::windows::core::GUID, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapDecoderStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapDecoderStatics2 {
    type Vtable = IBitmapDecoderStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ba68ea_99a1_40c4_80d9_aef0dafa6c3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapDecoderStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapEncoder(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapEncoder {
    type Vtable = IBitmapEncoder_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bc468e3_e1f8_4b54_95e8_32919551ce62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapEncoder_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, width: u32, height: u32, dpix: f64, dpiy: f64, pixels_array_size: u32, pixels: *const u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encodingoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapEncoderStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapEncoderStatics {
    type Vtable = IBitmapEncoderStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa74356a7_a4e4_4eb9_8e40_564de7e1ccb2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapEncoderStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encoderid: ::windows::core::GUID, stream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, encoderid: ::windows::core::GUID, stream: ::windows::core::RawPtr, encodingoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stream: ::windows::core::RawPtr, bitmapdecoder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmapdecoder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapEncoderStatics2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapEncoderStatics2 {
    type Vtable = IBitmapEncoderStatics2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33cbc259_fe31_41b1_b812_086d21e87e16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapEncoderStatics2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapEncoderWithSoftwareBitmap(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapEncoderWithSoftwareBitmap {
    type Vtable = IBitmapEncoderWithSoftwareBitmap_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x686cd241_4330_4c77_ace4_0334968b1768);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapEncoderWithSoftwareBitmap_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBitmapFrame(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapFrame {
    type Vtable = IBitmapFrame_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72a49a1c_8081_438d_91bc_94ecfc8185c6);
}
impl IBitmapFrame {
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageStream>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ImageStream>>(result__)
        }
    }
    pub fn BitmapProperties(&self) -> ::windows::core::Result<BitmapPropertiesView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapPropertiesView>(result__)
        }
    }
    pub fn BitmapPixelFormat(&self) -> ::windows::core::Result<BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: BitmapPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapPixelFormat>(result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> ::windows::core::Result<BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__: BitmapAlphaMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapAlphaMode>(result__)
        }
    }
    pub fn DpiX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn DpiY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn OrientedPixelWidth(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn OrientedPixelHeight(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PixelDataProvider>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataTransformedAsync<'a, Param2: ::windows::core::IntoParam<'a, BitmapTransform>>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: Param2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PixelDataProvider>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IBitmapFrame {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{72a49a1c-8081-438d-91bc-94ecfc8185c6}");
}
impl ::core::convert::From<IBitmapFrame> for ::windows::core::IUnknown {
    fn from(value: IBitmapFrame) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IBitmapFrame> for ::windows::core::IUnknown {
    fn from(value: &IBitmapFrame) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBitmapFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBitmapFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IBitmapFrame> for ::windows::core::IInspectable {
    fn from(value: IBitmapFrame) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBitmapFrame> for ::windows::core::IInspectable {
    fn from(value: &IBitmapFrame) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBitmapFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IBitmapFrame {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapFrame_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BitmapPixelFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BitmapAlphaMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: ::windows::core::RawPtr, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBitmapFrameWithSoftwareBitmap(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapFrameWithSoftwareBitmap {
    type Vtable = IBitmapFrameWithSoftwareBitmap_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe287c9a_420c_4963_87ad_691436e08383);
}
impl IBitmapFrameWithSoftwareBitmap {
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapConvertedAsync(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), pixelformat, alphamode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetSoftwareBitmapTransformedAsync<'a, Param2: ::windows::core::IntoParam<'a, BitmapTransform>>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: Param2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<ImageStream>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<ImageStream>>(result__)
        }
    }
    pub fn BitmapProperties(&self) -> ::windows::core::Result<BitmapPropertiesView> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapPropertiesView>(result__)
        }
    }
    pub fn BitmapPixelFormat(&self) -> ::windows::core::Result<BitmapPixelFormat> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: BitmapPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapPixelFormat>(result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> ::windows::core::Result<BitmapAlphaMode> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: BitmapAlphaMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapAlphaMode>(result__)
        }
    }
    pub fn DpiX(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn DpiY(&self) -> ::windows::core::Result<f64> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn OrientedPixelWidth(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn OrientedPixelHeight(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PixelDataProvider>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetPixelDataTransformedAsync<'a, Param2: ::windows::core::IntoParam<'a, BitmapTransform>>(&self, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: Param2, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PixelDataProvider>> {
        let this = &::windows::core::Interface::cast::<IBitmapFrame>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), pixelformat, alphamode, transform.into_param().abi(), exiforientationmode, colormanagementmode, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<PixelDataProvider>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IBitmapFrameWithSoftwareBitmap {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fe287c9a-420c-4963-87ad-691436e08383}");
}
impl ::core::convert::From<IBitmapFrameWithSoftwareBitmap> for ::windows::core::IUnknown {
    fn from(value: IBitmapFrameWithSoftwareBitmap) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IBitmapFrameWithSoftwareBitmap> for ::windows::core::IUnknown {
    fn from(value: &IBitmapFrameWithSoftwareBitmap) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBitmapFrameWithSoftwareBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBitmapFrameWithSoftwareBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IBitmapFrameWithSoftwareBitmap> for ::windows::core::IInspectable {
    fn from(value: IBitmapFrameWithSoftwareBitmap) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBitmapFrameWithSoftwareBitmap> for ::windows::core::IInspectable {
    fn from(value: &IBitmapFrameWithSoftwareBitmap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBitmapFrameWithSoftwareBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IBitmapFrameWithSoftwareBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IBitmapFrame> for IBitmapFrameWithSoftwareBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, IBitmapFrame> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IBitmapFrame> for &IBitmapFrameWithSoftwareBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, IBitmapFrame> {
        ::core::convert::TryInto::<IBitmapFrame>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapFrameWithSoftwareBitmap_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pixelformat: BitmapPixelFormat, alphamode: BitmapAlphaMode, transform: ::windows::core::RawPtr, exiforientationmode: ExifOrientationMode, colormanagementmode: ColorManagementMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapProperties {
    type Vtable = IBitmapProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea9f4f1b_b505_4450_a4d1_e8ca94529d8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertiestoset: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IBitmapPropertiesView(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapPropertiesView {
    type Vtable = IBitmapPropertiesView_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e0fe87a_3a70_48f8_9c55_196cf5a545f5);
}
impl IBitmapPropertiesView {
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn GetPropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, propertiestoretrieve: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<BitmapPropertySet>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), propertiestoretrieve.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<BitmapPropertySet>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IBitmapPropertiesView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{7e0fe87a-3a70-48f8-9c55-196cf5a545f5}");
}
impl ::core::convert::From<IBitmapPropertiesView> for ::windows::core::IUnknown {
    fn from(value: IBitmapPropertiesView) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IBitmapPropertiesView> for ::windows::core::IUnknown {
    fn from(value: &IBitmapPropertiesView) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBitmapPropertiesView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IBitmapPropertiesView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IBitmapPropertiesView> for ::windows::core::IInspectable {
    fn from(value: IBitmapPropertiesView) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IBitmapPropertiesView> for ::windows::core::IInspectable {
    fn from(value: &IBitmapPropertiesView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IBitmapPropertiesView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IBitmapPropertiesView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapPropertiesView_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, propertiestoretrieve: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapTransform(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapTransform {
    type Vtable = IBitmapTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae755344_e268_4d35_adcf_e995d31a8d34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapTransform_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BitmapInterpolationMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: BitmapInterpolationMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BitmapFlip) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: BitmapFlip) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BitmapRotation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: BitmapRotation) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BitmapBounds) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: BitmapBounds) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapTypedValue(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapTypedValue {
    type Vtable = IBitmapTypedValue_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd8044a9_2443_4000_b0cd_79316c56f589);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapTypedValue_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::PropertyType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IBitmapTypedValueFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IBitmapTypedValueFactory {
    type Vtable = IBitmapTypedValueFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92dbb599_ce13_46bb_9545_cb3a3f63eb8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapTypedValueFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr, r#type: super::super::Foundation::PropertyType, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPixelDataProvider(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPixelDataProvider {
    type Vtable = IPixelDataProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd831f25_185c_4595_9fb9_ccbe6ec18a6f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPixelDataProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result_size__: *mut u32, result__: *mut *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISoftwareBitmap(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISoftwareBitmap {
    type Vtable = ISoftwareBitmap_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x689e0708_7eef_483f_963f_da938818e073);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmap_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BitmapPixelFormat) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut BitmapAlphaMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: BitmapBufferAccessMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bitmap: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, buffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISoftwareBitmapFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISoftwareBitmapFactory {
    type Vtable = ISoftwareBitmapFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc99feb69_2d62_4d47_a6b3_4fdb6a07fdf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, format: BitmapPixelFormat, width: i32, height: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ISoftwareBitmapStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for ISoftwareBitmapStatics {
    type Vtable = ISoftwareBitmapStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf0385db_672f_4a9d_806e_c2442f343e86);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, source: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, source: ::windows::core::RawPtr, format: BitmapPixelFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, source: ::windows::core::RawPtr, format: BitmapPixelFormat, alpha: BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, source: ::windows::core::RawPtr, format: BitmapPixelFormat, width: i32, height: i32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(feature = "Storage_Streams")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, source: ::windows::core::RawPtr, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, surface: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, surface: ::windows::core::RawPtr, alpha: BitmapAlphaMode, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11")))] usize,
);
#[cfg(feature = "Storage_Streams")]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ImageStream(pub ::windows::core::IInspectable);
#[cfg(feature = "Storage_Streams")]
impl ImageStream {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IContentTypeProvider>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn ReadAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, buffer: Param0, count: u32, options: super::super::Storage::Streams::InputStreamOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IInputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), count, options, &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<super::super::Storage::Streams::IBuffer, u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn WriteAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), buffer.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn FlushAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IOutputStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Size(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn SetSize(&self, value: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetInputStreamAt(&self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<super::super::Storage::Streams::IInputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn GetOutputStreamAt(&self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IOutputStream> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), position, &mut result__).from_abi::<super::super::Storage::Streams::IOutputStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Position(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn Seek(&self, position: u64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), position).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CloneStream(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::Streams::IRandomAccessStream>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Storage::Streams::IRandomAccessStream>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::RuntimeType for ImageStream {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.ImageStream;{cc254827-4b3d-438f-9232-10c76bc7e038})");
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::windows::core::Interface for ImageStream {
    type Vtable = super::super::Storage::Streams::IRandomAccessStreamWithContentType_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc254827_4b3d_438f_9232_10c76bc7e038);
}
#[cfg(feature = "Storage_Streams")]
impl ::windows::core::RuntimeName for ImageStream {
    const NAME: &'static str = "Windows.Graphics.Imaging.ImageStream";
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<ImageStream> for ::windows::core::IUnknown {
    fn from(value: ImageStream) -> Self {
        value.0 .0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&ImageStream> for ::windows::core::IUnknown {
    fn from(value: &ImageStream) -> Self {
        value.0 .0.clone()
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ImageStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ImageStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<ImageStream> for ::windows::core::IInspectable {
    fn from(value: ImageStream) -> Self {
        value.0
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&ImageStream> for ::windows::core::IInspectable {
    fn from(value: &ImageStream) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ImageStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ImageStream {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<ImageStream> for super::super::Storage::Streams::IRandomAccessStreamWithContentType {
    fn from(value: ImageStream) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::From<&ImageStream> for super::super::Storage::Streams::IRandomAccessStreamWithContentType {
    fn from(value: &ImageStream) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> for ImageStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> for &ImageStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for ImageStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &ImageStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IContentTypeProvider> for ImageStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IContentTypeProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IContentTypeProvider> for &ImageStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IContentTypeProvider> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IContentTypeProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream> for ImageStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IInputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IInputStream> for &ImageStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IInputStream> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IInputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream> for ImageStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IOutputStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IOutputStream> for &ImageStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IOutputStream> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IOutputStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream> for ImageStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStream> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream> for &ImageStream {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Storage::Streams::IRandomAccessStream> {
        ::core::convert::TryInto::<super::super::Storage::Streams::IRandomAccessStream>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::core::marker::Send for ImageStream {}
#[cfg(feature = "Storage_Streams")]
unsafe impl ::core::marker::Sync for ImageStream {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct JpegSubsamplingMode(pub i32);
impl JpegSubsamplingMode {
    pub const Default: JpegSubsamplingMode = JpegSubsamplingMode(0i32);
    pub const Y4Cb2Cr0: JpegSubsamplingMode = JpegSubsamplingMode(1i32);
    pub const Y4Cb2Cr2: JpegSubsamplingMode = JpegSubsamplingMode(2i32);
    pub const Y4Cb4Cr4: JpegSubsamplingMode = JpegSubsamplingMode(3i32);
}
impl ::core::convert::From<i32> for JpegSubsamplingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for JpegSubsamplingMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for JpegSubsamplingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.JpegSubsamplingMode;i4)");
}
impl ::windows::core::DefaultType for JpegSubsamplingMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PixelDataProvider(pub ::windows::core::IInspectable);
impl PixelDataProvider {
    pub fn DetachPixelData(&self) -> ::windows::core::Result<::windows::core::Array<u8>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::Array<u8> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), ::windows::core::Array::<u8>::set_abi_len(&mut result__), &mut result__ as *mut _ as _).and_then(|| result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PixelDataProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.PixelDataProvider;{dd831f25-185c-4595-9fb9-ccbe6ec18a6f})");
}
unsafe impl ::windows::core::Interface for PixelDataProvider {
    type Vtable = IPixelDataProvider_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd831f25_185c_4595_9fb9_ccbe6ec18a6f);
}
impl ::windows::core::RuntimeName for PixelDataProvider {
    const NAME: &'static str = "Windows.Graphics.Imaging.PixelDataProvider";
}
impl ::core::convert::From<PixelDataProvider> for ::windows::core::IUnknown {
    fn from(value: PixelDataProvider) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PixelDataProvider> for ::windows::core::IUnknown {
    fn from(value: &PixelDataProvider) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PixelDataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PixelDataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PixelDataProvider> for ::windows::core::IInspectable {
    fn from(value: PixelDataProvider) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PixelDataProvider> for ::windows::core::IInspectable {
    fn from(value: &PixelDataProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PixelDataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PixelDataProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PixelDataProvider {}
unsafe impl ::core::marker::Sync for PixelDataProvider {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PngFilterMode(pub i32);
impl PngFilterMode {
    pub const Automatic: PngFilterMode = PngFilterMode(0i32);
    pub const None: PngFilterMode = PngFilterMode(1i32);
    pub const Sub: PngFilterMode = PngFilterMode(2i32);
    pub const Up: PngFilterMode = PngFilterMode(3i32);
    pub const Average: PngFilterMode = PngFilterMode(4i32);
    pub const Paeth: PngFilterMode = PngFilterMode(5i32);
    pub const Adaptive: PngFilterMode = PngFilterMode(6i32);
}
impl ::core::convert::From<i32> for PngFilterMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PngFilterMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PngFilterMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.PngFilterMode;i4)");
}
impl ::windows::core::DefaultType for PngFilterMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct SoftwareBitmap(pub ::windows::core::IInspectable);
impl SoftwareBitmap {
    pub fn BitmapPixelFormat(&self) -> ::windows::core::Result<BitmapPixelFormat> {
        let this = self;
        unsafe {
            let mut result__: BitmapPixelFormat = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapPixelFormat>(result__)
        }
    }
    pub fn BitmapAlphaMode(&self) -> ::windows::core::Result<BitmapAlphaMode> {
        let this = self;
        unsafe {
            let mut result__: BitmapAlphaMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BitmapAlphaMode>(result__)
        }
    }
    pub fn PixelWidth(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn PixelHeight(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetDpiX(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DpiX(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetDpiY(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn DpiY(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn LockBuffer(&self, mode: BitmapBufferAccessMode) -> ::windows::core::Result<BitmapBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), mode, &mut result__).from_abi::<BitmapBuffer>(result__)
        }
    }
    pub fn CopyTo<'a, Param0: ::windows::core::IntoParam<'a, SoftwareBitmap>>(&self, bitmap: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), bitmap.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CopyFromBuffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), buffer.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CopyToBuffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(&self, buffer: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), buffer.into_param().abi()).ok() }
    }
    pub fn GetReadOnlyView(&self) -> ::windows::core::Result<SoftwareBitmap> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<SoftwareBitmap>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn Create(format: BitmapPixelFormat, width: i32, height: i32) -> ::windows::core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), format, width, height, &mut result__).from_abi::<SoftwareBitmap>(result__)
        })
    }
    pub fn CreateWithAlpha(format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> ::windows::core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), format, width, height, alpha, &mut result__).from_abi::<SoftwareBitmap>(result__)
        })
    }
    pub fn Copy<'a, Param0: ::windows::core::IntoParam<'a, SoftwareBitmap>>(source: Param0) -> ::windows::core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), source.into_param().abi(), &mut result__).from_abi::<SoftwareBitmap>(result__)
        })
    }
    pub fn Convert<'a, Param0: ::windows::core::IntoParam<'a, SoftwareBitmap>>(source: Param0, format: BitmapPixelFormat) -> ::windows::core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), source.into_param().abi(), format, &mut result__).from_abi::<SoftwareBitmap>(result__)
        })
    }
    pub fn ConvertWithAlpha<'a, Param0: ::windows::core::IntoParam<'a, SoftwareBitmap>>(source: Param0, format: BitmapPixelFormat, alpha: BitmapAlphaMode) -> ::windows::core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), source.into_param().abi(), format, alpha, &mut result__).from_abi::<SoftwareBitmap>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCopyFromBuffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(source: Param0, format: BitmapPixelFormat, width: i32, height: i32) -> ::windows::core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), source.into_param().abi(), format, width, height, &mut result__).from_abi::<SoftwareBitmap>(result__)
        })
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn CreateCopyWithAlphaFromBuffer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Storage::Streams::IBuffer>>(source: Param0, format: BitmapPixelFormat, width: i32, height: i32, alpha: BitmapAlphaMode) -> ::windows::core::Result<SoftwareBitmap> {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), source.into_param().abi(), format, width, height, alpha, &mut result__).from_abi::<SoftwareBitmap>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn CreateCopyFromSurfaceAsync<'a, Param0: ::windows::core::IntoParam<'a, super::DirectX::Direct3D11::IDirect3DSurface>>(surface: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), surface.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        })
    }
    #[cfg(all(feature = "Foundation", feature = "Graphics_DirectX_Direct3D11"))]
    pub fn CreateCopyWithAlphaFromSurfaceAsync<'a, Param0: ::windows::core::IntoParam<'a, super::DirectX::Direct3D11::IDirect3DSurface>>(surface: Param0, alpha: BitmapAlphaMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<SoftwareBitmap>> {
        Self::ISoftwareBitmapStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), surface.into_param().abi(), alpha, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<SoftwareBitmap>>(result__)
        })
    }
    pub fn ISoftwareBitmapFactory<R, F: FnOnce(&ISoftwareBitmapFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SoftwareBitmap, ISoftwareBitmapFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ISoftwareBitmapStatics<R, F: FnOnce(&ISoftwareBitmapStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SoftwareBitmap, ISoftwareBitmapStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for SoftwareBitmap {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Graphics.Imaging.SoftwareBitmap;{689e0708-7eef-483f-963f-da938818e073})");
}
unsafe impl ::windows::core::Interface for SoftwareBitmap {
    type Vtable = ISoftwareBitmap_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x689e0708_7eef_483f_963f_da938818e073);
}
impl ::windows::core::RuntimeName for SoftwareBitmap {
    const NAME: &'static str = "Windows.Graphics.Imaging.SoftwareBitmap";
}
impl ::core::convert::From<SoftwareBitmap> for ::windows::core::IUnknown {
    fn from(value: SoftwareBitmap) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&SoftwareBitmap> for ::windows::core::IUnknown {
    fn from(value: &SoftwareBitmap) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SoftwareBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SoftwareBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<SoftwareBitmap> for ::windows::core::IInspectable {
    fn from(value: SoftwareBitmap) -> Self {
        value.0
    }
}
impl ::core::convert::From<&SoftwareBitmap> for ::windows::core::IInspectable {
    fn from(value: &SoftwareBitmap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SoftwareBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SoftwareBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for SoftwareBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::IClosable> for &SoftwareBitmap {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for SoftwareBitmap {}
unsafe impl ::core::marker::Sync for SoftwareBitmap {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct TiffCompressionMode(pub i32);
impl TiffCompressionMode {
    pub const Automatic: TiffCompressionMode = TiffCompressionMode(0i32);
    pub const None: TiffCompressionMode = TiffCompressionMode(1i32);
    pub const Ccitt3: TiffCompressionMode = TiffCompressionMode(2i32);
    pub const Ccitt4: TiffCompressionMode = TiffCompressionMode(3i32);
    pub const Lzw: TiffCompressionMode = TiffCompressionMode(4i32);
    pub const Rle: TiffCompressionMode = TiffCompressionMode(5i32);
    pub const Zip: TiffCompressionMode = TiffCompressionMode(6i32);
    pub const LzwhDifferencing: TiffCompressionMode = TiffCompressionMode(7i32);
}
impl ::core::convert::From<i32> for TiffCompressionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for TiffCompressionMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for TiffCompressionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.Imaging.TiffCompressionMode;i4)");
}
impl ::windows::core::DefaultType for TiffCompressionMode {
    type DefaultType = Self;
}
