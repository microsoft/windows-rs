#[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
#[repr(transparent)]
pub struct IWICImageEncoder(::windows::core::IUnknown);
impl IWICImageEncoder {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn WriteFrame<'a, P0, P1>(&self, pimage: P0, pframeencode: P1, pimageparameters: *const super::WICImageParameters) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Direct2D::ID2D1Image>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::IWICBitmapFrameEncode>>,
    {
        (::windows::core::Interface::vtable(self).WriteFrame)(::windows::core::Interface::as_raw(self), pimage.into().abi(), pframeencode.into().abi(), ::core::mem::transmute(pimageparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn WriteFrameThumbnail<'a, P0, P1>(&self, pimage: P0, pframeencode: P1, pimageparameters: *const super::WICImageParameters) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Direct2D::ID2D1Image>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::IWICBitmapFrameEncode>>,
    {
        (::windows::core::Interface::vtable(self).WriteFrameThumbnail)(::windows::core::Interface::as_raw(self), pimage.into().abi(), pframeencode.into().abi(), ::core::mem::transmute(pimageparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn WriteThumbnail<'a, P0, P1>(&self, pimage: P0, pencoder: P1, pimageparameters: *const super::WICImageParameters) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Direct2D::ID2D1Image>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::IWICBitmapEncoder>>,
    {
        (::windows::core::Interface::vtable(self).WriteThumbnail)(::windows::core::Interface::as_raw(self), pimage.into().abi(), pencoder.into().abi(), ::core::mem::transmute(pimageparameters)).ok()
    }
}
impl ::core::convert::From<IWICImageEncoder> for ::windows::core::IUnknown {
    fn from(value: IWICImageEncoder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWICImageEncoder> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWICImageEncoder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICImageEncoder> for ::windows::core::IUnknown {
    fn from(value: &IWICImageEncoder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWICImageEncoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICImageEncoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICImageEncoder {}
impl ::core::fmt::Debug for IWICImageEncoder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICImageEncoder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWICImageEncoder {
    type Vtable = IWICImageEncoder_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04c75bf8_3ce1_473b_acc5_3cc4f5e94999);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICImageEncoder_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub WriteFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimage: *mut ::core::ffi::c_void, pframeencode: *mut ::core::ffi::c_void, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    WriteFrame: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub WriteFrameThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimage: *mut ::core::ffi::c_void, pframeencode: *mut ::core::ffi::c_void, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    WriteFrameThumbnail: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub WriteThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimage: *mut ::core::ffi::c_void, pencoder: *mut ::core::ffi::c_void, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    WriteThumbnail: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
#[repr(transparent)]
pub struct IWICImagingFactory2(::windows::core::IUnknown);
impl IWICImagingFactory2 {
    pub unsafe fn CreateDecoderFromFilename<'a, P0>(&self, wzfilename: P0, pguidvendor: *const ::windows::core::GUID, dwdesiredaccess: u32, metadataoptions: super::WICDecodeOptions) -> ::windows::core::Result<super::IWICBitmapDecoder>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateDecoderFromFilename)(::windows::core::Interface::as_raw(self), wzfilename.into(), ::core::mem::transmute(pguidvendor), dwdesiredaccess, metadataoptions, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmapDecoder>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDecoderFromStream<'a, P0>(&self, pistream: P0, pguidvendor: *const ::windows::core::GUID, metadataoptions: super::WICDecodeOptions) -> ::windows::core::Result<super::IWICBitmapDecoder>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateDecoderFromStream)(::windows::core::Interface::as_raw(self), pistream.into().abi(), ::core::mem::transmute(pguidvendor), metadataoptions, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmapDecoder>(result__)
    }
    pub unsafe fn CreateDecoderFromFileHandle(&self, hfile: usize, pguidvendor: *const ::windows::core::GUID, metadataoptions: super::WICDecodeOptions) -> ::windows::core::Result<super::IWICBitmapDecoder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateDecoderFromFileHandle)(::windows::core::Interface::as_raw(self), hfile, ::core::mem::transmute(pguidvendor), metadataoptions, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmapDecoder>(result__)
    }
    pub unsafe fn CreateComponentInfo(&self, clsidcomponent: *const ::windows::core::GUID) -> ::windows::core::Result<super::IWICComponentInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateComponentInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(clsidcomponent), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICComponentInfo>(result__)
    }
    pub unsafe fn CreateDecoder(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<super::IWICBitmapDecoder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateDecoder)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guidcontainerformat), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmapDecoder>(result__)
    }
    pub unsafe fn CreateEncoder(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<super::IWICBitmapEncoder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateEncoder)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guidcontainerformat), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmapEncoder>(result__)
    }
    pub unsafe fn CreatePalette(&self) -> ::windows::core::Result<super::IWICPalette> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreatePalette)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICPalette>(result__)
    }
    pub unsafe fn CreateFormatConverter(&self) -> ::windows::core::Result<super::IWICFormatConverter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateFormatConverter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICFormatConverter>(result__)
    }
    pub unsafe fn CreateBitmapScaler(&self) -> ::windows::core::Result<super::IWICBitmapScaler> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateBitmapScaler)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmapScaler>(result__)
    }
    pub unsafe fn CreateBitmapClipper(&self) -> ::windows::core::Result<super::IWICBitmapClipper> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateBitmapClipper)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmapClipper>(result__)
    }
    pub unsafe fn CreateBitmapFlipRotator(&self) -> ::windows::core::Result<super::IWICBitmapFlipRotator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateBitmapFlipRotator)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmapFlipRotator>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStream(&self) -> ::windows::core::Result<super::IWICStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateStream)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICStream>(result__)
    }
    pub unsafe fn CreateColorContext(&self) -> ::windows::core::Result<super::IWICColorContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateColorContext)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICColorContext>(result__)
    }
    pub unsafe fn CreateColorTransformer(&self) -> ::windows::core::Result<super::IWICColorTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateColorTransformer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICColorTransform>(result__)
    }
    pub unsafe fn CreateBitmap(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, option: super::WICBitmapCreateCacheOption) -> ::windows::core::Result<super::IWICBitmap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateBitmap)(::windows::core::Interface::as_raw(self), uiwidth, uiheight, ::core::mem::transmute(pixelformat), option, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmap>(result__)
    }
    pub unsafe fn CreateBitmapFromSource<'a, P0>(&self, pibitmapsource: P0, option: super::WICBitmapCreateCacheOption) -> ::windows::core::Result<super::IWICBitmap>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateBitmapFromSource)(::windows::core::Interface::as_raw(self), pibitmapsource.into().abi(), option, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmap>(result__)
    }
    pub unsafe fn CreateBitmapFromSourceRect<'a, P0>(&self, pibitmapsource: P0, x: u32, y: u32, width: u32, height: u32) -> ::windows::core::Result<super::IWICBitmap>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateBitmapFromSourceRect)(::windows::core::Interface::as_raw(self), pibitmapsource.into().abi(), x, y, width, height, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmap>(result__)
    }
    pub unsafe fn CreateBitmapFromMemory(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, cbstride: u32, pbbuffer: &[u8]) -> ::windows::core::Result<super::IWICBitmap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateBitmapFromMemory)(::windows::core::Interface::as_raw(self), uiwidth, uiheight, ::core::mem::transmute(pixelformat), cbstride, pbbuffer.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbbuffer)), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmap>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateBitmapFromHBITMAP<'a, P0, P1>(&self, hbitmap: P0, hpalette: P1, options: super::WICBitmapAlphaChannelOption) -> ::windows::core::Result<super::IWICBitmap>
    where
        P0: ::std::convert::Into<super::super::Gdi::HBITMAP>,
        P1: ::std::convert::Into<super::super::Gdi::HPALETTE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateBitmapFromHBITMAP)(::windows::core::Interface::as_raw(self), hbitmap.into(), hpalette.into(), options, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmap>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn CreateBitmapFromHICON<'a, P0>(&self, hicon: P0) -> ::windows::core::Result<super::IWICBitmap>
    where
        P0: ::std::convert::Into<super::super::super::UI::WindowsAndMessaging::HICON>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateBitmapFromHICON)(::windows::core::Interface::as_raw(self), hicon.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICBitmap>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateComponentEnumerator(&self, componenttypes: u32, options: u32) -> ::windows::core::Result<super::super::super::System::Com::IEnumUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateComponentEnumerator)(::windows::core::Interface::as_raw(self), componenttypes, options, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::System::Com::IEnumUnknown>(result__)
    }
    pub unsafe fn CreateFastMetadataEncoderFromDecoder<'a, P0>(&self, pidecoder: P0) -> ::windows::core::Result<super::IWICFastMetadataEncoder>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IWICBitmapDecoder>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateFastMetadataEncoderFromDecoder)(::windows::core::Interface::as_raw(self), pidecoder.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICFastMetadataEncoder>(result__)
    }
    pub unsafe fn CreateFastMetadataEncoderFromFrameDecode<'a, P0>(&self, piframedecoder: P0) -> ::windows::core::Result<super::IWICFastMetadataEncoder>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IWICBitmapFrameDecode>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateFastMetadataEncoderFromFrameDecode)(::windows::core::Interface::as_raw(self), piframedecoder.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICFastMetadataEncoder>(result__)
    }
    pub unsafe fn CreateQueryWriter(&self, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<super::IWICMetadataQueryWriter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateQueryWriter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(guidmetadataformat), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICMetadataQueryWriter>(result__)
    }
    pub unsafe fn CreateQueryWriterFromReader<'a, P0>(&self, piqueryreader: P0, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<super::IWICMetadataQueryWriter>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::IWICMetadataQueryReader>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateQueryWriterFromReader)(::windows::core::Interface::as_raw(self), piqueryreader.into().abi(), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::IWICMetadataQueryWriter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn CreateImageEncoder<'a, P0>(&self, pd2ddevice: P0) -> ::windows::core::Result<IWICImageEncoder>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Direct2D::ID2D1Device>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateImageEncoder)(::windows::core::Interface::as_raw(self), pd2ddevice.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IWICImageEncoder>(result__)
    }
}
impl ::core::convert::From<IWICImagingFactory2> for ::windows::core::IUnknown {
    fn from(value: IWICImagingFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWICImagingFactory2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWICImagingFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICImagingFactory2> for ::windows::core::IUnknown {
    fn from(value: &IWICImagingFactory2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWICImagingFactory2> for super::IWICImagingFactory {
    fn from(value: IWICImagingFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWICImagingFactory2> for &'a super::IWICImagingFactory {
    fn from(value: &'a IWICImagingFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICImagingFactory2> for super::IWICImagingFactory {
    fn from(value: &IWICImagingFactory2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWICImagingFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWICImagingFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICImagingFactory2 {}
impl ::core::fmt::Debug for IWICImagingFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICImagingFactory2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IWICImagingFactory2 {
    type Vtable = IWICImagingFactory2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b816b45_1996_4476_b132_de9e247c8af0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICImagingFactory2_Vtbl {
    pub base__: super::IWICImagingFactory_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub CreateImageEncoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pd2ddevice: *mut ::core::ffi::c_void, ppwicimageencoder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))]
    CreateImageEncoder: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
