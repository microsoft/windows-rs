#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWICImageEncoder(pub ::windows::runtime::IUnknown);
impl IWICImageEncoder {
    #[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`, `Win32_Graphics_Direct2D`, `Win32_Graphics_Direct2D_Common`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn WriteFrame<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Direct2D::ID2D1Image>, Param1: ::windows::runtime::IntoParam<'a, super::IWICBitmapFrameEncode>>(&self, pimage: Param0, pframeencode: Param1, pimageparameters: *const super::WICImageParameters) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pimage.into_param().abi(), pframeencode.into_param().abi(), ::core::mem::transmute(pimageparameters)).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`, `Win32_Graphics_Direct2D`, `Win32_Graphics_Direct2D_Common`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn WriteFrameThumbnail<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Direct2D::ID2D1Image>, Param1: ::windows::runtime::IntoParam<'a, super::IWICBitmapFrameEncode>>(&self, pimage: Param0, pframeencode: Param1, pimageparameters: *const super::WICImageParameters) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pimage.into_param().abi(), pframeencode.into_param().abi(), ::core::mem::transmute(pimageparameters)).ok()
    }
    #[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`, `Win32_Graphics_Direct2D`, `Win32_Graphics_Direct2D_Common`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn WriteThumbnail<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Direct2D::ID2D1Image>, Param1: ::windows::runtime::IntoParam<'a, super::IWICBitmapEncoder>>(&self, pimage: Param0, pencoder: Param1, pimageparameters: *const super::WICImageParameters) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pimage.into_param().abi(), pencoder.into_param().abi(), ::core::mem::transmute(pimageparameters)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IWICImageEncoder {
    type Vtable = IWICImageEncoder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x04c75bf8_3ce1_473b_acc5_3cc4f5e94999);
}
impl ::core::convert::From<IWICImageEncoder> for ::windows::runtime::IUnknown {
    fn from(value: IWICImageEncoder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWICImageEncoder> for ::windows::runtime::IUnknown {
    fn from(value: &IWICImageEncoder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWICImageEncoder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWICImageEncoder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICImageEncoder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pimage: ::windows::runtime::RawPtr, pframeencode: ::windows::runtime::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))] usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pimage: ::windows::runtime::RawPtr, pframeencode: ::windows::runtime::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))] usize,
    #[cfg(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pimage: ::windows::runtime::RawPtr, pencoder: ::windows::runtime::RawPtr, pimageparameters: *const super::WICImageParameters) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common")))] usize,
);
#[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IWICImagingFactory2(pub ::windows::runtime::IUnknown);
impl IWICImagingFactory2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`, `Win32_Foundation`*"]
    pub unsafe fn CreateDecoderFromFilename<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, wzfilename: Param0, pguidvendor: *const ::windows::runtime::GUID, dwdesiredaccess: u32, metadataoptions: super::WICDecodeOptions) -> ::windows::runtime::Result<super::IWICBitmapDecoder> {
        let mut result__: <super::IWICBitmapDecoder as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), wzfilename.into_param().abi(), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(dwdesiredaccess), ::core::mem::transmute(metadataoptions), &mut result__).from_abi::<super::IWICBitmapDecoder>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`, `Win32_System_Com`*"]
    pub unsafe fn CreateDecoderFromStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::System::Com::IStream>>(&self, pistream: Param0, pguidvendor: *const ::windows::runtime::GUID, metadataoptions: super::WICDecodeOptions) -> ::windows::runtime::Result<super::IWICBitmapDecoder> {
        let mut result__: <super::IWICBitmapDecoder as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pistream.into_param().abi(), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(metadataoptions), &mut result__).from_abi::<super::IWICBitmapDecoder>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateDecoderFromFileHandle(&self, hfile: usize, pguidvendor: *const ::windows::runtime::GUID, metadataoptions: super::WICDecodeOptions) -> ::windows::runtime::Result<super::IWICBitmapDecoder> {
        let mut result__: <super::IWICBitmapDecoder as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(hfile), ::core::mem::transmute(pguidvendor), ::core::mem::transmute(metadataoptions), &mut result__).from_abi::<super::IWICBitmapDecoder>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateComponentInfo(&self, clsidcomponent: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::IWICComponentInfo> {
        let mut result__: <super::IWICComponentInfo as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(clsidcomponent), &mut result__).from_abi::<super::IWICComponentInfo>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateDecoder(&self, guidcontainerformat: *const ::windows::runtime::GUID, pguidvendor: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::IWICBitmapDecoder> {
        let mut result__: <super::IWICBitmapDecoder as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidcontainerformat), ::core::mem::transmute(pguidvendor), &mut result__).from_abi::<super::IWICBitmapDecoder>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateEncoder(&self, guidcontainerformat: *const ::windows::runtime::GUID, pguidvendor: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::IWICBitmapEncoder> {
        let mut result__: <super::IWICBitmapEncoder as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidcontainerformat), ::core::mem::transmute(pguidvendor), &mut result__).from_abi::<super::IWICBitmapEncoder>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreatePalette(&self) -> ::windows::runtime::Result<super::IWICPalette> {
        let mut result__: <super::IWICPalette as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::IWICPalette>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateFormatConverter(&self) -> ::windows::runtime::Result<super::IWICFormatConverter> {
        let mut result__: <super::IWICFormatConverter as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::IWICFormatConverter>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateBitmapScaler(&self) -> ::windows::runtime::Result<super::IWICBitmapScaler> {
        let mut result__: <super::IWICBitmapScaler as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::IWICBitmapScaler>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateBitmapClipper(&self) -> ::windows::runtime::Result<super::IWICBitmapClipper> {
        let mut result__: <super::IWICBitmapClipper as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::IWICBitmapClipper>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateBitmapFlipRotator(&self) -> ::windows::runtime::Result<super::IWICBitmapFlipRotator> {
        let mut result__: <super::IWICBitmapFlipRotator as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::IWICBitmapFlipRotator>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateStream(&self) -> ::windows::runtime::Result<super::IWICStream> {
        let mut result__: <super::IWICStream as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::IWICStream>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateColorContext(&self) -> ::windows::runtime::Result<super::IWICColorContext> {
        let mut result__: <super::IWICColorContext as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::IWICColorContext>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateColorTransformer(&self) -> ::windows::runtime::Result<super::IWICColorTransform> {
        let mut result__: <super::IWICColorTransform as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::IWICColorTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateBitmap(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::runtime::GUID, option: super::WICBitmapCreateCacheOption) -> ::windows::runtime::Result<super::IWICBitmap> {
        let mut result__: <super::IWICBitmap as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiwidth), ::core::mem::transmute(uiheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(option), &mut result__).from_abi::<super::IWICBitmap>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateBitmapFromSource<'a, Param0: ::windows::runtime::IntoParam<'a, super::IWICBitmapSource>>(&self, pibitmapsource: Param0, option: super::WICBitmapCreateCacheOption) -> ::windows::runtime::Result<super::IWICBitmap> {
        let mut result__: <super::IWICBitmap as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), pibitmapsource.into_param().abi(), ::core::mem::transmute(option), &mut result__).from_abi::<super::IWICBitmap>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateBitmapFromSourceRect<'a, Param0: ::windows::runtime::IntoParam<'a, super::IWICBitmapSource>>(&self, pibitmapsource: Param0, x: u32, y: u32, width: u32, height: u32) -> ::windows::runtime::Result<super::IWICBitmap> {
        let mut result__: <super::IWICBitmap as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), pibitmapsource.into_param().abi(), ::core::mem::transmute(x), ::core::mem::transmute(y), ::core::mem::transmute(width), ::core::mem::transmute(height), &mut result__).from_abi::<super::IWICBitmap>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateBitmapFromMemory(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::runtime::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8) -> ::windows::runtime::Result<super::IWICBitmap> {
        let mut result__: <super::IWICBitmap as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(uiwidth), ::core::mem::transmute(uiheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(cbstride), ::core::mem::transmute(cbbuffersize), ::core::mem::transmute(pbbuffer), &mut result__).from_abi::<super::IWICBitmap>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn CreateBitmapFromHBITMAP<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Gdi::HBITMAP>, Param1: ::windows::runtime::IntoParam<'a, super::super::Gdi::HPALETTE>>(&self, hbitmap: Param0, hpalette: Param1, options: super::WICBitmapAlphaChannelOption) -> ::windows::runtime::Result<super::IWICBitmap> {
        let mut result__: <super::IWICBitmap as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), hbitmap.into_param().abi(), hpalette.into_param().abi(), ::core::mem::transmute(options), &mut result__).from_abi::<super::IWICBitmap>(result__)
    }
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn CreateBitmapFromHICON<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::UI::WindowsAndMessaging::HICON>>(&self, hicon: Param0) -> ::windows::runtime::Result<super::IWICBitmap> {
        let mut result__: <super::IWICBitmap as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), hicon.into_param().abi(), &mut result__).from_abi::<super::IWICBitmap>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`, `Win32_System_Com`*"]
    pub unsafe fn CreateComponentEnumerator(&self, componenttypes: u32, options: u32) -> ::windows::runtime::Result<super::super::super::System::Com::IEnumUnknown> {
        let mut result__: <super::super::super::System::Com::IEnumUnknown as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), ::core::mem::transmute(componenttypes), ::core::mem::transmute(options), &mut result__).from_abi::<super::super::super::System::Com::IEnumUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateFastMetadataEncoderFromDecoder<'a, Param0: ::windows::runtime::IntoParam<'a, super::IWICBitmapDecoder>>(&self, pidecoder: Param0) -> ::windows::runtime::Result<super::IWICFastMetadataEncoder> {
        let mut result__: <super::IWICFastMetadataEncoder as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), pidecoder.into_param().abi(), &mut result__).from_abi::<super::IWICFastMetadataEncoder>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateFastMetadataEncoderFromFrameDecode<'a, Param0: ::windows::runtime::IntoParam<'a, super::IWICBitmapFrameDecode>>(&self, piframedecoder: Param0) -> ::windows::runtime::Result<super::IWICFastMetadataEncoder> {
        let mut result__: <super::IWICFastMetadataEncoder as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), piframedecoder.into_param().abi(), &mut result__).from_abi::<super::IWICFastMetadataEncoder>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateQueryWriter(&self, guidmetadataformat: *const ::windows::runtime::GUID, pguidvendor: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::IWICMetadataQueryWriter> {
        let mut result__: <super::IWICMetadataQueryWriter as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(guidmetadataformat), ::core::mem::transmute(pguidvendor), &mut result__).from_abi::<super::IWICMetadataQueryWriter>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`*"]
    pub unsafe fn CreateQueryWriterFromReader<'a, Param0: ::windows::runtime::IntoParam<'a, super::IWICMetadataQueryReader>>(&self, piqueryreader: Param0, pguidvendor: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<super::IWICMetadataQueryWriter> {
        let mut result__: <super::IWICMetadataQueryWriter as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::core::mem::transmute_copy(self), piqueryreader.into_param().abi(), ::core::mem::transmute(pguidvendor), &mut result__).from_abi::<super::IWICMetadataQueryWriter>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    #[doc = "*Required features: `Win32_Graphics_Imaging_D2D`, `Win32_Graphics_Direct2D`*"]
    pub unsafe fn CreateImageEncoder<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Direct2D::ID2D1Device>>(&self, pd2ddevice: Param0) -> ::windows::runtime::Result<IWICImageEncoder> {
        let mut result__: <IWICImageEncoder as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::core::mem::transmute_copy(self), pd2ddevice.into_param().abi(), &mut result__).from_abi::<IWICImageEncoder>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWICImagingFactory2 {
    type Vtable = IWICImagingFactory2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7b816b45_1996_4476_b132_de9e247c8af0);
}
impl ::core::convert::From<IWICImagingFactory2> for ::windows::runtime::IUnknown {
    fn from(value: IWICImagingFactory2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IWICImagingFactory2> for ::windows::runtime::IUnknown {
    fn from(value: &IWICImagingFactory2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IWICImagingFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IWICImagingFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, super::IWICImagingFactory> for IWICImagingFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IWICImagingFactory> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IWICImagingFactory> for &IWICImagingFactory2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IWICImagingFactory> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICImagingFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wzfilename: super::super::super::Foundation::PWSTR, pguidvendor: *const ::windows::runtime::GUID, dwdesiredaccess: u32, metadataoptions: super::WICDecodeOptions, ppidecoder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pistream: ::windows::runtime::RawPtr, pguidvendor: *const ::windows::runtime::GUID, metadataoptions: super::WICDecodeOptions, ppidecoder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hfile: usize, pguidvendor: *const ::windows::runtime::GUID, metadataoptions: super::WICDecodeOptions, ppidecoder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clsidcomponent: *const ::windows::runtime::GUID, ppiinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidcontainerformat: *const ::windows::runtime::GUID, pguidvendor: *const ::windows::runtime::GUID, ppidecoder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidcontainerformat: *const ::windows::runtime::GUID, pguidvendor: *const ::windows::runtime::GUID, ppiencoder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppipalette: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppiformatconverter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppibitmapscaler: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppibitmapclipper: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppibitmapfliprotator: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppiwicstream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppiwiccolorcontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppiwiccolortransform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::runtime::GUID, option: super::WICBitmapCreateCacheOption, ppibitmap: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pibitmapsource: ::windows::runtime::RawPtr, option: super::WICBitmapCreateCacheOption, ppibitmap: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pibitmapsource: ::windows::runtime::RawPtr, x: u32, y: u32, width: u32, height: u32, ppibitmap: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::runtime::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8, ppibitmap: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hbitmap: super::super::Gdi::HBITMAP, hpalette: super::super::Gdi::HPALETTE, options: super::WICBitmapAlphaChannelOption, ppibitmap: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hicon: super::super::super::UI::WindowsAndMessaging::HICON, ppibitmap: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, componenttypes: u32, options: u32, ppienumunknown: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pidecoder: ::windows::runtime::RawPtr, ppifastencoder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piframedecoder: ::windows::runtime::RawPtr, ppifastencoder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guidmetadataformat: *const ::windows::runtime::GUID, pguidvendor: *const ::windows::runtime::GUID, ppiquerywriter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piqueryreader: ::windows::runtime::RawPtr, pguidvendor: *const ::windows::runtime::GUID, ppiquerywriter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pd2ddevice: ::windows::runtime::RawPtr, ppwicimageencoder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
);
