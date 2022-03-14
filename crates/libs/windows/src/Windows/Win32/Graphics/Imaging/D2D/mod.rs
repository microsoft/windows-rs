#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
#[repr(transparent)]
pub struct IWICImageEncoder(::windows::core::IUnknown);
impl IWICImageEncoder {
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn WriteFrame<'a, Param0: ::windows::core::IntoParam<'a, super::super::Direct2D::ID2D1Image>, Param1: ::windows::core::IntoParam<'a, super::IWICBitmapFrameEncode>>(&self, pimage: Param0, pframeencode: Param1, pimageparameters: *const super::WICImageParameters) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteFrame)(::core::mem::transmute_copy(self), pimage.into_param().abi(), pframeencode.into_param().abi(), ::core::mem::transmute(pimageparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn WriteFrameThumbnail<'a, Param0: ::windows::core::IntoParam<'a, super::super::Direct2D::ID2D1Image>, Param1: ::windows::core::IntoParam<'a, super::IWICBitmapFrameEncode>>(&self, pimage: Param0, pframeencode: Param1, pimageparameters: *const super::WICImageParameters) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteFrameThumbnail)(::core::mem::transmute_copy(self), pimage.into_param().abi(), pframeencode.into_param().abi(), ::core::mem::transmute(pimageparameters)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn WriteThumbnail<'a, Param0: ::windows::core::IntoParam<'a, super::super::Direct2D::ID2D1Image>, Param1: ::windows::core::IntoParam<'a, super::IWICBitmapEncoder>>(&self, pimage: Param0, pencoder: Param1, pimageparameters: *const super::WICImageParameters) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteThumbnail)(::core::mem::transmute_copy(self), pimage.into_param().abi(), pencoder.into_param().abi(), ::core::mem::transmute(pimageparameters)).ok()
    }
}
impl ::core::convert::From<IWICImageEncoder> for ::windows::core::IUnknown {
    fn from(value: IWICImageEncoder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICImageEncoder> for ::windows::core::IUnknown {
    fn from(value: &IWICImageEncoder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICImageEncoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWICImageEncoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub WriteFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimage: ::windows::core::RawPtr, pframeencode: ::windows::core::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    WriteFrame: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub WriteFrameThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimage: ::windows::core::RawPtr, pframeencode: ::windows::core::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    WriteFrameThumbnail: usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    pub WriteThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimage: ::windows::core::RawPtr, pencoder: ::windows::core::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))]
    WriteThumbnail: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
#[repr(transparent)]
pub struct IWICImagingFactory2(::windows::core::IUnknown);
impl IWICImagingFactory2 {
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
    pub unsafe fn CreateDecoderFromFilename<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, wzfilename: Param0, pguidvendor: *const ::windows::core::GUID, dwdesiredaccess: u32, metadataoptions: super::WICDecodeOptions) -> ::windows::core::Result<super::IWICBitmapDecoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateDecoderFromFilename)(::core::mem::transmute_copy(self), wzfilename.into_param().abi(), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(metadataoptions), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICBitmapDecoder>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDecoderFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, pistream: Param0, pguidvendor: *const ::windows::core::GUID, metadataoptions: super::WICDecodeOptions) -> ::windows::core::Result<super::IWICBitmapDecoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateDecoderFromStream)(::core::mem::transmute_copy(self), pistream.into_param().abi(), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(metadataoptions), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICBitmapDecoder>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
    pub unsafe fn CreateDecoderFromFileHandle(&self, hfile: usize, pguidvendor: *const ::windows::core::GUID, metadataoptions: super::WICDecodeOptions) -> ::windows::core::Result<super::IWICBitmapDecoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateDecoderFromFileHandle)(::core::mem::transmute_copy(self), ::core::mem::transmute(hfile), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(metadataoptions), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICBitmapDecoder>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
    pub unsafe fn CreateComponentInfo(&self, clsidcomponent: *const ::windows::core::GUID) -> ::windows::core::Result<super::IWICComponentInfo> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateComponentInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsidcomponent), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICComponentInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
    pub unsafe fn CreateDecoder(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<super::IWICBitmapDecoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateDecoder)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidcontainerformat), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICBitmapDecoder>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
    pub unsafe fn CreateEncoder(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<super::IWICBitmapEncoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateEncoder)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidcontainerformat), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICBitmapEncoder>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
    pub unsafe fn CreatePalette(&self) -> ::windows::core::Result<super::IWICPalette> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreatePalette)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICPalette>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
    pub unsafe fn CreateFormatConverter(&self) -> ::windows::core::Result<super::IWICFormatConverter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateFormatConverter)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICFormatConverter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
    pub unsafe fn CreateBitmapScaler(&self) -> ::windows::core::Result<super::IWICBitmapScaler> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateBitmapScaler)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICBitmapScaler>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
    pub unsafe fn CreateBitmapClipper(&self) -> ::windows::core::Result<super::IWICBitmapClipper> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateBitmapClipper)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICBitmapClipper>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
    pub unsafe fn CreateBitmapFlipRotator(&self) -> ::windows::core::Result<super::IWICBitmapFlipRotator> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateBitmapFlipRotator)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICBitmapFlipRotator>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStream(&self) -> ::windows::core::Result<super::IWICStream> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateStream)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICStream>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
    pub unsafe fn CreateColorContext(&self) -> ::windows::core::Result<super::IWICColorContext> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateColorContext)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICColorContext>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
    pub unsafe fn CreateColorTransformer(&self) -> ::windows::core::Result<super::IWICColorTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateColorTransformer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICColorTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
    pub unsafe fn CreateBitmap(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, option: super::WICBitmapCreateCacheOption) -> ::windows::core::Result<super::IWICBitmap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateBitmap)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiwidth), ::core::mem::transmute(uiheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(option), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICBitmap>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
    pub unsafe fn CreateBitmapFromSource<'a, Param0: ::windows::core::IntoParam<'a, super::IWICBitmapSource>>(&self, pibitmapsource: Param0, option: super::WICBitmapCreateCacheOption) -> ::windows::core::Result<super::IWICBitmap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateBitmapFromSource)(::core::mem::transmute_copy(self), pibitmapsource.into_param().abi(), ::core::mem::transmute(option), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICBitmap>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
    pub unsafe fn CreateBitmapFromSourceRect<'a, Param0: ::windows::core::IntoParam<'a, super::IWICBitmapSource>>(&self, pibitmapsource: Param0, x: u32, y: u32, width: u32, height: u32) -> ::windows::core::Result<super::IWICBitmap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateBitmapFromSourceRect)(::core::mem::transmute_copy(self), pibitmapsource.into_param().abi(), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICBitmap>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
    pub unsafe fn CreateBitmapFromMemory(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, cbstride: u32, pbbuffer: &[u8]) -> ::windows::core::Result<super::IWICBitmap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateBitmapFromMemory)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiwidth), ::core::mem::transmute(uiheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(cbstride), pbbuffer.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbbuffer)), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICBitmap>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateBitmapFromHBITMAP<'a, Param0: ::windows::core::IntoParam<'a, super::super::Gdi::HBITMAP>, Param1: ::windows::core::IntoParam<'a, super::super::Gdi::HPALETTE>>(&self, hbitmap: Param0, hpalette: Param1, options: super::WICBitmapAlphaChannelOption) -> ::windows::core::Result<super::IWICBitmap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateBitmapFromHBITMAP)(::core::mem::transmute_copy(self), hbitmap.into_param().abi(), hpalette.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICBitmap>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn CreateBitmapFromHICON<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::UI::WindowsAndMessaging::HICON>>(&self, hicon: Param0) -> ::windows::core::Result<super::IWICBitmap> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateBitmapFromHICON)(::core::mem::transmute_copy(self), hicon.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICBitmap>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateComponentEnumerator(&self, componenttypes: u32, options: u32) -> ::windows::core::Result<super::super::super::System::Com::IEnumUnknown> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateComponentEnumerator)(::core::mem::transmute_copy(self), ::core::mem::transmute(componenttypes), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::System::Com::IEnumUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
    pub unsafe fn CreateFastMetadataEncoderFromDecoder<'a, Param0: ::windows::core::IntoParam<'a, super::IWICBitmapDecoder>>(&self, pidecoder: Param0) -> ::windows::core::Result<super::IWICFastMetadataEncoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateFastMetadataEncoderFromDecoder)(::core::mem::transmute_copy(self), pidecoder.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICFastMetadataEncoder>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
    pub unsafe fn CreateFastMetadataEncoderFromFrameDecode<'a, Param0: ::windows::core::IntoParam<'a, super::IWICBitmapFrameDecode>>(&self, piframedecoder: Param0) -> ::windows::core::Result<super::IWICFastMetadataEncoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateFastMetadataEncoderFromFrameDecode)(::core::mem::transmute_copy(self), piframedecoder.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICFastMetadataEncoder>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
    pub unsafe fn CreateQueryWriter(&self, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<super::IWICMetadataQueryWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateQueryWriter)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidmetadataformat), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICMetadataQueryWriter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`*"]
    pub unsafe fn CreateQueryWriterFromReader<'a, Param0: ::windows::core::IntoParam<'a, super::IWICMetadataQueryReader>>(&self, piqueryreader: Param0, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<super::IWICMetadataQueryWriter> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateQueryWriterFromReader)(::core::mem::transmute_copy(self), piqueryreader.into_param().abi(), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(&mut result__)).from_abi::<super::IWICMetadataQueryWriter>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Imaging_D2D\"`, `\"Win32_Graphics_Direct2D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn CreateImageEncoder<'a, Param0: ::windows::core::IntoParam<'a, super::super::Direct2D::ID2D1Device>>(&self, pd2ddevice: Param0) -> ::windows::core::Result<IWICImageEncoder> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateImageEncoder)(::core::mem::transmute_copy(self), pd2ddevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IWICImageEncoder>(result__)
    }
}
impl ::core::convert::From<IWICImagingFactory2> for ::windows::core::IUnknown {
    fn from(value: IWICImagingFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICImagingFactory2> for ::windows::core::IUnknown {
    fn from(value: &IWICImagingFactory2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICImagingFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWICImagingFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWICImagingFactory2> for super::IWICImagingFactory {
    fn from(value: IWICImagingFactory2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWICImagingFactory2> for super::IWICImagingFactory {
    fn from(value: &IWICImagingFactory2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IWICImagingFactory> for IWICImagingFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::IWICImagingFactory> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IWICImagingFactory> for &'a IWICImagingFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::IWICImagingFactory> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
    pub base: super::IWICImagingFactory_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub CreateImageEncoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pd2ddevice: ::windows::core::RawPtr, ppwicimageencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))]
    CreateImageEncoder: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
