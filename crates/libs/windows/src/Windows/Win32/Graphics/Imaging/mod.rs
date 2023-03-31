#[cfg(feature = "Win32_Graphics_Imaging_D2D")]
pub mod D2D;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[inline]
pub unsafe fn WICConvertBitmapSource<P0>(dstformat: *const ::windows::core::GUID, pisrc: P0) -> ::windows::core::Result<IWICBitmapSource>
where
    P0: ::windows::core::IntoParam<IWICBitmapSource>,
{
    ::windows_targets::link ! ( "windowscodecs.dll""system" fn WICConvertBitmapSource ( dstformat : *const ::windows::core::GUID , pisrc : * mut::core::ffi::c_void , ppidst : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IWICBitmapSource>();
    WICConvertBitmapSource(dstformat, pisrc.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WICCreateBitmapFromSection<P0>(width: u32, height: u32, pixelformat: *const ::windows::core::GUID, hsection: P0, stride: u32, offset: u32) -> ::windows::core::Result<IWICBitmap>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "windowscodecs.dll""system" fn WICCreateBitmapFromSection ( width : u32 , height : u32 , pixelformat : *const ::windows::core::GUID , hsection : super::super::Foundation:: HANDLE , stride : u32 , offset : u32 , ppibitmap : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IWICBitmap>();
    WICCreateBitmapFromSection(width, height, pixelformat, hsection.into_param().abi(), stride, offset, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WICCreateBitmapFromSectionEx<P0>(width: u32, height: u32, pixelformat: *const ::windows::core::GUID, hsection: P0, stride: u32, offset: u32, desiredaccesslevel: WICSectionAccessLevel) -> ::windows::core::Result<IWICBitmap>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "windowscodecs.dll""system" fn WICCreateBitmapFromSectionEx ( width : u32 , height : u32 , pixelformat : *const ::windows::core::GUID , hsection : super::super::Foundation:: HANDLE , stride : u32 , offset : u32 , desiredaccesslevel : WICSectionAccessLevel , ppibitmap : *mut * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<IWICBitmap>();
    WICCreateBitmapFromSectionEx(width, height, pixelformat, hsection.into_param().abi(), stride, offset, desiredaccesslevel, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[inline]
pub unsafe fn WICGetMetadataContentSize<P0>(guidcontainerformat: *const ::windows::core::GUID, piwriter: P0) -> ::windows::core::Result<u64>
where
    P0: ::windows::core::IntoParam<IWICMetadataWriter>,
{
    ::windows_targets::link ! ( "windowscodecs.dll""system" fn WICGetMetadataContentSize ( guidcontainerformat : *const ::windows::core::GUID , piwriter : * mut::core::ffi::c_void , pcbsize : *mut u64 ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<u64>();
    WICGetMetadataContentSize(guidcontainerformat, piwriter.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[inline]
pub unsafe fn WICMapGuidToShortName(guid: *const ::windows::core::GUID, wzname: ::core::option::Option<&mut [u16]>, pcchactual: *mut u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "windowscodecs.dll""system" fn WICMapGuidToShortName ( guid : *const ::windows::core::GUID , cchname : u32 , wzname : ::windows::core::PWSTR , pcchactual : *mut u32 ) -> ::windows::core::HRESULT );
    WICMapGuidToShortName(guid, wzname.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(wzname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pcchactual).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[inline]
pub unsafe fn WICMapSchemaToName<P0>(guidmetadataformat: *const ::windows::core::GUID, pwzschema: P0, wzname: ::core::option::Option<&mut [u16]>, pcchactual: *mut u32) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "windowscodecs.dll""system" fn WICMapSchemaToName ( guidmetadataformat : *const ::windows::core::GUID , pwzschema : ::windows::core::PCWSTR , cchname : u32 , wzname : ::windows::core::PWSTR , pcchactual : *mut u32 ) -> ::windows::core::HRESULT );
    WICMapSchemaToName(guidmetadataformat, pwzschema.into_param().abi(), wzname.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(wzname.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pcchactual).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[inline]
pub unsafe fn WICMapShortNameToGuid<P0>(wzname: P0) -> ::windows::core::Result<::windows::core::GUID>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "windowscodecs.dll""system" fn WICMapShortNameToGuid ( wzname : ::windows::core::PCWSTR , pguid : *mut ::windows::core::GUID ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
    WICMapShortNameToGuid(wzname.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn WICMatchMetadataContent<P0>(guidcontainerformat: *const ::windows::core::GUID, pguidvendor: ::core::option::Option<*const ::windows::core::GUID>, pistream: P0) -> ::windows::core::Result<::windows::core::GUID>
where
    P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
{
    ::windows_targets::link ! ( "windowscodecs.dll""system" fn WICMatchMetadataContent ( guidcontainerformat : *const ::windows::core::GUID , pguidvendor : *const ::windows::core::GUID , pistream : * mut::core::ffi::c_void , pguidmetadataformat : *mut ::windows::core::GUID ) -> ::windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
    WICMatchMetadataContent(guidcontainerformat, ::core::mem::transmute(pguidvendor.unwrap_or(::std::ptr::null())), pistream.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn WICSerializeMetadataContent<P0, P1>(guidcontainerformat: *const ::windows::core::GUID, piwriter: P0, dwpersistoptions: u32, pistream: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<IWICMetadataWriter>,
    P1: ::windows::core::IntoParam<super::super::System::Com::IStream>,
{
    ::windows_targets::link ! ( "windowscodecs.dll""system" fn WICSerializeMetadataContent ( guidcontainerformat : *const ::windows::core::GUID , piwriter : * mut::core::ffi::c_void , dwpersistoptions : u32 , pistream : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    WICSerializeMetadataContent(guidcontainerformat, piwriter.into_param().abi(), dwpersistoptions, pistream.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICBitmap(::windows::core::IUnknown);
impl IWICBitmap {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSize)(::windows::core::Interface::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetPixelFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetResolution)(::windows::core::Interface::as_raw(self), pdpix, pdpiy).ok()
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICPalette>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyPalette)(::windows::core::Interface::as_raw(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CopyPixels)(::windows::core::Interface::as_raw(self), prc, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
    pub unsafe fn Lock(&self, prclock: *const WICRect, flags: u32) -> ::windows::core::Result<IWICBitmapLock> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapLock>();
        (::windows::core::Interface::vtable(self).Lock)(::windows::core::Interface::as_raw(self), prclock, flags, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICPalette>,
    {
        (::windows::core::Interface::vtable(self).SetPalette)(::windows::core::Interface::as_raw(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn SetResolution(&self, dpix: f64, dpiy: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetResolution)(::windows::core::Interface::as_raw(self), dpix, dpiy).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICBitmap, ::windows::core::IUnknown, IWICBitmapSource);
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
unsafe impl ::windows::core::Interface for IWICBitmap {
    type Vtable = IWICBitmap_Vtbl;
}
impl ::core::clone::Clone for IWICBitmap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICBitmap {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000121_a8f2_4877_ba0a_fd2b6645fb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmap_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Lock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prclock: *const WICRect, flags: u32, ppilock: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dpix: f64, dpiy: f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICBitmapClipper(::windows::core::IUnknown);
impl IWICBitmapClipper {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSize)(::windows::core::Interface::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetPixelFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetResolution)(::windows::core::Interface::as_raw(self), pdpix, pdpiy).ok()
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICPalette>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyPalette)(::windows::core::Interface::as_raw(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CopyPixels)(::windows::core::Interface::as_raw(self), prc, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
    pub unsafe fn Initialize<P0>(&self, pisource: P0, prc: *const WICRect) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICBitmapSource>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pisource.into_param().abi(), prc).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICBitmapClipper, ::windows::core::IUnknown, IWICBitmapSource);
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
unsafe impl ::windows::core::Interface for IWICBitmapClipper {
    type Vtable = IWICBitmapClipper_Vtbl;
}
impl ::core::clone::Clone for IWICBitmapClipper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICBitmapClipper {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4fbcf03_223d_4e81_9333_d635556dd1b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapClipper_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisource: *mut ::core::ffi::c_void, prc: *const WICRect) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICBitmapCodecInfo(::windows::core::IUnknown);
impl IWICBitmapCodecInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__ = ::windows::core::zeroed::<WICComponentType>();
        (::windows::core::Interface::vtable(self).base__.GetComponentType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetCLSID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetSigningStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAuthor(&self, wzauthor: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAuthor)(::windows::core::Interface::as_raw(self), wzauthor.len() as _, ::core::mem::transmute(wzauthor.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetVendorGUID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self, wzversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetVersion)(::windows::core::Interface::as_raw(self), wzversion.len() as _, ::core::mem::transmute(wzversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetSpecVersion(&self, wzspecversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSpecVersion)(::windows::core::Interface::as_raw(self), wzspecversion.len() as _, ::core::mem::transmute(wzspecversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFriendlyName(&self, wzfriendlyname: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFriendlyName)(::windows::core::Interface::as_raw(self), wzfriendlyname.len() as _, ::core::mem::transmute(wzfriendlyname.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetContainerFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPixelFormats(&self, pguidpixelformats: &mut [::windows::core::GUID], pcactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPixelFormats)(::windows::core::Interface::as_raw(self), pguidpixelformats.len() as _, ::core::mem::transmute(pguidpixelformats.as_ptr()), pcactual).ok()
    }
    pub unsafe fn GetColorManagementVersion(&self, wzcolormanagementversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColorManagementVersion)(::windows::core::Interface::as_raw(self), wzcolormanagementversion.len() as _, ::core::mem::transmute(wzcolormanagementversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceManufacturer(&self, wzdevicemanufacturer: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeviceManufacturer)(::windows::core::Interface::as_raw(self), wzdevicemanufacturer.len() as _, ::core::mem::transmute(wzdevicemanufacturer.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceModels(&self, wzdevicemodels: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeviceModels)(::windows::core::Interface::as_raw(self), wzdevicemodels.len() as _, ::core::mem::transmute(wzdevicemodels.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetMimeTypes(&self, wzmimetypes: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMimeTypes)(::windows::core::Interface::as_raw(self), wzmimetypes.len() as _, ::core::mem::transmute(wzmimetypes.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFileExtensions(&self, wzfileextensions: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFileExtensions)(::windows::core::Interface::as_raw(self), wzfileextensions.len() as _, ::core::mem::transmute(wzfileextensions.as_ptr()), pcchactual).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportAnimation(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).DoesSupportAnimation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportChromakey(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).DoesSupportChromakey)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportLossless(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).DoesSupportLossless)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportMultiframe(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).DoesSupportMultiframe)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchesMimeType<P0>(&self, wzmimetype: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).MatchesMimeType)(::windows::core::Interface::as_raw(self), wzmimetype.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICBitmapCodecInfo, ::windows::core::IUnknown, IWICComponentInfo);
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
unsafe impl ::windows::core::Interface for IWICBitmapCodecInfo {
    type Vtable = IWICBitmapCodecInfo_Vtbl;
}
impl ::core::clone::Clone for IWICBitmapCodecInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICBitmapCodecInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe87a44c4_b76e_4c47_8b09_298eb12a2714);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapCodecInfo_Vtbl {
    pub base__: IWICComponentInfo_Vtbl,
    pub GetContainerFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetPixelFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cformats: u32, pguidpixelformats: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::HRESULT,
    pub GetColorManagementVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchcolormanagementversion: u32, wzcolormanagementversion: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    pub GetDeviceManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    pub GetDeviceModels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    pub GetMimeTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchmimetypes: u32, wzmimetypes: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    pub GetFileExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchfileextensions: u32, wzfileextensions: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DoesSupportAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportanimation: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoesSupportAnimation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DoesSupportChromakey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportchromakey: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoesSupportChromakey: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DoesSupportLossless: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportlossless: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoesSupportLossless: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DoesSupportMultiframe: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportmultiframe: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoesSupportMultiframe: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MatchesMimeType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wzmimetype: ::windows::core::PCWSTR, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MatchesMimeType: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICBitmapCodecProgressNotification(::windows::core::IUnknown);
impl IWICBitmapCodecProgressNotification {
    pub unsafe fn RegisterProgressNotification(&self, pfnprogressnotification: PFNProgressNotification, pvdata: ::core::option::Option<*const ::core::ffi::c_void>, dwprogressflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterProgressNotification)(::windows::core::Interface::as_raw(self), pfnprogressnotification, ::core::mem::transmute(pvdata.unwrap_or(::std::ptr::null())), dwprogressflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICBitmapCodecProgressNotification, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICBitmapCodecProgressNotification {
    type Vtable = IWICBitmapCodecProgressNotification_Vtbl;
}
impl ::core::clone::Clone for IWICBitmapCodecProgressNotification {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICBitmapCodecProgressNotification {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64c1024e_c3cf_4462_8078_88c2b11c46d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapCodecProgressNotification_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub RegisterProgressNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfnprogressnotification: PFNProgressNotification, pvdata: *const ::core::ffi::c_void, dwprogressflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICBitmapDecoder(::windows::core::IUnknown);
impl IWICBitmapDecoder {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryCapability<P0>(&self, pistream: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).QueryCapability)(::windows::core::Interface::as_raw(self), pistream.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pistream: P0, cacheoptions: WICDecodeOptions) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pistream.into_param().abi(), cacheoptions).ok()
    }
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetContainerFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDecoderInfo(&self) -> ::windows::core::Result<IWICBitmapDecoderInfo> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapDecoderInfo>();
        (::windows::core::Interface::vtable(self).GetDecoderInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICPalette>,
    {
        (::windows::core::Interface::vtable(self).CopyPalette)(::windows::core::Interface::as_raw(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn GetMetadataQueryReader(&self) -> ::windows::core::Result<IWICMetadataQueryReader> {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataQueryReader>();
        (::windows::core::Interface::vtable(self).GetMetadataQueryReader)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPreview(&self) -> ::windows::core::Result<IWICBitmapSource> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapSource>();
        (::windows::core::Interface::vtable(self).GetPreview)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColorContexts(&self, ppicolorcontexts: &mut [::core::option::Option<IWICColorContext>], pcactualcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColorContexts)(::windows::core::Interface::as_raw(self), ppicolorcontexts.len() as _, ::core::mem::transmute(ppicolorcontexts.as_ptr()), pcactualcount).ok()
    }
    pub unsafe fn GetThumbnail(&self) -> ::windows::core::Result<IWICBitmapSource> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapSource>();
        (::windows::core::Interface::vtable(self).GetThumbnail)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFrameCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetFrameCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFrame(&self, index: u32) -> ::windows::core::Result<IWICBitmapFrameDecode> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapFrameDecode>();
        (::windows::core::Interface::vtable(self).GetFrame)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICBitmapDecoder, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICBitmapDecoder {
    type Vtable = IWICBitmapDecoder_Vtbl;
}
impl ::core::clone::Clone for IWICBitmapDecoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICBitmapDecoder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9edde9e7_8dee_47ea_99df_e6faf2ed44bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapDecoder_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryCapability: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, pdwcapability: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryCapability: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, cacheoptions: WICDecodeOptions) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    pub GetContainerFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetDecoderInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppidecoderinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CopyPalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMetadataQueryReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimetadataqueryreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppibitmapsource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetColorContexts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut *mut ::core::ffi::c_void, pcactualcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppithumbnail: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFrameCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, ppibitmapframe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICBitmapDecoderInfo(::windows::core::IUnknown);
impl IWICBitmapDecoderInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__ = ::windows::core::zeroed::<WICComponentType>();
        (::windows::core::Interface::vtable(self).base__.base__.GetComponentType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.base__.GetCLSID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetSigningStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAuthor(&self, wzauthor: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetAuthor)(::windows::core::Interface::as_raw(self), wzauthor.len() as _, ::core::mem::transmute(wzauthor.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.base__.GetVendorGUID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self, wzversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetVersion)(::windows::core::Interface::as_raw(self), wzversion.len() as _, ::core::mem::transmute(wzversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetSpecVersion(&self, wzspecversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetSpecVersion)(::windows::core::Interface::as_raw(self), wzspecversion.len() as _, ::core::mem::transmute(wzspecversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFriendlyName(&self, wzfriendlyname: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFriendlyName)(::windows::core::Interface::as_raw(self), wzfriendlyname.len() as _, ::core::mem::transmute(wzfriendlyname.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetContainerFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPixelFormats(&self, pguidpixelformats: &mut [::windows::core::GUID], pcactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPixelFormats)(::windows::core::Interface::as_raw(self), pguidpixelformats.len() as _, ::core::mem::transmute(pguidpixelformats.as_ptr()), pcactual).ok()
    }
    pub unsafe fn GetColorManagementVersion(&self, wzcolormanagementversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetColorManagementVersion)(::windows::core::Interface::as_raw(self), wzcolormanagementversion.len() as _, ::core::mem::transmute(wzcolormanagementversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceManufacturer(&self, wzdevicemanufacturer: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDeviceManufacturer)(::windows::core::Interface::as_raw(self), wzdevicemanufacturer.len() as _, ::core::mem::transmute(wzdevicemanufacturer.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceModels(&self, wzdevicemodels: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDeviceModels)(::windows::core::Interface::as_raw(self), wzdevicemodels.len() as _, ::core::mem::transmute(wzdevicemodels.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetMimeTypes(&self, wzmimetypes: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetMimeTypes)(::windows::core::Interface::as_raw(self), wzmimetypes.len() as _, ::core::mem::transmute(wzmimetypes.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFileExtensions(&self, wzfileextensions: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFileExtensions)(::windows::core::Interface::as_raw(self), wzfileextensions.len() as _, ::core::mem::transmute(wzfileextensions.as_ptr()), pcchactual).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportAnimation(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.DoesSupportAnimation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportChromakey(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.DoesSupportChromakey)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportLossless(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.DoesSupportLossless)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportMultiframe(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.DoesSupportMultiframe)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchesMimeType<P0>(&self, wzmimetype: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.MatchesMimeType)(::windows::core::Interface::as_raw(self), wzmimetype.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPatterns(&self, cbsizepatterns: u32, ppatterns: ::core::option::Option<*mut WICBitmapPattern>, pcpatterns: ::core::option::Option<*mut u32>, pcbpatternsactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPatterns)(::windows::core::Interface::as_raw(self), cbsizepatterns, ::core::mem::transmute(ppatterns.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcpatterns.unwrap_or(::std::ptr::null_mut())), pcbpatternsactual).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn MatchesPattern<P0>(&self, pistream: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).MatchesPattern)(::windows::core::Interface::as_raw(self), pistream.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateInstance(&self) -> ::windows::core::Result<IWICBitmapDecoder> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapDecoder>();
        (::windows::core::Interface::vtable(self).CreateInstance)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICBitmapDecoderInfo, ::windows::core::IUnknown, IWICComponentInfo, IWICBitmapCodecInfo);
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
unsafe impl ::windows::core::Interface for IWICBitmapDecoderInfo {
    type Vtable = IWICBitmapDecoderInfo_Vtbl;
}
impl ::core::clone::Clone for IWICBitmapDecoderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICBitmapDecoderInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8cd007f_d08f_4191_9bfc_236ea7f0e4b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapDecoderInfo_Vtbl {
    pub base__: IWICBitmapCodecInfo_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPatterns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbsizepatterns: u32, ppatterns: *mut WICBitmapPattern, pcpatterns: *mut u32, pcbpatternsactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPatterns: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub MatchesPattern: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    MatchesPattern: usize,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppibitmapdecoder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICBitmapEncoder(::windows::core::IUnknown);
impl IWICBitmapEncoder {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pistream: P0, cacheoption: WICBitmapEncoderCacheOption) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pistream.into_param().abi(), cacheoption).ok()
    }
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetContainerFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetEncoderInfo(&self) -> ::windows::core::Result<IWICBitmapEncoderInfo> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapEncoderInfo>();
        (::windows::core::Interface::vtable(self).GetEncoderInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetColorContexts(&self, ppicolorcontext: &[::core::option::Option<IWICColorContext>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColorContexts)(::windows::core::Interface::as_raw(self), ppicolorcontext.len() as _, ::core::mem::transmute(ppicolorcontext.as_ptr())).ok()
    }
    pub unsafe fn SetPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICPalette>,
    {
        (::windows::core::Interface::vtable(self).SetPalette)(::windows::core::Interface::as_raw(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn SetThumbnail<P0>(&self, pithumbnail: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICBitmapSource>,
    {
        (::windows::core::Interface::vtable(self).SetThumbnail)(::windows::core::Interface::as_raw(self), pithumbnail.into_param().abi()).ok()
    }
    pub unsafe fn SetPreview<P0>(&self, pipreview: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICBitmapSource>,
    {
        (::windows::core::Interface::vtable(self).SetPreview)(::windows::core::Interface::as_raw(self), pipreview.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn CreateNewFrame(&self, ppiframeencode: *mut ::core::option::Option<IWICBitmapFrameEncode>, ppiencoderoptions: *mut ::core::option::Option<super::super::System::Com::StructuredStorage::IPropertyBag2>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateNewFrame)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppiframeencode), ::core::mem::transmute(ppiencoderoptions)).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetMetadataQueryWriter(&self) -> ::windows::core::Result<IWICMetadataQueryWriter> {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataQueryWriter>();
        (::windows::core::Interface::vtable(self).GetMetadataQueryWriter)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICBitmapEncoder, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICBitmapEncoder {
    type Vtable = IWICBitmapEncoder_Vtbl;
}
impl ::core::clone::Clone for IWICBitmapEncoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICBitmapEncoder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000103_a8f2_4877_ba0a_fd2b6645fb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapEncoder_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, cacheoption: WICBitmapEncoderCacheOption) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    pub GetContainerFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetEncoderInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiencoderinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetColorContexts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontext: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pithumbnail: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipreview: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub CreateNewFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiframeencode: *mut *mut ::core::ffi::c_void, ppiencoderoptions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    CreateNewFrame: usize,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMetadataQueryWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICBitmapEncoderInfo(::windows::core::IUnknown);
impl IWICBitmapEncoderInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__ = ::windows::core::zeroed::<WICComponentType>();
        (::windows::core::Interface::vtable(self).base__.base__.GetComponentType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.base__.GetCLSID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetSigningStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAuthor(&self, wzauthor: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetAuthor)(::windows::core::Interface::as_raw(self), wzauthor.len() as _, ::core::mem::transmute(wzauthor.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.base__.GetVendorGUID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self, wzversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetVersion)(::windows::core::Interface::as_raw(self), wzversion.len() as _, ::core::mem::transmute(wzversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetSpecVersion(&self, wzspecversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetSpecVersion)(::windows::core::Interface::as_raw(self), wzspecversion.len() as _, ::core::mem::transmute(wzspecversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFriendlyName(&self, wzfriendlyname: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFriendlyName)(::windows::core::Interface::as_raw(self), wzfriendlyname.len() as _, ::core::mem::transmute(wzfriendlyname.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetContainerFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPixelFormats(&self, pguidpixelformats: &mut [::windows::core::GUID], pcactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPixelFormats)(::windows::core::Interface::as_raw(self), pguidpixelformats.len() as _, ::core::mem::transmute(pguidpixelformats.as_ptr()), pcactual).ok()
    }
    pub unsafe fn GetColorManagementVersion(&self, wzcolormanagementversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetColorManagementVersion)(::windows::core::Interface::as_raw(self), wzcolormanagementversion.len() as _, ::core::mem::transmute(wzcolormanagementversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceManufacturer(&self, wzdevicemanufacturer: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDeviceManufacturer)(::windows::core::Interface::as_raw(self), wzdevicemanufacturer.len() as _, ::core::mem::transmute(wzdevicemanufacturer.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceModels(&self, wzdevicemodels: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDeviceModels)(::windows::core::Interface::as_raw(self), wzdevicemodels.len() as _, ::core::mem::transmute(wzdevicemodels.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetMimeTypes(&self, wzmimetypes: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetMimeTypes)(::windows::core::Interface::as_raw(self), wzmimetypes.len() as _, ::core::mem::transmute(wzmimetypes.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFileExtensions(&self, wzfileextensions: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFileExtensions)(::windows::core::Interface::as_raw(self), wzfileextensions.len() as _, ::core::mem::transmute(wzfileextensions.as_ptr()), pcchactual).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportAnimation(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.DoesSupportAnimation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportChromakey(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.DoesSupportChromakey)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportLossless(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.DoesSupportLossless)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportMultiframe(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.DoesSupportMultiframe)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MatchesMimeType<P0>(&self, wzmimetype: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.MatchesMimeType)(::windows::core::Interface::as_raw(self), wzmimetype.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateInstance(&self) -> ::windows::core::Result<IWICBitmapEncoder> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapEncoder>();
        (::windows::core::Interface::vtable(self).CreateInstance)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICBitmapEncoderInfo, ::windows::core::IUnknown, IWICComponentInfo, IWICBitmapCodecInfo);
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
unsafe impl ::windows::core::Interface for IWICBitmapEncoderInfo {
    type Vtable = IWICBitmapEncoderInfo_Vtbl;
}
impl ::core::clone::Clone for IWICBitmapEncoderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICBitmapEncoderInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94c9b4ee_a09f_4f92_8a1e_4a9bce7e76fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapEncoderInfo_Vtbl {
    pub base__: IWICBitmapCodecInfo_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppibitmapencoder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICBitmapFlipRotator(::windows::core::IUnknown);
impl IWICBitmapFlipRotator {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSize)(::windows::core::Interface::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetPixelFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetResolution)(::windows::core::Interface::as_raw(self), pdpix, pdpiy).ok()
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICPalette>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyPalette)(::windows::core::Interface::as_raw(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CopyPixels)(::windows::core::Interface::as_raw(self), prc, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
    pub unsafe fn Initialize<P0>(&self, pisource: P0, options: WICBitmapTransformOptions) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICBitmapSource>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pisource.into_param().abi(), options).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICBitmapFlipRotator, ::windows::core::IUnknown, IWICBitmapSource);
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
unsafe impl ::windows::core::Interface for IWICBitmapFlipRotator {
    type Vtable = IWICBitmapFlipRotator_Vtbl;
}
impl ::core::clone::Clone for IWICBitmapFlipRotator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICBitmapFlipRotator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5009834f_2d6a_41ce_9e1b_17c5aff7a782);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapFlipRotator_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisource: *mut ::core::ffi::c_void, options: WICBitmapTransformOptions) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICBitmapFrameDecode(::windows::core::IUnknown);
impl IWICBitmapFrameDecode {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSize)(::windows::core::Interface::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetPixelFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetResolution)(::windows::core::Interface::as_raw(self), pdpix, pdpiy).ok()
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICPalette>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyPalette)(::windows::core::Interface::as_raw(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CopyPixels)(::windows::core::Interface::as_raw(self), prc, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
    pub unsafe fn GetMetadataQueryReader(&self) -> ::windows::core::Result<IWICMetadataQueryReader> {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataQueryReader>();
        (::windows::core::Interface::vtable(self).GetMetadataQueryReader)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColorContexts(&self, ppicolorcontexts: &mut [::core::option::Option<IWICColorContext>], pcactualcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColorContexts)(::windows::core::Interface::as_raw(self), ppicolorcontexts.len() as _, ::core::mem::transmute(ppicolorcontexts.as_ptr()), pcactualcount).ok()
    }
    pub unsafe fn GetThumbnail(&self) -> ::windows::core::Result<IWICBitmapSource> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapSource>();
        (::windows::core::Interface::vtable(self).GetThumbnail)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICBitmapFrameDecode, ::windows::core::IUnknown, IWICBitmapSource);
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
unsafe impl ::windows::core::Interface for IWICBitmapFrameDecode {
    type Vtable = IWICBitmapFrameDecode_Vtbl;
}
impl ::core::clone::Clone for IWICBitmapFrameDecode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICBitmapFrameDecode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b16811b_6a43_4ec9_a813_3d930c13b940);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapFrameDecode_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub GetMetadataQueryReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimetadataqueryreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetColorContexts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontexts: *mut *mut ::core::ffi::c_void, pcactualcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppithumbnail: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICBitmapFrameEncode(::windows::core::IUnknown);
impl IWICBitmapFrameEncode {
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn Initialize<P0>(&self, piencoderoptions: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::StructuredStorage::IPropertyBag2>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), piencoderoptions.into_param().abi()).ok()
    }
    pub unsafe fn SetSize(&self, uiwidth: u32, uiheight: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSize)(::windows::core::Interface::as_raw(self), uiwidth, uiheight).ok()
    }
    pub unsafe fn SetResolution(&self, dpix: f64, dpiy: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetResolution)(::windows::core::Interface::as_raw(self), dpix, dpiy).ok()
    }
    pub unsafe fn SetPixelFormat(&self, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPixelFormat)(::windows::core::Interface::as_raw(self), ppixelformat).ok()
    }
    pub unsafe fn SetColorContexts(&self, ppicolorcontext: &[::core::option::Option<IWICColorContext>]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColorContexts)(::windows::core::Interface::as_raw(self), ppicolorcontext.len() as _, ::core::mem::transmute(ppicolorcontext.as_ptr())).ok()
    }
    pub unsafe fn SetPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICPalette>,
    {
        (::windows::core::Interface::vtable(self).SetPalette)(::windows::core::Interface::as_raw(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn SetThumbnail<P0>(&self, pithumbnail: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICBitmapSource>,
    {
        (::windows::core::Interface::vtable(self).SetThumbnail)(::windows::core::Interface::as_raw(self), pithumbnail.into_param().abi()).ok()
    }
    pub unsafe fn WritePixels(&self, linecount: u32, cbstride: u32, pbpixels: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WritePixels)(::windows::core::Interface::as_raw(self), linecount, cbstride, pbpixels.len() as _, ::core::mem::transmute(pbpixels.as_ptr())).ok()
    }
    pub unsafe fn WriteSource<P0>(&self, pibitmapsource: P0, prc: *const WICRect) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICBitmapSource>,
    {
        (::windows::core::Interface::vtable(self).WriteSource)(::windows::core::Interface::as_raw(self), pibitmapsource.into_param().abi(), prc).ok()
    }
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetMetadataQueryWriter(&self) -> ::windows::core::Result<IWICMetadataQueryWriter> {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataQueryWriter>();
        (::windows::core::Interface::vtable(self).GetMetadataQueryWriter)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICBitmapFrameEncode, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICBitmapFrameEncode {
    type Vtable = IWICBitmapFrameEncode_Vtbl;
}
impl ::core::clone::Clone for IWICBitmapFrameEncode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICBitmapFrameEncode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000105_a8f2_4877_ba0a_fd2b6645fb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapFrameEncode_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piencoderoptions: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    Initialize: usize,
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32) -> ::windows::core::HRESULT,
    pub SetResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dpix: f64, dpiy: f64) -> ::windows::core::HRESULT,
    pub SetPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetColorContexts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccount: u32, ppicolorcontext: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pithumbnail: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WritePixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linecount: u32, cbstride: u32, cbbuffersize: u32, pbpixels: *const u8) -> ::windows::core::HRESULT,
    pub WriteSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pibitmapsource: *mut ::core::ffi::c_void, prc: *const WICRect) -> ::windows::core::HRESULT,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMetadataQueryWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICBitmapLock(::windows::core::IUnknown);
impl IWICBitmapLock {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSize)(::windows::core::Interface::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetStride(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetStride)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDataPointer(&self, pcbbuffersize: *mut u32, ppbdata: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDataPointer)(::windows::core::Interface::as_raw(self), pcbbuffersize, ppbdata).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetPixelFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICBitmapLock, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICBitmapLock {
    type Vtable = IWICBitmapLock_Vtbl;
}
impl ::core::clone::Clone for IWICBitmapLock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICBitmapLock {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000123_a8f2_4877_ba0a_fd2b6645fb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapLock_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT,
    pub GetStride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbstride: *mut u32) -> ::windows::core::HRESULT,
    pub GetDataPointer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcbbuffersize: *mut u32, ppbdata: *mut *mut u8) -> ::windows::core::HRESULT,
    pub GetPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICBitmapScaler(::windows::core::IUnknown);
impl IWICBitmapScaler {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSize)(::windows::core::Interface::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetPixelFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetResolution)(::windows::core::Interface::as_raw(self), pdpix, pdpiy).ok()
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICPalette>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyPalette)(::windows::core::Interface::as_raw(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CopyPixels)(::windows::core::Interface::as_raw(self), prc, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
    pub unsafe fn Initialize<P0>(&self, pisource: P0, uiwidth: u32, uiheight: u32, mode: WICBitmapInterpolationMode) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICBitmapSource>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pisource.into_param().abi(), uiwidth, uiheight, mode).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICBitmapScaler, ::windows::core::IUnknown, IWICBitmapSource);
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
unsafe impl ::windows::core::Interface for IWICBitmapScaler {
    type Vtable = IWICBitmapScaler_Vtbl;
}
impl ::core::clone::Clone for IWICBitmapScaler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICBitmapScaler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000302_a8f2_4877_ba0a_fd2b6645fb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapScaler_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisource: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, mode: WICBitmapInterpolationMode) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICBitmapSource(::windows::core::IUnknown);
impl IWICBitmapSource {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSize)(::windows::core::Interface::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetPixelFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetResolution)(::windows::core::Interface::as_raw(self), pdpix, pdpiy).ok()
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICPalette>,
    {
        (::windows::core::Interface::vtable(self).CopyPalette)(::windows::core::Interface::as_raw(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopyPixels)(::windows::core::Interface::as_raw(self), prc, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICBitmapSource, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICBitmapSource {
    type Vtable = IWICBitmapSource_Vtbl;
}
impl ::core::clone::Clone for IWICBitmapSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICBitmapSource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000120_a8f2_4877_ba0a_fd2b6645fb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapSource_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT,
    pub GetPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppixelformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::HRESULT,
    pub CopyPalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CopyPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICBitmapSourceTransform(::windows::core::IUnknown);
impl IWICBitmapSourceTransform {
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const ::windows::core::GUID, dsttransform: WICBitmapTransformOptions, nstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopyPixels)(::windows::core::Interface::as_raw(self), prc, uiwidth, uiheight, pguiddstformat, dsttransform, nstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
    pub unsafe fn GetClosestSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClosestSize)(::windows::core::Interface::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetClosestPixelFormat(&self, pguiddstformat: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClosestPixelFormat)(::windows::core::Interface::as_raw(self), pguiddstformat).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportTransform(&self, dsttransform: WICBitmapTransformOptions) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).DoesSupportTransform)(::windows::core::Interface::as_raw(self), dsttransform, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICBitmapSourceTransform, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICBitmapSourceTransform {
    type Vtable = IWICBitmapSourceTransform_Vtbl;
}
impl ::core::clone::Clone for IWICBitmapSourceTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICBitmapSourceTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b16811b_6a43_4ec9_b713_3d5a0c13b940);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICBitmapSourceTransform_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CopyPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prc: *const WICRect, uiwidth: u32, uiheight: u32, pguiddstformat: *const ::windows::core::GUID, dsttransform: WICBitmapTransformOptions, nstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT,
    pub GetClosestSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::HRESULT,
    pub GetClosestPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguiddstformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DoesSupportTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dsttransform: WICBitmapTransformOptions, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoesSupportTransform: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICColorContext(::windows::core::IUnknown);
impl IWICColorContext {
    pub unsafe fn InitializeFromFilename<P0>(&self, wzfilename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).InitializeFromFilename)(::windows::core::Interface::as_raw(self), wzfilename.into_param().abi()).ok()
    }
    pub unsafe fn InitializeFromMemory(&self, pbbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitializeFromMemory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbbuffer.as_ptr()), pbbuffer.len() as _).ok()
    }
    pub unsafe fn InitializeFromExifColorSpace(&self, value: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitializeFromExifColorSpace)(::windows::core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<WICColorContextType> {
        let mut result__ = ::windows::core::zeroed::<WICColorContextType>();
        (::windows::core::Interface::vtable(self).GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetProfileBytes(&self, pbbuffer: &mut [u8], pcbactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProfileBytes)(::windows::core::Interface::as_raw(self), pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr()), pcbactual).ok()
    }
    pub unsafe fn GetExifColorSpace(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetExifColorSpace)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICColorContext, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICColorContext {
    type Vtable = IWICColorContext_Vtbl;
}
impl ::core::clone::Clone for IWICColorContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICColorContext {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c613a02_34b2_44ea_9a7c_45aea9c6fd6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICColorContext_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub InitializeFromFilename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wzfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub InitializeFromMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows::core::HRESULT,
    pub InitializeFromExifColorSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WICColorContextType) -> ::windows::core::HRESULT,
    pub GetProfileBytes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbbuffer: u32, pbbuffer: *mut u8, pcbactual: *mut u32) -> ::windows::core::HRESULT,
    pub GetExifColorSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICColorTransform(::windows::core::IUnknown);
impl IWICColorTransform {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSize)(::windows::core::Interface::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetPixelFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetResolution)(::windows::core::Interface::as_raw(self), pdpix, pdpiy).ok()
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICPalette>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyPalette)(::windows::core::Interface::as_raw(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CopyPixels)(::windows::core::Interface::as_raw(self), prc, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
    pub unsafe fn Initialize<P0, P1, P2>(&self, pibitmapsource: P0, picontextsource: P1, picontextdest: P2, pixelfmtdest: *const ::windows::core::GUID) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICBitmapSource>,
        P1: ::windows::core::IntoParam<IWICColorContext>,
        P2: ::windows::core::IntoParam<IWICColorContext>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pibitmapsource.into_param().abi(), picontextsource.into_param().abi(), picontextdest.into_param().abi(), pixelfmtdest).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICColorTransform, ::windows::core::IUnknown, IWICBitmapSource);
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
unsafe impl ::windows::core::Interface for IWICColorTransform {
    type Vtable = IWICColorTransform_Vtbl;
}
impl ::core::clone::Clone for IWICColorTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICColorTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb66f034f_d0e2_40ab_b436_6de39e321a94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICColorTransform_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pibitmapsource: *mut ::core::ffi::c_void, picontextsource: *mut ::core::ffi::c_void, picontextdest: *mut ::core::ffi::c_void, pixelfmtdest: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICComponentFactory(::windows::core::IUnknown);
impl IWICComponentFactory {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDecoderFromFilename<P0>(&self, wzfilename: P0, pguidvendor: ::core::option::Option<*const ::windows::core::GUID>, dwdesiredaccess: super::super::Foundation::GENERIC_ACCESS_RIGHTS, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapDecoder>();
        (::windows::core::Interface::vtable(self).base__.CreateDecoderFromFilename)(::windows::core::Interface::as_raw(self), wzfilename.into_param().abi(), ::core::mem::transmute(pguidvendor.unwrap_or(::std::ptr::null())), dwdesiredaccess, metadataoptions, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDecoderFromStream<P0>(&self, pistream: P0, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapDecoder>();
        (::windows::core::Interface::vtable(self).base__.CreateDecoderFromStream)(::windows::core::Interface::as_raw(self), pistream.into_param().abi(), pguidvendor, metadataoptions, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateDecoderFromFileHandle(&self, hfile: usize, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapDecoder>();
        (::windows::core::Interface::vtable(self).base__.CreateDecoderFromFileHandle)(::windows::core::Interface::as_raw(self), hfile, pguidvendor, metadataoptions, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateComponentInfo(&self, clsidcomponent: *const ::windows::core::GUID) -> ::windows::core::Result<IWICComponentInfo> {
        let mut result__ = ::windows::core::zeroed::<IWICComponentInfo>();
        (::windows::core::Interface::vtable(self).base__.CreateComponentInfo)(::windows::core::Interface::as_raw(self), clsidcomponent, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateDecoder(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICBitmapDecoder> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapDecoder>();
        (::windows::core::Interface::vtable(self).base__.CreateDecoder)(::windows::core::Interface::as_raw(self), guidcontainerformat, pguidvendor, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateEncoder(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICBitmapEncoder> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapEncoder>();
        (::windows::core::Interface::vtable(self).base__.CreateEncoder)(::windows::core::Interface::as_raw(self), guidcontainerformat, pguidvendor, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreatePalette(&self) -> ::windows::core::Result<IWICPalette> {
        let mut result__ = ::windows::core::zeroed::<IWICPalette>();
        (::windows::core::Interface::vtable(self).base__.CreatePalette)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateFormatConverter(&self) -> ::windows::core::Result<IWICFormatConverter> {
        let mut result__ = ::windows::core::zeroed::<IWICFormatConverter>();
        (::windows::core::Interface::vtable(self).base__.CreateFormatConverter)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBitmapScaler(&self) -> ::windows::core::Result<IWICBitmapScaler> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapScaler>();
        (::windows::core::Interface::vtable(self).base__.CreateBitmapScaler)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBitmapClipper(&self) -> ::windows::core::Result<IWICBitmapClipper> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapClipper>();
        (::windows::core::Interface::vtable(self).base__.CreateBitmapClipper)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBitmapFlipRotator(&self) -> ::windows::core::Result<IWICBitmapFlipRotator> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapFlipRotator>();
        (::windows::core::Interface::vtable(self).base__.CreateBitmapFlipRotator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStream(&self) -> ::windows::core::Result<IWICStream> {
        let mut result__ = ::windows::core::zeroed::<IWICStream>();
        (::windows::core::Interface::vtable(self).base__.CreateStream)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateColorContext(&self) -> ::windows::core::Result<IWICColorContext> {
        let mut result__ = ::windows::core::zeroed::<IWICColorContext>();
        (::windows::core::Interface::vtable(self).base__.CreateColorContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateColorTransformer(&self) -> ::windows::core::Result<IWICColorTransform> {
        let mut result__ = ::windows::core::zeroed::<IWICColorTransform>();
        (::windows::core::Interface::vtable(self).base__.CreateColorTransformer)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBitmap(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, option: WICBitmapCreateCacheOption) -> ::windows::core::Result<IWICBitmap> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmap>();
        (::windows::core::Interface::vtable(self).base__.CreateBitmap)(::windows::core::Interface::as_raw(self), uiwidth, uiheight, pixelformat, option, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBitmapFromSource<P0>(&self, pibitmapsource: P0, option: WICBitmapCreateCacheOption) -> ::windows::core::Result<IWICBitmap>
    where
        P0: ::windows::core::IntoParam<IWICBitmapSource>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICBitmap>();
        (::windows::core::Interface::vtable(self).base__.CreateBitmapFromSource)(::windows::core::Interface::as_raw(self), pibitmapsource.into_param().abi(), option, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBitmapFromSourceRect<P0>(&self, pibitmapsource: P0, x: u32, y: u32, width: u32, height: u32) -> ::windows::core::Result<IWICBitmap>
    where
        P0: ::windows::core::IntoParam<IWICBitmapSource>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICBitmap>();
        (::windows::core::Interface::vtable(self).base__.CreateBitmapFromSourceRect)(::windows::core::Interface::as_raw(self), pibitmapsource.into_param().abi(), x, y, width, height, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBitmapFromMemory(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, cbstride: u32, pbbuffer: &[u8]) -> ::windows::core::Result<IWICBitmap> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmap>();
        (::windows::core::Interface::vtable(self).base__.CreateBitmapFromMemory)(::windows::core::Interface::as_raw(self), uiwidth, uiheight, pixelformat, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr()), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateBitmapFromHBITMAP<P0, P1>(&self, hbitmap: P0, hpalette: P1, options: WICBitmapAlphaChannelOption) -> ::windows::core::Result<IWICBitmap>
    where
        P0: ::windows::core::IntoParam<super::Gdi::HBITMAP>,
        P1: ::windows::core::IntoParam<super::Gdi::HPALETTE>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICBitmap>();
        (::windows::core::Interface::vtable(self).base__.CreateBitmapFromHBITMAP)(::windows::core::Interface::as_raw(self), hbitmap.into_param().abi(), hpalette.into_param().abi(), options, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn CreateBitmapFromHICON<P0>(&self, hicon: P0) -> ::windows::core::Result<IWICBitmap>
    where
        P0: ::windows::core::IntoParam<super::super::UI::WindowsAndMessaging::HICON>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICBitmap>();
        (::windows::core::Interface::vtable(self).base__.CreateBitmapFromHICON)(::windows::core::Interface::as_raw(self), hicon.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateComponentEnumerator(&self, componenttypes: u32, options: u32) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IEnumUnknown>();
        (::windows::core::Interface::vtable(self).base__.CreateComponentEnumerator)(::windows::core::Interface::as_raw(self), componenttypes, options, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateFastMetadataEncoderFromDecoder<P0>(&self, pidecoder: P0) -> ::windows::core::Result<IWICFastMetadataEncoder>
    where
        P0: ::windows::core::IntoParam<IWICBitmapDecoder>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICFastMetadataEncoder>();
        (::windows::core::Interface::vtable(self).base__.CreateFastMetadataEncoderFromDecoder)(::windows::core::Interface::as_raw(self), pidecoder.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateFastMetadataEncoderFromFrameDecode<P0>(&self, piframedecoder: P0) -> ::windows::core::Result<IWICFastMetadataEncoder>
    where
        P0: ::windows::core::IntoParam<IWICBitmapFrameDecode>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICFastMetadataEncoder>();
        (::windows::core::Interface::vtable(self).base__.CreateFastMetadataEncoderFromFrameDecode)(::windows::core::Interface::as_raw(self), piframedecoder.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateQueryWriter(&self, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICMetadataQueryWriter> {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataQueryWriter>();
        (::windows::core::Interface::vtable(self).base__.CreateQueryWriter)(::windows::core::Interface::as_raw(self), guidmetadataformat, pguidvendor, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateQueryWriterFromReader<P0>(&self, piqueryreader: P0, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICMetadataQueryWriter>
    where
        P0: ::windows::core::IntoParam<IWICMetadataQueryReader>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataQueryWriter>();
        (::windows::core::Interface::vtable(self).base__.CreateQueryWriterFromReader)(::windows::core::Interface::as_raw(self), piqueryreader.into_param().abi(), pguidvendor, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateMetadataReader<P0>(&self, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwoptions: u32, pistream: P0) -> ::windows::core::Result<IWICMetadataReader>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataReader>();
        (::windows::core::Interface::vtable(self).CreateMetadataReader)(::windows::core::Interface::as_raw(self), guidmetadataformat, pguidvendor, dwoptions, pistream.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateMetadataReaderFromContainer<P0>(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwoptions: u32, pistream: P0) -> ::windows::core::Result<IWICMetadataReader>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataReader>();
        (::windows::core::Interface::vtable(self).CreateMetadataReaderFromContainer)(::windows::core::Interface::as_raw(self), guidcontainerformat, pguidvendor, dwoptions, pistream.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateMetadataWriter(&self, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwmetadataoptions: u32) -> ::windows::core::Result<IWICMetadataWriter> {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataWriter>();
        (::windows::core::Interface::vtable(self).CreateMetadataWriter)(::windows::core::Interface::as_raw(self), guidmetadataformat, pguidvendor, dwmetadataoptions, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateMetadataWriterFromReader<P0>(&self, pireader: P0, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICMetadataWriter>
    where
        P0: ::windows::core::IntoParam<IWICMetadataReader>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataWriter>();
        (::windows::core::Interface::vtable(self).CreateMetadataWriterFromReader)(::windows::core::Interface::as_raw(self), pireader.into_param().abi(), pguidvendor, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateQueryReaderFromBlockReader<P0>(&self, piblockreader: P0) -> ::windows::core::Result<IWICMetadataQueryReader>
    where
        P0: ::windows::core::IntoParam<IWICMetadataBlockReader>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataQueryReader>();
        (::windows::core::Interface::vtable(self).CreateQueryReaderFromBlockReader)(::windows::core::Interface::as_raw(self), piblockreader.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateQueryWriterFromBlockWriter<P0>(&self, piblockwriter: P0) -> ::windows::core::Result<IWICMetadataQueryWriter>
    where
        P0: ::windows::core::IntoParam<IWICMetadataBlockWriter>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataQueryWriter>();
        (::windows::core::Interface::vtable(self).CreateQueryWriterFromBlockWriter)(::windows::core::Interface::as_raw(self), piblockwriter.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn CreateEncoderPropertyBag(&self, ppropoptions: &[super::super::System::Com::StructuredStorage::PROPBAG2]) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::IPropertyBag2> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::StructuredStorage::IPropertyBag2>();
        (::windows::core::Interface::vtable(self).CreateEncoderPropertyBag)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppropoptions.as_ptr()), ppropoptions.len() as _, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICComponentFactory, ::windows::core::IUnknown, IWICImagingFactory);
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
unsafe impl ::windows::core::Interface for IWICComponentFactory {
    type Vtable = IWICComponentFactory_Vtbl;
}
impl ::core::clone::Clone for IWICComponentFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICComponentFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x412d0c3a_9650_44fa_af5b_dd2a06c8e8fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICComponentFactory_Vtbl {
    pub base__: IWICImagingFactory_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateMetadataReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwoptions: u32, pistream: *mut ::core::ffi::c_void, ppireader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateMetadataReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateMetadataReaderFromContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwoptions: u32, pistream: *mut ::core::ffi::c_void, ppireader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateMetadataReaderFromContainer: usize,
    pub CreateMetadataWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, dwmetadataoptions: u32, ppiwriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateMetadataWriterFromReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pireader: *mut ::core::ffi::c_void, pguidvendor: *const ::windows::core::GUID, ppiwriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateQueryReaderFromBlockReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piblockreader: *mut ::core::ffi::c_void, ppiqueryreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateQueryWriterFromBlockWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piblockwriter: *mut ::core::ffi::c_void, ppiquerywriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub CreateEncoderPropertyBag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppropoptions: *const super::super::System::Com::StructuredStorage::PROPBAG2, ccount: u32, ppipropertybag: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    CreateEncoderPropertyBag: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICComponentInfo(::windows::core::IUnknown);
impl IWICComponentInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__ = ::windows::core::zeroed::<WICComponentType>();
        (::windows::core::Interface::vtable(self).GetComponentType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetCLSID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetSigningStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAuthor(&self, wzauthor: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAuthor)(::windows::core::Interface::as_raw(self), wzauthor.len() as _, ::core::mem::transmute(wzauthor.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetVendorGUID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self, wzversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVersion)(::windows::core::Interface::as_raw(self), wzversion.len() as _, ::core::mem::transmute(wzversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetSpecVersion(&self, wzspecversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSpecVersion)(::windows::core::Interface::as_raw(self), wzspecversion.len() as _, ::core::mem::transmute(wzspecversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFriendlyName(&self, wzfriendlyname: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFriendlyName)(::windows::core::Interface::as_raw(self), wzfriendlyname.len() as _, ::core::mem::transmute(wzfriendlyname.as_ptr()), pcchactual).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICComponentInfo, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICComponentInfo {
    type Vtable = IWICComponentInfo_Vtbl;
}
impl ::core::clone::Clone for IWICComponentInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICComponentInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23bc3f0a_698b_4357_886b_f24d50671334);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICComponentInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetComponentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptype: *mut WICComponentType) -> ::windows::core::HRESULT,
    pub GetCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetSigningStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstatus: *mut u32) -> ::windows::core::HRESULT,
    pub GetAuthor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchauthor: u32, wzauthor: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    pub GetVendorGUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchversion: u32, wzversion: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    pub GetSpecVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchspecversion: u32, wzspecversion: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    pub GetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchfriendlyname: u32, wzfriendlyname: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICDdsDecoder(::windows::core::IUnknown);
impl IWICDdsDecoder {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetParameters(&self, pparameters: *mut WICDdsParameters) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetParameters)(::windows::core::Interface::as_raw(self), pparameters).ok()
    }
    pub unsafe fn GetFrame(&self, arrayindex: u32, miplevel: u32, sliceindex: u32) -> ::windows::core::Result<IWICBitmapFrameDecode> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapFrameDecode>();
        (::windows::core::Interface::vtable(self).GetFrame)(::windows::core::Interface::as_raw(self), arrayindex, miplevel, sliceindex, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICDdsDecoder, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICDdsDecoder {
    type Vtable = IWICDdsDecoder_Vtbl;
}
impl ::core::clone::Clone for IWICDdsDecoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICDdsDecoder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x409cd537_8532_40cb_9774_e2feb2df4e9c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICDdsDecoder_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparameters: *mut WICDdsParameters) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetParameters: usize,
    pub GetFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arrayindex: u32, miplevel: u32, sliceindex: u32, ppibitmapframe: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICDdsEncoder(::windows::core::IUnknown);
impl IWICDdsEncoder {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetParameters(&self, pparameters: *const WICDdsParameters) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetParameters)(::windows::core::Interface::as_raw(self), pparameters).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetParameters(&self, pparameters: *mut WICDdsParameters) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetParameters)(::windows::core::Interface::as_raw(self), pparameters).ok()
    }
    pub unsafe fn CreateNewFrame(&self, ppiframeencode: *mut ::core::option::Option<IWICBitmapFrameEncode>, parrayindex: *mut u32, pmiplevel: *mut u32, psliceindex: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CreateNewFrame)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppiframeencode), parrayindex, pmiplevel, psliceindex).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICDdsEncoder, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICDdsEncoder {
    type Vtable = IWICDdsEncoder_Vtbl;
}
impl ::core::clone::Clone for IWICDdsEncoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICDdsEncoder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cacdb4c_407e_41b3_b936_d0f010cd6732);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICDdsEncoder_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparameters: *const WICDdsParameters) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetParameters: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pparameters: *mut WICDdsParameters) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetParameters: usize,
    pub CreateNewFrame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiframeencode: *mut *mut ::core::ffi::c_void, parrayindex: *mut u32, pmiplevel: *mut u32, psliceindex: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICDdsFrameDecode(::windows::core::IUnknown);
impl IWICDdsFrameDecode {
    pub unsafe fn GetSizeInBlocks(&self, pwidthinblocks: *mut u32, pheightinblocks: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSizeInBlocks)(::windows::core::Interface::as_raw(self), pwidthinblocks, pheightinblocks).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFormatInfo(&self) -> ::windows::core::Result<WICDdsFormatInfo> {
        let mut result__ = ::windows::core::zeroed::<WICDdsFormatInfo>();
        (::windows::core::Interface::vtable(self).GetFormatInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CopyBlocks(&self, prcboundsinblocks: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopyBlocks)(::windows::core::Interface::as_raw(self), prcboundsinblocks, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICDdsFrameDecode, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICDdsFrameDecode {
    type Vtable = IWICDdsFrameDecode_Vtbl;
}
impl ::core::clone::Clone for IWICDdsFrameDecode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICDdsFrameDecode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d4c0c61_18a4_41e4_bd80_481a4fc9f464);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICDdsFrameDecode_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSizeInBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwidthinblocks: *mut u32, pheightinblocks: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetFormatInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformatinfo: *mut WICDdsFormatInfo) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetFormatInfo: usize,
    pub CopyBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcboundsinblocks: *const WICRect, cbstride: u32, cbbuffersize: u32, pbbuffer: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICDevelopRaw(::windows::core::IUnknown);
impl IWICDevelopRaw {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetSize)(::windows::core::Interface::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.base__.GetPixelFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetResolution)(::windows::core::Interface::as_raw(self), pdpix, pdpiy).ok()
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICPalette>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.CopyPalette)(::windows::core::Interface::as_raw(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.CopyPixels)(::windows::core::Interface::as_raw(self), prc, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
    pub unsafe fn GetMetadataQueryReader(&self) -> ::windows::core::Result<IWICMetadataQueryReader> {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataQueryReader>();
        (::windows::core::Interface::vtable(self).base__.GetMetadataQueryReader)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColorContexts(&self, ppicolorcontexts: &mut [::core::option::Option<IWICColorContext>], pcactualcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetColorContexts)(::windows::core::Interface::as_raw(self), ppicolorcontexts.len() as _, ::core::mem::transmute(ppicolorcontexts.as_ptr()), pcactualcount).ok()
    }
    pub unsafe fn GetThumbnail(&self) -> ::windows::core::Result<IWICBitmapSource> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapSource>();
        (::windows::core::Interface::vtable(self).base__.GetThumbnail)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryRawCapabilitiesInfo(&self, pinfo: *mut WICRawCapabilitiesInfo) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryRawCapabilitiesInfo)(::windows::core::Interface::as_raw(self), pinfo).ok()
    }
    pub unsafe fn LoadParameterSet(&self, parameterset: WICRawParameterSet) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadParameterSet)(::windows::core::Interface::as_raw(self), parameterset).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub unsafe fn GetCurrentParameterSet(&self) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::IPropertyBag2> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::StructuredStorage::IPropertyBag2>();
        (::windows::core::Interface::vtable(self).GetCurrentParameterSet)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetExposureCompensation(&self, ev: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetExposureCompensation)(::windows::core::Interface::as_raw(self), ev).ok()
    }
    pub unsafe fn GetExposureCompensation(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::windows::core::zeroed::<f64>();
        (::windows::core::Interface::vtable(self).GetExposureCompensation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetWhitePointRGB(&self, red: u32, green: u32, blue: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWhitePointRGB)(::windows::core::Interface::as_raw(self), red, green, blue).ok()
    }
    pub unsafe fn GetWhitePointRGB(&self, pred: *mut u32, pgreen: *mut u32, pblue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetWhitePointRGB)(::windows::core::Interface::as_raw(self), pred, pgreen, pblue).ok()
    }
    pub unsafe fn SetNamedWhitePoint(&self, whitepoint: WICNamedWhitePoint) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNamedWhitePoint)(::windows::core::Interface::as_raw(self), whitepoint).ok()
    }
    pub unsafe fn GetNamedWhitePoint(&self) -> ::windows::core::Result<WICNamedWhitePoint> {
        let mut result__ = ::windows::core::zeroed::<WICNamedWhitePoint>();
        (::windows::core::Interface::vtable(self).GetNamedWhitePoint)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetWhitePointKelvin(&self, whitepointkelvin: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWhitePointKelvin)(::windows::core::Interface::as_raw(self), whitepointkelvin).ok()
    }
    pub unsafe fn GetWhitePointKelvin(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetWhitePointKelvin)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetKelvinRangeInfo(&self, pminkelvintemp: *mut u32, pmaxkelvintemp: *mut u32, pkelvintempstepvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetKelvinRangeInfo)(::windows::core::Interface::as_raw(self), pminkelvintemp, pmaxkelvintemp, pkelvintempstepvalue).ok()
    }
    pub unsafe fn SetContrast(&self, contrast: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetContrast)(::windows::core::Interface::as_raw(self), contrast).ok()
    }
    pub unsafe fn GetContrast(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::windows::core::zeroed::<f64>();
        (::windows::core::Interface::vtable(self).GetContrast)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetGamma(&self, gamma: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGamma)(::windows::core::Interface::as_raw(self), gamma).ok()
    }
    pub unsafe fn GetGamma(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::windows::core::zeroed::<f64>();
        (::windows::core::Interface::vtable(self).GetGamma)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSharpness(&self, sharpness: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSharpness)(::windows::core::Interface::as_raw(self), sharpness).ok()
    }
    pub unsafe fn GetSharpness(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::windows::core::zeroed::<f64>();
        (::windows::core::Interface::vtable(self).GetSharpness)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSaturation(&self, saturation: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSaturation)(::windows::core::Interface::as_raw(self), saturation).ok()
    }
    pub unsafe fn GetSaturation(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::windows::core::zeroed::<f64>();
        (::windows::core::Interface::vtable(self).GetSaturation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTint(&self, tint: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTint)(::windows::core::Interface::as_raw(self), tint).ok()
    }
    pub unsafe fn GetTint(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::windows::core::zeroed::<f64>();
        (::windows::core::Interface::vtable(self).GetTint)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNoiseReduction(&self, noisereduction: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNoiseReduction)(::windows::core::Interface::as_raw(self), noisereduction).ok()
    }
    pub unsafe fn GetNoiseReduction(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::windows::core::zeroed::<f64>();
        (::windows::core::Interface::vtable(self).GetNoiseReduction)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDestinationColorContext<P0>(&self, pcolorcontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICColorContext>,
    {
        (::windows::core::Interface::vtable(self).SetDestinationColorContext)(::windows::core::Interface::as_raw(self), pcolorcontext.into_param().abi()).ok()
    }
    pub unsafe fn SetToneCurve(&self, cbtonecurvesize: u32, ptonecurve: *const WICRawToneCurve) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetToneCurve)(::windows::core::Interface::as_raw(self), cbtonecurvesize, ptonecurve).ok()
    }
    pub unsafe fn GetToneCurve(&self, cbtonecurvebuffersize: u32, ptonecurve: ::core::option::Option<*mut WICRawToneCurve>, pcbactualtonecurvebuffersize: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetToneCurve)(::windows::core::Interface::as_raw(self), cbtonecurvebuffersize, ::core::mem::transmute(ptonecurve.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbactualtonecurvebuffersize.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetRotation(&self, rotation: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRotation)(::windows::core::Interface::as_raw(self), rotation).ok()
    }
    pub unsafe fn GetRotation(&self) -> ::windows::core::Result<f64> {
        let mut result__ = ::windows::core::zeroed::<f64>();
        (::windows::core::Interface::vtable(self).GetRotation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRenderMode(&self, rendermode: WICRawRenderMode) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRenderMode)(::windows::core::Interface::as_raw(self), rendermode).ok()
    }
    pub unsafe fn GetRenderMode(&self) -> ::windows::core::Result<WICRawRenderMode> {
        let mut result__ = ::windows::core::zeroed::<WICRawRenderMode>();
        (::windows::core::Interface::vtable(self).GetRenderMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetNotificationCallback<P0>(&self, pcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICDevelopRawNotificationCallback>,
    {
        (::windows::core::Interface::vtable(self).SetNotificationCallback)(::windows::core::Interface::as_raw(self), pcallback.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICDevelopRaw, ::windows::core::IUnknown, IWICBitmapSource, IWICBitmapFrameDecode);
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
unsafe impl ::windows::core::Interface for IWICDevelopRaw {
    type Vtable = IWICDevelopRaw_Vtbl;
}
impl ::core::clone::Clone for IWICDevelopRaw {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICDevelopRaw {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbec5e44_f7be_4b65_b7f8_c0c81fef026d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICDevelopRaw_Vtbl {
    pub base__: IWICBitmapFrameDecode_Vtbl,
    pub QueryRawCapabilitiesInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *mut WICRawCapabilitiesInfo) -> ::windows::core::HRESULT,
    pub LoadParameterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parameterset: WICRawParameterSet) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub GetCurrentParameterSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcurrentparameterset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com_StructuredStorage"))]
    GetCurrentParameterSet: usize,
    pub SetExposureCompensation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ev: f64) -> ::windows::core::HRESULT,
    pub GetExposureCompensation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pev: *mut f64) -> ::windows::core::HRESULT,
    pub SetWhitePointRGB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, red: u32, green: u32, blue: u32) -> ::windows::core::HRESULT,
    pub GetWhitePointRGB: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pred: *mut u32, pgreen: *mut u32, pblue: *mut u32) -> ::windows::core::HRESULT,
    pub SetNamedWhitePoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, whitepoint: WICNamedWhitePoint) -> ::windows::core::HRESULT,
    pub GetNamedWhitePoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwhitepoint: *mut WICNamedWhitePoint) -> ::windows::core::HRESULT,
    pub SetWhitePointKelvin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, whitepointkelvin: u32) -> ::windows::core::HRESULT,
    pub GetWhitePointKelvin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pwhitepointkelvin: *mut u32) -> ::windows::core::HRESULT,
    pub GetKelvinRangeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pminkelvintemp: *mut u32, pmaxkelvintemp: *mut u32, pkelvintempstepvalue: *mut u32) -> ::windows::core::HRESULT,
    pub SetContrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contrast: f64) -> ::windows::core::HRESULT,
    pub GetContrast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontrast: *mut f64) -> ::windows::core::HRESULT,
    pub SetGamma: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gamma: f64) -> ::windows::core::HRESULT,
    pub GetGamma: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pgamma: *mut f64) -> ::windows::core::HRESULT,
    pub SetSharpness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharpness: f64) -> ::windows::core::HRESULT,
    pub GetSharpness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psharpness: *mut f64) -> ::windows::core::HRESULT,
    pub SetSaturation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, saturation: f64) -> ::windows::core::HRESULT,
    pub GetSaturation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psaturation: *mut f64) -> ::windows::core::HRESULT,
    pub SetTint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tint: f64) -> ::windows::core::HRESULT,
    pub GetTint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptint: *mut f64) -> ::windows::core::HRESULT,
    pub SetNoiseReduction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noisereduction: f64) -> ::windows::core::HRESULT,
    pub GetNoiseReduction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnoisereduction: *mut f64) -> ::windows::core::HRESULT,
    pub SetDestinationColorContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolorcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetToneCurve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbtonecurvesize: u32, ptonecurve: *const WICRawToneCurve) -> ::windows::core::HRESULT,
    pub GetToneCurve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbtonecurvebuffersize: u32, ptonecurve: *mut WICRawToneCurve, pcbactualtonecurvebuffersize: *mut u32) -> ::windows::core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotation: f64) -> ::windows::core::HRESULT,
    pub GetRotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, protation: *mut f64) -> ::windows::core::HRESULT,
    pub SetRenderMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rendermode: WICRawRenderMode) -> ::windows::core::HRESULT,
    pub GetRenderMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prendermode: *mut WICRawRenderMode) -> ::windows::core::HRESULT,
    pub SetNotificationCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICDevelopRawNotificationCallback(::windows::core::IUnknown);
impl IWICDevelopRawNotificationCallback {
    pub unsafe fn Notify(&self, notificationmask: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Notify)(::windows::core::Interface::as_raw(self), notificationmask).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICDevelopRawNotificationCallback, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICDevelopRawNotificationCallback {
    type Vtable = IWICDevelopRawNotificationCallback_Vtbl;
}
impl ::core::clone::Clone for IWICDevelopRawNotificationCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICDevelopRawNotificationCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95c75a6e_3e8c_4ec2_85a8_aebcc551e59b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICDevelopRawNotificationCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, notificationmask: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICEnumMetadataItem(::windows::core::IUnknown);
impl IWICEnumMetadataItem {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn Next(&self, celt: u32, rgeltschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), celt, rgeltschema, rgeltid, rgeltvalue, pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IWICEnumMetadataItem> {
        let mut result__ = ::windows::core::zeroed::<IWICEnumMetadataItem>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICEnumMetadataItem, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICEnumMetadataItem {
    type Vtable = IWICEnumMetadataItem_Vtbl;
}
impl ::core::clone::Clone for IWICEnumMetadataItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICEnumMetadataItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc2bb46d_3f07_481e_8625_220c4aedbb33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICEnumMetadataItem_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgeltschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, rgeltvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienummetadataitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICFastMetadataEncoder(::windows::core::IUnknown);
impl IWICFastMetadataEncoder {
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetMetadataQueryWriter(&self) -> ::windows::core::Result<IWICMetadataQueryWriter> {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataQueryWriter>();
        (::windows::core::Interface::vtable(self).GetMetadataQueryWriter)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICFastMetadataEncoder, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICFastMetadataEncoder {
    type Vtable = IWICFastMetadataEncoder_Vtbl;
}
impl ::core::clone::Clone for IWICFastMetadataEncoder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICFastMetadataEncoder {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb84e2c09_78c9_4ac4_8bd3_524ae1663a2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICFastMetadataEncoder_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMetadataQueryWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimetadataquerywriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICFormatConverter(::windows::core::IUnknown);
impl IWICFormatConverter {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSize)(::windows::core::Interface::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetPixelFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetResolution)(::windows::core::Interface::as_raw(self), pdpix, pdpiy).ok()
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICPalette>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyPalette)(::windows::core::Interface::as_raw(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CopyPixels)(::windows::core::Interface::as_raw(self), prc, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
    pub unsafe fn Initialize<P0, P1>(&self, pisource: P0, dstformat: *const ::windows::core::GUID, dither: WICBitmapDitherType, pipalette: P1, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICBitmapSource>,
        P1: ::windows::core::IntoParam<IWICPalette>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pisource.into_param().abi(), dstformat, dither, pipalette.into_param().abi(), alphathresholdpercent, palettetranslate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanConvert(&self, srcpixelformat: *const ::windows::core::GUID, dstpixelformat: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).CanConvert)(::windows::core::Interface::as_raw(self), srcpixelformat, dstpixelformat, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICFormatConverter, ::windows::core::IUnknown, IWICBitmapSource);
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
unsafe impl ::windows::core::Interface for IWICFormatConverter {
    type Vtable = IWICFormatConverter_Vtbl;
}
impl ::core::clone::Clone for IWICFormatConverter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICFormatConverter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000301_a8f2_4877_ba0a_fd2b6645fb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICFormatConverter_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisource: *mut ::core::ffi::c_void, dstformat: *const ::windows::core::GUID, dither: WICBitmapDitherType, pipalette: *mut ::core::ffi::c_void, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CanConvert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, srcpixelformat: *const ::windows::core::GUID, dstpixelformat: *const ::windows::core::GUID, pfcanconvert: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanConvert: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICFormatConverterInfo(::windows::core::IUnknown);
impl IWICFormatConverterInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__ = ::windows::core::zeroed::<WICComponentType>();
        (::windows::core::Interface::vtable(self).base__.GetComponentType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetCLSID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetSigningStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAuthor(&self, wzauthor: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAuthor)(::windows::core::Interface::as_raw(self), wzauthor.len() as _, ::core::mem::transmute(wzauthor.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetVendorGUID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self, wzversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetVersion)(::windows::core::Interface::as_raw(self), wzversion.len() as _, ::core::mem::transmute(wzversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetSpecVersion(&self, wzspecversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSpecVersion)(::windows::core::Interface::as_raw(self), wzspecversion.len() as _, ::core::mem::transmute(wzspecversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFriendlyName(&self, wzfriendlyname: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFriendlyName)(::windows::core::Interface::as_raw(self), wzfriendlyname.len() as _, ::core::mem::transmute(wzfriendlyname.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetPixelFormats(&self, ppixelformatguids: &mut [::windows::core::GUID], pcactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPixelFormats)(::windows::core::Interface::as_raw(self), ppixelformatguids.len() as _, ::core::mem::transmute(ppixelformatguids.as_ptr()), pcactual).ok()
    }
    pub unsafe fn CreateInstance(&self) -> ::windows::core::Result<IWICFormatConverter> {
        let mut result__ = ::windows::core::zeroed::<IWICFormatConverter>();
        (::windows::core::Interface::vtable(self).CreateInstance)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICFormatConverterInfo, ::windows::core::IUnknown, IWICComponentInfo);
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
unsafe impl ::windows::core::Interface for IWICFormatConverterInfo {
    type Vtable = IWICFormatConverterInfo_Vtbl;
}
impl ::core::clone::Clone for IWICFormatConverterInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICFormatConverterInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f34fb65_13f4_4f15_bc57_3726b5e53d9f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICFormatConverterInfo_Vtbl {
    pub base__: IWICComponentInfo_Vtbl,
    pub GetPixelFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cformats: u32, ppixelformatguids: *mut ::windows::core::GUID, pcactual: *mut u32) -> ::windows::core::HRESULT,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiconverter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICImagingFactory(::windows::core::IUnknown);
impl IWICImagingFactory {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDecoderFromFilename<P0>(&self, wzfilename: P0, pguidvendor: ::core::option::Option<*const ::windows::core::GUID>, dwdesiredaccess: super::super::Foundation::GENERIC_ACCESS_RIGHTS, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapDecoder>();
        (::windows::core::Interface::vtable(self).CreateDecoderFromFilename)(::windows::core::Interface::as_raw(self), wzfilename.into_param().abi(), ::core::mem::transmute(pguidvendor.unwrap_or(::std::ptr::null())), dwdesiredaccess, metadataoptions, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDecoderFromStream<P0>(&self, pistream: P0, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapDecoder>();
        (::windows::core::Interface::vtable(self).CreateDecoderFromStream)(::windows::core::Interface::as_raw(self), pistream.into_param().abi(), pguidvendor, metadataoptions, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateDecoderFromFileHandle(&self, hfile: usize, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions) -> ::windows::core::Result<IWICBitmapDecoder> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapDecoder>();
        (::windows::core::Interface::vtable(self).CreateDecoderFromFileHandle)(::windows::core::Interface::as_raw(self), hfile, pguidvendor, metadataoptions, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateComponentInfo(&self, clsidcomponent: *const ::windows::core::GUID) -> ::windows::core::Result<IWICComponentInfo> {
        let mut result__ = ::windows::core::zeroed::<IWICComponentInfo>();
        (::windows::core::Interface::vtable(self).CreateComponentInfo)(::windows::core::Interface::as_raw(self), clsidcomponent, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateDecoder(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICBitmapDecoder> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapDecoder>();
        (::windows::core::Interface::vtable(self).CreateDecoder)(::windows::core::Interface::as_raw(self), guidcontainerformat, pguidvendor, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateEncoder(&self, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICBitmapEncoder> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapEncoder>();
        (::windows::core::Interface::vtable(self).CreateEncoder)(::windows::core::Interface::as_raw(self), guidcontainerformat, pguidvendor, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreatePalette(&self) -> ::windows::core::Result<IWICPalette> {
        let mut result__ = ::windows::core::zeroed::<IWICPalette>();
        (::windows::core::Interface::vtable(self).CreatePalette)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateFormatConverter(&self) -> ::windows::core::Result<IWICFormatConverter> {
        let mut result__ = ::windows::core::zeroed::<IWICFormatConverter>();
        (::windows::core::Interface::vtable(self).CreateFormatConverter)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBitmapScaler(&self) -> ::windows::core::Result<IWICBitmapScaler> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapScaler>();
        (::windows::core::Interface::vtable(self).CreateBitmapScaler)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBitmapClipper(&self) -> ::windows::core::Result<IWICBitmapClipper> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapClipper>();
        (::windows::core::Interface::vtable(self).CreateBitmapClipper)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBitmapFlipRotator(&self) -> ::windows::core::Result<IWICBitmapFlipRotator> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmapFlipRotator>();
        (::windows::core::Interface::vtable(self).CreateBitmapFlipRotator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateStream(&self) -> ::windows::core::Result<IWICStream> {
        let mut result__ = ::windows::core::zeroed::<IWICStream>();
        (::windows::core::Interface::vtable(self).CreateStream)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateColorContext(&self) -> ::windows::core::Result<IWICColorContext> {
        let mut result__ = ::windows::core::zeroed::<IWICColorContext>();
        (::windows::core::Interface::vtable(self).CreateColorContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateColorTransformer(&self) -> ::windows::core::Result<IWICColorTransform> {
        let mut result__ = ::windows::core::zeroed::<IWICColorTransform>();
        (::windows::core::Interface::vtable(self).CreateColorTransformer)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBitmap(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, option: WICBitmapCreateCacheOption) -> ::windows::core::Result<IWICBitmap> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmap>();
        (::windows::core::Interface::vtable(self).CreateBitmap)(::windows::core::Interface::as_raw(self), uiwidth, uiheight, pixelformat, option, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBitmapFromSource<P0>(&self, pibitmapsource: P0, option: WICBitmapCreateCacheOption) -> ::windows::core::Result<IWICBitmap>
    where
        P0: ::windows::core::IntoParam<IWICBitmapSource>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICBitmap>();
        (::windows::core::Interface::vtable(self).CreateBitmapFromSource)(::windows::core::Interface::as_raw(self), pibitmapsource.into_param().abi(), option, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBitmapFromSourceRect<P0>(&self, pibitmapsource: P0, x: u32, y: u32, width: u32, height: u32) -> ::windows::core::Result<IWICBitmap>
    where
        P0: ::windows::core::IntoParam<IWICBitmapSource>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICBitmap>();
        (::windows::core::Interface::vtable(self).CreateBitmapFromSourceRect)(::windows::core::Interface::as_raw(self), pibitmapsource.into_param().abi(), x, y, width, height, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateBitmapFromMemory(&self, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, cbstride: u32, pbbuffer: &[u8]) -> ::windows::core::Result<IWICBitmap> {
        let mut result__ = ::windows::core::zeroed::<IWICBitmap>();
        (::windows::core::Interface::vtable(self).CreateBitmapFromMemory)(::windows::core::Interface::as_raw(self), uiwidth, uiheight, pixelformat, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr()), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreateBitmapFromHBITMAP<P0, P1>(&self, hbitmap: P0, hpalette: P1, options: WICBitmapAlphaChannelOption) -> ::windows::core::Result<IWICBitmap>
    where
        P0: ::windows::core::IntoParam<super::Gdi::HBITMAP>,
        P1: ::windows::core::IntoParam<super::Gdi::HPALETTE>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICBitmap>();
        (::windows::core::Interface::vtable(self).CreateBitmapFromHBITMAP)(::windows::core::Interface::as_raw(self), hbitmap.into_param().abi(), hpalette.into_param().abi(), options, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn CreateBitmapFromHICON<P0>(&self, hicon: P0) -> ::windows::core::Result<IWICBitmap>
    where
        P0: ::windows::core::IntoParam<super::super::UI::WindowsAndMessaging::HICON>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICBitmap>();
        (::windows::core::Interface::vtable(self).CreateBitmapFromHICON)(::windows::core::Interface::as_raw(self), hicon.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateComponentEnumerator(&self, componenttypes: u32, options: u32) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IEnumUnknown>();
        (::windows::core::Interface::vtable(self).CreateComponentEnumerator)(::windows::core::Interface::as_raw(self), componenttypes, options, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateFastMetadataEncoderFromDecoder<P0>(&self, pidecoder: P0) -> ::windows::core::Result<IWICFastMetadataEncoder>
    where
        P0: ::windows::core::IntoParam<IWICBitmapDecoder>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICFastMetadataEncoder>();
        (::windows::core::Interface::vtable(self).CreateFastMetadataEncoderFromDecoder)(::windows::core::Interface::as_raw(self), pidecoder.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateFastMetadataEncoderFromFrameDecode<P0>(&self, piframedecoder: P0) -> ::windows::core::Result<IWICFastMetadataEncoder>
    where
        P0: ::windows::core::IntoParam<IWICBitmapFrameDecode>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICFastMetadataEncoder>();
        (::windows::core::Interface::vtable(self).CreateFastMetadataEncoderFromFrameDecode)(::windows::core::Interface::as_raw(self), piframedecoder.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateQueryWriter(&self, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICMetadataQueryWriter> {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataQueryWriter>();
        (::windows::core::Interface::vtable(self).CreateQueryWriter)(::windows::core::Interface::as_raw(self), guidmetadataformat, pguidvendor, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateQueryWriterFromReader<P0>(&self, piqueryreader: P0, pguidvendor: *const ::windows::core::GUID) -> ::windows::core::Result<IWICMetadataQueryWriter>
    where
        P0: ::windows::core::IntoParam<IWICMetadataQueryReader>,
    {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataQueryWriter>();
        (::windows::core::Interface::vtable(self).CreateQueryWriterFromReader)(::windows::core::Interface::as_raw(self), piqueryreader.into_param().abi(), pguidvendor, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICImagingFactory, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICImagingFactory {
    type Vtable = IWICImagingFactory_Vtbl;
}
impl ::core::clone::Clone for IWICImagingFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICImagingFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec5ec8a9_c395_4314_9c77_54d7a935ff70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICImagingFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateDecoderFromFilename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wzfilename: ::windows::core::PCWSTR, pguidvendor: *const ::windows::core::GUID, dwdesiredaccess: super::super::Foundation::GENERIC_ACCESS_RIGHTS, metadataoptions: WICDecodeOptions, ppidecoder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateDecoderFromFilename: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDecoderFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDecoderFromStream: usize,
    pub CreateDecoderFromFileHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hfile: usize, pguidvendor: *const ::windows::core::GUID, metadataoptions: WICDecodeOptions, ppidecoder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateComponentInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clsidcomponent: *const ::windows::core::GUID, ppiinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateDecoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppidecoder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateEncoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppiencoder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreatePalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppipalette: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFormatConverter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiformatconverter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBitmapScaler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppibitmapscaler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBitmapClipper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppibitmapclipper: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBitmapFlipRotator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppibitmapfliprotator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwicstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateStream: usize,
    pub CreateColorContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwiccolorcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateColorTransformer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwiccolortransform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, option: WICBitmapCreateCacheOption, ppibitmap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBitmapFromSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pibitmapsource: *mut ::core::ffi::c_void, option: WICBitmapCreateCacheOption, ppibitmap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBitmapFromSourceRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pibitmapsource: *mut ::core::ffi::c_void, x: u32, y: u32, width: u32, height: u32, ppibitmap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateBitmapFromMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uiwidth: u32, uiheight: u32, pixelformat: *const ::windows::core::GUID, cbstride: u32, cbbuffersize: u32, pbbuffer: *const u8, ppibitmap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreateBitmapFromHBITMAP: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hbitmap: super::Gdi::HBITMAP, hpalette: super::Gdi::HPALETTE, options: WICBitmapAlphaChannelOption, ppibitmap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreateBitmapFromHBITMAP: usize,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub CreateBitmapFromHICON: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hicon: super::super::UI::WindowsAndMessaging::HICON, ppibitmap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    CreateBitmapFromHICON: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateComponentEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, componenttypes: u32, options: u32, ppienumunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateComponentEnumerator: usize,
    pub CreateFastMetadataEncoderFromDecoder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidecoder: *mut ::core::ffi::c_void, ppifastencoder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateFastMetadataEncoderFromFrameDecode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piframedecoder: *mut ::core::ffi::c_void, ppifastencoder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateQueryWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidmetadataformat: *const ::windows::core::GUID, pguidvendor: *const ::windows::core::GUID, ppiquerywriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateQueryWriterFromReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piqueryreader: *mut ::core::ffi::c_void, pguidvendor: *const ::windows::core::GUID, ppiquerywriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICJpegFrameDecode(::windows::core::IUnknown);
impl IWICJpegFrameDecode {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportIndexing(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).DoesSupportIndexing)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIndexing(&self, options: WICJpegIndexingOptions, horizontalintervalsize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIndexing)(::windows::core::Interface::as_raw(self), options, horizontalintervalsize).ok()
    }
    pub unsafe fn ClearIndexing(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearIndexing)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetAcHuffmanTable(&self, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAcHuffmanTable)(::windows::core::Interface::as_raw(self), scanindex, tableindex, pachuffmantable).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDcHuffmanTable(&self, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDcHuffmanTable)(::windows::core::Interface::as_raw(self), scanindex, tableindex, pdchuffmantable).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetQuantizationTable(&self, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetQuantizationTable)(::windows::core::Interface::as_raw(self), scanindex, tableindex, pquantizationtable).ok()
    }
    pub unsafe fn GetFrameHeader(&self, pframeheader: *mut WICJpegFrameHeader) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFrameHeader)(::windows::core::Interface::as_raw(self), pframeheader).ok()
    }
    pub unsafe fn GetScanHeader(&self, scanindex: u32, pscanheader: *mut WICJpegScanHeader) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetScanHeader)(::windows::core::Interface::as_raw(self), scanindex, pscanheader).ok()
    }
    pub unsafe fn CopyScan(&self, scanindex: u32, scanoffset: u32, pbscandata: &mut [u8], pcbscandataactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopyScan)(::windows::core::Interface::as_raw(self), scanindex, scanoffset, pbscandata.len() as _, ::core::mem::transmute(pbscandata.as_ptr()), pcbscandataactual).ok()
    }
    pub unsafe fn CopyMinimalStream(&self, streamoffset: u32, pbstreamdata: &mut [u8], pcbstreamdataactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopyMinimalStream)(::windows::core::Interface::as_raw(self), streamoffset, pbstreamdata.len() as _, ::core::mem::transmute(pbstreamdata.as_ptr()), pcbstreamdataactual).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICJpegFrameDecode, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICJpegFrameDecode {
    type Vtable = IWICJpegFrameDecode_Vtbl;
}
impl ::core::clone::Clone for IWICJpegFrameDecode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICJpegFrameDecode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8939f66e_c46a_4c21_a9d1_98b327ce1679);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICJpegFrameDecode_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DoesSupportIndexing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfindexingsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoesSupportIndexing: usize,
    pub SetIndexing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: WICJpegIndexingOptions, horizontalintervalsize: u32) -> ::windows::core::HRESULT,
    pub ClearIndexing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetAcHuffmanTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetAcHuffmanTable: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDcHuffmanTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDcHuffmanTable: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetQuantizationTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetQuantizationTable: usize,
    pub GetFrameHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pframeheader: *mut WICJpegFrameHeader) -> ::windows::core::HRESULT,
    pub GetScanHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scanindex: u32, pscanheader: *mut WICJpegScanHeader) -> ::windows::core::HRESULT,
    pub CopyScan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scanindex: u32, scanoffset: u32, cbscandata: u32, pbscandata: *mut u8, pcbscandataactual: *mut u32) -> ::windows::core::HRESULT,
    pub CopyMinimalStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, streamoffset: u32, cbstreamdata: u32, pbstreamdata: *mut u8, pcbstreamdataactual: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICJpegFrameEncode(::windows::core::IUnknown);
impl IWICJpegFrameEncode {
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetAcHuffmanTable(&self, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAcHuffmanTable)(::windows::core::Interface::as_raw(self), scanindex, tableindex, pachuffmantable).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDcHuffmanTable(&self, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDcHuffmanTable)(::windows::core::Interface::as_raw(self), scanindex, tableindex, pdchuffmantable).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetQuantizationTable(&self, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetQuantizationTable)(::windows::core::Interface::as_raw(self), scanindex, tableindex, pquantizationtable).ok()
    }
    pub unsafe fn WriteScan(&self, pbscandata: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteScan)(::windows::core::Interface::as_raw(self), pbscandata.len() as _, ::core::mem::transmute(pbscandata.as_ptr())).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICJpegFrameEncode, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICJpegFrameEncode {
    type Vtable = IWICJpegFrameEncode_Vtbl;
}
impl ::core::clone::Clone for IWICJpegFrameEncode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICJpegFrameEncode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2f0c601f_d2c6_468c_abfa_49495d983ed1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICJpegFrameEncode_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetAcHuffmanTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pachuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_AC_HUFFMAN_TABLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetAcHuffmanTable: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetDcHuffmanTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pdchuffmantable: *mut super::Dxgi::Common::DXGI_JPEG_DC_HUFFMAN_TABLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetDcHuffmanTable: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetQuantizationTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scanindex: u32, tableindex: u32, pquantizationtable: *mut super::Dxgi::Common::DXGI_JPEG_QUANTIZATION_TABLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetQuantizationTable: usize,
    pub WriteScan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cbscandata: u32, pbscandata: *const u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICMetadataBlockReader(::windows::core::IUnknown);
impl IWICMetadataBlockReader {
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetContainerFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetReaderByIndex(&self, nindex: u32) -> ::windows::core::Result<IWICMetadataReader> {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataReader>();
        (::windows::core::Interface::vtable(self).GetReaderByIndex)(::windows::core::Interface::as_raw(self), nindex, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IEnumUnknown>();
        (::windows::core::Interface::vtable(self).GetEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICMetadataBlockReader, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICMetadataBlockReader {
    type Vtable = IWICMetadataBlockReader_Vtbl;
}
impl ::core::clone::Clone for IWICMetadataBlockReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICMetadataBlockReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfeaa2a8d_b3f3_43e4_b25c_d1de990a1ae1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataBlockReader_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetContainerFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT,
    pub GetReaderByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, ppimetadatareader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienummetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEnumerator: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICMetadataBlockWriter(::windows::core::IUnknown);
impl IWICMetadataBlockWriter {
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetContainerFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetReaderByIndex(&self, nindex: u32) -> ::windows::core::Result<IWICMetadataReader> {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataReader>();
        (::windows::core::Interface::vtable(self).base__.GetReaderByIndex)(::windows::core::Interface::as_raw(self), nindex, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<super::super::System::Com::IEnumUnknown> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IEnumUnknown>();
        (::windows::core::Interface::vtable(self).base__.GetEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn InitializeFromBlockReader<P0>(&self, pimdblockreader: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICMetadataBlockReader>,
    {
        (::windows::core::Interface::vtable(self).InitializeFromBlockReader)(::windows::core::Interface::as_raw(self), pimdblockreader.into_param().abi()).ok()
    }
    pub unsafe fn GetWriterByIndex(&self, nindex: u32) -> ::windows::core::Result<IWICMetadataWriter> {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataWriter>();
        (::windows::core::Interface::vtable(self).GetWriterByIndex)(::windows::core::Interface::as_raw(self), nindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn AddWriter<P0>(&self, pimetadatawriter: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICMetadataWriter>,
    {
        (::windows::core::Interface::vtable(self).AddWriter)(::windows::core::Interface::as_raw(self), pimetadatawriter.into_param().abi()).ok()
    }
    pub unsafe fn SetWriterByIndex<P0>(&self, nindex: u32, pimetadatawriter: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICMetadataWriter>,
    {
        (::windows::core::Interface::vtable(self).SetWriterByIndex)(::windows::core::Interface::as_raw(self), nindex, pimetadatawriter.into_param().abi()).ok()
    }
    pub unsafe fn RemoveWriterByIndex(&self, nindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveWriterByIndex)(::windows::core::Interface::as_raw(self), nindex).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICMetadataBlockWriter, ::windows::core::IUnknown, IWICMetadataBlockReader);
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
unsafe impl ::windows::core::Interface for IWICMetadataBlockWriter {
    type Vtable = IWICMetadataBlockWriter_Vtbl;
}
impl ::core::clone::Clone for IWICMetadataBlockWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICMetadataBlockWriter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x08fb9676_b444_41e8_8dbe_6a53a542bff1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataBlockWriter_Vtbl {
    pub base__: IWICMetadataBlockReader_Vtbl,
    pub InitializeFromBlockReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimdblockreader: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetWriterByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, ppimetadatawriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pimetadatawriter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetWriterByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, pimetadatawriter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveWriterByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICMetadataHandlerInfo(::windows::core::IUnknown);
impl IWICMetadataHandlerInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__ = ::windows::core::zeroed::<WICComponentType>();
        (::windows::core::Interface::vtable(self).base__.GetComponentType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetCLSID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetSigningStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAuthor(&self, wzauthor: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAuthor)(::windows::core::Interface::as_raw(self), wzauthor.len() as _, ::core::mem::transmute(wzauthor.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetVendorGUID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self, wzversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetVersion)(::windows::core::Interface::as_raw(self), wzversion.len() as _, ::core::mem::transmute(wzversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetSpecVersion(&self, wzspecversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSpecVersion)(::windows::core::Interface::as_raw(self), wzspecversion.len() as _, ::core::mem::transmute(wzspecversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFriendlyName(&self, wzfriendlyname: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFriendlyName)(::windows::core::Interface::as_raw(self), wzfriendlyname.len() as _, ::core::mem::transmute(wzfriendlyname.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetMetadataFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetMetadataFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContainerFormats(&self, pguidcontainerformats: &mut [::windows::core::GUID], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetContainerFormats)(::windows::core::Interface::as_raw(self), pguidcontainerformats.len() as _, ::core::mem::transmute(pguidcontainerformats.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceManufacturer(&self, wzdevicemanufacturer: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeviceManufacturer)(::windows::core::Interface::as_raw(self), wzdevicemanufacturer.len() as _, ::core::mem::transmute(wzdevicemanufacturer.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceModels(&self, wzdevicemodels: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeviceModels)(::windows::core::Interface::as_raw(self), wzdevicemodels.len() as _, ::core::mem::transmute(wzdevicemodels.as_ptr()), pcchactual).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesRequireFullStream(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).DoesRequireFullStream)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportPadding(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).DoesSupportPadding)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesRequireFixedSize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).DoesRequireFixedSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICMetadataHandlerInfo, ::windows::core::IUnknown, IWICComponentInfo);
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
unsafe impl ::windows::core::Interface for IWICMetadataHandlerInfo {
    type Vtable = IWICMetadataHandlerInfo_Vtbl;
}
impl ::core::clone::Clone for IWICMetadataHandlerInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICMetadataHandlerInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaba958bf_c672_44d1_8d61_ce6df2e682c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataHandlerInfo_Vtbl {
    pub base__: IWICComponentInfo_Vtbl,
    pub GetMetadataFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidmetadataformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetContainerFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccontainerformats: u32, pguidcontainerformats: *mut ::windows::core::GUID, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    pub GetDeviceManufacturer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchdevicemanufacturer: u32, wzdevicemanufacturer: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    pub GetDeviceModels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchdevicemodels: u32, wzdevicemodels: ::windows::core::PWSTR, pcchactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub DoesRequireFullStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfrequiresfullstream: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoesRequireFullStream: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DoesSupportPadding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportspadding: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoesSupportPadding: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub DoesRequireFixedSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pffixedsize: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoesRequireFixedSize: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICMetadataQueryReader(::windows::core::IUnknown);
impl IWICMetadataQueryReader {
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetContainerFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLocation(&self, wznamespace: &mut [u16], pcchactuallength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLocation)(::windows::core::Interface::as_raw(self), wznamespace.len() as _, ::core::mem::transmute(wznamespace.as_ptr()), pcchactuallength).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetMetadataByName<P0>(&self, wzname: P0, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetMetadataByName)(::windows::core::Interface::as_raw(self), wzname.into_param().abi(), pvarvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<super::super::System::Com::IEnumString> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IEnumString>();
        (::windows::core::Interface::vtable(self).GetEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICMetadataQueryReader, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICMetadataQueryReader {
    type Vtable = IWICMetadataQueryReader_Vtbl;
}
impl ::core::clone::Clone for IWICMetadataQueryReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICMetadataQueryReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30989668_e1c9_4597_b395_458eedb808df);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataQueryReader_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetContainerFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcontainerformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchmaxlength: u32, wznamespace: ::windows::core::PWSTR, pcchactuallength: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetMetadataByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wzname: ::windows::core::PCWSTR, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetMetadataByName: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienumstring: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEnumerator: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICMetadataQueryWriter(::windows::core::IUnknown);
impl IWICMetadataQueryWriter {
    pub unsafe fn GetContainerFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetContainerFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLocation(&self, wznamespace: &mut [u16], pcchactuallength: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetLocation)(::windows::core::Interface::as_raw(self), wznamespace.len() as _, ::core::mem::transmute(wznamespace.as_ptr()), pcchactuallength).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetMetadataByName<P0>(&self, wzname: P0, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.GetMetadataByName)(::windows::core::Interface::as_raw(self), wzname.into_param().abi(), pvarvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<super::super::System::Com::IEnumString> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IEnumString>();
        (::windows::core::Interface::vtable(self).base__.GetEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetMetadataByName<P0>(&self, wzname: P0, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetMetadataByName)(::windows::core::Interface::as_raw(self), wzname.into_param().abi(), pvarvalue).ok()
    }
    pub unsafe fn RemoveMetadataByName<P0>(&self, wzname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).RemoveMetadataByName)(::windows::core::Interface::as_raw(self), wzname.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICMetadataQueryWriter, ::windows::core::IUnknown, IWICMetadataQueryReader);
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
unsafe impl ::windows::core::Interface for IWICMetadataQueryWriter {
    type Vtable = IWICMetadataQueryWriter_Vtbl;
}
impl ::core::clone::Clone for IWICMetadataQueryWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICMetadataQueryWriter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa721791a_0def_4d06_bd91_2118bf1db10b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataQueryWriter_Vtbl {
    pub base__: IWICMetadataQueryReader_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetMetadataByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wzname: ::windows::core::PCWSTR, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    SetMetadataByName: usize,
    pub RemoveMetadataByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wzname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICMetadataReader(::windows::core::IUnknown);
impl IWICMetadataReader {
    pub unsafe fn GetMetadataFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetMetadataFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMetadataHandlerInfo(&self) -> ::windows::core::Result<IWICMetadataHandlerInfo> {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataHandlerInfo>();
        (::windows::core::Interface::vtable(self).GetMetadataHandlerInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValueByIndex(&self, nindex: u32, pvarschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetValueByIndex)(::windows::core::Interface::as_raw(self), nindex, pvarschema, pvarid, pvarvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValue(&self, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetValue)(::windows::core::Interface::as_raw(self), pvarschema, pvarid, pvarvalue).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IWICEnumMetadataItem> {
        let mut result__ = ::windows::core::zeroed::<IWICEnumMetadataItem>();
        (::windows::core::Interface::vtable(self).GetEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICMetadataReader, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICMetadataReader {
    type Vtable = IWICMetadataReader_Vtbl;
}
impl ::core::clone::Clone for IWICMetadataReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICMetadataReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9204fe99_d8fc_4fd5_a001_9536b067a899);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataReader_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetMetadataFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidmetadataformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetMetadataHandlerInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppihandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetValueByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, pvarschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetValueByIndex: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    GetValue: usize,
    pub GetEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienummetadata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICMetadataReaderInfo(::windows::core::IUnknown);
impl IWICMetadataReaderInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__ = ::windows::core::zeroed::<WICComponentType>();
        (::windows::core::Interface::vtable(self).base__.base__.GetComponentType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.base__.GetCLSID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetSigningStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAuthor(&self, wzauthor: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetAuthor)(::windows::core::Interface::as_raw(self), wzauthor.len() as _, ::core::mem::transmute(wzauthor.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.base__.GetVendorGUID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self, wzversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetVersion)(::windows::core::Interface::as_raw(self), wzversion.len() as _, ::core::mem::transmute(wzversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetSpecVersion(&self, wzspecversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetSpecVersion)(::windows::core::Interface::as_raw(self), wzspecversion.len() as _, ::core::mem::transmute(wzspecversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFriendlyName(&self, wzfriendlyname: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFriendlyName)(::windows::core::Interface::as_raw(self), wzfriendlyname.len() as _, ::core::mem::transmute(wzfriendlyname.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetMetadataFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetMetadataFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContainerFormats(&self, pguidcontainerformats: &mut [::windows::core::GUID], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetContainerFormats)(::windows::core::Interface::as_raw(self), pguidcontainerformats.len() as _, ::core::mem::transmute(pguidcontainerformats.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceManufacturer(&self, wzdevicemanufacturer: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDeviceManufacturer)(::windows::core::Interface::as_raw(self), wzdevicemanufacturer.len() as _, ::core::mem::transmute(wzdevicemanufacturer.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceModels(&self, wzdevicemodels: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDeviceModels)(::windows::core::Interface::as_raw(self), wzdevicemodels.len() as _, ::core::mem::transmute(wzdevicemodels.as_ptr()), pcchactual).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesRequireFullStream(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.DoesRequireFullStream)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportPadding(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.DoesSupportPadding)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesRequireFixedSize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.DoesRequireFixedSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPatterns(&self, guidcontainerformat: *const ::windows::core::GUID, cbsize: u32, ppattern: ::core::option::Option<*mut WICMetadataPattern>, pccount: ::core::option::Option<*mut u32>, pcbactual: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPatterns)(::windows::core::Interface::as_raw(self), guidcontainerformat, cbsize, ::core::mem::transmute(ppattern.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pccount.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbactual.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn MatchesPattern<P0>(&self, guidcontainerformat: *const ::windows::core::GUID, pistream: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).MatchesPattern)(::windows::core::Interface::as_raw(self), guidcontainerformat, pistream.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateInstance(&self) -> ::windows::core::Result<IWICMetadataReader> {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataReader>();
        (::windows::core::Interface::vtable(self).CreateInstance)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICMetadataReaderInfo, ::windows::core::IUnknown, IWICComponentInfo, IWICMetadataHandlerInfo);
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
unsafe impl ::windows::core::Interface for IWICMetadataReaderInfo {
    type Vtable = IWICMetadataReaderInfo_Vtbl;
}
impl ::core::clone::Clone for IWICMetadataReaderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICMetadataReaderInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeebf1f5b_07c1_4447_a3ab_22acaf78a804);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataReaderInfo_Vtbl {
    pub base__: IWICMetadataHandlerInfo_Vtbl,
    pub GetPatterns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, cbsize: u32, ppattern: *mut WICMetadataPattern, pccount: *mut u32, pcbactual: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub MatchesPattern: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, pistream: *mut ::core::ffi::c_void, pfmatches: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    MatchesPattern: usize,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppireader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICMetadataWriter(::windows::core::IUnknown);
impl IWICMetadataWriter {
    pub unsafe fn GetMetadataFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetMetadataFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMetadataHandlerInfo(&self) -> ::windows::core::Result<IWICMetadataHandlerInfo> {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataHandlerInfo>();
        (::windows::core::Interface::vtable(self).base__.GetMetadataHandlerInfo)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValueByIndex(&self, nindex: u32, pvarschema: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetValueByIndex)(::windows::core::Interface::as_raw(self), nindex, pvarschema, pvarid, pvarvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn GetValue(&self, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetValue)(::windows::core::Interface::as_raw(self), pvarschema, pvarid, pvarvalue).ok()
    }
    pub unsafe fn GetEnumerator(&self) -> ::windows::core::Result<IWICEnumMetadataItem> {
        let mut result__ = ::windows::core::zeroed::<IWICEnumMetadataItem>();
        (::windows::core::Interface::vtable(self).base__.GetEnumerator)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetValue(&self, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValue)(::windows::core::Interface::as_raw(self), pvarschema, pvarid, pvarvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn SetValueByIndex(&self, nindex: u32, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValueByIndex)(::windows::core::Interface::as_raw(self), nindex, pvarschema, pvarid, pvarvalue).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub unsafe fn RemoveValue(&self, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveValue)(::windows::core::Interface::as_raw(self), pvarschema, pvarid).ok()
    }
    pub unsafe fn RemoveValueByIndex(&self, nindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveValueByIndex)(::windows::core::Interface::as_raw(self), nindex).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICMetadataWriter, ::windows::core::IUnknown, IWICMetadataReader);
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
unsafe impl ::windows::core::Interface for IWICMetadataWriter {
    type Vtable = IWICMetadataWriter_Vtbl;
}
impl ::core::clone::Clone for IWICMetadataWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICMetadataWriter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7836e16_3be0_470b_86bb_160d0aecd7de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataWriter_Vtbl {
    pub base__: IWICMetadataReader_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    SetValue: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub SetValueByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    SetValueByIndex: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub RemoveValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvarschema: *const super::super::System::Com::StructuredStorage::PROPVARIANT, pvarid: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage")))]
    RemoveValue: usize,
    pub RemoveValueByIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICMetadataWriterInfo(::windows::core::IUnknown);
impl IWICMetadataWriterInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__ = ::windows::core::zeroed::<WICComponentType>();
        (::windows::core::Interface::vtable(self).base__.base__.GetComponentType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.base__.GetCLSID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetSigningStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAuthor(&self, wzauthor: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetAuthor)(::windows::core::Interface::as_raw(self), wzauthor.len() as _, ::core::mem::transmute(wzauthor.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.base__.GetVendorGUID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self, wzversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetVersion)(::windows::core::Interface::as_raw(self), wzversion.len() as _, ::core::mem::transmute(wzversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetSpecVersion(&self, wzspecversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetSpecVersion)(::windows::core::Interface::as_raw(self), wzspecversion.len() as _, ::core::mem::transmute(wzspecversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFriendlyName(&self, wzfriendlyname: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFriendlyName)(::windows::core::Interface::as_raw(self), wzfriendlyname.len() as _, ::core::mem::transmute(wzfriendlyname.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetMetadataFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetMetadataFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContainerFormats(&self, pguidcontainerformats: &mut [::windows::core::GUID], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetContainerFormats)(::windows::core::Interface::as_raw(self), pguidcontainerformats.len() as _, ::core::mem::transmute(pguidcontainerformats.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceManufacturer(&self, wzdevicemanufacturer: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDeviceManufacturer)(::windows::core::Interface::as_raw(self), wzdevicemanufacturer.len() as _, ::core::mem::transmute(wzdevicemanufacturer.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetDeviceModels(&self, wzdevicemodels: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDeviceModels)(::windows::core::Interface::as_raw(self), wzdevicemodels.len() as _, ::core::mem::transmute(wzdevicemodels.as_ptr()), pcchactual).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesRequireFullStream(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.DoesRequireFullStream)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportPadding(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.DoesSupportPadding)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesRequireFixedSize(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).base__.DoesRequireFixedSize)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetHeader(&self, guidcontainerformat: *const ::windows::core::GUID, cbsize: u32, pheader: ::core::option::Option<*mut WICMetadataHeader>, pcbactual: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetHeader)(::windows::core::Interface::as_raw(self), guidcontainerformat, cbsize, ::core::mem::transmute(pheader.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbactual.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateInstance(&self) -> ::windows::core::Result<IWICMetadataWriter> {
        let mut result__ = ::windows::core::zeroed::<IWICMetadataWriter>();
        (::windows::core::Interface::vtable(self).CreateInstance)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICMetadataWriterInfo, ::windows::core::IUnknown, IWICComponentInfo, IWICMetadataHandlerInfo);
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
unsafe impl ::windows::core::Interface for IWICMetadataWriterInfo {
    type Vtable = IWICMetadataWriterInfo_Vtbl;
}
impl ::core::clone::Clone for IWICMetadataWriterInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICMetadataWriterInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb22e3fba_3925_4323_b5c1_9ebfc430f236);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICMetadataWriterInfo_Vtbl {
    pub base__: IWICMetadataHandlerInfo_Vtbl,
    pub GetHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, guidcontainerformat: *const ::windows::core::GUID, cbsize: u32, pheader: *mut WICMetadataHeader, pcbactual: *mut u32) -> ::windows::core::HRESULT,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppiwriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICPalette(::windows::core::IUnknown);
impl IWICPalette {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializePredefined<P0>(&self, epalettetype: WICBitmapPaletteType, faddtransparentcolor: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).InitializePredefined)(::windows::core::Interface::as_raw(self), epalettetype, faddtransparentcolor.into_param().abi()).ok()
    }
    pub unsafe fn InitializeCustom(&self, pcolors: &[u32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitializeCustom)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcolors.as_ptr()), pcolors.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InitializeFromBitmap<P0, P1>(&self, pisurface: P0, ccount: u32, faddtransparentcolor: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICBitmapSource>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).InitializeFromBitmap)(::windows::core::Interface::as_raw(self), pisurface.into_param().abi(), ccount, faddtransparentcolor.into_param().abi()).ok()
    }
    pub unsafe fn InitializeFromPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICPalette>,
    {
        (::windows::core::Interface::vtable(self).InitializeFromPalette)(::windows::core::Interface::as_raw(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<WICBitmapPaletteType> {
        let mut result__ = ::windows::core::zeroed::<WICBitmapPaletteType>();
        (::windows::core::Interface::vtable(self).GetType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColorCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetColorCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColors(&self, pcolors: &mut [u32], pcactualcolors: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColors)(::windows::core::Interface::as_raw(self), pcolors.len() as _, ::core::mem::transmute(pcolors.as_ptr()), pcactualcolors).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsBlackWhite(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsBlackWhite)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsGrayscale(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsGrayscale)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasAlpha(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).HasAlpha)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICPalette, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICPalette {
    type Vtable = IWICPalette_Vtbl;
}
impl ::core::clone::Clone for IWICPalette {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICPalette {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00000040_a8f2_4877_ba0a_fd2b6645fb94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICPalette_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializePredefined: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, epalettetype: WICBitmapPaletteType, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializePredefined: usize,
    pub InitializeCustom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolors: *const u32, ccount: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InitializeFromBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisurface: *mut ::core::ffi::c_void, ccount: u32, faddtransparentcolor: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InitializeFromBitmap: usize,
    pub InitializeFromPalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pipalette: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pepalettetype: *mut WICBitmapPaletteType) -> ::windows::core::HRESULT,
    pub GetColorCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pccount: *mut u32) -> ::windows::core::HRESULT,
    pub GetColors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ccount: u32, pcolors: *mut u32, pcactualcolors: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsBlackWhite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisblackwhite: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsBlackWhite: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub IsGrayscale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisgrayscale: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsGrayscale: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub HasAlpha: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfhasalpha: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasAlpha: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWICPersistStream(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWICPersistStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.base__.GetClassID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn IsDirty(&self) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.IsDirty)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Load<P0>(&self, pstm: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).base__.Load)(::windows::core::Interface::as_raw(self), pstm.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Save<P0, P1>(&self, pstm: P0, fcleardirty: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Save)(::windows::core::Interface::as_raw(self), pstm.into_param().abi(), fcleardirty.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetSizeMax(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::windows::core::zeroed::<u64>();
        (::windows::core::Interface::vtable(self).base__.GetSizeMax)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadEx<P0>(&self, pistream: P0, pguidpreferredvendor: *const ::windows::core::GUID, dwpersistoptions: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).LoadEx)(::windows::core::Interface::as_raw(self), pistream.into_param().abi(), pguidpreferredvendor, dwpersistoptions).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn SaveEx<P0, P1>(&self, pistream: P0, dwpersistoptions: u32, fcleardirty: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SaveEx)(::windows::core::Interface::as_raw(self), pistream.into_param().abi(), dwpersistoptions, fcleardirty.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IWICPersistStream, ::windows::core::IUnknown, super::super::System::Com::IPersist, super::super::System::Com::IPersistStream);
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
unsafe impl ::windows::core::Interface for IWICPersistStream {
    type Vtable = IWICPersistStream_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWICPersistStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IWICPersistStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00675040_6908_45f8_86a3_49c7dfd6d9ad);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWICPersistStream_Vtbl {
    pub base__: super::super::System::Com::IPersistStream_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, pguidpreferredvendor: *const ::windows::core::GUID, dwpersistoptions: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadEx: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub SaveEx: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, dwpersistoptions: u32, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    SaveEx: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICPixelFormatInfo(::windows::core::IUnknown);
impl IWICPixelFormatInfo {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__ = ::windows::core::zeroed::<WICComponentType>();
        (::windows::core::Interface::vtable(self).base__.GetComponentType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetCLSID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetSigningStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAuthor(&self, wzauthor: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetAuthor)(::windows::core::Interface::as_raw(self), wzauthor.len() as _, ::core::mem::transmute(wzauthor.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetVendorGUID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self, wzversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetVersion)(::windows::core::Interface::as_raw(self), wzversion.len() as _, ::core::mem::transmute(wzversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetSpecVersion(&self, wzspecversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSpecVersion)(::windows::core::Interface::as_raw(self), wzspecversion.len() as _, ::core::mem::transmute(wzspecversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFriendlyName(&self, wzfriendlyname: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetFriendlyName)(::windows::core::Interface::as_raw(self), wzfriendlyname.len() as _, ::core::mem::transmute(wzfriendlyname.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFormatGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetFormatGUID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColorContext(&self) -> ::windows::core::Result<IWICColorContext> {
        let mut result__ = ::windows::core::zeroed::<IWICColorContext>();
        (::windows::core::Interface::vtable(self).GetColorContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBitsPerPixel(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetBitsPerPixel)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetChannelCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetChannelMask(&self, uichannelindex: u32, pbmaskbuffer: &mut [u8], pcbactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetChannelMask)(::windows::core::Interface::as_raw(self), uichannelindex, pbmaskbuffer.len() as _, ::core::mem::transmute(pbmaskbuffer.as_ptr()), pcbactual).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICPixelFormatInfo, ::windows::core::IUnknown, IWICComponentInfo);
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
unsafe impl ::windows::core::Interface for IWICPixelFormatInfo {
    type Vtable = IWICPixelFormatInfo_Vtbl;
}
impl ::core::clone::Clone for IWICPixelFormatInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICPixelFormatInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8eda601_3d48_431a_ab44_69059be88bbe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICPixelFormatInfo_Vtbl {
    pub base__: IWICComponentInfo_Vtbl,
    pub GetFormatGUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pformat: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetColorContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppicolorcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBitsPerPixel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puibitsperpixel: *mut u32) -> ::windows::core::HRESULT,
    pub GetChannelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puichannelcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetChannelMask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uichannelindex: u32, cbmaskbuffer: u32, pbmaskbuffer: *mut u8, pcbactual: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICPixelFormatInfo2(::windows::core::IUnknown);
impl IWICPixelFormatInfo2 {
    pub unsafe fn GetComponentType(&self) -> ::windows::core::Result<WICComponentType> {
        let mut result__ = ::windows::core::zeroed::<WICComponentType>();
        (::windows::core::Interface::vtable(self).base__.base__.GetComponentType)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCLSID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.base__.GetCLSID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSigningStatus(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.base__.GetSigningStatus)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAuthor(&self, wzauthor: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetAuthor)(::windows::core::Interface::as_raw(self), wzauthor.len() as _, ::core::mem::transmute(wzauthor.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.base__.GetVendorGUID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self, wzversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetVersion)(::windows::core::Interface::as_raw(self), wzversion.len() as _, ::core::mem::transmute(wzversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetSpecVersion(&self, wzspecversion: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetSpecVersion)(::windows::core::Interface::as_raw(self), wzspecversion.len() as _, ::core::mem::transmute(wzspecversion.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFriendlyName(&self, wzfriendlyname: &mut [u16], pcchactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetFriendlyName)(::windows::core::Interface::as_raw(self), wzfriendlyname.len() as _, ::core::mem::transmute(wzfriendlyname.as_ptr()), pcchactual).ok()
    }
    pub unsafe fn GetFormatGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetFormatGUID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColorContext(&self) -> ::windows::core::Result<IWICColorContext> {
        let mut result__ = ::windows::core::zeroed::<IWICColorContext>();
        (::windows::core::Interface::vtable(self).base__.GetColorContext)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBitsPerPixel(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetBitsPerPixel)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetChannelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).base__.GetChannelCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetChannelMask(&self, uichannelindex: u32, pbmaskbuffer: &mut [u8], pcbactual: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetChannelMask)(::windows::core::Interface::as_raw(self), uichannelindex, pbmaskbuffer.len() as _, ::core::mem::transmute(pbmaskbuffer.as_ptr()), pcbactual).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsTransparency(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).SupportsTransparency)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNumericRepresentation(&self) -> ::windows::core::Result<WICPixelFormatNumericRepresentation> {
        let mut result__ = ::windows::core::zeroed::<WICPixelFormatNumericRepresentation>();
        (::windows::core::Interface::vtable(self).GetNumericRepresentation)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICPixelFormatInfo2, ::windows::core::IUnknown, IWICComponentInfo, IWICPixelFormatInfo);
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
unsafe impl ::windows::core::Interface for IWICPixelFormatInfo2 {
    type Vtable = IWICPixelFormatInfo2_Vtbl;
}
impl ::core::clone::Clone for IWICPixelFormatInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICPixelFormatInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9db33a2_af5f_43c7_b679_74f5984b5aa4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICPixelFormatInfo2_Vtbl {
    pub base__: IWICPixelFormatInfo_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SupportsTransparency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfsupportstransparency: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SupportsTransparency: usize,
    pub GetNumericRepresentation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnumericrepresentation: *mut WICPixelFormatNumericRepresentation) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICPlanarBitmapFrameEncode(::windows::core::IUnknown);
impl IWICPlanarBitmapFrameEncode {
    pub unsafe fn WritePixels(&self, linecount: u32, pplanes: &[WICBitmapPlane]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WritePixels)(::windows::core::Interface::as_raw(self), linecount, ::core::mem::transmute(pplanes.as_ptr()), pplanes.len() as _).ok()
    }
    pub unsafe fn WriteSource(&self, ppplanes: &[::core::option::Option<IWICBitmapSource>], prcsource: *const WICRect) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WriteSource)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppplanes.as_ptr()), ppplanes.len() as _, prcsource).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICPlanarBitmapFrameEncode, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICPlanarBitmapFrameEncode {
    type Vtable = IWICPlanarBitmapFrameEncode_Vtbl;
}
impl ::core::clone::Clone for IWICPlanarBitmapFrameEncode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICPlanarBitmapFrameEncode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf928b7b8_2221_40c1_b72e_7e82f1974d1a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICPlanarBitmapFrameEncode_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub WritePixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linecount: u32, pplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows::core::HRESULT,
    pub WriteSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppplanes: *const *mut ::core::ffi::c_void, cplanes: u32, prcsource: *const WICRect) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICPlanarBitmapSourceTransform(::windows::core::IUnknown);
impl IWICPlanarBitmapSourceTransform {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DoesSupportTransform(&self, puiwidth: *mut u32, puiheight: *mut u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pguiddstformats: *const ::windows::core::GUID, pplanedescriptions: *mut WICBitmapPlaneDescription, cplanes: u32, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DoesSupportTransform)(::windows::core::Interface::as_raw(self), puiwidth, puiheight, dsttransform, dstplanaroptions, pguiddstformats, pplanedescriptions, cplanes, pfissupported).ok()
    }
    pub unsafe fn CopyPixels(&self, prcsource: *const WICRect, uiwidth: u32, uiheight: u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pdstplanes: &[WICBitmapPlane]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopyPixels)(::windows::core::Interface::as_raw(self), prcsource, uiwidth, uiheight, dsttransform, dstplanaroptions, ::core::mem::transmute(pdstplanes.as_ptr()), pdstplanes.len() as _).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICPlanarBitmapSourceTransform, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICPlanarBitmapSourceTransform {
    type Vtable = IWICPlanarBitmapSourceTransform_Vtbl;
}
impl ::core::clone::Clone for IWICPlanarBitmapSourceTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICPlanarBitmapSourceTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3aff9cce_be95_4303_b927_e7d16ff4a613);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICPlanarBitmapSourceTransform_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub DoesSupportTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, puiwidth: *mut u32, puiheight: *mut u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pguiddstformats: *const ::windows::core::GUID, pplanedescriptions: *mut WICBitmapPlaneDescription, cplanes: u32, pfissupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    DoesSupportTransform: usize,
    pub CopyPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prcsource: *const WICRect, uiwidth: u32, uiheight: u32, dsttransform: WICBitmapTransformOptions, dstplanaroptions: WICPlanarOptions, pdstplanes: *const WICBitmapPlane, cplanes: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICPlanarFormatConverter(::windows::core::IUnknown);
impl IWICPlanarFormatConverter {
    pub unsafe fn GetSize(&self, puiwidth: *mut u32, puiheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSize)(::windows::core::Interface::as_raw(self), puiwidth, puiheight).ok()
    }
    pub unsafe fn GetPixelFormat(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).base__.GetPixelFormat)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetResolution(&self, pdpix: *mut f64, pdpiy: *mut f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetResolution)(::windows::core::Interface::as_raw(self), pdpix, pdpiy).ok()
    }
    pub unsafe fn CopyPalette<P0>(&self, pipalette: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICPalette>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyPalette)(::windows::core::Interface::as_raw(self), pipalette.into_param().abi()).ok()
    }
    pub unsafe fn CopyPixels(&self, prc: *const WICRect, cbstride: u32, pbbuffer: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.CopyPixels)(::windows::core::Interface::as_raw(self), prc, cbstride, pbbuffer.len() as _, ::core::mem::transmute(pbbuffer.as_ptr())).ok()
    }
    pub unsafe fn Initialize<P0>(&self, ppplanes: &[::core::option::Option<IWICBitmapSource>], dstformat: *const ::windows::core::GUID, dither: WICBitmapDitherType, pipalette: P0, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IWICPalette>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppplanes.as_ptr()), ppplanes.len() as _, dstformat, dither, pipalette.into_param().abi(), alphathresholdpercent, palettetranslate).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CanConvert(&self, psrcpixelformats: &[::windows::core::GUID], dstpixelformat: *const ::windows::core::GUID) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).CanConvert)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psrcpixelformats.as_ptr()), psrcpixelformats.len() as _, dstpixelformat, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IWICPlanarFormatConverter, ::windows::core::IUnknown, IWICBitmapSource);
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
unsafe impl ::windows::core::Interface for IWICPlanarFormatConverter {
    type Vtable = IWICPlanarFormatConverter_Vtbl;
}
impl ::core::clone::Clone for IWICPlanarFormatConverter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICPlanarFormatConverter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbebee9cb_83b0_4dcc_8132_b0aaa55eac96);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICPlanarFormatConverter_Vtbl {
    pub base__: IWICBitmapSource_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppplanes: *const *mut ::core::ffi::c_void, cplanes: u32, dstformat: *const ::windows::core::GUID, dither: WICBitmapDitherType, pipalette: *mut ::core::ffi::c_void, alphathresholdpercent: f64, palettetranslate: WICBitmapPaletteType) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CanConvert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psrcpixelformats: *const ::windows::core::GUID, csrcplanes: u32, dstpixelformat: *const ::windows::core::GUID, pfcanconvert: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CanConvert: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICProgressCallback(::windows::core::IUnknown);
impl IWICProgressCallback {
    pub unsafe fn Notify(&self, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Notify)(::windows::core::Interface::as_raw(self), uframenum, operation, dblprogress).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICProgressCallback, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICProgressCallback {
    type Vtable = IWICProgressCallback_Vtbl;
}
impl ::core::clone::Clone for IWICProgressCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICProgressCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4776f9cd_9517_45fa_bf24_e89c5ec5c60c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICProgressCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICProgressiveLevelControl(::windows::core::IUnknown);
impl IWICProgressiveLevelControl {
    pub unsafe fn GetLevelCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetLevelCount)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCurrentLevel(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetCurrentLevel)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCurrentLevel(&self, nlevel: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCurrentLevel)(::windows::core::Interface::as_raw(self), nlevel).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICProgressiveLevelControl, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICProgressiveLevelControl {
    type Vtable = IWICProgressiveLevelControl_Vtbl;
}
impl ::core::clone::Clone for IWICProgressiveLevelControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICProgressiveLevelControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdaac296f_7aa5_4dbf_8d15_225c5976f891);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICProgressiveLevelControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetLevelCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pclevels: *mut u32) -> ::windows::core::HRESULT,
    pub GetCurrentLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pnlevel: *mut u32) -> ::windows::core::HRESULT,
    pub SetCurrentLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nlevel: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct IWICStream(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl IWICStream {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.Read)(::windows::core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Interface::vtable(self).base__.base__.Write)(::windows::core::Interface::as_raw(self), pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::super::System::Com::STREAM_SEEK, plibnewposition: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Seek)(::windows::core::Interface::as_raw(self), dlibmove, dworigin, ::core::mem::transmute(plibnewposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetSize)(::windows::core::Interface::as_raw(self), libnewsize).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: u64, pcbread: ::core::option::Option<*mut u64>, pcbwritten: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).base__.CopyTo)(::windows::core::Interface::as_raw(self), pstm.into_param().abi(), cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Commit(&self, grfcommitflags: super::super::System::Com::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Commit)(::windows::core::Interface::as_raw(self), grfcommitflags).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Revert)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: super::super::System::Com::LOCKTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.LockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.UnlockRegion)(::windows::core::Interface::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Stat(&self, pstatstg: *mut super::super::System::Com::STATSTG, grfstatflag: super::super::System::Com::STATFLAG) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Stat)(::windows::core::Interface::as_raw(self), pstatstg, grfstatflag).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IStream>();
        (::windows::core::Interface::vtable(self).base__.Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromIStream<P0>(&self, pistream: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).InitializeFromIStream)(::windows::core::Interface::as_raw(self), pistream.into_param().abi()).ok()
    }
    pub unsafe fn InitializeFromFilename<P0>(&self, wzfilename: P0, dwdesiredaccess: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).InitializeFromFilename)(::windows::core::Interface::as_raw(self), wzfilename.into_param().abi(), dwdesiredaccess).ok()
    }
    pub unsafe fn InitializeFromMemory(&self, pbbuffer: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitializeFromMemory)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbbuffer.as_ptr()), pbbuffer.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InitializeFromIStreamRegion<P0>(&self, pistream: P0, uloffset: u64, ulmaxsize: u64) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows::core::Interface::vtable(self).InitializeFromIStreamRegion)(::windows::core::Interface::as_raw(self), pistream.into_param().abi(), uloffset, ulmaxsize).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(IWICStream, ::windows::core::IUnknown, super::super::System::Com::ISequentialStream, super::super::System::Com::IStream);
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
unsafe impl ::windows::core::Interface for IWICStream {
    type Vtable = IWICStream_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for IWICStream {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for IWICStream {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x135ff860_22b7_4ddf_b0f6_218f4f299a43);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct IWICStream_Vtbl {
    pub base__: super::super::System::Com::IStream_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromIStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromIStream: usize,
    pub InitializeFromFilename: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, wzfilename: ::windows::core::PCWSTR, dwdesiredaccess: u32) -> ::windows::core::HRESULT,
    pub InitializeFromMemory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbbuffer: *const u8, cbbuffersize: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub InitializeFromIStreamRegion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pistream: *mut ::core::ffi::c_void, uloffset: u64, ulmaxsize: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    InitializeFromIStreamRegion: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
pub struct IWICStreamProvider(::windows::core::IUnknown);
impl IWICStreamProvider {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::windows::core::zeroed::<super::super::System::Com::IStream>();
        (::windows::core::Interface::vtable(self).GetStream)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPersistOptions(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetPersistOptions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPreferredVendorGUID(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetPreferredVendorGUID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn RefreshStream(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RefreshStream)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IWICStreamProvider, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Interface for IWICStreamProvider {
    type Vtable = IWICStreamProvider_Vtbl;
}
impl ::core::clone::Clone for IWICStreamProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWICStreamProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x449494bc_b468_4927_96d7_ba90d31ab505);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWICStreamProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppistream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    pub GetPersistOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwpersistoptions: *mut u32) -> ::windows::core::HRESULT,
    pub GetPreferredVendorGUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidpreferredvendor: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RefreshStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CATID_WICBitmapDecoders: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7ed96837_96f0_4812_b211_f13c24117ed3);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CATID_WICBitmapEncoders: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac757296_3522_4e11_9862_c17be5a1767e);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CATID_WICFormatConverters: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7835eae8_bf14_49d1_93ce_533a407b2248);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CATID_WICMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05af94d8_7174_4cd2_be4a_4124b80ee4b8);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CATID_WICMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xabe3b9a4_257d_4b97_bd1a_294af496222e);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CATID_WICPixelFormats: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b46e70f_cda7_473e_89f6_dc9630a2390b);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WIC8BIMIPTCDigestMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02805f1e_d5aa_415b_82c5_61c033a988a6);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WIC8BIMIPTCDigestMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2db5e62b_0d67_495f_8f9d_c2f0188647ac);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WIC8BIMIPTCMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0010668c_0801_4da6_a4a4_826522b6d28f);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WIC8BIMIPTCMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00108226_ee41_44a2_9e9c_4be4d5b1d2cd);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WIC8BIMResolutionInfoMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5805137a_e348_4f7c_b3cc_6db9965a0599);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WIC8BIMResolutionInfoMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ff2fe0e_e74a_4b71_98c4_ab7dc16707ba);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICAPEMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1767b93a_b021_44ea_920f_863c11f4f768);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICAPEMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd6edfca_2890_482f_b233_8d7339a1cf8d);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICAdngDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x981d9411_909e_42a7_8f5d_a747ff052edb);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICApp0MetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43324b33_a78f_480f_9111_9638aaccc832);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICApp0MetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3c633a2_46c8_498e_8fbb_cc6f721bbcde);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICApp13MetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa7e3c50_864c_4604_bc04_8b0b76e637f6);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICApp13MetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b19a919_a9d6_49e5_bd45_02c34e4e4cd5);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICApp1MetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdde33513_774e_4bcd_ae79_02f4adfe62fc);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICApp1MetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee366069_1832_420f_b381_0479ad066f19);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICBmpDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b462062_7cbf_400d_9fdb_813dd10f2778);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICBmpEncoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69be8bb4_d66d_47c8_865a_ed1589433782);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICDdsDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9053699f_a341_429d_9e90_ee437cf80c73);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICDdsEncoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa61dde94_66ce_4ac1_881b_71680588895e);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICDdsMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x276c88ca_7533_4a86_b676_66b36080d484);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICDdsMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd688bbd_31ed_4db7_a723_934927d38367);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICDefaultFormatConverter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a3f11dc_b514_4b17_8c5f_2154513852f1);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICExifMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9403860_297f_4a49_bf9b_77898150a442);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICExifMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9a14cda_c339_460b_9078_d4debcfabe91);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICFormatConverterHighColor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac75d454_9f37_48f8_b972_4e19bc856011);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICFormatConverterNChannel: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc17cabb2_d4a3_47d7_a557_339b2efbd4f1);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICFormatConverterWMPhoto: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cb5172b_d600_46ba_ab77_77bb7e3a00d9);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICGCEMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb92e345d_f52d_41f3_b562_081bc772e3b9);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICGCEMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaf95dc76_16b2_47f4_b3ea_3c31796693e7);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICGifCommentMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x32557d3b_69dc_4f95_836e_f5972b2f6159);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICGifCommentMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa02797fc_c4ae_418c_af95_e637c7ead2a1);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICGifDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x381dda3c_9ce9_4834_a23e_1f98f8fc52be);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICGifEncoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x114f5598_0b22_40a0_86a1_c83ea495adbd);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICGpsMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3697790b_223b_484e_9925_c4869218f17a);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICGpsMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb8c13e4_62b5_4c96_a48b_6ba6ace39c76);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICHeifDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9a4a80a_44fe_4de4_8971_7150b10a5199);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICHeifEncoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0dbecec1_9eb3_4860_9c6f_ddbe86634575);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICHeifHDRMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2438de3d_94d9_4be8_84a8_4de95a575e75);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICHeifMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xacddfc3f_85ec_41bc_bdef_1bc262e4db05);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICHeifMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ae45e79_40bc_4401_ace5_dd3cb16e6afe);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICIMDMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7447a267_0015_42c8_a8f1_fb3b94c68361);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICIMDMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c89071f_452e_4e95_9682_9d1024627172);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICIPTCMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03012959_f4f6_44d7_9d09_daa087a9db57);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICIPTCMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1249b20c_5dd0_44fe_b0b3_8f92c8e6d080);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICIRBMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4dcd3d7_b4c2_47d9_a6bf_b89ba396a4a3);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICIRBMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c5c1935_0235_4434_80bc_251bc1ec39c6);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICIcoDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc61bfcdf_2e0f_4aad_a8d7_e06bafebcdfe);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICIfdMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f914656_9d0a_4eb2_9019_0bf96d8a9ee6);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICIfdMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1ebfc28_c9bd_47a2_8d33_b948769777a7);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICImagingCategories: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfae3d380_fea4_4623_8c75_c6b61110b681);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICImagingFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcacaf262_9370_4615_a13b_9f5539da4c0a);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICImagingFactory1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcacaf262_9370_4615_a13b_9f5539da4c0a);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICImagingFactory2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x317d06e8_5f24_433d_bdf7_79ce68d8abc2);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICInteropMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5c8b898_0074_459f_b700_860d4651ea14);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICInteropMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x122ec645_cd7e_44d8_b186_2c8c20c3b50f);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICJpegChrominanceMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50b1904b_f28f_4574_93f4_0bade82c69e9);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICJpegChrominanceMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3ff566f0_6e6b_49d4_96e6_b78886692c62);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICJpegCommentMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f66347c_60c4_4c4d_ab58_d2358685f607);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICJpegCommentMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe573236f_55b1_4eda_81ea_9f65db0290d3);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICJpegDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9456a480_e88b_43ea_9e73_0b2d9b71b1ca);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICJpegEncoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a34f5c1_4a5a_46dc_b644_1f4567e7a676);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICJpegLuminanceMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x356f2f88_05a6_4728_b9a4_1bfbce04d838);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICJpegLuminanceMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d583abc_8a0e_4657_9982_a380ca58fb4b);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICJpegQualcommPhoneEncoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68ed5c62_f534_4979_b2b3_686a12b2b34c);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICLSDMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41070793_59e4_479a_a1f7_954adc2ef5fc);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICLSDMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x73c037e7_e5d9_4954_876a_6da81d6e5768);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPlanarFormatConverter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x184132b8_32f8_4784_9131_dd7224b23438);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngBkgdMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ce7a4a6_03e8_4a60_9d15_282ef32ee7da);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngBkgdMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68e3f2fd_31ae_4441_bb6a_fd7047525f90);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngChrmMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf90b5f36_367b_402a_9dd1_bc0fd59d8f62);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngChrmMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe23ce3eb_5608_4e83_bcef_27b1987e51d7);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x389ea17b_5078_4cde_b6ef_25c15175c751);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngDecoder1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x389ea17b_5078_4cde_b6ef_25c15175c751);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngDecoder2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe018945b_aa86_4008_9bd4_6777a1e40c11);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngEncoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x27949969_876a_41d7_9447_568f6a35a4dc);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngGamaMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3692ca39_e082_4350_9e1f_3704cb083cd5);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngGamaMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff036d13_5d4b_46dd_b10f_106693d9fe4f);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngHistMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x877a0bb7_a313_4491_87b5_2e6d0594f520);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngHistMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a03e749_672e_446e_bf1f_2c11d233b6ff);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngIccpMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5d3e63b_cb0f_4628_a478_6d8244be36b1);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngIccpMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16671e5f_0ce6_4cc4_9768_e89fe5018ade);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngItxtMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaabfb2fa_3e1e_4a8f_8977_5556fb94ea23);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngItxtMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31879719_e751_4df8_981d_68dff67704ed);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngSrgbMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb40360c_547e_4956_a3b9_d4418859ba66);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngSrgbMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6ee35c6_87ec_47df_9f22_1d5aad840c82);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngTextMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b59afcc_b8c3_408a_b670_89e5fab6fda7);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngTextMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5ebafb9_253e_4a72_a744_0762d2685683);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngTimeMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd94edf02_efe5_4f0d_85c8_f5a68b3000b1);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICPngTimeMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ab78400_b5a3_4d91_8ace_33fcd1499be6);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICRAWDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x41945702_8302_44a6_9445_ac98e8afa086);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICSubIfdMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50d42f09_ecd1_4b41_b65d_da1fdaa75663);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICSubIfdMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ade5386_8e9b_4f4c_acf2_f0008706b238);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICThumbnailMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb012959_f4f6_44d7_9d09_daa087a9db57);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICThumbnailMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd049b20c_5dd0_44fe_b0b3_8f92c8e6d080);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICTiffDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb54e85d9_fe23_499f_8b88_6acea713752b);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICTiffEncoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0131be10_2001_4c5f_a9b0_cc88fab64ce8);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICUnknownMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x699745c2_5066_4b82_a8e3_d40478dbec8c);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICUnknownMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa09cca86_27ba_4f39_9053_121fa4dc08fc);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICWebpAnimMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x076f9911_a348_465c_a807_a252f3f2d3de);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICWebpAnmfMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85a10b03_c9f6_439f_be5e_c0fbef67807c);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICWebpDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7693e886_51c9_4070_8419_9f70738ec8fa);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICWmpDecoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa26cec36_234c_4950_ae16_e34aace71d0d);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICWmpEncoder: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac4ce3cb_e1c1_44cd_8215_5a1665509ec2);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICXMPAltMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa94dcc2_b8b0_4898_b835_000aabd74393);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICXMPAltMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x076c2a6c_f78f_4c46_a723_3583e70876ea);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICXMPBagMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7e79a30_4f2c_4fab_8d00_394f2d6bbebe);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICXMPBagMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed822c8c_d6be_4301_a631_0e1416bad28f);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICXMPMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72b624df_ae11_4948_a65c_351eb0829419);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICXMPMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1765e14e_1bd4_462e_b6b1_590bf1262ac6);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICXMPSeqMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f12e753_fc71_43d7_a51d_92f35977abb5);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICXMPSeqMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d68d1de_d432_4b0f_923a_091183a9bda7);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICXMPStructMetadataReader: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x01b90d9a_8209_47f7_9c52_e1244bf50ced);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const CLSID_WICXMPStructMetadataWriter: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22c21f93_7ddb_411c_9b17_c5b7bd064abc);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const FACILITY_WINCODEC_ERR: u32 = 2200u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_ContainerFormatAdng: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3ff6d0d_38c0_41c4_b1fe_1f3824f17b84);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_ContainerFormatBmp: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0af1d87e_fcfe_4188_bdeb_a7906471cbe3);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_ContainerFormatDds: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9967cb95_2e85_4ac8_8ca2_83d7ccd425c9);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_ContainerFormatGif: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f8a5601_7d4d_4cbd_9c82_1bc8d4eeb9a5);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_ContainerFormatHeif: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1e62521_6787_405b_a339_500715b5763f);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_ContainerFormatIco: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3a860c4_338f_4c17_919a_fba4b5628f21);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_ContainerFormatJpeg: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19e4a5aa_5662_4fc5_a0c0_1758028e1057);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_ContainerFormatPng: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b7cfaf4_713f_473c_bbcd_6137425faeaf);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_ContainerFormatRaw: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe99ce60_f19c_433c_a3ae_00acefa9ca21);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_ContainerFormatTiff: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x163bcc30_e2e9_4f0b_961d_a3e9fdb788a3);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_ContainerFormatWebp: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe094b0e2_67f2_45b3_b0ea_115337ca7cf3);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_ContainerFormatWmp: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57a37caa_367a_4540_916b_f183c5093a4b);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormat8BIMIPTC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0010568c_0852_4e6a_b191_5c33ac5b0430);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormat8BIMIPTCDigest: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ca32285_9ccd_4786_8bd8_79539db6a006);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormat8BIMResolutionInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x739f305d_81db_43cb_ac5e_55013ef9f003);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatAPE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e043dc2_c967_4e05_875e_618bf67e85c3);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatApp0: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79007028_268d_45d6_a3c2_354e6a504bc9);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatApp1: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fd3dfc3_f951_492b_817f_69c2e6d9a5b0);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatApp13: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x326556a2_f502_4354_9cc0_8e3f48eaf6b5);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatChunkbKGD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe14d3571_6b47_4dea_b60a_87ce0a78dfb7);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatChunkcHRM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9db3655b_2842_44b3_8067_12e9b375556a);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatChunkgAMA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf00935a5_1d5d_4cd1_81b2_9324d7eca781);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatChunkhIST: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc59a82da_db74_48a4_bd6a_b69c4931ef95);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatChunkiCCP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeb4349ab_b685_450f_91b5_e802e892536c);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatChunkiTXt: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2bec729_0b68_4b77_aa0e_6295a6ac1814);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatChunksRGB: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc115fd36_cc6f_4e3f_8363_524b87c6b0d9);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatChunktEXt: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x568d8936_c0a9_4923_905d_df2b38238fbc);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatChunktIME: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b00ae2d_e24b_460a_98b6_878bd03072fd);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatDds: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a064603_8c33_4e60_9c29_136231702d08);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatExif: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c3c4f9d_b84a_467d_9493_36cfbd59ea57);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatGCE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a25cad8_deeb_4c69_a788_0ec2266dcafd);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatGifComment: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4b6e0e0_cfb4_4ad3_ab33_9aad2355a34a);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatGps: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7134ab8a_9351_44ad_af62_448db6b502ec);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatHeif: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x817ef3e1_1288_45f4_a852_260d9e7cce83);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatHeifHDR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x568b8d8a_1e65_438c_8968_d60e1012beb9);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatIMD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd2bb086_4d52_48dd_9677_db483e85ae8f);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatIPTC: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4fab0914_e129_4087_a1d1_bc812d45a7b5);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatIRB: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16100d66_8570_4bb9_b92d_fda4b23ece67);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatIfd: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x537396c6_2d8a_4bb6_9bf8_2f0a8e2a3adf);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatInterop: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed686f8e_681f_4c8b_bd41_a8addbf6b3fc);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatJpegChrominance: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf73d0dcf_cec6_4f85_9b0e_1c3956b1bef7);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatJpegComment: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x220e5f33_afd3_474e_9d31_7d4fe730f557);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatJpegLuminance: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86908007_edfc_4860_8d4b_4ee6e83e6058);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatLSD: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe256031e_6299_4929_b98d_5ac884afba92);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatSubIfd: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58a2e128_2db9_4e57_bb14_5177891ed331);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatThumbnail: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x243dcee9_8703_40ee_8ef0_22a600b8058c);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatUnknown: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa45e592f_9078_4a7c_adb5_4edc4fd61b1f);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatWebpANIM: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dc4fda6_78e6_4102_ae35_bcfa1edcc78b);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatWebpANMF: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43c105ee_b93b_4abb_b003_a08c0d870471);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatXMP: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb5acc38_f216_4cec_a6c5_5f6e739763a9);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatXMPAlt: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b08a675_91aa_481b_a798_4da94908613b);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatXMPBag: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x833cca5f_dcb7_4516_806f_6596ab26dce4);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatXMPSeq: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63e8df02_eb6c_456c_a224_b25e794fd648);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_MetadataFormatXMPStruct: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22383cf1_ed17_4e2e_af17_d85b8f6b30d0);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_VendorMicrosoft: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0e749ca_edef_4589_a73a_ee0e626a2a2b);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_VendorMicrosoftBuiltIn: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x257a30fd_06b6_462b_aea4_63f70b86e533);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat112bpp6ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc937);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat112bpp7Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92a);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat128bpp7ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc938);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat128bpp8Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92b);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat128bppPRGBAFloat: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91a);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat128bppRGBAFixedPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91e);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat128bppRGBAFloat: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc919);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat128bppRGBFixedPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc941);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat128bppRGBFloat: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91b);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat144bpp8ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc939);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat16bppBGR555: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc909);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat16bppBGR565: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90a);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat16bppBGRA5551: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05ec7c2b_f1e6_4961_ad46_e1cc810a87d2);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat16bppCbCr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff95ba6e_11e0_4263_bb45_01721f3460a4);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat16bppCbQuantizedDctCoefficients: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd2c4ff61_56a5_49c2_8b5c_4c1925964837);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat16bppCrQuantizedDctCoefficients: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2fe354f0_1680_42d8_9231_e73c0565bfc1);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat16bppGray: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90b);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat16bppGrayFixedPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc913);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat16bppGrayHalf: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93e);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat16bppYQuantizedDctCoefficients: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa355f433_48e8_4a42_84d8_e2aa26ca80a4);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat1bppIndexed: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc901);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat24bpp3Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc920);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat24bppBGR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90c);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat24bppRGB: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90d);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat2bppGray: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc906);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat2bppIndexed: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc902);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat32bpp3ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92e);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat32bpp4Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc921);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat32bppBGR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90e);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat32bppBGR101010: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc914);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat32bppBGRA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc90f);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat32bppCMYK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91c);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat32bppGrayFixedPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93f);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat32bppGrayFloat: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc911);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat32bppPBGRA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc910);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat32bppPRGBA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3cc4a650_a527_4d37_a916_3142c7ebedba);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat32bppR10G10B10A2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x604e1bb5_8a3c_4b65_b11c_bc0b8dd75b7f);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat32bppR10G10B10A2HDR10: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c215c5d_1acc_4f0e_a4bc_70fb3ae8fd28);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat32bppRGB: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd98c6b95_3efe_47d6_bb25_eb1748ab0cf1);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat32bppRGBA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf5c7ad2d_6a8d_43dd_a7a8_a29935261ae9);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat32bppRGBA1010102: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25238d72_fcf9_4522_b514_5578e5ad55e0);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat32bppRGBA1010102XR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00de6b9a_c101_434b_b502_d0165ee1122c);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat32bppRGBE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93d);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat40bpp4ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92f);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat40bpp5Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc922);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat40bppCMYKAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92c);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat48bpp3Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc926);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat48bpp5ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc930);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat48bpp6Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc923);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat48bppBGR: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe605a384_b468_46ce_bb2e_36f180e64313);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat48bppBGRFixedPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49ca140e_cab6_493b_9ddf_60187c37532a);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat48bppRGB: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc915);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat48bppRGBFixedPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc912);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat48bppRGBHalf: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93b);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat4bppGray: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc907);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat4bppIndexed: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc903);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat56bpp6ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc931);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat56bpp7Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc924);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat64bpp3ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc934);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat64bpp4Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc927);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat64bpp7ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc932);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat64bpp8Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc925);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat64bppBGRA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1562ff7c_d352_46f9_979e_42976b792246);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat64bppBGRAFixedPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x356de33c_54d2_4a23_bb04_9b7bf9b1d42d);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat64bppCMYK: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91f);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat64bppPBGRA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c518e8e_a4ec_468b_ae70_c9a35a9c5530);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat64bppPRGBA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc917);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat64bppPRGBAHalf: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x58ad26c2_c623_4d9d_b320_387e49f8c442);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat64bppRGB: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1182111_186d_4d42_bc6a_9c8303a8dff9);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat64bppRGBA: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc916);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat64bppRGBAFixedPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc91d);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat64bppRGBAHalf: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc93a);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat64bppRGBFixedPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc940);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat64bppRGBHalf: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc942);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat72bpp8ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc933);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat80bpp4ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc935);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat80bpp5Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc928);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat80bppCMYKAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc92d);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat8bppAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6cd0116_eeba_4161_aa85_27dd9fb3a895);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat8bppCb: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1339f224_6bfe_4c3e_9302_e4f3a6d0ca2a);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat8bppCr: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8145053_2116_49f0_8835_ed844b205c51);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat8bppGray: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc908);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat8bppIndexed: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc904);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat8bppY: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91b4db54_2df9_42f0_b449_2909bb3df88e);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat96bpp5ChannelsAlpha: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc936);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat96bpp6Channels: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc929);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat96bppRGBFixedPoint: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc918);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormat96bppRGBFloat: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3fed78f_e8db_4acf_84c1_e97f6136b327);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormatBlackWhite: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc905);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const GUID_WICPixelFormatDontCare: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fddc324_4e03_4bfe_b185_3d77768dc900);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_Contrast: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_DestinationColorContext: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_ExposureCompensation: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_Gamma: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_KelvinWhitePoint: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_NamedWhitePoint: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_NoiseReduction: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_RGBWhitePoint: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_RenderMode: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_Rotation: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_Saturation: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_Sharpness: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_Tint: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawChangeNotification_ToneCurve: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_HUFFMAN_BASELINE_ONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_HUFFMAN_BASELINE_THREE: u32 = 1118464u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_MAX_COMPONENT_COUNT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_MAX_TABLE_INDEX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_QUANTIZATION_BASELINE_ONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_QUANTIZATION_BASELINE_THREE: u32 = 65792u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_SAMPLE_FACTORS_ONE: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_420: u32 = 1118498u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_422: u32 = 1118497u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_440: u32 = 1118482u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC_JPEG_SAMPLE_FACTORS_THREE_444: u32 = 1118481u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WINCODEC_ERR_ABORTED: i32 = -2147467260i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WINCODEC_ERR_ACCESSDENIED: i32 = -2147024891i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WINCODEC_ERR_BASE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WINCODEC_ERR_GENERIC_ERROR: i32 = -2147467259i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WINCODEC_ERR_INVALIDPARAMETER: i32 = -2147024809i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WINCODEC_ERR_NOTIMPLEMENTED: i32 = -2147467263i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WINCODEC_ERR_OUTOFMEMORY: i32 = -2147024882i32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WINCODEC_SDK_VERSION: u32 = 567u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WINCODEC_SDK_VERSION1: u32 = 566u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WINCODEC_SDK_VERSION2: u32 = 567u32;
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WIC8BIMIptcDigestProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMIptcDigestPString: WIC8BIMIptcDigestProperties = WIC8BIMIptcDigestProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMIptcDigestIptcDigest: WIC8BIMIptcDigestProperties = WIC8BIMIptcDigestProperties(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMIptcDigestProperties_FORCE_DWORD: WIC8BIMIptcDigestProperties = WIC8BIMIptcDigestProperties(2147483647u32);
impl ::core::marker::Copy for WIC8BIMIptcDigestProperties {}
impl ::core::clone::Clone for WIC8BIMIptcDigestProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WIC8BIMIptcDigestProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WIC8BIMIptcDigestProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WIC8BIMIptcDigestProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIC8BIMIptcDigestProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WIC8BIMIptcProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMIptcPString: WIC8BIMIptcProperties = WIC8BIMIptcProperties(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMIptcEmbeddedIPTC: WIC8BIMIptcProperties = WIC8BIMIptcProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMIptcProperties_FORCE_DWORD: WIC8BIMIptcProperties = WIC8BIMIptcProperties(2147483647u32);
impl ::core::marker::Copy for WIC8BIMIptcProperties {}
impl ::core::clone::Clone for WIC8BIMIptcProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WIC8BIMIptcProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WIC8BIMIptcProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WIC8BIMIptcProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIC8BIMIptcProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WIC8BIMResolutionInfoProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMResolutionInfoPString: WIC8BIMResolutionInfoProperties = WIC8BIMResolutionInfoProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMResolutionInfoHResolution: WIC8BIMResolutionInfoProperties = WIC8BIMResolutionInfoProperties(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMResolutionInfoHResolutionUnit: WIC8BIMResolutionInfoProperties = WIC8BIMResolutionInfoProperties(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMResolutionInfoWidthUnit: WIC8BIMResolutionInfoProperties = WIC8BIMResolutionInfoProperties(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMResolutionInfoVResolution: WIC8BIMResolutionInfoProperties = WIC8BIMResolutionInfoProperties(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMResolutionInfoVResolutionUnit: WIC8BIMResolutionInfoProperties = WIC8BIMResolutionInfoProperties(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMResolutionInfoHeightUnit: WIC8BIMResolutionInfoProperties = WIC8BIMResolutionInfoProperties(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WIC8BIMResolutionInfoProperties_FORCE_DWORD: WIC8BIMResolutionInfoProperties = WIC8BIMResolutionInfoProperties(2147483647u32);
impl ::core::marker::Copy for WIC8BIMResolutionInfoProperties {}
impl ::core::clone::Clone for WIC8BIMResolutionInfoProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WIC8BIMResolutionInfoProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WIC8BIMResolutionInfoProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WIC8BIMResolutionInfoProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WIC8BIMResolutionInfoProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICBitmapAlphaChannelOption(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapUseAlpha: WICBitmapAlphaChannelOption = WICBitmapAlphaChannelOption(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapUsePremultipliedAlpha: WICBitmapAlphaChannelOption = WICBitmapAlphaChannelOption(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapIgnoreAlpha: WICBitmapAlphaChannelOption = WICBitmapAlphaChannelOption(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBITMAPALPHACHANNELOPTIONS_FORCE_DWORD: WICBitmapAlphaChannelOption = WICBitmapAlphaChannelOption(2147483647i32);
impl ::core::marker::Copy for WICBitmapAlphaChannelOption {}
impl ::core::clone::Clone for WICBitmapAlphaChannelOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICBitmapAlphaChannelOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICBitmapAlphaChannelOption {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICBitmapAlphaChannelOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICBitmapAlphaChannelOption").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICBitmapCreateCacheOption(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapNoCache: WICBitmapCreateCacheOption = WICBitmapCreateCacheOption(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapCacheOnDemand: WICBitmapCreateCacheOption = WICBitmapCreateCacheOption(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapCacheOnLoad: WICBitmapCreateCacheOption = WICBitmapCreateCacheOption(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBITMAPCREATECACHEOPTION_FORCE_DWORD: WICBitmapCreateCacheOption = WICBitmapCreateCacheOption(2147483647i32);
impl ::core::marker::Copy for WICBitmapCreateCacheOption {}
impl ::core::clone::Clone for WICBitmapCreateCacheOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICBitmapCreateCacheOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICBitmapCreateCacheOption {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICBitmapCreateCacheOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICBitmapCreateCacheOption").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICBitmapDecoderCapabilities(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDecoderCapabilitySameEncoder: WICBitmapDecoderCapabilities = WICBitmapDecoderCapabilities(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDecoderCapabilityCanDecodeAllImages: WICBitmapDecoderCapabilities = WICBitmapDecoderCapabilities(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDecoderCapabilityCanDecodeSomeImages: WICBitmapDecoderCapabilities = WICBitmapDecoderCapabilities(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDecoderCapabilityCanEnumerateMetadata: WICBitmapDecoderCapabilities = WICBitmapDecoderCapabilities(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDecoderCapabilityCanDecodeThumbnail: WICBitmapDecoderCapabilities = WICBitmapDecoderCapabilities(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBITMAPDECODERCAPABILITIES_FORCE_DWORD: WICBitmapDecoderCapabilities = WICBitmapDecoderCapabilities(2147483647i32);
impl ::core::marker::Copy for WICBitmapDecoderCapabilities {}
impl ::core::clone::Clone for WICBitmapDecoderCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICBitmapDecoderCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICBitmapDecoderCapabilities {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICBitmapDecoderCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICBitmapDecoderCapabilities").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICBitmapDitherType(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDitherTypeNone: WICBitmapDitherType = WICBitmapDitherType(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDitherTypeSolid: WICBitmapDitherType = WICBitmapDitherType(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDitherTypeOrdered4x4: WICBitmapDitherType = WICBitmapDitherType(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDitherTypeOrdered8x8: WICBitmapDitherType = WICBitmapDitherType(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDitherTypeOrdered16x16: WICBitmapDitherType = WICBitmapDitherType(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDitherTypeSpiral4x4: WICBitmapDitherType = WICBitmapDitherType(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDitherTypeSpiral8x8: WICBitmapDitherType = WICBitmapDitherType(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDitherTypeDualSpiral4x4: WICBitmapDitherType = WICBitmapDitherType(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDitherTypeDualSpiral8x8: WICBitmapDitherType = WICBitmapDitherType(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapDitherTypeErrorDiffusion: WICBitmapDitherType = WICBitmapDitherType(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBITMAPDITHERTYPE_FORCE_DWORD: WICBitmapDitherType = WICBitmapDitherType(2147483647i32);
impl ::core::marker::Copy for WICBitmapDitherType {}
impl ::core::clone::Clone for WICBitmapDitherType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICBitmapDitherType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICBitmapDitherType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICBitmapDitherType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICBitmapDitherType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICBitmapEncoderCacheOption(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapEncoderCacheInMemory: WICBitmapEncoderCacheOption = WICBitmapEncoderCacheOption(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapEncoderCacheTempFile: WICBitmapEncoderCacheOption = WICBitmapEncoderCacheOption(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapEncoderNoCache: WICBitmapEncoderCacheOption = WICBitmapEncoderCacheOption(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBITMAPENCODERCACHEOPTION_FORCE_DWORD: WICBitmapEncoderCacheOption = WICBitmapEncoderCacheOption(2147483647i32);
impl ::core::marker::Copy for WICBitmapEncoderCacheOption {}
impl ::core::clone::Clone for WICBitmapEncoderCacheOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICBitmapEncoderCacheOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICBitmapEncoderCacheOption {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICBitmapEncoderCacheOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICBitmapEncoderCacheOption").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICBitmapInterpolationMode(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapInterpolationModeNearestNeighbor: WICBitmapInterpolationMode = WICBitmapInterpolationMode(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapInterpolationModeLinear: WICBitmapInterpolationMode = WICBitmapInterpolationMode(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapInterpolationModeCubic: WICBitmapInterpolationMode = WICBitmapInterpolationMode(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapInterpolationModeFant: WICBitmapInterpolationMode = WICBitmapInterpolationMode(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapInterpolationModeHighQualityCubic: WICBitmapInterpolationMode = WICBitmapInterpolationMode(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBITMAPINTERPOLATIONMODE_FORCE_DWORD: WICBitmapInterpolationMode = WICBitmapInterpolationMode(2147483647i32);
impl ::core::marker::Copy for WICBitmapInterpolationMode {}
impl ::core::clone::Clone for WICBitmapInterpolationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICBitmapInterpolationMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICBitmapInterpolationMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICBitmapInterpolationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICBitmapInterpolationMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICBitmapLockFlags(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapLockRead: WICBitmapLockFlags = WICBitmapLockFlags(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapLockWrite: WICBitmapLockFlags = WICBitmapLockFlags(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBITMAPLOCKFLAGS_FORCE_DWORD: WICBitmapLockFlags = WICBitmapLockFlags(2147483647i32);
impl ::core::marker::Copy for WICBitmapLockFlags {}
impl ::core::clone::Clone for WICBitmapLockFlags {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICBitmapLockFlags {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICBitmapLockFlags {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICBitmapLockFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICBitmapLockFlags").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICBitmapPaletteType(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeCustom: WICBitmapPaletteType = WICBitmapPaletteType(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeMedianCut: WICBitmapPaletteType = WICBitmapPaletteType(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedBW: WICBitmapPaletteType = WICBitmapPaletteType(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedHalftone8: WICBitmapPaletteType = WICBitmapPaletteType(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedHalftone27: WICBitmapPaletteType = WICBitmapPaletteType(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedHalftone64: WICBitmapPaletteType = WICBitmapPaletteType(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedHalftone125: WICBitmapPaletteType = WICBitmapPaletteType(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedHalftone216: WICBitmapPaletteType = WICBitmapPaletteType(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedWebPalette: WICBitmapPaletteType = WICBitmapPaletteType(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedHalftone252: WICBitmapPaletteType = WICBitmapPaletteType(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedHalftone256: WICBitmapPaletteType = WICBitmapPaletteType(9i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedGray4: WICBitmapPaletteType = WICBitmapPaletteType(10i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedGray16: WICBitmapPaletteType = WICBitmapPaletteType(11i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapPaletteTypeFixedGray256: WICBitmapPaletteType = WICBitmapPaletteType(12i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBITMAPPALETTETYPE_FORCE_DWORD: WICBitmapPaletteType = WICBitmapPaletteType(2147483647i32);
impl ::core::marker::Copy for WICBitmapPaletteType {}
impl ::core::clone::Clone for WICBitmapPaletteType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICBitmapPaletteType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICBitmapPaletteType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICBitmapPaletteType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICBitmapPaletteType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICBitmapTransformOptions(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapTransformRotate0: WICBitmapTransformOptions = WICBitmapTransformOptions(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapTransformRotate90: WICBitmapTransformOptions = WICBitmapTransformOptions(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapTransformRotate180: WICBitmapTransformOptions = WICBitmapTransformOptions(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapTransformRotate270: WICBitmapTransformOptions = WICBitmapTransformOptions(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapTransformFlipHorizontal: WICBitmapTransformOptions = WICBitmapTransformOptions(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBitmapTransformFlipVertical: WICBitmapTransformOptions = WICBitmapTransformOptions(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICBITMAPTRANSFORMOPTIONS_FORCE_DWORD: WICBitmapTransformOptions = WICBitmapTransformOptions(2147483647i32);
impl ::core::marker::Copy for WICBitmapTransformOptions {}
impl ::core::clone::Clone for WICBitmapTransformOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICBitmapTransformOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICBitmapTransformOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICBitmapTransformOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICBitmapTransformOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICColorContextType(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICColorContextUninitialized: WICColorContextType = WICColorContextType(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICColorContextProfile: WICColorContextType = WICColorContextType(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICColorContextExifColorSpace: WICColorContextType = WICColorContextType(2i32);
impl ::core::marker::Copy for WICColorContextType {}
impl ::core::clone::Clone for WICColorContextType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICColorContextType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICColorContextType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICColorContextType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICColorContextType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICComponentEnumerateOptions(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICComponentEnumerateDefault: WICComponentEnumerateOptions = WICComponentEnumerateOptions(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICComponentEnumerateRefresh: WICComponentEnumerateOptions = WICComponentEnumerateOptions(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICComponentEnumerateDisabled: WICComponentEnumerateOptions = WICComponentEnumerateOptions(-2147483648i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICComponentEnumerateUnsigned: WICComponentEnumerateOptions = WICComponentEnumerateOptions(1073741824i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICComponentEnumerateBuiltInOnly: WICComponentEnumerateOptions = WICComponentEnumerateOptions(536870912i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICCOMPONENTENUMERATEOPTIONS_FORCE_DWORD: WICComponentEnumerateOptions = WICComponentEnumerateOptions(2147483647i32);
impl ::core::marker::Copy for WICComponentEnumerateOptions {}
impl ::core::clone::Clone for WICComponentEnumerateOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICComponentEnumerateOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICComponentEnumerateOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICComponentEnumerateOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICComponentEnumerateOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICComponentSigning(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICComponentSigned: WICComponentSigning = WICComponentSigning(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICComponentUnsigned: WICComponentSigning = WICComponentSigning(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICComponentSafe: WICComponentSigning = WICComponentSigning(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICComponentDisabled: WICComponentSigning = WICComponentSigning(-2147483648i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICCOMPONENTSIGNING_FORCE_DWORD: WICComponentSigning = WICComponentSigning(2147483647i32);
impl ::core::marker::Copy for WICComponentSigning {}
impl ::core::clone::Clone for WICComponentSigning {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICComponentSigning {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICComponentSigning {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICComponentSigning {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICComponentSigning").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICComponentType(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDecoder: WICComponentType = WICComponentType(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICEncoder: WICComponentType = WICComponentType(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPixelFormatConverter: WICComponentType = WICComponentType(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICMetadataReader: WICComponentType = WICComponentType(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICMetadataWriter: WICComponentType = WICComponentType(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPixelFormat: WICComponentType = WICComponentType(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICAllComponents: WICComponentType = WICComponentType(63i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICCOMPONENTTYPE_FORCE_DWORD: WICComponentType = WICComponentType(2147483647i32);
impl ::core::marker::Copy for WICComponentType {}
impl ::core::clone::Clone for WICComponentType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICComponentType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICComponentType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICComponentType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICComponentType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICDdsAlphaMode(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDdsAlphaModeUnknown: WICDdsAlphaMode = WICDdsAlphaMode(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDdsAlphaModeStraight: WICDdsAlphaMode = WICDdsAlphaMode(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDdsAlphaModePremultiplied: WICDdsAlphaMode = WICDdsAlphaMode(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDdsAlphaModeOpaque: WICDdsAlphaMode = WICDdsAlphaMode(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDdsAlphaModeCustom: WICDdsAlphaMode = WICDdsAlphaMode(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDDSALPHAMODE_FORCE_DWORD: WICDdsAlphaMode = WICDdsAlphaMode(2147483647i32);
impl ::core::marker::Copy for WICDdsAlphaMode {}
impl ::core::clone::Clone for WICDdsAlphaMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICDdsAlphaMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICDdsAlphaMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICDdsAlphaMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICDdsAlphaMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICDdsDimension(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDdsTexture1D: WICDdsDimension = WICDdsDimension(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDdsTexture2D: WICDdsDimension = WICDdsDimension(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDdsTexture3D: WICDdsDimension = WICDdsDimension(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDdsTextureCube: WICDdsDimension = WICDdsDimension(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDDSTEXTURE_FORCE_DWORD: WICDdsDimension = WICDdsDimension(2147483647i32);
impl ::core::marker::Copy for WICDdsDimension {}
impl ::core::clone::Clone for WICDdsDimension {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICDdsDimension {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICDdsDimension {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICDdsDimension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICDdsDimension").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICDecodeOptions(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDecodeMetadataCacheOnDemand: WICDecodeOptions = WICDecodeOptions(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICDecodeMetadataCacheOnLoad: WICDecodeOptions = WICDecodeOptions(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICMETADATACACHEOPTION_FORCE_DWORD: WICDecodeOptions = WICDecodeOptions(2147483647i32);
impl ::core::marker::Copy for WICDecodeOptions {}
impl ::core::clone::Clone for WICDecodeOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICDecodeOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICDecodeOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICDecodeOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICDecodeOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICGifApplicationExtensionProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifApplicationExtensionApplication: WICGifApplicationExtensionProperties = WICGifApplicationExtensionProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifApplicationExtensionData: WICGifApplicationExtensionProperties = WICGifApplicationExtensionProperties(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifApplicationExtensionProperties_FORCE_DWORD: WICGifApplicationExtensionProperties = WICGifApplicationExtensionProperties(2147483647u32);
impl ::core::marker::Copy for WICGifApplicationExtensionProperties {}
impl ::core::clone::Clone for WICGifApplicationExtensionProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICGifApplicationExtensionProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICGifApplicationExtensionProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICGifApplicationExtensionProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICGifApplicationExtensionProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICGifCommentExtensionProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifCommentExtensionText: WICGifCommentExtensionProperties = WICGifCommentExtensionProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifCommentExtensionProperties_FORCE_DWORD: WICGifCommentExtensionProperties = WICGifCommentExtensionProperties(2147483647u32);
impl ::core::marker::Copy for WICGifCommentExtensionProperties {}
impl ::core::clone::Clone for WICGifCommentExtensionProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICGifCommentExtensionProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICGifCommentExtensionProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICGifCommentExtensionProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICGifCommentExtensionProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICGifGraphicControlExtensionProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifGraphicControlExtensionDisposal: WICGifGraphicControlExtensionProperties = WICGifGraphicControlExtensionProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifGraphicControlExtensionUserInputFlag: WICGifGraphicControlExtensionProperties = WICGifGraphicControlExtensionProperties(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifGraphicControlExtensionTransparencyFlag: WICGifGraphicControlExtensionProperties = WICGifGraphicControlExtensionProperties(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifGraphicControlExtensionDelay: WICGifGraphicControlExtensionProperties = WICGifGraphicControlExtensionProperties(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifGraphicControlExtensionTransparentColorIndex: WICGifGraphicControlExtensionProperties = WICGifGraphicControlExtensionProperties(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifGraphicControlExtensionProperties_FORCE_DWORD: WICGifGraphicControlExtensionProperties = WICGifGraphicControlExtensionProperties(2147483647u32);
impl ::core::marker::Copy for WICGifGraphicControlExtensionProperties {}
impl ::core::clone::Clone for WICGifGraphicControlExtensionProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICGifGraphicControlExtensionProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICGifGraphicControlExtensionProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICGifGraphicControlExtensionProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICGifGraphicControlExtensionProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICGifImageDescriptorProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifImageDescriptorLeft: WICGifImageDescriptorProperties = WICGifImageDescriptorProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifImageDescriptorTop: WICGifImageDescriptorProperties = WICGifImageDescriptorProperties(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifImageDescriptorWidth: WICGifImageDescriptorProperties = WICGifImageDescriptorProperties(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifImageDescriptorHeight: WICGifImageDescriptorProperties = WICGifImageDescriptorProperties(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifImageDescriptorLocalColorTableFlag: WICGifImageDescriptorProperties = WICGifImageDescriptorProperties(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifImageDescriptorInterlaceFlag: WICGifImageDescriptorProperties = WICGifImageDescriptorProperties(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifImageDescriptorSortFlag: WICGifImageDescriptorProperties = WICGifImageDescriptorProperties(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifImageDescriptorLocalColorTableSize: WICGifImageDescriptorProperties = WICGifImageDescriptorProperties(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifImageDescriptorProperties_FORCE_DWORD: WICGifImageDescriptorProperties = WICGifImageDescriptorProperties(2147483647u32);
impl ::core::marker::Copy for WICGifImageDescriptorProperties {}
impl ::core::clone::Clone for WICGifImageDescriptorProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICGifImageDescriptorProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICGifImageDescriptorProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICGifImageDescriptorProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICGifImageDescriptorProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICGifLogicalScreenDescriptorProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifLogicalScreenSignature: WICGifLogicalScreenDescriptorProperties = WICGifLogicalScreenDescriptorProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifLogicalScreenDescriptorWidth: WICGifLogicalScreenDescriptorProperties = WICGifLogicalScreenDescriptorProperties(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifLogicalScreenDescriptorHeight: WICGifLogicalScreenDescriptorProperties = WICGifLogicalScreenDescriptorProperties(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifLogicalScreenDescriptorGlobalColorTableFlag: WICGifLogicalScreenDescriptorProperties = WICGifLogicalScreenDescriptorProperties(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifLogicalScreenDescriptorColorResolution: WICGifLogicalScreenDescriptorProperties = WICGifLogicalScreenDescriptorProperties(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifLogicalScreenDescriptorSortFlag: WICGifLogicalScreenDescriptorProperties = WICGifLogicalScreenDescriptorProperties(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifLogicalScreenDescriptorGlobalColorTableSize: WICGifLogicalScreenDescriptorProperties = WICGifLogicalScreenDescriptorProperties(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifLogicalScreenDescriptorBackgroundColorIndex: WICGifLogicalScreenDescriptorProperties = WICGifLogicalScreenDescriptorProperties(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifLogicalScreenDescriptorPixelAspectRatio: WICGifLogicalScreenDescriptorProperties = WICGifLogicalScreenDescriptorProperties(9u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICGifLogicalScreenDescriptorProperties_FORCE_DWORD: WICGifLogicalScreenDescriptorProperties = WICGifLogicalScreenDescriptorProperties(2147483647u32);
impl ::core::marker::Copy for WICGifLogicalScreenDescriptorProperties {}
impl ::core::clone::Clone for WICGifLogicalScreenDescriptorProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICGifLogicalScreenDescriptorProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICGifLogicalScreenDescriptorProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICGifLogicalScreenDescriptorProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICGifLogicalScreenDescriptorProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICHeifHdrProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICHeifHdrMaximumLuminanceLevel: WICHeifHdrProperties = WICHeifHdrProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICHeifHdrMaximumFrameAverageLuminanceLevel: WICHeifHdrProperties = WICHeifHdrProperties(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICHeifHdrMinimumMasteringDisplayLuminanceLevel: WICHeifHdrProperties = WICHeifHdrProperties(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICHeifHdrMaximumMasteringDisplayLuminanceLevel: WICHeifHdrProperties = WICHeifHdrProperties(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICHeifHdrCustomVideoPrimaries: WICHeifHdrProperties = WICHeifHdrProperties(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICHeifHdrProperties_FORCE_DWORD: WICHeifHdrProperties = WICHeifHdrProperties(2147483647u32);
impl ::core::marker::Copy for WICHeifHdrProperties {}
impl ::core::clone::Clone for WICHeifHdrProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICHeifHdrProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICHeifHdrProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICHeifHdrProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICHeifHdrProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICHeifProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICHeifOrientation: WICHeifProperties = WICHeifProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICHeifProperties_FORCE_DWORD: WICHeifProperties = WICHeifProperties(2147483647u32);
impl ::core::marker::Copy for WICHeifProperties {}
impl ::core::clone::Clone for WICHeifProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICHeifProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICHeifProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICHeifProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICHeifProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICJpegChrominanceProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegChrominanceTable: WICJpegChrominanceProperties = WICJpegChrominanceProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegChrominanceProperties_FORCE_DWORD: WICJpegChrominanceProperties = WICJpegChrominanceProperties(2147483647u32);
impl ::core::marker::Copy for WICJpegChrominanceProperties {}
impl ::core::clone::Clone for WICJpegChrominanceProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICJpegChrominanceProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICJpegChrominanceProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICJpegChrominanceProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICJpegChrominanceProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICJpegCommentProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegCommentText: WICJpegCommentProperties = WICJpegCommentProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegCommentProperties_FORCE_DWORD: WICJpegCommentProperties = WICJpegCommentProperties(2147483647u32);
impl ::core::marker::Copy for WICJpegCommentProperties {}
impl ::core::clone::Clone for WICJpegCommentProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICJpegCommentProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICJpegCommentProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICJpegCommentProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICJpegCommentProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICJpegIndexingOptions(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegIndexingOptionsGenerateOnDemand: WICJpegIndexingOptions = WICJpegIndexingOptions(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegIndexingOptionsGenerateOnLoad: WICJpegIndexingOptions = WICJpegIndexingOptions(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegIndexingOptions_FORCE_DWORD: WICJpegIndexingOptions = WICJpegIndexingOptions(2147483647u32);
impl ::core::marker::Copy for WICJpegIndexingOptions {}
impl ::core::clone::Clone for WICJpegIndexingOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICJpegIndexingOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICJpegIndexingOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICJpegIndexingOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICJpegIndexingOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICJpegLuminanceProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegLuminanceTable: WICJpegLuminanceProperties = WICJpegLuminanceProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegLuminanceProperties_FORCE_DWORD: WICJpegLuminanceProperties = WICJpegLuminanceProperties(2147483647u32);
impl ::core::marker::Copy for WICJpegLuminanceProperties {}
impl ::core::clone::Clone for WICJpegLuminanceProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICJpegLuminanceProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICJpegLuminanceProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICJpegLuminanceProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICJpegLuminanceProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICJpegScanType(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegScanTypeInterleaved: WICJpegScanType = WICJpegScanType(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegScanTypePlanarComponents: WICJpegScanType = WICJpegScanType(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegScanTypeProgressive: WICJpegScanType = WICJpegScanType(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegScanType_FORCE_DWORD: WICJpegScanType = WICJpegScanType(2147483647u32);
impl ::core::marker::Copy for WICJpegScanType {}
impl ::core::clone::Clone for WICJpegScanType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICJpegScanType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICJpegScanType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICJpegScanType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICJpegScanType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICJpegTransferMatrix(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegTransferMatrixIdentity: WICJpegTransferMatrix = WICJpegTransferMatrix(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegTransferMatrixBT601: WICJpegTransferMatrix = WICJpegTransferMatrix(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegTransferMatrix_FORCE_DWORD: WICJpegTransferMatrix = WICJpegTransferMatrix(2147483647u32);
impl ::core::marker::Copy for WICJpegTransferMatrix {}
impl ::core::clone::Clone for WICJpegTransferMatrix {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICJpegTransferMatrix {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICJpegTransferMatrix {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICJpegTransferMatrix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICJpegTransferMatrix").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICJpegYCrCbSubsamplingOption(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegYCrCbSubsamplingDefault: WICJpegYCrCbSubsamplingOption = WICJpegYCrCbSubsamplingOption(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegYCrCbSubsampling420: WICJpegYCrCbSubsamplingOption = WICJpegYCrCbSubsamplingOption(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegYCrCbSubsampling422: WICJpegYCrCbSubsamplingOption = WICJpegYCrCbSubsamplingOption(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegYCrCbSubsampling444: WICJpegYCrCbSubsamplingOption = WICJpegYCrCbSubsamplingOption(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJpegYCrCbSubsampling440: WICJpegYCrCbSubsamplingOption = WICJpegYCrCbSubsamplingOption(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICJPEGYCRCBSUBSAMPLING_FORCE_DWORD: WICJpegYCrCbSubsamplingOption = WICJpegYCrCbSubsamplingOption(2147483647i32);
impl ::core::marker::Copy for WICJpegYCrCbSubsamplingOption {}
impl ::core::clone::Clone for WICJpegYCrCbSubsamplingOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICJpegYCrCbSubsamplingOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICJpegYCrCbSubsamplingOption {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICJpegYCrCbSubsamplingOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICJpegYCrCbSubsamplingOption").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICMetadataCreationOptions(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICMetadataCreationDefault: WICMetadataCreationOptions = WICMetadataCreationOptions(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICMetadataCreationAllowUnknown: WICMetadataCreationOptions = WICMetadataCreationOptions(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICMetadataCreationFailUnknown: WICMetadataCreationOptions = WICMetadataCreationOptions(65536i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICMetadataCreationMask: WICMetadataCreationOptions = WICMetadataCreationOptions(-65536i32);
impl ::core::marker::Copy for WICMetadataCreationOptions {}
impl ::core::clone::Clone for WICMetadataCreationOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICMetadataCreationOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICMetadataCreationOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICMetadataCreationOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICMetadataCreationOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICNamedWhitePoint(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointDefault: WICNamedWhitePoint = WICNamedWhitePoint(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointDaylight: WICNamedWhitePoint = WICNamedWhitePoint(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointCloudy: WICNamedWhitePoint = WICNamedWhitePoint(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointShade: WICNamedWhitePoint = WICNamedWhitePoint(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointTungsten: WICNamedWhitePoint = WICNamedWhitePoint(16i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointFluorescent: WICNamedWhitePoint = WICNamedWhitePoint(32i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointFlash: WICNamedWhitePoint = WICNamedWhitePoint(64i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointUnderwater: WICNamedWhitePoint = WICNamedWhitePoint(128i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointCustom: WICNamedWhitePoint = WICNamedWhitePoint(256i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointAutoWhiteBalance: WICNamedWhitePoint = WICNamedWhitePoint(512i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWhitePointAsShot: WICNamedWhitePoint = WICNamedWhitePoint(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICNAMEDWHITEPOINT_FORCE_DWORD: WICNamedWhitePoint = WICNamedWhitePoint(2147483647i32);
impl ::core::marker::Copy for WICNamedWhitePoint {}
impl ::core::clone::Clone for WICNamedWhitePoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICNamedWhitePoint {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICNamedWhitePoint {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICNamedWhitePoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICNamedWhitePoint").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICPersistOptions(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPersistOptionDefault: WICPersistOptions = WICPersistOptions(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPersistOptionLittleEndian: WICPersistOptions = WICPersistOptions(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPersistOptionBigEndian: WICPersistOptions = WICPersistOptions(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPersistOptionStrictFormat: WICPersistOptions = WICPersistOptions(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPersistOptionNoCacheStream: WICPersistOptions = WICPersistOptions(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPersistOptionPreferUTF8: WICPersistOptions = WICPersistOptions(8i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPersistOptionMask: WICPersistOptions = WICPersistOptions(65535i32);
impl ::core::marker::Copy for WICPersistOptions {}
impl ::core::clone::Clone for WICPersistOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICPersistOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICPersistOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICPersistOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPersistOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICPixelFormatNumericRepresentation(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPixelFormatNumericRepresentationUnspecified: WICPixelFormatNumericRepresentation = WICPixelFormatNumericRepresentation(0u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPixelFormatNumericRepresentationIndexed: WICPixelFormatNumericRepresentation = WICPixelFormatNumericRepresentation(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPixelFormatNumericRepresentationUnsignedInteger: WICPixelFormatNumericRepresentation = WICPixelFormatNumericRepresentation(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPixelFormatNumericRepresentationSignedInteger: WICPixelFormatNumericRepresentation = WICPixelFormatNumericRepresentation(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPixelFormatNumericRepresentationFixed: WICPixelFormatNumericRepresentation = WICPixelFormatNumericRepresentation(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPixelFormatNumericRepresentationFloat: WICPixelFormatNumericRepresentation = WICPixelFormatNumericRepresentation(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPixelFormatNumericRepresentation_FORCE_DWORD: WICPixelFormatNumericRepresentation = WICPixelFormatNumericRepresentation(2147483647u32);
impl ::core::marker::Copy for WICPixelFormatNumericRepresentation {}
impl ::core::clone::Clone for WICPixelFormatNumericRepresentation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICPixelFormatNumericRepresentation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICPixelFormatNumericRepresentation {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICPixelFormatNumericRepresentation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPixelFormatNumericRepresentation").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICPlanarOptions(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPlanarOptionsDefault: WICPlanarOptions = WICPlanarOptions(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPlanarOptionsPreserveSubsampling: WICPlanarOptions = WICPlanarOptions(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPLANAROPTIONS_FORCE_DWORD: WICPlanarOptions = WICPlanarOptions(2147483647i32);
impl ::core::marker::Copy for WICPlanarOptions {}
impl ::core::clone::Clone for WICPlanarOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICPlanarOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICPlanarOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICPlanarOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPlanarOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICPngBkgdProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngBkgdBackgroundColor: WICPngBkgdProperties = WICPngBkgdProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngBkgdProperties_FORCE_DWORD: WICPngBkgdProperties = WICPngBkgdProperties(2147483647u32);
impl ::core::marker::Copy for WICPngBkgdProperties {}
impl ::core::clone::Clone for WICPngBkgdProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICPngBkgdProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICPngBkgdProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICPngBkgdProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPngBkgdProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICPngChrmProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngChrmWhitePointX: WICPngChrmProperties = WICPngChrmProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngChrmWhitePointY: WICPngChrmProperties = WICPngChrmProperties(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngChrmRedX: WICPngChrmProperties = WICPngChrmProperties(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngChrmRedY: WICPngChrmProperties = WICPngChrmProperties(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngChrmGreenX: WICPngChrmProperties = WICPngChrmProperties(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngChrmGreenY: WICPngChrmProperties = WICPngChrmProperties(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngChrmBlueX: WICPngChrmProperties = WICPngChrmProperties(7u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngChrmBlueY: WICPngChrmProperties = WICPngChrmProperties(8u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngChrmProperties_FORCE_DWORD: WICPngChrmProperties = WICPngChrmProperties(2147483647u32);
impl ::core::marker::Copy for WICPngChrmProperties {}
impl ::core::clone::Clone for WICPngChrmProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICPngChrmProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICPngChrmProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICPngChrmProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPngChrmProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICPngFilterOption(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngFilterUnspecified: WICPngFilterOption = WICPngFilterOption(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngFilterNone: WICPngFilterOption = WICPngFilterOption(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngFilterSub: WICPngFilterOption = WICPngFilterOption(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngFilterUp: WICPngFilterOption = WICPngFilterOption(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngFilterAverage: WICPngFilterOption = WICPngFilterOption(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngFilterPaeth: WICPngFilterOption = WICPngFilterOption(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngFilterAdaptive: WICPngFilterOption = WICPngFilterOption(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPNGFILTEROPTION_FORCE_DWORD: WICPngFilterOption = WICPngFilterOption(2147483647i32);
impl ::core::marker::Copy for WICPngFilterOption {}
impl ::core::clone::Clone for WICPngFilterOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICPngFilterOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICPngFilterOption {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICPngFilterOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPngFilterOption").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICPngGamaProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngGamaGamma: WICPngGamaProperties = WICPngGamaProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngGamaProperties_FORCE_DWORD: WICPngGamaProperties = WICPngGamaProperties(2147483647u32);
impl ::core::marker::Copy for WICPngGamaProperties {}
impl ::core::clone::Clone for WICPngGamaProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICPngGamaProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICPngGamaProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICPngGamaProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPngGamaProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICPngHistProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngHistFrequencies: WICPngHistProperties = WICPngHistProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngHistProperties_FORCE_DWORD: WICPngHistProperties = WICPngHistProperties(2147483647u32);
impl ::core::marker::Copy for WICPngHistProperties {}
impl ::core::clone::Clone for WICPngHistProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICPngHistProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICPngHistProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICPngHistProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPngHistProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICPngIccpProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngIccpProfileName: WICPngIccpProperties = WICPngIccpProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngIccpProfileData: WICPngIccpProperties = WICPngIccpProperties(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngIccpProperties_FORCE_DWORD: WICPngIccpProperties = WICPngIccpProperties(2147483647u32);
impl ::core::marker::Copy for WICPngIccpProperties {}
impl ::core::clone::Clone for WICPngIccpProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICPngIccpProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICPngIccpProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICPngIccpProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPngIccpProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICPngItxtProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngItxtKeyword: WICPngItxtProperties = WICPngItxtProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngItxtCompressionFlag: WICPngItxtProperties = WICPngItxtProperties(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngItxtLanguageTag: WICPngItxtProperties = WICPngItxtProperties(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngItxtTranslatedKeyword: WICPngItxtProperties = WICPngItxtProperties(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngItxtText: WICPngItxtProperties = WICPngItxtProperties(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngItxtProperties_FORCE_DWORD: WICPngItxtProperties = WICPngItxtProperties(2147483647u32);
impl ::core::marker::Copy for WICPngItxtProperties {}
impl ::core::clone::Clone for WICPngItxtProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICPngItxtProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICPngItxtProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICPngItxtProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPngItxtProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICPngSrgbProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngSrgbRenderingIntent: WICPngSrgbProperties = WICPngSrgbProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngSrgbProperties_FORCE_DWORD: WICPngSrgbProperties = WICPngSrgbProperties(2147483647u32);
impl ::core::marker::Copy for WICPngSrgbProperties {}
impl ::core::clone::Clone for WICPngSrgbProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICPngSrgbProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICPngSrgbProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICPngSrgbProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPngSrgbProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICPngTimeProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngTimeYear: WICPngTimeProperties = WICPngTimeProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngTimeMonth: WICPngTimeProperties = WICPngTimeProperties(2u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngTimeDay: WICPngTimeProperties = WICPngTimeProperties(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngTimeHour: WICPngTimeProperties = WICPngTimeProperties(4u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngTimeMinute: WICPngTimeProperties = WICPngTimeProperties(5u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngTimeSecond: WICPngTimeProperties = WICPngTimeProperties(6u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPngTimeProperties_FORCE_DWORD: WICPngTimeProperties = WICPngTimeProperties(2147483647u32);
impl ::core::marker::Copy for WICPngTimeProperties {}
impl ::core::clone::Clone for WICPngTimeProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICPngTimeProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICPngTimeProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICPngTimeProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICPngTimeProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICProgressNotification(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICProgressNotificationBegin: WICProgressNotification = WICProgressNotification(65536i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICProgressNotificationEnd: WICProgressNotification = WICProgressNotification(131072i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICProgressNotificationFrequent: WICProgressNotification = WICProgressNotification(262144i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICProgressNotificationAll: WICProgressNotification = WICProgressNotification(-65536i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPROGRESSNOTIFICATION_FORCE_DWORD: WICProgressNotification = WICProgressNotification(2147483647i32);
impl ::core::marker::Copy for WICProgressNotification {}
impl ::core::clone::Clone for WICProgressNotification {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICProgressNotification {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICProgressNotification {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICProgressNotification {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICProgressNotification").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICProgressOperation(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICProgressOperationCopyPixels: WICProgressOperation = WICProgressOperation(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICProgressOperationWritePixels: WICProgressOperation = WICProgressOperation(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICProgressOperationAll: WICProgressOperation = WICProgressOperation(65535i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICPROGRESSOPERATION_FORCE_DWORD: WICProgressOperation = WICProgressOperation(2147483647i32);
impl ::core::marker::Copy for WICProgressOperation {}
impl ::core::clone::Clone for WICProgressOperation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICProgressOperation {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICProgressOperation {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICProgressOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICProgressOperation").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICRawCapabilities(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawCapabilityNotSupported: WICRawCapabilities = WICRawCapabilities(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawCapabilityGetSupported: WICRawCapabilities = WICRawCapabilities(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawCapabilityFullySupported: WICRawCapabilities = WICRawCapabilities(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRAWCAPABILITIES_FORCE_DWORD: WICRawCapabilities = WICRawCapabilities(2147483647i32);
impl ::core::marker::Copy for WICRawCapabilities {}
impl ::core::clone::Clone for WICRawCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICRawCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICRawCapabilities {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICRawCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICRawCapabilities").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICRawParameterSet(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICAsShotParameterSet: WICRawParameterSet = WICRawParameterSet(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICUserAdjustedParameterSet: WICRawParameterSet = WICRawParameterSet(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICAutoAdjustedParameterSet: WICRawParameterSet = WICRawParameterSet(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRAWPARAMETERSET_FORCE_DWORD: WICRawParameterSet = WICRawParameterSet(2147483647i32);
impl ::core::marker::Copy for WICRawParameterSet {}
impl ::core::clone::Clone for WICRawParameterSet {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICRawParameterSet {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICRawParameterSet {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICRawParameterSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICRawParameterSet").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICRawRenderMode(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawRenderModeDraft: WICRawRenderMode = WICRawRenderMode(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawRenderModeNormal: WICRawRenderMode = WICRawRenderMode(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawRenderModeBestQuality: WICRawRenderMode = WICRawRenderMode(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRAWRENDERMODE_FORCE_DWORD: WICRawRenderMode = WICRawRenderMode(2147483647i32);
impl ::core::marker::Copy for WICRawRenderMode {}
impl ::core::clone::Clone for WICRawRenderMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICRawRenderMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICRawRenderMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICRawRenderMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICRawRenderMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICRawRotationCapabilities(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawRotationCapabilityNotSupported: WICRawRotationCapabilities = WICRawRotationCapabilities(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawRotationCapabilityGetSupported: WICRawRotationCapabilities = WICRawRotationCapabilities(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawRotationCapabilityNinetyDegreesSupported: WICRawRotationCapabilities = WICRawRotationCapabilities(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRawRotationCapabilityFullySupported: WICRawRotationCapabilities = WICRawRotationCapabilities(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICRAWROTATIONCAPABILITIES_FORCE_DWORD: WICRawRotationCapabilities = WICRawRotationCapabilities(2147483647i32);
impl ::core::marker::Copy for WICRawRotationCapabilities {}
impl ::core::clone::Clone for WICRawRotationCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICRawRotationCapabilities {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICRawRotationCapabilities {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICRawRotationCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICRawRotationCapabilities").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICSectionAccessLevel(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICSectionAccessLevelRead: WICSectionAccessLevel = WICSectionAccessLevel(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICSectionAccessLevelReadWrite: WICSectionAccessLevel = WICSectionAccessLevel(3u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICSectionAccessLevel_FORCE_DWORD: WICSectionAccessLevel = WICSectionAccessLevel(2147483647u32);
impl ::core::marker::Copy for WICSectionAccessLevel {}
impl ::core::clone::Clone for WICSectionAccessLevel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICSectionAccessLevel {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICSectionAccessLevel {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICSectionAccessLevel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICSectionAccessLevel").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICTiffCompressionOption(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICTiffCompressionDontCare: WICTiffCompressionOption = WICTiffCompressionOption(0i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICTiffCompressionNone: WICTiffCompressionOption = WICTiffCompressionOption(1i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICTiffCompressionCCITT3: WICTiffCompressionOption = WICTiffCompressionOption(2i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICTiffCompressionCCITT4: WICTiffCompressionOption = WICTiffCompressionOption(3i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICTiffCompressionLZW: WICTiffCompressionOption = WICTiffCompressionOption(4i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICTiffCompressionRLE: WICTiffCompressionOption = WICTiffCompressionOption(5i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICTiffCompressionZIP: WICTiffCompressionOption = WICTiffCompressionOption(6i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICTiffCompressionLZWHDifferencing: WICTiffCompressionOption = WICTiffCompressionOption(7i32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICTIFFCOMPRESSIONOPTION_FORCE_DWORD: WICTiffCompressionOption = WICTiffCompressionOption(2147483647i32);
impl ::core::marker::Copy for WICTiffCompressionOption {}
impl ::core::clone::Clone for WICTiffCompressionOption {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICTiffCompressionOption {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICTiffCompressionOption {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICTiffCompressionOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICTiffCompressionOption").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICWebpAnimProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWebpAnimLoopCount: WICWebpAnimProperties = WICWebpAnimProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWebpAnimProperties_FORCE_DWORD: WICWebpAnimProperties = WICWebpAnimProperties(2147483647u32);
impl ::core::marker::Copy for WICWebpAnimProperties {}
impl ::core::clone::Clone for WICWebpAnimProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICWebpAnimProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICWebpAnimProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICWebpAnimProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICWebpAnimProperties").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WICWebpAnmfProperties(pub u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWebpAnmfFrameDuration: WICWebpAnmfProperties = WICWebpAnmfProperties(1u32);
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub const WICWebpAnmfProperties_FORCE_DWORD: WICWebpAnmfProperties = WICWebpAnmfProperties(2147483647u32);
impl ::core::marker::Copy for WICWebpAnmfProperties {}
impl ::core::clone::Clone for WICWebpAnmfProperties {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WICWebpAnmfProperties {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WICWebpAnmfProperties {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WICWebpAnmfProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WICWebpAnmfProperties").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WICBitmapPattern {
    pub Position: u64,
    pub Length: u32,
    pub Pattern: *mut u8,
    pub Mask: *mut u8,
    pub EndOfStream: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WICBitmapPattern {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WICBitmapPattern {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WICBitmapPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICBitmapPattern").field("Position", &self.Position).field("Length", &self.Length).field("Pattern", &self.Pattern).field("Mask", &self.Mask).field("EndOfStream", &self.EndOfStream).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WICBitmapPattern {
    type TypeKind = ::windows::core::CopyType;
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
impl ::core::default::Default for WICBitmapPattern {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub struct WICBitmapPlane {
    pub Format: ::windows::core::GUID,
    pub pbBuffer: *mut u8,
    pub cbStride: u32,
    pub cbBufferSize: u32,
}
impl ::core::marker::Copy for WICBitmapPlane {}
impl ::core::clone::Clone for WICBitmapPlane {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WICBitmapPlane {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICBitmapPlane").field("Format", &self.Format).field("pbBuffer", &self.pbBuffer).field("cbStride", &self.cbStride).field("cbBufferSize", &self.cbBufferSize).finish()
    }
}
impl ::windows::core::TypeKind for WICBitmapPlane {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WICBitmapPlane {
    fn eq(&self, other: &Self) -> bool {
        self.Format == other.Format && self.pbBuffer == other.pbBuffer && self.cbStride == other.cbStride && self.cbBufferSize == other.cbBufferSize
    }
}
impl ::core::cmp::Eq for WICBitmapPlane {}
impl ::core::default::Default for WICBitmapPlane {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub struct WICBitmapPlaneDescription {
    pub Format: ::windows::core::GUID,
    pub Width: u32,
    pub Height: u32,
}
impl ::core::marker::Copy for WICBitmapPlaneDescription {}
impl ::core::clone::Clone for WICBitmapPlaneDescription {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WICBitmapPlaneDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICBitmapPlaneDescription").field("Format", &self.Format).field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::windows::core::TypeKind for WICBitmapPlaneDescription {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WICBitmapPlaneDescription {
    fn eq(&self, other: &Self) -> bool {
        self.Format == other.Format && self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for WICBitmapPlaneDescription {}
impl ::core::default::Default for WICBitmapPlaneDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct WICDdsFormatInfo {
    pub DxgiFormat: super::Dxgi::Common::DXGI_FORMAT,
    pub BytesPerBlock: u32,
    pub BlockWidth: u32,
    pub BlockHeight: u32,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for WICDdsFormatInfo {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for WICDdsFormatInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for WICDdsFormatInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICDdsFormatInfo").field("DxgiFormat", &self.DxgiFormat).field("BytesPerBlock", &self.BytesPerBlock).field("BlockWidth", &self.BlockWidth).field("BlockHeight", &self.BlockHeight).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows::core::TypeKind for WICDdsFormatInfo {
    type TypeKind = ::windows::core::CopyType;
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
impl ::core::default::Default for WICDdsFormatInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct WICDdsParameters {
    pub Width: u32,
    pub Height: u32,
    pub Depth: u32,
    pub MipLevels: u32,
    pub ArraySize: u32,
    pub DxgiFormat: super::Dxgi::Common::DXGI_FORMAT,
    pub Dimension: WICDdsDimension,
    pub AlphaMode: WICDdsAlphaMode,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for WICDdsParameters {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for WICDdsParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for WICDdsParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICDdsParameters").field("Width", &self.Width).field("Height", &self.Height).field("Depth", &self.Depth).field("MipLevels", &self.MipLevels).field("ArraySize", &self.ArraySize).field("DxgiFormat", &self.DxgiFormat).field("Dimension", &self.Dimension).field("AlphaMode", &self.AlphaMode).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows::core::TypeKind for WICDdsParameters {
    type TypeKind = ::windows::core::CopyType;
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
impl ::core::default::Default for WICDdsParameters {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct WICImageParameters {
    pub PixelFormat: super::Direct2D::Common::D2D1_PIXEL_FORMAT,
    pub DpiX: f32,
    pub DpiY: f32,
    pub Top: f32,
    pub Left: f32,
    pub PixelWidth: u32,
    pub PixelHeight: u32,
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for WICImageParameters {}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for WICImageParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for WICImageParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICImageParameters").field("PixelFormat", &self.PixelFormat).field("DpiX", &self.DpiX).field("DpiY", &self.DpiY).field("Top", &self.Top).field("Left", &self.Left).field("PixelWidth", &self.PixelWidth).field("PixelHeight", &self.PixelHeight).finish()
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::TypeKind for WICImageParameters {
    type TypeKind = ::windows::core::CopyType;
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
impl ::core::default::Default for WICImageParameters {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub struct WICJpegFrameHeader {
    pub Width: u32,
    pub Height: u32,
    pub TransferMatrix: WICJpegTransferMatrix,
    pub ScanType: WICJpegScanType,
    pub cComponents: u32,
    pub ComponentIdentifiers: u32,
    pub SampleFactors: u32,
    pub QuantizationTableIndices: u32,
}
impl ::core::marker::Copy for WICJpegFrameHeader {}
impl ::core::clone::Clone for WICJpegFrameHeader {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WICJpegFrameHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICJpegFrameHeader").field("Width", &self.Width).field("Height", &self.Height).field("TransferMatrix", &self.TransferMatrix).field("ScanType", &self.ScanType).field("cComponents", &self.cComponents).field("ComponentIdentifiers", &self.ComponentIdentifiers).field("SampleFactors", &self.SampleFactors).field("QuantizationTableIndices", &self.QuantizationTableIndices).finish()
    }
}
impl ::windows::core::TypeKind for WICJpegFrameHeader {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WICJpegFrameHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.TransferMatrix == other.TransferMatrix && self.ScanType == other.ScanType && self.cComponents == other.cComponents && self.ComponentIdentifiers == other.ComponentIdentifiers && self.SampleFactors == other.SampleFactors && self.QuantizationTableIndices == other.QuantizationTableIndices
    }
}
impl ::core::cmp::Eq for WICJpegFrameHeader {}
impl ::core::default::Default for WICJpegFrameHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub struct WICJpegScanHeader {
    pub cComponents: u32,
    pub RestartInterval: u32,
    pub ComponentSelectors: u32,
    pub HuffmanTableIndices: u32,
    pub StartSpectralSelection: u8,
    pub EndSpectralSelection: u8,
    pub SuccessiveApproximationHigh: u8,
    pub SuccessiveApproximationLow: u8,
}
impl ::core::marker::Copy for WICJpegScanHeader {}
impl ::core::clone::Clone for WICJpegScanHeader {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::windows::core::TypeKind for WICJpegScanHeader {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WICJpegScanHeader {
    fn eq(&self, other: &Self) -> bool {
        self.cComponents == other.cComponents && self.RestartInterval == other.RestartInterval && self.ComponentSelectors == other.ComponentSelectors && self.HuffmanTableIndices == other.HuffmanTableIndices && self.StartSpectralSelection == other.StartSpectralSelection && self.EndSpectralSelection == other.EndSpectralSelection && self.SuccessiveApproximationHigh == other.SuccessiveApproximationHigh && self.SuccessiveApproximationLow == other.SuccessiveApproximationLow
    }
}
impl ::core::cmp::Eq for WICJpegScanHeader {}
impl ::core::default::Default for WICJpegScanHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub struct WICMetadataHeader {
    pub Position: u64,
    pub Length: u32,
    pub Header: *mut u8,
    pub DataOffset: u64,
}
impl ::core::marker::Copy for WICMetadataHeader {}
impl ::core::clone::Clone for WICMetadataHeader {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WICMetadataHeader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICMetadataHeader").field("Position", &self.Position).field("Length", &self.Length).field("Header", &self.Header).field("DataOffset", &self.DataOffset).finish()
    }
}
impl ::windows::core::TypeKind for WICMetadataHeader {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WICMetadataHeader {
    fn eq(&self, other: &Self) -> bool {
        self.Position == other.Position && self.Length == other.Length && self.Header == other.Header && self.DataOffset == other.DataOffset
    }
}
impl ::core::cmp::Eq for WICMetadataHeader {}
impl ::core::default::Default for WICMetadataHeader {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub struct WICMetadataPattern {
    pub Position: u64,
    pub Length: u32,
    pub Pattern: *mut u8,
    pub Mask: *mut u8,
    pub DataOffset: u64,
}
impl ::core::marker::Copy for WICMetadataPattern {}
impl ::core::clone::Clone for WICMetadataPattern {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WICMetadataPattern {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICMetadataPattern").field("Position", &self.Position).field("Length", &self.Length).field("Pattern", &self.Pattern).field("Mask", &self.Mask).field("DataOffset", &self.DataOffset).finish()
    }
}
impl ::windows::core::TypeKind for WICMetadataPattern {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WICMetadataPattern {
    fn eq(&self, other: &Self) -> bool {
        self.Position == other.Position && self.Length == other.Length && self.Pattern == other.Pattern && self.Mask == other.Mask && self.DataOffset == other.DataOffset
    }
}
impl ::core::cmp::Eq for WICMetadataPattern {}
impl ::core::default::Default for WICMetadataPattern {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub struct WICRawCapabilitiesInfo {
    pub cbSize: u32,
    pub CodecMajorVersion: u32,
    pub CodecMinorVersion: u32,
    pub ExposureCompensationSupport: WICRawCapabilities,
    pub ContrastSupport: WICRawCapabilities,
    pub RGBWhitePointSupport: WICRawCapabilities,
    pub NamedWhitePointSupport: WICRawCapabilities,
    pub NamedWhitePointSupportMask: u32,
    pub KelvinWhitePointSupport: WICRawCapabilities,
    pub GammaSupport: WICRawCapabilities,
    pub TintSupport: WICRawCapabilities,
    pub SaturationSupport: WICRawCapabilities,
    pub SharpnessSupport: WICRawCapabilities,
    pub NoiseReductionSupport: WICRawCapabilities,
    pub DestinationColorProfileSupport: WICRawCapabilities,
    pub ToneCurveSupport: WICRawCapabilities,
    pub RotationSupport: WICRawRotationCapabilities,
    pub RenderModeSupport: WICRawCapabilities,
}
impl ::core::marker::Copy for WICRawCapabilitiesInfo {}
impl ::core::clone::Clone for WICRawCapabilitiesInfo {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::windows::core::TypeKind for WICRawCapabilitiesInfo {
    type TypeKind = ::windows::core::CopyType;
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
impl ::core::default::Default for WICRawCapabilitiesInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub struct WICRawToneCurve {
    pub cPoints: u32,
    pub aPoints: [WICRawToneCurvePoint; 1],
}
impl ::core::marker::Copy for WICRawToneCurve {}
impl ::core::clone::Clone for WICRawToneCurve {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WICRawToneCurve {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICRawToneCurve").field("cPoints", &self.cPoints).field("aPoints", &self.aPoints).finish()
    }
}
impl ::windows::core::TypeKind for WICRawToneCurve {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WICRawToneCurve {
    fn eq(&self, other: &Self) -> bool {
        self.cPoints == other.cPoints && self.aPoints == other.aPoints
    }
}
impl ::core::cmp::Eq for WICRawToneCurve {}
impl ::core::default::Default for WICRawToneCurve {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub struct WICRawToneCurvePoint {
    pub Input: f64,
    pub Output: f64,
}
impl ::core::marker::Copy for WICRawToneCurvePoint {}
impl ::core::clone::Clone for WICRawToneCurvePoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WICRawToneCurvePoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICRawToneCurvePoint").field("Input", &self.Input).field("Output", &self.Output).finish()
    }
}
impl ::windows::core::TypeKind for WICRawToneCurvePoint {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WICRawToneCurvePoint {
    fn eq(&self, other: &Self) -> bool {
        self.Input == other.Input && self.Output == other.Output
    }
}
impl ::core::cmp::Eq for WICRawToneCurvePoint {}
impl ::core::default::Default for WICRawToneCurvePoint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub struct WICRect {
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
}
impl ::core::marker::Copy for WICRect {}
impl ::core::clone::Clone for WICRect {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WICRect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WICRect").field("X", &self.X).field("Y", &self.Y).field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
impl ::windows::core::TypeKind for WICRect {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WICRect {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Width == other.Width && self.Height == other.Height
    }
}
impl ::core::cmp::Eq for WICRect {}
impl ::core::default::Default for WICRect {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Imaging\"`*"]
pub type PFNProgressNotification = ::core::option::Option<unsafe extern "system" fn(pvdata: *const ::core::ffi::c_void, uframenum: u32, operation: WICProgressOperation, dblprogress: f64) -> ::windows::core::HRESULT>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
