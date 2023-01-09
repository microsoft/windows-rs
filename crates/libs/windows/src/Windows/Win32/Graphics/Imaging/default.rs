impl ::core::cmp::PartialEq for IWICBitmap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmap {}
impl ::core::fmt::Debug for IWICBitmap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICBitmap").field(&self.0).finish()
    }
}
impl IWICBitmap {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSize)(::windows::core::Vtable::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPixelFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetResolution)(::windows::core::Vtable::as_raw(self), pdpix, pdpiy).ok()
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWICPalette>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyPalette)(::windows::core::Vtable::as_raw(self), pipalette.into().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CopyPixels)(::windows::core::Vtable::as_raw(self), prc, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
}
impl ::core::cmp::PartialEq for IWICBitmapClipper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapClipper {}
impl ::core::fmt::Debug for IWICBitmapClipper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICBitmapClipper").field(&self.0).finish()
    }
}
impl IWICBitmapClipper {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSize)(::windows::core::Vtable::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPixelFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetResolution)(::windows::core::Vtable::as_raw(self), pdpix, pdpiy).ok()
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWICPalette>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyPalette)(::windows::core::Vtable::as_raw(self), pipalette.into().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CopyPixels)(::windows::core::Vtable::as_raw(self), prc, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
}
impl ::core::cmp::PartialEq for IWICBitmapCodecInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapCodecInfo {}
impl ::core::fmt::Debug for IWICBitmapCodecInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICBitmapCodecInfo").field(&self.0).finish()
    }
}
impl IWICBitmapCodecInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetComponentType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCLSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSigningStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAuthor(&self, wzauthor: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAuthor)(::windows::core::Vtable::as_raw(self), wzauthor.len() as _, ::core::mem::transmute(wzauthor.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVendorGUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self, wzversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetVersion)(::windows::core::Vtable::as_raw(self), wzversion.len() as _, ::core::mem::transmute(wzversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetSpecVersion(&self, wzspecversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSpecVersion)(::windows::core::Vtable::as_raw(self), wzspecversion.len() as _, ::core::mem::transmute(wzspecversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFriendlyName(&self, wzfriendlyname: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFriendlyName)(::windows::core::Vtable::as_raw(self), wzfriendlyname.len() as _, ::core::mem::transmute(wzfriendlyname.as_ptr()), pcchactual).ok()
    }
}
impl ::core::cmp::PartialEq for IWICBitmapCodecProgressNotification {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapCodecProgressNotification {}
impl ::core::fmt::Debug for IWICBitmapCodecProgressNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICBitmapCodecProgressNotification").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICBitmapDecoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapDecoder {}
impl ::core::fmt::Debug for IWICBitmapDecoder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICBitmapDecoder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICBitmapDecoderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapDecoderInfo {}
impl ::core::fmt::Debug for IWICBitmapDecoderInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICBitmapDecoderInfo").field(&self.0).finish()
    }
}
impl IWICBitmapDecoderInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetComponentType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCLSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSigningStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAuthor(&self, wzauthor: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAuthor)(::windows::core::Vtable::as_raw(self), wzauthor.len() as _, ::core::mem::transmute(wzauthor.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetVendorGUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self, wzversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetVersion)(::windows::core::Vtable::as_raw(self), wzversion.len() as _, ::core::mem::transmute(wzversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetSpecVersion(&self, wzspecversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSpecVersion)(::windows::core::Vtable::as_raw(self), wzspecversion.len() as _, ::core::mem::transmute(wzspecversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFriendlyName(&self, wzfriendlyname: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFriendlyName)(::windows::core::Vtable::as_raw(self), wzfriendlyname.len() as _, ::core::mem::transmute(wzfriendlyname.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContainerFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPixelFormats(&self, pguidpixelformats: &mut [::windows::core::GUID], pcactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPixelFormats)(::windows::core::Vtable::as_raw(self), pguidpixelformats.len() as _, ::core::mem::transmute(pguidpixelformats.as_ptr()), pcactual).ok()
    }
    pub unsafe fn GetColorManagementVersion(&self, wzcolormanagementversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetColorManagementVersion)(::windows::core::Vtable::as_raw(self), wzcolormanagementversion.len() as _, ::core::mem::transmute(wzcolormanagementversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceManufacturer(&self, wzdevicemanufacturer: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceManufacturer)(::windows::core::Vtable::as_raw(self), wzdevicemanufacturer.len() as _, ::core::mem::transmute(wzdevicemanufacturer.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceModels(&self, wzdevicemodels: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceModels)(::windows::core::Vtable::as_raw(self), wzdevicemodels.len() as _, ::core::mem::transmute(wzdevicemodels.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetMimeTypes(&self, wzmimetypes: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetMimeTypes)(::windows::core::Vtable::as_raw(self), wzmimetypes.len() as _, ::core::mem::transmute(wzmimetypes.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFileExtensions(&self, wzfileextensions: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFileExtensions)(::windows::core::Vtable::as_raw(self), wzfileextensions.len() as _, ::core::mem::transmute(wzfileextensions.as_ptr()), pcchactual).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportAnimation(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DoesSupportAnimation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportChromakey(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DoesSupportChromakey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportLossless(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DoesSupportLossless)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportMultiframe(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DoesSupportMultiframe)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchesMimeType<P0>(&self, wzmimetype: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MatchesMimeType)(::windows::core::Vtable::as_raw(self), wzmimetype.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWICBitmapEncoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapEncoder {}
impl ::core::fmt::Debug for IWICBitmapEncoder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICBitmapEncoder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICBitmapEncoderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapEncoderInfo {}
impl ::core::fmt::Debug for IWICBitmapEncoderInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICBitmapEncoderInfo").field(&self.0).finish()
    }
}
impl IWICBitmapEncoderInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetComponentType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCLSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSigningStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAuthor(&self, wzauthor: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAuthor)(::windows::core::Vtable::as_raw(self), wzauthor.len() as _, ::core::mem::transmute(wzauthor.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetVendorGUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self, wzversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetVersion)(::windows::core::Vtable::as_raw(self), wzversion.len() as _, ::core::mem::transmute(wzversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetSpecVersion(&self, wzspecversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSpecVersion)(::windows::core::Vtable::as_raw(self), wzspecversion.len() as _, ::core::mem::transmute(wzspecversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFriendlyName(&self, wzfriendlyname: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFriendlyName)(::windows::core::Vtable::as_raw(self), wzfriendlyname.len() as _, ::core::mem::transmute(wzfriendlyname.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContainerFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPixelFormats(&self, pguidpixelformats: &mut [::windows::core::GUID], pcactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPixelFormats)(::windows::core::Vtable::as_raw(self), pguidpixelformats.len() as _, ::core::mem::transmute(pguidpixelformats.as_ptr()), pcactual).ok()
    }
    pub unsafe fn GetColorManagementVersion(&self, wzcolormanagementversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetColorManagementVersion)(::windows::core::Vtable::as_raw(self), wzcolormanagementversion.len() as _, ::core::mem::transmute(wzcolormanagementversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceManufacturer(&self, wzdevicemanufacturer: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceManufacturer)(::windows::core::Vtable::as_raw(self), wzdevicemanufacturer.len() as _, ::core::mem::transmute(wzdevicemanufacturer.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceModels(&self, wzdevicemodels: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceModels)(::windows::core::Vtable::as_raw(self), wzdevicemodels.len() as _, ::core::mem::transmute(wzdevicemodels.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetMimeTypes(&self, wzmimetypes: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetMimeTypes)(::windows::core::Vtable::as_raw(self), wzmimetypes.len() as _, ::core::mem::transmute(wzmimetypes.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFileExtensions(&self, wzfileextensions: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFileExtensions)(::windows::core::Vtable::as_raw(self), wzfileextensions.len() as _, ::core::mem::transmute(wzfileextensions.as_ptr()), pcchactual).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportAnimation(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DoesSupportAnimation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportChromakey(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DoesSupportChromakey)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportLossless(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DoesSupportLossless)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportMultiframe(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DoesSupportMultiframe)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchesMimeType<P0>(&self, wzmimetype: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.MatchesMimeType)(::windows::core::Vtable::as_raw(self), wzmimetype.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWICBitmapFlipRotator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapFlipRotator {}
impl ::core::fmt::Debug for IWICBitmapFlipRotator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICBitmapFlipRotator").field(&self.0).finish()
    }
}
impl IWICBitmapFlipRotator {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSize)(::windows::core::Vtable::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPixelFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetResolution)(::windows::core::Vtable::as_raw(self), pdpix, pdpiy).ok()
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWICPalette>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyPalette)(::windows::core::Vtable::as_raw(self), pipalette.into().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CopyPixels)(::windows::core::Vtable::as_raw(self), prc, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
}
impl ::core::cmp::PartialEq for IWICBitmapFrameDecode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapFrameDecode {}
impl ::core::fmt::Debug for IWICBitmapFrameDecode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICBitmapFrameDecode").field(&self.0).finish()
    }
}
impl IWICBitmapFrameDecode {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSize)(::windows::core::Vtable::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPixelFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetResolution)(::windows::core::Vtable::as_raw(self), pdpix, pdpiy).ok()
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWICPalette>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyPalette)(::windows::core::Vtable::as_raw(self), pipalette.into().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CopyPixels)(::windows::core::Vtable::as_raw(self), prc, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
}
impl ::core::cmp::PartialEq for IWICBitmapFrameEncode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapFrameEncode {}
impl ::core::fmt::Debug for IWICBitmapFrameEncode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICBitmapFrameEncode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICBitmapLock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapLock {}
impl ::core::fmt::Debug for IWICBitmapLock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICBitmapLock").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICBitmapScaler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapScaler {}
impl ::core::fmt::Debug for IWICBitmapScaler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICBitmapScaler").field(&self.0).finish()
    }
}
impl IWICBitmapScaler {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSize)(::windows::core::Vtable::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPixelFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetResolution)(::windows::core::Vtable::as_raw(self), pdpix, pdpiy).ok()
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWICPalette>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyPalette)(::windows::core::Vtable::as_raw(self), pipalette.into().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CopyPixels)(::windows::core::Vtable::as_raw(self), prc, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
}
impl ::core::cmp::PartialEq for IWICBitmapSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapSource {}
impl ::core::fmt::Debug for IWICBitmapSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICBitmapSource").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICBitmapSourceTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICBitmapSourceTransform {}
impl ::core::fmt::Debug for IWICBitmapSourceTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICBitmapSourceTransform").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICColorContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICColorContext {}
impl ::core::fmt::Debug for IWICColorContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICColorContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICColorTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICColorTransform {}
impl ::core::fmt::Debug for IWICColorTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICColorTransform").field(&self.0).finish()
    }
}
impl IWICColorTransform {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSize)(::windows::core::Vtable::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPixelFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetResolution)(::windows::core::Vtable::as_raw(self), pdpix, pdpiy).ok()
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWICPalette>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyPalette)(::windows::core::Vtable::as_raw(self), pipalette.into().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CopyPixels)(::windows::core::Vtable::as_raw(self), prc, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
}
impl ::core::cmp::PartialEq for IWICComponentFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICComponentFactory {}
impl ::core::fmt::Debug for IWICComponentFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICComponentFactory").field(&self.0).finish()
    }
}
impl IWICComponentFactory {
    pub unsafe fn CreateDecoderFromFilename<P0>(&self, wzfilename: P0, pguidvendor: ::core::option::Option<*const ::windows::core::GUID>, dwdesiredaccess: u32, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDecoderFromFilename)(::windows::core::Vtable::as_raw(self), wzfilename.into().abi(), ::core::mem::transmute(pguidvendor.unwrap_or(::std::ptr::null())), dwdesiredaccess, metadataoptions, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDecoderFromStream<P0>(&self, pistream: P0, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDecoderFromStream)(::windows::core::Vtable::as_raw(self), pistream.into().abi(), pguidvendor, metadataoptions, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateDecoderFromFileHandle(&self, hfile: usize, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDecoderFromFileHandle)(::windows::core::Vtable::as_raw(self), hfile, pguidvendor, metadataoptions, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateComponentInfo(&self, clsidcomponent: *const ::windows::core::GUID) -> ::windows::core::Result<IWICComponentInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateComponentInfo)(::windows::core::Vtable::as_raw(self), clsidcomponent, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateDecoder(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICBitmapDecoder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDecoder)(::windows::core::Vtable::as_raw(self), guidcontainerformat, pguidvendor, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateEncoder(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICBitmapEncoder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateEncoder)(::windows::core::Vtable::as_raw(self), guidcontainerformat, pguidvendor, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePalette(&self) -> ::windows::core::Result<IWICPalette> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePalette)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFormatConverter(&self) -> ::windows::core::Result<IWICFormatConverter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFormatConverter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateBitmapScaler(&self) -> ::windows::core::Result<IWICBitmapScaler> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmapScaler)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateBitmapClipper(&self) -> ::windows::core::Result<IWICBitmapClipper> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmapClipper)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateBitmapFlipRotator(&self) -> ::windows::core::Result<IWICBitmapFlipRotator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmapFlipRotator)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStream(&self) -> ::windows::core::Result<IWICStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorContext(&self) -> ::windows::core::Result<IWICColorContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateColorContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateColorTransformer(&self) -> ::windows::core::Result<IWICColorTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateColorTransformer)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateBitmap(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, option: WICBitmapCreateCacheOption) -> ::windows::core::Result<IWICBitmap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmap)(::windows::core::Vtable::as_raw(self), uiwidth, uiheight, pixelformat, option, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateBitmapFromSource<P0>(&self, pibitmapsource: P0, option: WICBitmapCreateCacheOption) -> ::windows::core::Result<IWICBitmap>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmapFromSource)(::windows::core::Vtable::as_raw(self), pibitmapsource.into().abi(), option, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateBitmapFromSourceRect<P0>(&self, pibitmapsource: P0, x: u32, y: u32, width: u32, height: u32) -> ::windows::core::Result<IWICBitmap>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWICBitmapSource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmapFromSourceRect)(::windows::core::Vtable::as_raw(self), pibitmapsource.into().abi(), x, y, width, height, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateBitmapFromMemory(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, cbstride: u32, pbbuffer: &[u8]) -> ::windows::core::Result<IWICBitmap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmapFromMemory)(::windows::core::Vtable::as_raw(self), uiwidth, uiheight, pixelformat, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateBitmapFromHBITMAP<P0, P1>(&self, hbitmap: P0, hpalette: P1, options: WICBitmapAlphaChannelOption) -> ::windows::core::Result<IWICBitmap>
    where
        P0: ::std::convert::Into<super::Gdi::HBITMAP>,
        P1: ::std::convert::Into<super::Gdi::HPALETTE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmapFromHBITMAP)(::windows::core::Vtable::as_raw(self), hbitmap.into(), hpalette.into(), options, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn CreateBitmapFromHICON<P0>(&self, hicon: P0) -> ::windows::core::Result<IWICBitmap>
    where
        P0: ::std::convert::Into<super::super::UI::WindowsAndMessaging::HICON>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateBitmapFromHICON)(::windows::core::Vtable::as_raw(self), hicon.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateComponentEnumerator(&self, componenttypes: u32, options: u32) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateComponentEnumerator)(::windows::core::Vtable::as_raw(self), componenttypes, options, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFastMetadataEncoderFromDecoder<P0>(&self, pidecoder: P0) -> ::windows::core::Result<IWICFastMetadataEncoder>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWICBitmapDecoder>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFastMetadataEncoderFromDecoder)(::windows::core::Vtable::as_raw(self), pidecoder.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateFastMetadataEncoderFromFrameDecode<P0>(&self, piframedecoder: P0) -> ::windows::core::Result<IWICFastMetadataEncoder>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWICBitmapFrameDecode>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFastMetadataEncoderFromFrameDecode)(::windows::core::Vtable::as_raw(self), piframedecoder.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateQueryWriter(&self, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICMetadataQueryWriter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateQueryWriter)(::windows::core::Vtable::as_raw(self), guidmetadataformat, pguidvendor, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateQueryWriterFromReader<P0>(&self, piqueryreader: P0, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICMetadataQueryWriter>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWICMetadataQueryReader>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateQueryWriterFromReader)(::windows::core::Vtable::as_raw(self), piqueryreader.into().abi(), pguidvendor, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWICComponentInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICComponentInfo {}
impl ::core::fmt::Debug for IWICComponentInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICComponentInfo").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICDdsDecoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICDdsDecoder {}
impl ::core::fmt::Debug for IWICDdsDecoder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICDdsDecoder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICDdsEncoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICDdsEncoder {}
impl ::core::fmt::Debug for IWICDdsEncoder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICDdsEncoder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICDdsFrameDecode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICDdsFrameDecode {}
impl ::core::fmt::Debug for IWICDdsFrameDecode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICDdsFrameDecode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICDevelopRaw {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICDevelopRaw {}
impl ::core::fmt::Debug for IWICDevelopRaw {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICDevelopRaw").field(&self.0).finish()
    }
}
impl IWICDevelopRaw {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSize)(::windows::core::Vtable::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPixelFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetResolution)(::windows::core::Vtable::as_raw(self), pdpix, pdpiy).ok()
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWICPalette>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyPalette)(::windows::core::Vtable::as_raw(self), pipalette.into().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyPixels)(::windows::core::Vtable::as_raw(self), prc, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
    pub unsafe fn GetMetadataQueryReader(&self) -> ::windows::core::Result<IWICMetadataQueryReader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMetadataQueryReader)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetColorContexts(&self, ppicolorcontexts: &mut [::core::option::Option<IWICColorContext>], pcactualcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetColorContexts)(::windows::core::Vtable::as_raw(self), ppicolorcontexts.len() as _, ::core::mem::transmute(ppicolorcontexts.as_ptr()), pcactualcount).ok()
    }
    pub unsafe fn GetThumbnail(&self) -> ::windows::core::Result<IWICBitmapSource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetThumbnail)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWICDevelopRawNotificationCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICDevelopRawNotificationCallback {}
impl ::core::fmt::Debug for IWICDevelopRawNotificationCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICDevelopRawNotificationCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICEnumMetadataItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICEnumMetadataItem {}
impl ::core::fmt::Debug for IWICEnumMetadataItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICEnumMetadataItem").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICFastMetadataEncoder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICFastMetadataEncoder {}
impl ::core::fmt::Debug for IWICFastMetadataEncoder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICFastMetadataEncoder").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICFormatConverter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICFormatConverter {}
impl ::core::fmt::Debug for IWICFormatConverter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICFormatConverter").field(&self.0).finish()
    }
}
impl IWICFormatConverter {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSize)(::windows::core::Vtable::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPixelFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetResolution)(::windows::core::Vtable::as_raw(self), pdpix, pdpiy).ok()
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWICPalette>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyPalette)(::windows::core::Vtable::as_raw(self), pipalette.into().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CopyPixels)(::windows::core::Vtable::as_raw(self), prc, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
}
impl ::core::cmp::PartialEq for IWICFormatConverterInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICFormatConverterInfo {}
impl ::core::fmt::Debug for IWICFormatConverterInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICFormatConverterInfo").field(&self.0).finish()
    }
}
impl IWICFormatConverterInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetComponentType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCLSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSigningStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAuthor(&self, wzauthor: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAuthor)(::windows::core::Vtable::as_raw(self), wzauthor.len() as _, ::core::mem::transmute(wzauthor.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVendorGUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self, wzversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetVersion)(::windows::core::Vtable::as_raw(self), wzversion.len() as _, ::core::mem::transmute(wzversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetSpecVersion(&self, wzspecversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSpecVersion)(::windows::core::Vtable::as_raw(self), wzspecversion.len() as _, ::core::mem::transmute(wzspecversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFriendlyName(&self, wzfriendlyname: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFriendlyName)(::windows::core::Vtable::as_raw(self), wzfriendlyname.len() as _, ::core::mem::transmute(wzfriendlyname.as_ptr()), pcchactual).ok()
    }
}
impl ::core::cmp::PartialEq for IWICImagingFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICImagingFactory {}
impl ::core::fmt::Debug for IWICImagingFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICImagingFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICJpegFrameDecode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICJpegFrameDecode {}
impl ::core::fmt::Debug for IWICJpegFrameDecode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICJpegFrameDecode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICJpegFrameEncode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICJpegFrameEncode {}
impl ::core::fmt::Debug for IWICJpegFrameEncode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICJpegFrameEncode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICMetadataBlockReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICMetadataBlockReader {}
impl ::core::fmt::Debug for IWICMetadataBlockReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICMetadataBlockReader").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICMetadataBlockWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICMetadataBlockWriter {}
impl ::core::fmt::Debug for IWICMetadataBlockWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICMetadataBlockWriter").field(&self.0).finish()
    }
}
impl IWICMetadataBlockWriter {
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContainerFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetReaderByIndex(&self, nindex: u32) -> ::windows::core::Result<IWICMetadataReader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetReaderByIndex)(::windows::core::Vtable::as_raw(self), nindex, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEnumerator)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWICMetadataHandlerInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICMetadataHandlerInfo {}
impl ::core::fmt::Debug for IWICMetadataHandlerInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICMetadataHandlerInfo").field(&self.0).finish()
    }
}
impl IWICMetadataHandlerInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetComponentType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCLSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSigningStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAuthor(&self, wzauthor: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAuthor)(::windows::core::Vtable::as_raw(self), wzauthor.len() as _, ::core::mem::transmute(wzauthor.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVendorGUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self, wzversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetVersion)(::windows::core::Vtable::as_raw(self), wzversion.len() as _, ::core::mem::transmute(wzversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetSpecVersion(&self, wzspecversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSpecVersion)(::windows::core::Vtable::as_raw(self), wzspecversion.len() as _, ::core::mem::transmute(wzspecversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFriendlyName(&self, wzfriendlyname: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFriendlyName)(::windows::core::Vtable::as_raw(self), wzfriendlyname.len() as _, ::core::mem::transmute(wzfriendlyname.as_ptr()), pcchactual).ok()
    }
}
impl ::core::cmp::PartialEq for IWICMetadataQueryReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICMetadataQueryReader {}
impl ::core::fmt::Debug for IWICMetadataQueryReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICMetadataQueryReader").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICMetadataQueryWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICMetadataQueryWriter {}
impl ::core::fmt::Debug for IWICMetadataQueryWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICMetadataQueryWriter").field(&self.0).finish()
    }
}
impl IWICMetadataQueryWriter {
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContainerFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLocation(&self, wznamespace: &mut [u16], pcchactuallength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetLocation)(::windows::core::Vtable::as_raw(self), wznamespace.len() as _, ::core::mem::transmute(wznamespace.as_ptr()), pcchactuallength).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetMetadataByName<P0>(&self, wzname: P0, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMetadataByName)(::windows::core::Vtable::as_raw(self), wzname.into().abi(), pvarvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<super::super::System::Com::IEnumString> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEnumerator)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWICMetadataReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICMetadataReader {}
impl ::core::fmt::Debug for IWICMetadataReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICMetadataReader").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICMetadataReaderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICMetadataReaderInfo {}
impl ::core::fmt::Debug for IWICMetadataReaderInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICMetadataReaderInfo").field(&self.0).finish()
    }
}
impl IWICMetadataReaderInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetComponentType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCLSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSigningStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAuthor(&self, wzauthor: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAuthor)(::windows::core::Vtable::as_raw(self), wzauthor.len() as _, ::core::mem::transmute(wzauthor.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetVendorGUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self, wzversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetVersion)(::windows::core::Vtable::as_raw(self), wzversion.len() as _, ::core::mem::transmute(wzversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetSpecVersion(&self, wzspecversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSpecVersion)(::windows::core::Vtable::as_raw(self), wzspecversion.len() as _, ::core::mem::transmute(wzspecversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFriendlyName(&self, wzfriendlyname: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFriendlyName)(::windows::core::Vtable::as_raw(self), wzfriendlyname.len() as _, ::core::mem::transmute(wzfriendlyname.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetMetadataFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMetadataFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetContainerFormats(&self, pguidcontainerformats: &mut [::windows::core::GUID], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetContainerFormats)(::windows::core::Vtable::as_raw(self), pguidcontainerformats.len() as _, ::core::mem::transmute(pguidcontainerformats.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceManufacturer(&self, wzdevicemanufacturer: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceManufacturer)(::windows::core::Vtable::as_raw(self), wzdevicemanufacturer.len() as _, ::core::mem::transmute(wzdevicemanufacturer.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceModels(&self, wzdevicemodels: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceModels)(::windows::core::Vtable::as_raw(self), wzdevicemodels.len() as _, ::core::mem::transmute(wzdevicemodels.as_ptr()), pcchactual).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesRequireFullStream(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DoesRequireFullStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportPadding(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DoesSupportPadding)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesRequireFixedSize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DoesRequireFixedSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWICMetadataWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICMetadataWriter {}
impl ::core::fmt::Debug for IWICMetadataWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICMetadataWriter").field(&self.0).finish()
    }
}
impl IWICMetadataWriter {
    pub unsafe fn GetMetadataFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMetadataFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMetadataHandlerInfo(&self) -> ::windows::core::Result<IWICMetadataHandlerInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMetadataHandlerInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValueByIndex(&self, nindex: u32, pvarschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetValueByIndex)(::windows::core::Vtable::as_raw(self), nindex, pvarschema, pvarid, pvarvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValue(&self, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetValue)(::windows::core::Vtable::as_raw(self), pvarschema, pvarid, pvarvalue).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IWICEnumMetadataItem> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEnumerator)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWICMetadataWriterInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICMetadataWriterInfo {}
impl ::core::fmt::Debug for IWICMetadataWriterInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICMetadataWriterInfo").field(&self.0).finish()
    }
}
impl IWICMetadataWriterInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetComponentType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCLSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSigningStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAuthor(&self, wzauthor: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAuthor)(::windows::core::Vtable::as_raw(self), wzauthor.len() as _, ::core::mem::transmute(wzauthor.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetVendorGUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self, wzversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetVersion)(::windows::core::Vtable::as_raw(self), wzversion.len() as _, ::core::mem::transmute(wzversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetSpecVersion(&self, wzspecversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSpecVersion)(::windows::core::Vtable::as_raw(self), wzspecversion.len() as _, ::core::mem::transmute(wzspecversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFriendlyName(&self, wzfriendlyname: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFriendlyName)(::windows::core::Vtable::as_raw(self), wzfriendlyname.len() as _, ::core::mem::transmute(wzfriendlyname.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetMetadataFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMetadataFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetContainerFormats(&self, pguidcontainerformats: &mut [::windows::core::GUID], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetContainerFormats)(::windows::core::Vtable::as_raw(self), pguidcontainerformats.len() as _, ::core::mem::transmute(pguidcontainerformats.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceManufacturer(&self, wzdevicemanufacturer: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceManufacturer)(::windows::core::Vtable::as_raw(self), wzdevicemanufacturer.len() as _, ::core::mem::transmute(wzdevicemanufacturer.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceModels(&self, wzdevicemodels: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceModels)(::windows::core::Vtable::as_raw(self), wzdevicemodels.len() as _, ::core::mem::transmute(wzdevicemodels.as_ptr()), pcchactual).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesRequireFullStream(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DoesRequireFullStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportPadding(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DoesSupportPadding)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesRequireFixedSize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DoesRequireFixedSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWICPalette {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICPalette {}
impl ::core::fmt::Debug for IWICPalette {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICPalette").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWICPersistStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWICPersistStream {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWICPersistStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICPersistStream").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWICPersistStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetClassID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsDirty(&self) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.IsDirty)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Load<P0>(&self, pstm: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Load)(::windows::core::Vtable::as_raw(self), pstm.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Save<P0, P1>(&self, pstm: P0, fcleardirty: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Save)(::windows::core::Vtable::as_raw(self), pstm.into().abi(), fcleardirty.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSizeMax)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWICPixelFormatInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICPixelFormatInfo {}
impl ::core::fmt::Debug for IWICPixelFormatInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICPixelFormatInfo").field(&self.0).finish()
    }
}
impl IWICPixelFormatInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetComponentType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCLSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSigningStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAuthor(&self, wzauthor: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetAuthor)(::windows::core::Vtable::as_raw(self), wzauthor.len() as _, ::core::mem::transmute(wzauthor.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVendorGUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self, wzversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetVersion)(::windows::core::Vtable::as_raw(self), wzversion.len() as _, ::core::mem::transmute(wzversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetSpecVersion(&self, wzspecversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSpecVersion)(::windows::core::Vtable::as_raw(self), wzspecversion.len() as _, ::core::mem::transmute(wzspecversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFriendlyName(&self, wzfriendlyname: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFriendlyName)(::windows::core::Vtable::as_raw(self), wzfriendlyname.len() as _, ::core::mem::transmute(wzfriendlyname.as_ptr()), pcchactual).ok()
    }
}
impl ::core::cmp::PartialEq for IWICPixelFormatInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICPixelFormatInfo2 {}
impl ::core::fmt::Debug for IWICPixelFormatInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICPixelFormatInfo2").field(&self.0).finish()
    }
}
impl IWICPixelFormatInfo2 {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetComponentType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCLSID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSigningStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAuthor(&self, wzauthor: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetAuthor)(::windows::core::Vtable::as_raw(self), wzauthor.len() as _, ::core::mem::transmute(wzauthor.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetVendorGUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self, wzversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetVersion)(::windows::core::Vtable::as_raw(self), wzversion.len() as _, ::core::mem::transmute(wzversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetSpecVersion(&self, wzspecversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSpecVersion)(::windows::core::Vtable::as_raw(self), wzspecversion.len() as _, ::core::mem::transmute(wzspecversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFriendlyName(&self, wzfriendlyname: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFriendlyName)(::windows::core::Vtable::as_raw(self), wzfriendlyname.len() as _, ::core::mem::transmute(wzfriendlyname.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFormatGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetFormatGUID)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetColorContext(&self) -> ::windows::core::Result<IWICColorContext> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetColorContext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBitsPerPixel(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBitsPerPixel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetChannelCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetChannelMask(&self, uichannelindex: u32, pbmaskbuffer: &mut [u8], pcbactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetChannelMask)(::windows::core::Vtable::as_raw(self), uichannelindex, pbmaskbuffer.len() as _, ::core::mem::transmute(pbmaskbuffer.as_ptr()), pcbactual).ok()
    }
}
impl ::core::cmp::PartialEq for IWICPlanarBitmapFrameEncode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICPlanarBitmapFrameEncode {}
impl ::core::fmt::Debug for IWICPlanarBitmapFrameEncode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICPlanarBitmapFrameEncode").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICPlanarBitmapSourceTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICPlanarBitmapSourceTransform {}
impl ::core::fmt::Debug for IWICPlanarBitmapSourceTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICPlanarBitmapSourceTransform").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICPlanarFormatConverter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICPlanarFormatConverter {}
impl ::core::fmt::Debug for IWICPlanarFormatConverter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICPlanarFormatConverter").field(&self.0).finish()
    }
}
impl IWICPlanarFormatConverter {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSize)(::windows::core::Vtable::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPixelFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetResolution)(::windows::core::Vtable::as_raw(self), pdpix, pdpiy).ok()
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWICPalette>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyPalette)(::windows::core::Vtable::as_raw(self), pipalette.into().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CopyPixels)(::windows::core::Vtable::as_raw(self), prc, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
}
impl ::core::cmp::PartialEq for IWICProgressCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICProgressCallback {}
impl ::core::fmt::Debug for IWICProgressCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICProgressCallback").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWICProgressiveLevelControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICProgressiveLevelControl {}
impl ::core::fmt::Debug for IWICProgressiveLevelControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICProgressiveLevelControl").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWICStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWICStream {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWICStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICStream").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl IWICStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.base__.Read)(::windows::core::Vtable::as_raw(self), pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.base__.Write)(::windows::core::Vtable::as_raw(self), pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK, plibnewposition: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Seek)(::windows::core::Vtable::as_raw(self), dlibmove, dworigin, ::core::mem::transmute(plibnewposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSize)(::windows::core::Vtable::as_raw(self), libnewsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: u64, pcbread: ::core::option::Option<*mut u64>, pcbwritten: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyTo)(::windows::core::Vtable::as_raw(self), pstm.into().abi(), cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit(&self, grfcommitflags: super::super::System::Com::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Commit)(::windows::core::Vtable::as_raw(self), grfcommitflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Revert)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: super::super::System::Com::LOCKTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LockRegion)(::windows::core::Vtable::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnlockRegion)(::windows::core::Vtable::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: super::super::System::Com::STATFLAG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Stat)(::windows::core::Vtable::as_raw(self), pstatstg, grfstatflag).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWICStreamProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWICStreamProvider {}
impl ::core::fmt::Debug for IWICStreamProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWICStreamProvider").field(&self.0).finish()
    }
}
impl ::core::default::Default for WIC8BIMIptcDigestProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WIC8BIMIptcDigestProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIC8BIMIptcDigestProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WIC8BIMIptcProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WIC8BIMIptcProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIC8BIMIptcProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WIC8BIMResolutionInfoProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WIC8BIMResolutionInfoProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIC8BIMResolutionInfoProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICBitmapAlphaChannelOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICBitmapAlphaChannelOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICBitmapAlphaChannelOption").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICBitmapCreateCacheOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICBitmapCreateCacheOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICBitmapCreateCacheOption").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICBitmapDecoderCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICBitmapDecoderCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICBitmapDecoderCapabilities").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICBitmapDitherType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICBitmapDitherType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICBitmapDitherType").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICBitmapEncoderCacheOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICBitmapEncoderCacheOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICBitmapEncoderCacheOption").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICBitmapInterpolationMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICBitmapInterpolationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICBitmapInterpolationMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICBitmapLockFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICBitmapLockFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICBitmapLockFlags").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICBitmapPaletteType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICBitmapPaletteType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICBitmapPaletteType").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WICBitmapPattern {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WICBitmapPattern {
    fn eq(&self, other: &Self) -> bool {
        self.Position == other.Position && self.Length == other.Length && self.Pattern == other.Pattern && self.Mask == other.Mask && self.EndOfStream == other.EndOfStream
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WICBitmapPattern {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WICBitmapPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICBitmapPattern").field("Position", &self.Position).field("Length", &self.Length).field("Pattern", &self.Pattern).field("Mask", &self.Mask).field("EndOfStream", &self.EndOfStream).finish()
    }
}
impl ::core::default::Default for WICBitmapPlane {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WICBitmapPlane {
    fn eq(&self, other: &Self) -> bool {
        self.Format == other.Format && self.pbBuffer == other.pbBuffer && self.cbStride == other.cbStride && self.cbBufferSize == other.cbBufferSize
    }
}
impl ::core::cmp::Eq for WICBitmapPlane {}
impl ::core::fmt::Debug for WICBitmapPlane {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICBitmapPlane").field("Format", &self.Format).field("pbBuffer", &self.pbBuffer).field("cbStride", &self.cbStride).field("cbBufferSize", &self.cbBufferSize).finish()
    }
}
impl ::core::default::Default for WICBitmapPlaneDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WICBitmapPlaneDescription {
    fn eq(&self, other: &Self) -> bool {
        self.Format == other.Format && self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for WICBitmapPlaneDescription {}
impl ::core::fmt::Debug for WICBitmapPlaneDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICBitmapPlaneDescription").field("Format", &self.Format).field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::core::default::Default for WICBitmapTransformOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICBitmapTransformOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICBitmapTransformOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICColorContextType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICColorContextType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICColorContextType").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICComponentEnumerateOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICComponentEnumerateOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICComponentEnumerateOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICComponentSigning {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICComponentSigning {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICComponentSigning").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICComponentType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICComponentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICComponentType").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICDdsAlphaMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICDdsAlphaMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICDdsAlphaMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICDdsDimension {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICDdsDimension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICDdsDimension").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for WICDdsFormatInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for WICDdsFormatInfo {
    fn eq(&self, other: &Self) -> bool {
        self.DxgiFormat == other.DxgiFormat && self.BytesPerBlock == other.BytesPerBlock && self.BlockWidth == other.BlockWidth && self.BlockHeight == other.BlockHeight
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for WICDdsFormatInfo {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for WICDdsFormatInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICDdsFormatInfo").field("DxgiFormat", &self.DxgiFormat).field("BytesPerBlock", &self.BytesPerBlock).field("BlockWidth", &self.BlockWidth).field("BlockHeight", &self.BlockHeight).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for WICDdsParameters {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for WICDdsParameters {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Depth == other.Depth && self.MipLevels == other.MipLevels && self.ArraySize == other.ArraySize && self.DxgiFormat == other.DxgiFormat && self.Dimension == other.Dimension && self.AlphaMode == other.AlphaMode
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for WICDdsParameters {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for WICDdsParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICDdsParameters").field("Width", &self.Width).field("Height", &self.Height).field("Depth", &self.Depth).field("MipLevels", &self.MipLevels).field("ArraySize", &self.ArraySize).field("DxgiFormat", &self.DxgiFormat).field("Dimension", &self.Dimension).field("AlphaMode", &self.AlphaMode).finish()
    }
}
impl ::core::default::Default for WICDecodeOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICDecodeOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICDecodeOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICGifApplicationExtensionProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICGifApplicationExtensionProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICGifApplicationExtensionProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICGifCommentExtensionProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICGifCommentExtensionProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICGifCommentExtensionProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICGifGraphicControlExtensionProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICGifGraphicControlExtensionProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICGifGraphicControlExtensionProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICGifImageDescriptorProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICGifImageDescriptorProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICGifImageDescriptorProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICGifLogicalScreenDescriptorProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICGifLogicalScreenDescriptorProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICGifLogicalScreenDescriptorProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICHeifHdrProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICHeifHdrProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICHeifHdrProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICHeifProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICHeifProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICHeifProperties").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for WICImageParameters {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for WICImageParameters {
    fn eq(&self, other: &Self) -> bool {
        self.PixelFormat == other.PixelFormat && self.DpiX == other.DpiX && self.DpiY == other.DpiY && self.Top == other.Top && self.Left == other.Left && self.PixelWidth == other.PixelWidth && self.PixelHeight == other.PixelHeight
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for WICImageParameters {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for WICImageParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICImageParameters").field("PixelFormat", &self.PixelFormat).field("DpiX", &self.DpiX).field("DpiY", &self.DpiY).field("Top", &self.Top).field("Left", &self.Left).field("PixelWidth", &self.PixelWidth).field("PixelHeight", &self.PixelHeight).finish()
    }
}
impl ::core::default::Default for WICJpegChrominanceProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICJpegChrominanceProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICJpegChrominanceProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICJpegCommentProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICJpegCommentProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICJpegCommentProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICJpegFrameHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WICJpegFrameHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.TransferMatrix == other.TransferMatrix && self.ScanType == other.ScanType && self.cComponents == other.cComponents && self.ComponentIdentifiers == other.ComponentIdentifiers && self.SampleFactors == other.SampleFactors && self.QuantizationTableIndices == other.QuantizationTableIndices
    }
}
impl ::core::cmp::Eq for WICJpegFrameHeader {}
impl ::core::fmt::Debug for WICJpegFrameHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICJpegFrameHeader").field("Width", &self.Width).field("Height", &self.Height).field("TransferMatrix", &self.TransferMatrix).field("ScanType", &self.ScanType).field("cComponents", &self.cComponents).field("ComponentIdentifiers", &self.ComponentIdentifiers).field("SampleFactors", &self.SampleFactors).field("QuantizationTableIndices", &self.QuantizationTableIndices).finish()
    }
}
impl ::core::default::Default for WICJpegIndexingOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICJpegIndexingOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICJpegIndexingOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICJpegLuminanceProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICJpegLuminanceProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICJpegLuminanceProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICJpegScanHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WICJpegScanHeader {
    fn eq(&self, other: &Self) -> bool {
        self.cComponents == other.cComponents && self.RestartInterval == other.RestartInterval && self.ComponentSelectors == other.ComponentSelectors && self.HuffmanTableIndices == other.HuffmanTableIndices && self.StartSpectralSelection == other.StartSpectralSelection && self.EndSpectralSelection == other.EndSpectralSelection && self.SuccessiveApproximationHigh == other.SuccessiveApproximationHigh && self.SuccessiveApproximationLow == other.SuccessiveApproximationLow
    }
}
impl ::core::cmp::Eq for WICJpegScanHeader {}
impl ::core::fmt::Debug for WICJpegScanHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICJpegScanHeader")
            .field("cComponents", &self.cComponents)
            .field("RestartInterval", &self.RestartInterval)
            .field("ComponentSelectors", &self.ComponentSelectors)
            .field("HuffmanTableIndices", &self.HuffmanTableIndices)
            .field("StartSpectralSelection", &self.StartSpectralSelection)
            .field("EndSpectralSelection", &self.EndSpectralSelection)
            .field("SuccessiveApproximationHigh", &self.SuccessiveApproximationHigh)
            .field("SuccessiveApproximationLow", &self.SuccessiveApproximationLow)
            .finish()
    }
}
impl ::core::default::Default for WICJpegScanType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICJpegScanType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICJpegScanType").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICJpegTransferMatrix {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICJpegTransferMatrix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICJpegTransferMatrix").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICJpegYCrCbSubsamplingOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICJpegYCrCbSubsamplingOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICJpegYCrCbSubsamplingOption").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICMetadataCreationOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICMetadataCreationOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICMetadataCreationOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICMetadataHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WICMetadataHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Position == other.Position && self.Length == other.Length && self.Header == other.Header && self.DataOffset == other.DataOffset
    }
}
impl ::core::cmp::Eq for WICMetadataHeader {}
impl ::core::fmt::Debug for WICMetadataHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICMetadataHeader").field("Position", &self.Position).field("Length", &self.Length).field("Header", &self.Header).field("DataOffset", &self.DataOffset).finish()
    }
}
impl ::core::default::Default for WICMetadataPattern {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WICMetadataPattern {
    fn eq(&self, other: &Self) -> bool {
        self.Position == other.Position && self.Length == other.Length && self.Pattern == other.Pattern && self.Mask == other.Mask && self.DataOffset == other.DataOffset
    }
}
impl ::core::cmp::Eq for WICMetadataPattern {}
impl ::core::fmt::Debug for WICMetadataPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICMetadataPattern").field("Position", &self.Position).field("Length", &self.Length).field("Pattern", &self.Pattern).field("Mask", &self.Mask).field("DataOffset", &self.DataOffset).finish()
    }
}
impl ::core::default::Default for WICNamedWhitePoint {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICNamedWhitePoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICNamedWhitePoint").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICPersistOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICPersistOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPersistOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICPixelFormatNumericRepresentation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICPixelFormatNumericRepresentation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPixelFormatNumericRepresentation").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICPlanarOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICPlanarOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPlanarOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICPngBkgdProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICPngBkgdProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPngBkgdProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICPngChrmProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICPngChrmProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPngChrmProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICPngFilterOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICPngFilterOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPngFilterOption").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICPngGamaProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICPngGamaProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPngGamaProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICPngHistProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICPngHistProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPngHistProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICPngIccpProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICPngIccpProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPngIccpProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICPngItxtProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICPngItxtProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPngItxtProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICPngSrgbProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICPngSrgbProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPngSrgbProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICPngTimeProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICPngTimeProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPngTimeProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICProgressNotification {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICProgressNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICProgressNotification").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICProgressOperation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICProgressOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICProgressOperation").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICRawCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICRawCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICRawCapabilities").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICRawCapabilitiesInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WICRawCapabilitiesInfo {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.CodecMajorVersion == other.CodecMajorVersion
            && self.CodecMinorVersion == other.CodecMinorVersion
            && self.ExposureCompensationSupport == other.ExposureCompensationSupport
            && self.ContrastSupport == other.ContrastSupport
            && self.RGBWhitePointSupport == other.RGBWhitePointSupport
            && self.NamedWhitePointSupport == other.NamedWhitePointSupport
            && self.NamedWhitePointSupportMask == other.NamedWhitePointSupportMask
            && self.KelvinWhitePointSupport == other.KelvinWhitePointSupport
            && self.GammaSupport == other.GammaSupport
            && self.TintSupport == other.TintSupport
            && self.SaturationSupport == other.SaturationSupport
            && self.SharpnessSupport == other.SharpnessSupport
            && self.NoiseReductionSupport == other.NoiseReductionSupport
            && self.DestinationColorProfileSupport == other.DestinationColorProfileSupport
            && self.ToneCurveSupport == other.ToneCurveSupport
            && self.RotationSupport == other.RotationSupport
            && self.RenderModeSupport == other.RenderModeSupport
    }
}
impl ::core::cmp::Eq for WICRawCapabilitiesInfo {}
impl ::core::fmt::Debug for WICRawCapabilitiesInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICRawCapabilitiesInfo")
            .field("cbSize", &self.cbSize)
            .field("CodecMajorVersion", &self.CodecMajorVersion)
            .field("CodecMinorVersion", &self.CodecMinorVersion)
            .field("ExposureCompensationSupport", &self.ExposureCompensationSupport)
            .field("ContrastSupport", &self.ContrastSupport)
            .field("RGBWhitePointSupport", &self.RGBWhitePointSupport)
            .field("NamedWhitePointSupport", &self.NamedWhitePointSupport)
            .field("NamedWhitePointSupportMask", &self.NamedWhitePointSupportMask)
            .field("KelvinWhitePointSupport", &self.KelvinWhitePointSupport)
            .field("GammaSupport", &self.GammaSupport)
            .field("TintSupport", &self.TintSupport)
            .field("SaturationSupport", &self.SaturationSupport)
            .field("SharpnessSupport", &self.SharpnessSupport)
            .field("NoiseReductionSupport", &self.NoiseReductionSupport)
            .field("DestinationColorProfileSupport", &self.DestinationColorProfileSupport)
            .field("ToneCurveSupport", &self.ToneCurveSupport)
            .field("RotationSupport", &self.RotationSupport)
            .field("RenderModeSupport", &self.RenderModeSupport)
            .finish()
    }
}
impl ::core::default::Default for WICRawParameterSet {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICRawParameterSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICRawParameterSet").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICRawRenderMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICRawRenderMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICRawRenderMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICRawRotationCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICRawRotationCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICRawRotationCapabilities").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICRawToneCurve {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WICRawToneCurve {
    fn eq(&self, other: &Self) -> bool {
        self.cPoints == other.cPoints && self.aPoints == other.aPoints
    }
}
impl ::core::cmp::Eq for WICRawToneCurve {}
impl ::core::fmt::Debug for WICRawToneCurve {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICRawToneCurve").field("cPoints", &self.cPoints).field("aPoints", &self.aPoints).finish()
    }
}
impl ::core::default::Default for WICRawToneCurvePoint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WICRawToneCurvePoint {
    fn eq(&self, other: &Self) -> bool {
        self.Input == other.Input && self.Output == other.Output
    }
}
impl ::core::cmp::Eq for WICRawToneCurvePoint {}
impl ::core::fmt::Debug for WICRawToneCurvePoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICRawToneCurvePoint").field("Input", &self.Input).field("Output", &self.Output).finish()
    }
}
impl ::core::default::Default for WICRect {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WICRect {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for WICRect {}
impl ::core::fmt::Debug for WICRect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICRect").field("X", &self.X).field("Y", &self.Y).field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::core::default::Default for WICSectionAccessLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICSectionAccessLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICSectionAccessLevel").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICTiffCompressionOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICTiffCompressionOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICTiffCompressionOption").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICWebpAnimProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICWebpAnimProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICWebpAnimProperties").field(&self.0).finish()
    }
}
impl ::core::default::Default for WICWebpAnmfProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WICWebpAnmfProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICWebpAnmfProperties").field(&self.0).finish()
    }
}
