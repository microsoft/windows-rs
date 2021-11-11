#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWICImageEncoder(pub ::windows::core::IUnknown);
impl IWICImageEncoder {
    #[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`, `Win32_Graphics_Direct2D`, `Win32_Graphics_Direct2D_Common`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn WriteFrame<'a, Param0: ::windows::core::IntoParam<'a, super::super::Direct2D::ID2D1Image>, Param1: ::windows::core::IntoParam<'a, super::IWICBitmapFrameEncode>>(&self, pimage: Param0, pframeencode: Param1, pimageparameters: *const super::WICImageParameters) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pimage.into_param().abi(), pframeencode.into_param().abi(), ::core::mem::transmute(pimageparameters)).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`, `Win32_Graphics_Direct2D`, `Win32_Graphics_Direct2D_Common`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn WriteFrameThumbnail<'a, Param0: ::windows::core::IntoParam<'a, super::super::Direct2D::ID2D1Image>, Param1: ::windows::core::IntoParam<'a, super::IWICBitmapFrameEncode>>(&self, pimage: Param0, pframeencode: Param1, pimageparameters: *const super::WICImageParameters) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pimage.into_param().abi(), pframeencode.into_param().abi(), ::core::mem::transmute(pimageparameters)).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`, `Win32_Graphics_Direct2D`, `Win32_Graphics_Direct2D_Common`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn WriteThumbnail<'a, Param0: ::windows::core::IntoParam<'a, super::super::Direct2D::ID2D1Image>, Param1: ::windows::core::IntoParam<'a, super::IWICBitmapEncoder>>(&self, pimage: Param0, pencoder: Param1, pimageparameters: *const super::WICImageParameters) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pimage.into_param().abi(), pencoder.into_param().abi(), ::core::mem::transmute(pimageparameters)).ok()
    }
}
unsafe impl ::windows::core::Interface for IWICImageEncoder {
    type Vtable = IWICImageEncoder_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x04c75bf8_3ce1_473b_acc5_3cc4f5e94999);
}
impl ::core::convert::From<IWICImageEncoder> for ::windows::core::IUnknown {
    fn from(value: IWICImageEncoder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWICImageEncoder> for ::windows::core::IUnknown {
    fn from(value: &IWICImageEncoder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICImageEncoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWICImageEncoder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICImageEncoder_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pimage: ::windows::core::RawPtr, pframeencode: ::windows::core::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))] usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pimage: ::windows::core::RawPtr, pframeencode: ::windows::core::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))] usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pimage: ::windows::core::RawPtr, pencoder: ::windows::core::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))] usize,
);
#[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWICImagingFactory2(pub ::windows::core::IUnknown);
impl IWICImagingFactory2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`, `Win32_Foundation`*"]
    pub unsafe fn CreateDecoderFromFilename<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, wzfilename: Param0, pguidvendor: *const ::windows::core::GUID, dwdesiredaccess: u32, metadataoptions: super::WICDecodeOptions) -> ::windows::core::Result<super::IWICBitmapDecoder> {
        let mut result__: <super::IWICBitmapDecoder as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wzfilename.into_param().abi(), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(metadataoptions), &mut result__).from_abi::<super::IWICBitmapDecoder>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`, `Win32_System_Com`*"]
    pub unsafe fn CreateDecoderFromStream<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, pistream: Param0, pguidvendor: *const ::windows::core::GUID, metadataoptions: super::WICDecodeOptions) -> ::windows::core::Result<super::IWICBitmapDecoder> {
        let mut result__: <super::IWICBitmapDecoder as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pistream.into_param().abi(), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(metadataoptions), &mut result__).from_abi::<super::IWICBitmapDecoder>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateDecoderFromFileHandle(&self, hfile: usize, pguidvendor: *const ::windows::core::GUID, metadataoptions: super::WICDecodeOptions) -> ::windows::core::Result<super::IWICBitmapDecoder> {
        let mut result__: <super::IWICBitmapDecoder as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(hfile), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(metadataoptions), &mut result__).from_abi::<super::IWICBitmapDecoder>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateComponentInfo(&self, clsidcomponent: *const ::windows::core::GUID) -> ::windows::core::Result<super::IWICComponentInfo> {
        let mut result__: <super::IWICComponentInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsidcomponent), &mut result__).from_abi::<super::IWICComponentInfo>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateDecoder(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<super::IWICBitmapDecoder> {
        let mut result__: <super::IWICBitmapDecoder as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidcontainerformat), ::core::mem::transmute(pguidvendor), &mut result__).from_abi::<super::IWICBitmapDecoder>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateEncoder(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<super::IWICBitmapEncoder> {
        let mut result__: <super::IWICBitmapEncoder as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidcontainerformat), ::core::mem::transmute(pguidvendor), &mut result__).from_abi::<super::IWICBitmapEncoder>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreatePalette(&self) -> ::windows::core::Result<super::IWICPalette> {
        let mut result__: <super::IWICPalette as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::IWICPalette>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateFormatConverter(&self) -> ::windows::core::Result<super::IWICFormatConverter> {
        let mut result__: <super::IWICFormatConverter as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::IWICFormatConverter>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateBitmapScaler(&self) -> ::windows::core::Result<super::IWICBitmapScaler> {
        let mut result__: <super::IWICBitmapScaler as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::IWICBitmapScaler>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateBitmapClipper(&self) -> ::windows::core::Result<super::IWICBitmapClipper> {
        let mut result__: <super::IWICBitmapClipper as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::IWICBitmapClipper>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateBitmapFlipRotator(&self) -> ::windows::core::Result<super::IWICBitmapFlipRotator> {
        let mut result__: <super::IWICBitmapFlipRotator as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::IWICBitmapFlipRotator>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateStream(&self) -> ::windows::core::Result<super::IWICStream> {
        let mut result__: <super::IWICStream as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::IWICStream>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateColorContext(&self) -> ::windows::core::Result<super::IWICColorContext> {
        let mut result__: <super::IWICColorContext as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::IWICColorContext>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateColorTransformer(&self) -> ::windows::core::Result<super::IWICColorTransform> {
        let mut result__: <super::IWICColorTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::IWICColorTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateBitmap(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, option: super::WICBitmapCreateCacheOption) -> ::windows::core::Result<super::IWICBitmap> {
        let mut result__: <super::IWICBitmap as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiwidth), ::core::mem::transmute(uiheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(option), &mut result__).from_abi::<super::IWICBitmap>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateBitmapFromSource<'a, Param0: ::windows::core::IntoParam<'a, super::IWICBitmapSource>>(&self, pibitmapsource: Param0, option: super::WICBitmapCreateCacheOption) -> ::windows::core::Result<super::IWICBitmap> {
        let mut result__: <super::IWICBitmap as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pibitmapsource.into_param().abi(), ::core::mem::transmute(option), &mut result__).from_abi::<super::IWICBitmap>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateBitmapFromSourceRect<'a, Param0: ::windows::core::IntoParam<'a, super::IWICBitmapSource>>(&self, pibitmapsource: Param0, x: u32, y: u32, width: u32, height: u32) -> ::windows::core::Result<super::IWICBitmap> {
        let mut result__: <super::IWICBitmap as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pibitmapsource.into_param().abi(), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(width), ::core::mem::transmute(height), &mut result__).from_abi::<super::IWICBitmap>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateBitmapFromMemory(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8) -> ::windows::core::Result<super::IWICBitmap> {
        let mut result__: <super::IWICBitmap as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiwidth), ::core::mem::transmute(uiheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(cbstride), ::core::mem::transmute(cbbuffersize), ::core::mem::transmute(pbbuffer), &mut result__).from_abi::<super::IWICBitmap>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn CreateBitmapFromHBITMAP<'a, Param0: ::windows::core::IntoParam<'a, super::super::Gdi::HBITMAP>, Param1: ::windows::core::IntoParam<'a, super::super::Gdi::HPALETTE>>(&self, hbitmap: Param0, hpalette: Param1, options: super::WICBitmapAlphaChannelOption) -> ::windows::core::Result<super::IWICBitmap> {
        let mut result__: <super::IWICBitmap as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), hbitmap.into_param().abi(), hpalette.into_param().abi(), ::core::mem::transmute(options), &mut result__).from_abi::<super::IWICBitmap>(result__)
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn CreateBitmapFromHICON<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::UI::WindowsAndMessaging::HICON>>(&self, hicon: Param0) -> ::windows::core::Result<super::IWICBitmap> {
        let mut result__: <super::IWICBitmap as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), hicon.into_param().abi(), &mut result__).from_abi::<super::IWICBitmap>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`, `Win32_System_Com`*"]
    pub unsafe fn CreateComponentEnumerator(&self, componenttypes: u32, options: u32) -> ::windows::core::Result<super::super::super::System::Com::IEnumUnknown> {
        let mut result__: <super::super::super::System::Com::IEnumUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(componenttypes), ::core::mem::transmute(options), &mut result__).from_abi::<super::super::super::System::Com::IEnumUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateFastMetadataEncoderFromDecoder<'a, Param0: ::windows::core::IntoParam<'a, super::IWICBitmapDecoder>>(&self, pidecoder: Param0) -> ::windows::core::Result<super::IWICFastMetadataEncoder> {
        let mut result__: <super::IWICFastMetadataEncoder as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), pidecoder.into_param().abi(), &mut result__).from_abi::<super::IWICFastMetadataEncoder>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateFastMetadataEncoderFromFrameDecode<'a, Param0: ::windows::core::IntoParam<'a, super::IWICBitmapFrameDecode>>(&self, piframedecoder: Param0) -> ::windows::core::Result<super::IWICFastMetadataEncoder> {
        let mut result__: <super::IWICFastMetadataEncoder as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), piframedecoder.into_param().abi(), &mut result__).from_abi::<super::IWICFastMetadataEncoder>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateQueryWriter(&self, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<super::IWICMetadataQueryWriter> {
        let mut result__: <super::IWICMetadataQueryWriter as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidmetadataformat), ::core::mem::transmute(pguidvendor), &mut result__).from_abi::<super::IWICMetadataQueryWriter>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateQueryWriterFromReader<'a, Param0: ::windows::core::IntoParam<'a, super::IWICMetadataQueryReader>>(&self, piqueryreader: Param0, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<super::IWICMetadataQueryWriter> {
        let mut result__: <super::IWICMetadataQueryWriter as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), piqueryreader.into_param().abi(), ::core::mem::transmute(pguidvendor), &mut result__).from_abi::<super::IWICMetadataQueryWriter>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`, `Win32_Graphics_Direct2D`*"]
    pub unsafe fn CreateImageEncoder<'a, Param0: ::windows::core::IntoParam<'a, super::super::Direct2D::ID2D1Device>>(&self, pd2ddevice: Param0) -> ::windows::core::Result<IWICImageEncoder> {
        let mut result__: <IWICImageEncoder as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), pd2ddevice.into_param().abi(), &mut result__).from_abi::<IWICImageEncoder>(result__)
    }
}
unsafe impl ::windows::core::Interface for IWICImagingFactory2 {
    type Vtable = IWICImagingFactory2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b816b45_1996_4476_b132_de9e247c8af0);
}
impl ::core::convert::From<IWICImagingFactory2> for ::windows::core::IUnknown {
    fn from(value: IWICImagingFactory2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWICImagingFactory2> for ::windows::core::IUnknown {
    fn from(value: &IWICImagingFactory2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWICImagingFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWICImagingFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, super::IWICImagingFactory> for &IWICImagingFactory2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::IWICImagingFactory> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICImagingFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, wzfilename: super::super::super::Foundation::PWSTR, pguidvendor: *const ::windows::core::GUID, dwdesiredaccess: u32, metadataoptions: super::WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pistream: ::windows::core::RawPtr, pguidvendor: *const ::windows::core::GUID, metadataoptions: super::WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hfile: usize, pguidvendor: *const ::windows::core::GUID, metadataoptions: super::WICDecodeOptions, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clsidcomponent: *const ::windows::core::GUID, ppiinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppidecoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppiencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppipalette: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppiformatconverter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppibitmapscaler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppibitmapclipper: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppibitmapfliprotator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppiwicstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppiwiccolorcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppiwiccolortransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, option: super::WICBitmapCreateCacheOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pibitmapsource: ::windows::core::RawPtr, option: super::WICBitmapCreateCacheOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pibitmapsource: ::windows::core::RawPtr, x: u32, y: u32, width: u32, height: u32, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hbitmap: super::super::Gdi::HBITMAP, hpalette: super::super::Gdi::HPALETTE, options: super::WICBitmapAlphaChannelOption, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hicon: super::super::super::UI::WindowsAndMessaging::HICON, ppibitmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, componenttypes: u32, options: u32, ppienumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pidecoder: ::windows::core::RawPtr, ppifastencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, piframedecoder: ::windows::core::RawPtr, ppifastencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppiquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, piqueryreader: ::windows::core::RawPtr, pguidvendor: *const ::windows::core::GUID, ppiquerywriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pd2ddevice: ::windows::core::RawPtr, ppwicimageencoder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
);
