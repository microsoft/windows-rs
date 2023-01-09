#[cfg(feature = "Win32_Storage_Xps_Printing")]
pub mod Printing;
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn AbortDoc<P0>(hdc: P0) -> i32
where
    P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn AbortDoc ( hdc : super::super::Graphics::Gdi:: HDC ) -> i32 );
    AbortDoc(hdc.into())
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DeviceCapabilitiesA<P0, P1>(pdevice: P0, pport: P1, fwcapability: DEVICE_CAPABILITIES, poutput: ::windows::core::PSTR, pdevmode: ::core::option::Option<*const super::super::Graphics::Gdi::DEVMODEA>) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeviceCapabilitiesA ( pdevice : :: windows::core::PCSTR , pport : :: windows::core::PCSTR , fwcapability : DEVICE_CAPABILITIES , poutput : :: windows::core::PSTR , pdevmode : *const super::super::Graphics::Gdi:: DEVMODEA ) -> i32 );
    DeviceCapabilitiesA(pdevice.into().abi(), pport.into().abi(), fwcapability, ::core::mem::transmute(poutput), ::core::mem::transmute(pdevmode.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DeviceCapabilitiesW<P0, P1>(pdevice: P0, pport: P1, fwcapability: DEVICE_CAPABILITIES, poutput: ::windows::core::PWSTR, pdevmode: ::core::option::Option<*const super::super::Graphics::Gdi::DEVMODEW>) -> i32
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "winspool.drv""system" fn DeviceCapabilitiesW ( pdevice : :: windows::core::PCWSTR , pport : :: windows::core::PCWSTR , fwcapability : DEVICE_CAPABILITIES , poutput : :: windows::core::PWSTR , pdevmode : *const super::super::Graphics::Gdi:: DEVMODEW ) -> i32 );
    DeviceCapabilitiesW(pdevice.into().abi(), pport.into().abi(), fwcapability, ::core::mem::transmute(poutput), ::core::mem::transmute(pdevmode.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EndDoc<P0>(hdc: P0) -> i32
where
    P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn EndDoc ( hdc : super::super::Graphics::Gdi:: HDC ) -> i32 );
    EndDoc(hdc.into())
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EndPage<P0>(hdc: P0) -> i32
where
    P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn EndPage ( hdc : super::super::Graphics::Gdi:: HDC ) -> i32 );
    EndPage(hdc.into())
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn Escape<P0>(hdc: P0, iescape: i32, pvin: ::core::option::Option<&[u8]>, pvout: ::core::option::Option<*mut ::core::ffi::c_void>) -> i32
where
    P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn Escape ( hdc : super::super::Graphics::Gdi:: HDC , iescape : i32 , cjin : i32 , pvin : :: windows::core::PCSTR , pvout : *mut ::core::ffi::c_void ) -> i32 );
    Escape(hdc.into(), iescape, pvin.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pvin.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(pvout.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ExtEscape<P0>(hdc: P0, iescape: i32, lpindata: ::core::option::Option<&[u8]>, lpoutdata: ::core::option::Option<&mut [u8]>) -> i32
where
    P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn ExtEscape ( hdc : super::super::Graphics::Gdi:: HDC , iescape : i32 , cjinput : i32 , lpindata : :: windows::core::PCSTR , cjoutput : i32 , lpoutdata : :: windows::core::PSTR ) -> i32 );
    ExtEscape(hdc.into(), iescape, lpindata.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpindata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpoutdata.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(lpoutdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn PrintWindow<P0, P1>(hwnd: P0, hdcblt: P1, nflags: PRINT_WINDOW_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "user32.dll""system" fn PrintWindow ( hwnd : super::super::Foundation:: HWND , hdcblt : super::super::Graphics::Gdi:: HDC , nflags : PRINT_WINDOW_FLAGS ) -> super::super::Foundation:: BOOL );
    PrintWindow(hwnd.into(), hdcblt.into(), nflags)
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn SetAbortProc<P0>(hdc: P0, proc: ABORTPROC) -> i32
where
    P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn SetAbortProc ( hdc : super::super::Graphics::Gdi:: HDC , proc : ABORTPROC ) -> i32 );
    SetAbortProc(hdc.into(), proc)
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn StartDocA<P0>(hdc: P0, lpdi: *const DOCINFOA) -> i32
where
    P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn StartDocA ( hdc : super::super::Graphics::Gdi:: HDC , lpdi : *const DOCINFOA ) -> i32 );
    StartDocA(hdc.into(), lpdi)
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn StartDocW<P0>(hdc: P0, lpdi: *const DOCINFOW) -> i32
where
    P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn StartDocW ( hdc : super::super::Graphics::Gdi:: HDC , lpdi : *const DOCINFOW ) -> i32 );
    StartDocW(hdc.into(), lpdi)
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn StartPage<P0>(hdc: P0) -> i32
where
    P0: ::std::convert::Into<super::super::Graphics::Gdi::HDC>,
{
    ::windows::core::link ! ( "gdi32.dll""system" fn StartPage ( hdc : super::super::Graphics::Gdi:: HDC ) -> i32 );
    StartPage(hdc.into())
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsDocumentPackageTarget(::windows::core::IUnknown);
impl IXpsDocumentPackageTarget {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetXpsOMPackageWriter<P0, P1>(&self, documentsequencepartname: P0, discardcontrolpartname: P1) -> ::windows::core::Result<IXpsOMPackageWriter>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetXpsOMPackageWriter)(::windows::core::Vtable::as_raw(self), documentsequencepartname.into().abi(), discardcontrolpartname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetXpsOMFactory(&self) -> ::windows::core::Result<IXpsOMObjectFactory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetXpsOMFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetXpsType(&self) -> ::windows::core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetXpsType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsDocumentPackageTarget, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsDocumentPackageTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsDocumentPackageTarget {
    type Vtable = IXpsDocumentPackageTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsDocumentPackageTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b0b6d38_53ad_41da_b212_d37637a6714e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsDocumentPackageTarget_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetXpsOMPackageWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequencepartname: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetXpsOMPackageWriter: usize,
    pub GetXpsOMFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpsfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetXpsType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsDocumentPackageTarget3D(::windows::core::IUnknown);
impl IXpsDocumentPackageTarget3D {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetXpsOMPackageWriter3D<P0, P1, P2, P3>(&self, documentsequencepartname: P0, discardcontrolpartname: P1, modelpartname: P2, modeldata: P3) -> ::windows::core::Result<IXpsOMPackageWriter3D>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P3: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetXpsOMPackageWriter3D)(::windows::core::Vtable::as_raw(self), documentsequencepartname.into().abi(), discardcontrolpartname.into().abi(), modelpartname.into().abi(), modeldata.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetXpsOMFactory(&self) -> ::windows::core::Result<IXpsOMObjectFactory> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetXpsOMFactory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsDocumentPackageTarget3D, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsDocumentPackageTarget3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsDocumentPackageTarget3D {
    type Vtable = IXpsDocumentPackageTarget3D_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsDocumentPackageTarget3D {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60ba71b8_3101_4984_9199_f4ea775ff01d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsDocumentPackageTarget3D_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetXpsOMPackageWriter3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequencepartname: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, modelpartname: *mut ::core::ffi::c_void, modeldata: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetXpsOMPackageWriter3D: usize,
    pub GetXpsOMFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpsfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMBrush(::windows::core::IUnknown);
impl IXpsOMBrush {
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOpacity)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOpacity)(::windows::core::Vtable::as_raw(self), opacity).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMBrush, ::windows::core::IUnknown, IXpsOMShareable);
impl ::core::clone::Clone for IXpsOMBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMBrush {
    type Vtable = IXpsOMBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMBrush {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56a3f80c_ea4c_4187_a57b_a2a473b2b42b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMBrush_Vtbl {
    pub base__: IXpsOMShareable_Vtbl,
    pub GetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMCanvas(::windows::core::IUnknown);
impl IXpsOMCanvas {
    pub unsafe fn GetVisuals(&self) -> ::windows::core::Result<IXpsOMVisualCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetVisuals)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUseAliasedEdgeMode(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUseAliasedEdgeMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseAliasedEdgeMode<P0>(&self, usealiasededgemode: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetUseAliasedEdgeMode)(::windows::core::Vtable::as_raw(self), usealiasededgemode.into()).ok()
    }
    pub unsafe fn GetAccessibilityShortDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAccessibilityShortDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAccessibilityShortDescription<P0>(&self, shortdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetAccessibilityShortDescription)(::windows::core::Vtable::as_raw(self), shortdescription.into().abi()).ok()
    }
    pub unsafe fn GetAccessibilityLongDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAccessibilityLongDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAccessibilityLongDescription<P0>(&self, longdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetAccessibilityLongDescription)(::windows::core::Vtable::as_raw(self), longdescription.into().abi()).ok()
    }
    pub unsafe fn GetDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDictionary)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDictionaryLocal(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDictionaryLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDictionaryLocal<P0>(&self, resourcedictionary: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMDictionary>>,
    {
        (::windows::core::Vtable::vtable(self).SetDictionaryLocal)(::windows::core::Vtable::as_raw(self), resourcedictionary.into().abi()).ok()
    }
    pub unsafe fn GetDictionaryResource(&self) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDictionaryResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDictionaryResource<P0>(&self, remotedictionaryresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMRemoteDictionaryResource>>,
    {
        (::windows::core::Vtable::vtable(self).SetDictionaryResource)(::windows::core::Vtable::as_raw(self), remotedictionaryresource.into().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMCanvas> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMCanvas, ::windows::core::IUnknown, IXpsOMShareable, IXpsOMVisual);
impl ::core::clone::Clone for IXpsOMCanvas {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMCanvas {
    type Vtable = IXpsOMCanvas_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMCanvas {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x221d1452_331e_47c6_87e9_6ccefb9b5ba3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMCanvas_Vtbl {
    pub base__: IXpsOMVisual_Vtbl,
    pub GetVisuals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visuals: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUseAliasedEdgeMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usealiasededgemode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUseAliasedEdgeMode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUseAliasedEdgeMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usealiasededgemode: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUseAliasedEdgeMode: usize,
    pub GetAccessibilityShortDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortdescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetAccessibilityShortDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortdescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetAccessibilityLongDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, longdescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetAccessibilityLongDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, longdescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetDictionary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDictionaryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDictionaryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDictionaryResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDictionaryResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, canvas: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMColorProfileResource(::windows::core::IUnknown);
impl IXpsOMColorProfileResource {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, partname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).SetContent)(::windows::core::Vtable::as_raw(self), sourcestream.into().abi(), partname.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMColorProfileResource, ::windows::core::IUnknown, IXpsOMPart, IXpsOMResource);
impl ::core::clone::Clone for IXpsOMColorProfileResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMColorProfileResource {
    type Vtable = IXpsOMColorProfileResource_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMColorProfileResource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x67bd7d69_1eef_4bb1_b5e7_6f4f87be8abe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMColorProfileResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMColorProfileResourceCollection(::windows::core::IUnknown);
impl IXpsOMColorProfileResourceCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMColorProfileResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, object: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMColorProfileResource>>,
    {
        (::windows::core::Vtable::vtable(self).InsertAt)(::windows::core::Vtable::as_raw(self), index, object.into().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveAt)(::windows::core::Vtable::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, object: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMColorProfileResource>>,
    {
        (::windows::core::Vtable::vtable(self).SetAt)(::windows::core::Vtable::as_raw(self), index, object.into().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, object: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMColorProfileResource>>,
    {
        (::windows::core::Vtable::vtable(self).Append)(::windows::core::Vtable::as_raw(self), object.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<P0>(&self, partname: P0) -> ::windows::core::Result<IXpsOMColorProfileResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetByPartName)(::windows::core::Vtable::as_raw(self), partname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMColorProfileResourceCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMColorProfileResourceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMColorProfileResourceCollection {
    type Vtable = IXpsOMColorProfileResourceCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMColorProfileResourceCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x12759630_5fba_4283_8f7d_cca849809edb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMColorProfileResourceCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMCoreProperties(::windows::core::IUnknown);
impl IXpsOMCoreProperties {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMPackage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCategory(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCategory)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCategory<P0>(&self, category: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetCategory)(::windows::core::Vtable::as_raw(self), category.into().abi()).ok()
    }
    pub unsafe fn GetContentStatus(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetContentStatus)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetContentStatus<P0>(&self, contentstatus: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetContentStatus)(::windows::core::Vtable::as_raw(self), contentstatus.into().abi()).ok()
    }
    pub unsafe fn GetContentType(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetContentType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetContentType<P0>(&self, contenttype: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetContentType)(::windows::core::Vtable::as_raw(self), contenttype.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCreated(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCreated)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCreated(&self, created: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCreated)(::windows::core::Vtable::as_raw(self), created).ok()
    }
    pub unsafe fn GetCreator(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCreator)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCreator<P0>(&self, creator: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetCreator)(::windows::core::Vtable::as_raw(self), creator.into().abi()).ok()
    }
    pub unsafe fn GetDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, description: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetDescription)(::windows::core::Vtable::as_raw(self), description.into().abi()).ok()
    }
    pub unsafe fn GetIdentifier(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIdentifier)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetIdentifier<P0>(&self, identifier: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetIdentifier)(::windows::core::Vtable::as_raw(self), identifier.into().abi()).ok()
    }
    pub unsafe fn GetKeywords(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetKeywords)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetKeywords<P0>(&self, keywords: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetKeywords)(::windows::core::Vtable::as_raw(self), keywords.into().abi()).ok()
    }
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLanguage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLanguage<P0>(&self, language: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetLanguage)(::windows::core::Vtable::as_raw(self), language.into().abi()).ok()
    }
    pub unsafe fn GetLastModifiedBy(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLastModifiedBy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLastModifiedBy<P0>(&self, lastmodifiedby: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetLastModifiedBy)(::windows::core::Vtable::as_raw(self), lastmodifiedby.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastPrinted(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLastPrinted)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastPrinted(&self, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetLastPrinted)(::windows::core::Vtable::as_raw(self), lastprinted).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetModified(&self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetModified)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetModified(&self, modified: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetModified)(::windows::core::Vtable::as_raw(self), modified).ok()
    }
    pub unsafe fn GetRevision(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRevision)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRevision<P0>(&self, revision: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetRevision)(::windows::core::Vtable::as_raw(self), revision.into().abi()).ok()
    }
    pub unsafe fn GetSubject(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSubject)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSubject<P0>(&self, subject: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetSubject)(::windows::core::Vtable::as_raw(self), subject.into().abi()).ok()
    }
    pub unsafe fn GetTitle(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTitle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTitle<P0>(&self, title: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetTitle)(::windows::core::Vtable::as_raw(self), title.into().abi()).ok()
    }
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetVersion<P0>(&self, version: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetVersion)(::windows::core::Vtable::as_raw(self), version.into().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMCoreProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMCoreProperties, ::windows::core::IUnknown, IXpsOMPart);
impl ::core::clone::Clone for IXpsOMCoreProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMCoreProperties {
    type Vtable = IXpsOMCoreProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMCoreProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3340fe8f_4027_4aa1_8f5f_d35ae45fe597);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMCoreProperties_Vtbl {
    pub base__: IXpsOMPart_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetContentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentstatus: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetContentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentstatus: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, created: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCreated: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, created: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCreated: usize,
    pub GetCreator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, creator: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetCreator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, creator: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetKeywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keywords: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetKeywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keywords: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetLastModifiedBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodifiedby: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetLastModifiedBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodifiedby: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLastPrinted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastprinted: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLastPrinted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLastPrinted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLastPrinted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modified: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetModified: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modified: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetModified: usize,
    pub GetRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, revision: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, revision: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subject: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subject: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coreproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMDashCollection(::windows::core::IUnknown);
impl IXpsOMDashCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<XPS_DASH> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InsertAt(&self, index: u32, dash: *const XPS_DASH) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).InsertAt)(::windows::core::Vtable::as_raw(self), index, dash).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveAt)(::windows::core::Vtable::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt(&self, index: u32, dash: *const XPS_DASH) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAt)(::windows::core::Vtable::as_raw(self), index, dash).ok()
    }
    pub unsafe fn Append(&self, dash: *const XPS_DASH) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Append)(::windows::core::Vtable::as_raw(self), dash).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMDashCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMDashCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMDashCollection {
    type Vtable = IXpsOMDashCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMDashCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x081613f4_74eb_48f2_83b3_37a9ce2d7dc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDashCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, dash: *mut XPS_DASH) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dash: *const XPS_DASH) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMDictionary(::windows::core::IUnknown);
impl IXpsOMDictionary {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32, key: *mut ::windows::core::PWSTR, entry: *mut ::core::option::Option<IXpsOMShareable>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), index, key, ::core::mem::transmute(entry)).ok()
    }
    pub unsafe fn GetByKey<P0, P1>(&self, key: P0, beforeentry: P1) -> ::windows::core::Result<IXpsOMShareable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IXpsOMShareable>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetByKey)(::windows::core::Vtable::as_raw(self), key.into().abi(), beforeentry.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetIndex<P0>(&self, entry: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMShareable>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIndex)(::windows::core::Vtable::as_raw(self), entry.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Append<P0, P1>(&self, key: P0, entry: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IXpsOMShareable>>,
    {
        (::windows::core::Vtable::vtable(self).Append)(::windows::core::Vtable::as_raw(self), key.into().abi(), entry.into().abi()).ok()
    }
    pub unsafe fn InsertAt<P0, P1>(&self, index: u32, key: P0, entry: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IXpsOMShareable>>,
    {
        (::windows::core::Vtable::vtable(self).InsertAt)(::windows::core::Vtable::as_raw(self), index, key.into().abi(), entry.into().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveAt)(::windows::core::Vtable::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0, P1>(&self, index: u32, key: P0, entry: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IXpsOMShareable>>,
    {
        (::windows::core::Vtable::vtable(self).SetAt)(::windows::core::Vtable::as_raw(self), index, key.into().abi(), entry.into().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMDictionary, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMDictionary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMDictionary {
    type Vtable = IXpsOMDictionary_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMDictionary {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x897c86b8_8eaf_4ae3_bdde_56419fcf4236);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDictionary_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, key: *mut ::windows::core::PWSTR, entry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetByKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::PCWSTR, beforeentry: *mut ::core::ffi::c_void, entry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entry: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::PCWSTR, entry: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, key: ::windows::core::PCWSTR, entry: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, key: ::windows::core::PCWSTR, entry: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMDocument(::windows::core::IUnknown);
impl IXpsOMDocument {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMDocumentSequence> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPageReferences(&self) -> ::windows::core::Result<IXpsOMPageReferenceCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPageReferences)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPrintTicketResource(&self) -> ::windows::core::Result<IXpsOMPrintTicketResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPrintTicketResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrintTicketResource<P0>(&self, printticketresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMPrintTicketResource>>,
    {
        (::windows::core::Vtable::vtable(self).SetPrintTicketResource)(::windows::core::Vtable::as_raw(self), printticketresource.into().abi()).ok()
    }
    pub unsafe fn GetDocumentStructureResource(&self) -> ::windows::core::Result<IXpsOMDocumentStructureResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDocumentStructureResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDocumentStructureResource<P0>(&self, documentstructureresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMDocumentStructureResource>>,
    {
        (::windows::core::Vtable::vtable(self).SetDocumentStructureResource)(::windows::core::Vtable::as_raw(self), documentstructureresource.into().abi()).ok()
    }
    pub unsafe fn GetSignatureBlockResources(&self) -> ::windows::core::Result<IXpsOMSignatureBlockResourceCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSignatureBlockResources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMDocument> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMDocument, ::windows::core::IUnknown, IXpsOMPart);
impl ::core::clone::Clone for IXpsOMDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMDocument {
    type Vtable = IXpsOMDocument_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMDocument {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2c2c94cb_ac5f_4254_8ee9_23948309d9f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocument_Vtbl {
    pub base__: IXpsOMPart_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPageReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagereferences: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDocumentStructureResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentstructureresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDocumentStructureResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentstructureresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSignatureBlockResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureblockresources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMDocumentCollection(::windows::core::IUnknown);
impl IXpsOMDocumentCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMDocument> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, document: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMDocument>>,
    {
        (::windows::core::Vtable::vtable(self).InsertAt)(::windows::core::Vtable::as_raw(self), index, document.into().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveAt)(::windows::core::Vtable::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, document: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMDocument>>,
    {
        (::windows::core::Vtable::vtable(self).SetAt)(::windows::core::Vtable::as_raw(self), index, document.into().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, document: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMDocument>>,
    {
        (::windows::core::Vtable::vtable(self).Append)(::windows::core::Vtable::as_raw(self), document.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMDocumentCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMDocumentCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMDocumentCollection {
    type Vtable = IXpsOMDocumentCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMDocumentCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1c87f0d_e947_4754_8a25_971478f7e83e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocumentCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, document: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, document: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, document: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMDocumentSequence(::windows::core::IUnknown);
impl IXpsOMDocumentSequence {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMPackage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDocuments(&self) -> ::windows::core::Result<IXpsOMDocumentCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDocuments)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPrintTicketResource(&self) -> ::windows::core::Result<IXpsOMPrintTicketResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPrintTicketResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrintTicketResource<P0>(&self, printticketresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMPrintTicketResource>>,
    {
        (::windows::core::Vtable::vtable(self).SetPrintTicketResource)(::windows::core::Vtable::as_raw(self), printticketresource.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMDocumentSequence, ::windows::core::IUnknown, IXpsOMPart);
impl ::core::clone::Clone for IXpsOMDocumentSequence {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMDocumentSequence {
    type Vtable = IXpsOMDocumentSequence_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMDocumentSequence {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56492eb4_d8d5_425e_8256_4c2b64ad0264);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocumentSequence_Vtbl {
    pub base__: IXpsOMPart_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDocuments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documents: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMDocumentStructureResource(::windows::core::IUnknown);
impl IXpsOMDocumentStructureResource {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMDocument> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, partname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).SetContent)(::windows::core::Vtable::as_raw(self), sourcestream.into().abi(), partname.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMDocumentStructureResource, ::windows::core::IUnknown, IXpsOMPart, IXpsOMResource);
impl ::core::clone::Clone for IXpsOMDocumentStructureResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMDocumentStructureResource {
    type Vtable = IXpsOMDocumentStructureResource_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMDocumentStructureResource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85febc8a_6b63_48a9_af07_7064e4ecff30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocumentStructureResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMFontResource(::windows::core::IUnknown);
impl IXpsOMFontResource {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, embeddingoption: XPS_FONT_EMBEDDING, partname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).SetContent)(::windows::core::Vtable::as_raw(self), sourcestream.into().abi(), embeddingoption, partname.into().abi()).ok()
    }
    pub unsafe fn GetEmbeddingOption(&self) -> ::windows::core::Result<XPS_FONT_EMBEDDING> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEmbeddingOption)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMFontResource, ::windows::core::IUnknown, IXpsOMPart, IXpsOMResource);
impl ::core::clone::Clone for IXpsOMFontResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMFontResource {
    type Vtable = IXpsOMFontResource_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMFontResource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8c45708_47d9_4af4_8d20_33b48c9b8485);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMFontResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readerstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, embeddingoption: XPS_FONT_EMBEDDING, partname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
    pub GetEmbeddingOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, embeddingoption: *mut XPS_FONT_EMBEDDING) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMFontResourceCollection(::windows::core::IUnknown);
impl IXpsOMFontResourceCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMFontResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMFontResource>>,
    {
        (::windows::core::Vtable::vtable(self).SetAt)(::windows::core::Vtable::as_raw(self), index, value.into().abi()).ok()
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMFontResource>>,
    {
        (::windows::core::Vtable::vtable(self).InsertAt)(::windows::core::Vtable::as_raw(self), index, value.into().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMFontResource>>,
    {
        (::windows::core::Vtable::vtable(self).Append)(::windows::core::Vtable::as_raw(self), value.into().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveAt)(::windows::core::Vtable::as_raw(self), index).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<P0>(&self, partname: P0) -> ::windows::core::Result<IXpsOMFontResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetByPartName)(::windows::core::Vtable::as_raw(self), partname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMFontResourceCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMFontResourceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMFontResourceCollection {
    type Vtable = IXpsOMFontResourceCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMFontResourceCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70b4a6bb_88d4_4fa8_aaf9_6d9c596fdbad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMFontResourceCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMGeometry(::windows::core::IUnknown);
impl IXpsOMGeometry {
    pub unsafe fn GetFigures(&self) -> ::windows::core::Result<IXpsOMGeometryFigureCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFigures)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFillRule(&self) -> ::windows::core::Result<XPS_FILL_RULE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFillRule)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFillRule(&self, fillrule: XPS_FILL_RULE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFillRule)(::windows::core::Vtable::as_raw(self), fillrule).ok()
    }
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTransform)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTransformLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, transform: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMMatrixTransform>>,
    {
        (::windows::core::Vtable::vtable(self).SetTransformLocal)(::windows::core::Vtable::as_raw(self), transform.into().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTransformLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, lookup: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetTransformLookup)(::windows::core::Vtable::as_raw(self), lookup.into().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMGeometry, ::windows::core::IUnknown, IXpsOMShareable);
impl ::core::clone::Clone for IXpsOMGeometry {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMGeometry {
    type Vtable = IXpsOMGeometry_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMGeometry {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64fcf3d7_4d58_44ba_ad73_a13af6492072);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGeometry_Vtbl {
    pub base__: IXpsOMShareable_Vtbl,
    pub GetFigures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, figures: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFillRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillrule: *mut XPS_FILL_RULE) -> ::windows::core::HRESULT,
    pub SetFillRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillrule: XPS_FILL_RULE) -> ::windows::core::HRESULT,
    pub GetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMGeometryFigure(::windows::core::IUnknown);
impl IXpsOMGeometryFigure {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSegmentData(&self, datacount: *mut u32, segmentdata: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetSegmentData)(::windows::core::Vtable::as_raw(self), datacount, segmentdata).ok()
    }
    pub unsafe fn GetSegmentTypes(&self, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetSegmentTypes)(::windows::core::Vtable::as_raw(self), segmentcount, segmenttypes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSegmentStrokes(&self, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetSegmentStrokes)(::windows::core::Vtable::as_raw(self), segmentcount, segmentstrokes).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSegments(&self, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSegments)(::windows::core::Vtable::as_raw(self), segmentcount, segmentdatacount, segmenttypes, segmentdata, segmentstrokes).ok()
    }
    pub unsafe fn GetStartPoint(&self) -> ::windows::core::Result<XPS_POINT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStartPoint)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStartPoint(&self, startpoint: *const XPS_POINT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetStartPoint)(::windows::core::Vtable::as_raw(self), startpoint).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsClosed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIsClosed)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsClosed<P0>(&self, isclosed: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetIsClosed)(::windows::core::Vtable::as_raw(self), isclosed.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsFilled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIsFilled)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsFilled<P0>(&self, isfilled: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetIsFilled)(::windows::core::Vtable::as_raw(self), isfilled.into()).ok()
    }
    pub unsafe fn GetSegmentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSegmentCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSegmentDataCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSegmentDataCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSegmentStrokePattern(&self) -> ::windows::core::Result<XPS_SEGMENT_STROKE_PATTERN> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSegmentStrokePattern)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMGeometryFigure> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMGeometryFigure, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMGeometryFigure {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMGeometryFigure {
    type Vtable = IXpsOMGeometryFigure_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMGeometryFigure {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd410dc83_908c_443e_8947_b1795d3c165a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGeometryFigure_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSegmentData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datacount: *mut u32, segmentdata: *mut f32) -> ::windows::core::HRESULT,
    pub GetSegmentTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSegmentStrokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSegmentStrokes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSegments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSegments: usize,
    pub GetStartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *mut XPS_POINT) -> ::windows::core::HRESULT,
    pub SetStartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsClosed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isclosed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsClosed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsFilled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isfilled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsFilled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsFilled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isfilled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsFilled: usize,
    pub GetSegmentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetSegmentDataCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentdatacount: *mut u32) -> ::windows::core::HRESULT,
    pub GetSegmentStrokePattern: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentstrokepattern: *mut XPS_SEGMENT_STROKE_PATTERN) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometryfigure: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMGeometryFigureCollection(::windows::core::IUnknown);
impl IXpsOMGeometryFigureCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMGeometryFigure> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, geometryfigure: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMGeometryFigure>>,
    {
        (::windows::core::Vtable::vtable(self).InsertAt)(::windows::core::Vtable::as_raw(self), index, geometryfigure.into().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveAt)(::windows::core::Vtable::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, geometryfigure: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMGeometryFigure>>,
    {
        (::windows::core::Vtable::vtable(self).SetAt)(::windows::core::Vtable::as_raw(self), index, geometryfigure.into().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, geometryfigure: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMGeometryFigure>>,
    {
        (::windows::core::Vtable::vtable(self).Append)(::windows::core::Vtable::as_raw(self), geometryfigure.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMGeometryFigureCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMGeometryFigureCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMGeometryFigureCollection {
    type Vtable = IXpsOMGeometryFigureCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMGeometryFigureCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd48c3f3_a58e_4b5a_8826_1de54abe72b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGeometryFigureCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometryfigure: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMGlyphs(::windows::core::IUnknown);
impl IXpsOMGlyphs {
    pub unsafe fn GetUnicodeString(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUnicodeString)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGlyphIndexCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGlyphIndexCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGlyphIndices(&self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetGlyphIndices)(::windows::core::Vtable::as_raw(self), indexcount, glyphindices).ok()
    }
    pub unsafe fn GetGlyphMappingCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGlyphMappingCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGlyphMappings(&self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetGlyphMappings)(::windows::core::Vtable::as_raw(self), glyphmappingcount, glyphmappings).ok()
    }
    pub unsafe fn GetProhibitedCaretStopCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetProhibitedCaretStopCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetProhibitedCaretStops(&self, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetProhibitedCaretStops)(::windows::core::Vtable::as_raw(self), prohibitedcaretstopcount, prohibitedcaretstops).ok()
    }
    pub unsafe fn GetBidiLevel(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBidiLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsSideways(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIsSideways)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceFontName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDeviceFontName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStyleSimulations(&self) -> ::windows::core::Result<XPS_STYLE_SIMULATION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStyleSimulations)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStyleSimulations(&self, stylesimulations: XPS_STYLE_SIMULATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetStyleSimulations)(::windows::core::Vtable::as_raw(self), stylesimulations).ok()
    }
    pub unsafe fn GetOrigin(&self) -> ::windows::core::Result<XPS_POINT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOrigin)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOrigin(&self, origin: *const XPS_POINT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOrigin)(::windows::core::Vtable::as_raw(self), origin).ok()
    }
    pub unsafe fn GetFontRenderingEmSize(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFontRenderingEmSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFontRenderingEmSize(&self, fontrenderingemsize: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFontRenderingEmSize)(::windows::core::Vtable::as_raw(self), fontrenderingemsize).ok()
    }
    pub unsafe fn GetFontResource(&self) -> ::windows::core::Result<IXpsOMFontResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFontResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFontResource<P0>(&self, fontresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMFontResource>>,
    {
        (::windows::core::Vtable::vtable(self).SetFontResource)(::windows::core::Vtable::as_raw(self), fontresource.into().abi()).ok()
    }
    pub unsafe fn GetFontFaceIndex(&self) -> ::windows::core::Result<i16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFontFaceIndex)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFontFaceIndex(&self, fontfaceindex: i16) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFontFaceIndex)(::windows::core::Vtable::as_raw(self), fontfaceindex).ok()
    }
    pub unsafe fn GetFillBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFillBrush)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFillBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFillBrushLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFillBrushLocal<P0>(&self, fillbrush: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMBrush>>,
    {
        (::windows::core::Vtable::vtable(self).SetFillBrushLocal)(::windows::core::Vtable::as_raw(self), fillbrush.into().abi()).ok()
    }
    pub unsafe fn GetFillBrushLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFillBrushLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFillBrushLookup<P0>(&self, key: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetFillBrushLookup)(::windows::core::Vtable::as_raw(self), key.into().abi()).ok()
    }
    pub unsafe fn GetGlyphsEditor(&self) -> ::windows::core::Result<IXpsOMGlyphsEditor> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGlyphsEditor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMGlyphs> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMGlyphs, ::windows::core::IUnknown, IXpsOMShareable, IXpsOMVisual);
impl ::core::clone::Clone for IXpsOMGlyphs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMGlyphs {
    type Vtable = IXpsOMGlyphs_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMGlyphs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x819b3199_0a5a_4b64_bec7_a9e17e780de2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGlyphs_Vtbl {
    pub base__: IXpsOMVisual_Vtbl,
    pub GetUnicodeString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unicodestring: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetGlyphIndexCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetGlyphIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::HRESULT,
    pub GetGlyphMappingCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetGlyphMappings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT,
    pub GetProhibitedCaretStopCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetProhibitedCaretStops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::HRESULT,
    pub GetBidiLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bidilevel: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsSideways: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsSideways: usize,
    pub GetDeviceFontName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicefontname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetStyleSimulations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stylesimulations: *mut XPS_STYLE_SIMULATION) -> ::windows::core::HRESULT,
    pub SetStyleSimulations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stylesimulations: XPS_STYLE_SIMULATION) -> ::windows::core::HRESULT,
    pub GetOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: *mut XPS_POINT) -> ::windows::core::HRESULT,
    pub SetOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: *const XPS_POINT) -> ::windows::core::HRESULT,
    pub GetFontRenderingEmSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontrenderingemsize: *mut f32) -> ::windows::core::HRESULT,
    pub SetFontRenderingEmSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontrenderingemsize: f32) -> ::windows::core::HRESULT,
    pub GetFontResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFontResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFontFaceIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfaceindex: *mut i16) -> ::windows::core::HRESULT,
    pub SetFontFaceIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfaceindex: i16) -> ::windows::core::HRESULT,
    pub GetFillBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillbrush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFillBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillbrush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFillBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillbrush: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFillBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetFillBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetGlyphsEditor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, editor: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMGlyphsEditor(::windows::core::IUnknown);
impl IXpsOMGlyphsEditor {
    pub unsafe fn ApplyEdits(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).ApplyEdits)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetUnicodeString(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUnicodeString)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetUnicodeString<P0>(&self, unicodestring: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetUnicodeString)(::windows::core::Vtable::as_raw(self), unicodestring.into().abi()).ok()
    }
    pub unsafe fn GetGlyphIndexCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGlyphIndexCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGlyphIndices(&self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetGlyphIndices)(::windows::core::Vtable::as_raw(self), indexcount, glyphindices).ok()
    }
    pub unsafe fn SetGlyphIndices(&self, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetGlyphIndices)(::windows::core::Vtable::as_raw(self), indexcount, glyphindices).ok()
    }
    pub unsafe fn GetGlyphMappingCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGlyphMappingCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGlyphMappings(&self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetGlyphMappings)(::windows::core::Vtable::as_raw(self), glyphmappingcount, glyphmappings).ok()
    }
    pub unsafe fn SetGlyphMappings(&self, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetGlyphMappings)(::windows::core::Vtable::as_raw(self), glyphmappingcount, glyphmappings).ok()
    }
    pub unsafe fn GetProhibitedCaretStopCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetProhibitedCaretStopCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetProhibitedCaretStops(&self, count: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetProhibitedCaretStops)(::windows::core::Vtable::as_raw(self), count, prohibitedcaretstops).ok()
    }
    pub unsafe fn SetProhibitedCaretStops(&self, count: u32, prohibitedcaretstops: *const u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetProhibitedCaretStops)(::windows::core::Vtable::as_raw(self), count, prohibitedcaretstops).ok()
    }
    pub unsafe fn GetBidiLevel(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBidiLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBidiLevel(&self, bidilevel: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetBidiLevel)(::windows::core::Vtable::as_raw(self), bidilevel).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsSideways(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIsSideways)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsSideways<P0>(&self, issideways: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetIsSideways)(::windows::core::Vtable::as_raw(self), issideways.into()).ok()
    }
    pub unsafe fn GetDeviceFontName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDeviceFontName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDeviceFontName<P0>(&self, devicefontname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetDeviceFontName)(::windows::core::Vtable::as_raw(self), devicefontname.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMGlyphsEditor, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMGlyphsEditor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMGlyphsEditor {
    type Vtable = IXpsOMGlyphsEditor_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMGlyphsEditor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa5ab8616_5b16_4b9f_9629_89b323ed7909);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGlyphsEditor_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ApplyEdits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetUnicodeString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unicodestring: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetUnicodeString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unicodestring: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetGlyphIndexCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetGlyphIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows::core::HRESULT,
    pub SetGlyphIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> ::windows::core::HRESULT,
    pub GetGlyphMappingCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetGlyphMappings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT,
    pub SetGlyphMappings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> ::windows::core::HRESULT,
    pub GetProhibitedCaretStopCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetProhibitedCaretStops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows::core::HRESULT,
    pub SetProhibitedCaretStops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, prohibitedcaretstops: *const u32) -> ::windows::core::HRESULT,
    pub GetBidiLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bidilevel: *mut u32) -> ::windows::core::HRESULT,
    pub SetBidiLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bidilevel: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsSideways: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsSideways: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsSideways: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, issideways: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsSideways: usize,
    pub GetDeviceFontName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicefontname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetDeviceFontName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicefontname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMGradientBrush(::windows::core::IUnknown);
impl IXpsOMGradientBrush {
    pub unsafe fn GetGradientStops(&self) -> ::windows::core::Result<IXpsOMGradientStopCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGradientStops)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTransform)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTransformLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, transform: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMMatrixTransform>>,
    {
        (::windows::core::Vtable::vtable(self).SetTransformLocal)(::windows::core::Vtable::as_raw(self), transform.into().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTransformLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetTransformLookup)(::windows::core::Vtable::as_raw(self), key.into().abi()).ok()
    }
    pub unsafe fn GetSpreadMethod(&self) -> ::windows::core::Result<XPS_SPREAD_METHOD> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSpreadMethod)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSpreadMethod(&self, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSpreadMethod)(::windows::core::Vtable::as_raw(self), spreadmethod).ok()
    }
    pub unsafe fn GetColorInterpolationMode(&self) -> ::windows::core::Result<XPS_COLOR_INTERPOLATION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetColorInterpolationMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetColorInterpolationMode(&self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetColorInterpolationMode)(::windows::core::Vtable::as_raw(self), colorinterpolationmode).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMGradientBrush, ::windows::core::IUnknown, IXpsOMShareable, IXpsOMBrush);
impl ::core::clone::Clone for IXpsOMGradientBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMGradientBrush {
    type Vtable = IXpsOMGradientBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMGradientBrush {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedb59622_61a2_42c3_bace_acf2286c06bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGradientBrush_Vtbl {
    pub base__: IXpsOMBrush_Vtbl,
    pub GetGradientStops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradientstops: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetSpreadMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spreadmethod: *mut XPS_SPREAD_METHOD) -> ::windows::core::HRESULT,
    pub SetSpreadMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::core::HRESULT,
    pub GetColorInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorinterpolationmode: *mut XPS_COLOR_INTERPOLATION) -> ::windows::core::HRESULT,
    pub SetColorInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMGradientStop(::windows::core::IUnknown);
impl IXpsOMGradientStop {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMGradientBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOffset(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOffset)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOffset(&self, offset: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOffset)(::windows::core::Vtable::as_raw(self), offset).ok()
    }
    pub unsafe fn GetColor(&self, color: *mut XPS_COLOR, colorprofile: *mut ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetColor)(::windows::core::Vtable::as_raw(self), color, ::core::mem::transmute(colorprofile)).ok()
    }
    pub unsafe fn SetColor<P0>(&self, color: *const XPS_COLOR, colorprofile: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMColorProfileResource>>,
    {
        (::windows::core::Vtable::vtable(self).SetColor)(::windows::core::Vtable::as_raw(self), color, colorprofile.into().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMGradientStop> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMGradientStop, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMGradientStop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMGradientStop {
    type Vtable = IXpsOMGradientStop_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMGradientStop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5cf4f5cc_3969_49b5_a70a_5550b618fe49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGradientStop_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: *mut f32) -> ::windows::core::HRESULT,
    pub SetOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: f32) -> ::windows::core::HRESULT,
    pub GetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradientstop: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMGradientStopCollection(::windows::core::IUnknown);
impl IXpsOMGradientStopCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMGradientStop> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, stop: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMGradientStop>>,
    {
        (::windows::core::Vtable::vtable(self).InsertAt)(::windows::core::Vtable::as_raw(self), index, stop.into().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveAt)(::windows::core::Vtable::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, stop: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMGradientStop>>,
    {
        (::windows::core::Vtable::vtable(self).SetAt)(::windows::core::Vtable::as_raw(self), index, stop.into().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, stop: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMGradientStop>>,
    {
        (::windows::core::Vtable::vtable(self).Append)(::windows::core::Vtable::as_raw(self), stop.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMGradientStopCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMGradientStopCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMGradientStopCollection {
    type Vtable = IXpsOMGradientStopCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMGradientStopCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9174c3a_3cd3_4319_bda4_11a39392ceef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGradientStopCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, stop: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, stop: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, stop: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stop: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMImageBrush(::windows::core::IUnknown);
impl IXpsOMImageBrush {
    pub unsafe fn GetImageResource(&self) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetImageResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetImageResource<P0>(&self, imageresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMImageResource>>,
    {
        (::windows::core::Vtable::vtable(self).SetImageResource)(::windows::core::Vtable::as_raw(self), imageresource.into().abi()).ok()
    }
    pub unsafe fn GetColorProfileResource(&self) -> ::windows::core::Result<IXpsOMColorProfileResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetColorProfileResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetColorProfileResource<P0>(&self, colorprofileresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMColorProfileResource>>,
    {
        (::windows::core::Vtable::vtable(self).SetColorProfileResource)(::windows::core::Vtable::as_raw(self), colorprofileresource.into().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMImageBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMImageBrush, ::windows::core::IUnknown, IXpsOMShareable, IXpsOMBrush, IXpsOMTileBrush);
impl ::core::clone::Clone for IXpsOMImageBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMImageBrush {
    type Vtable = IXpsOMImageBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMImageBrush {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3df0b466_d382_49ef_8550_dd94c80242e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMImageBrush_Vtbl {
    pub base__: IXpsOMTileBrush_Vtbl,
    pub GetImageResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetImageResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetColorProfileResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorprofileresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetColorProfileResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorprofileresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagebrush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMImageResource(::windows::core::IUnknown);
impl IXpsOMImageResource {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, imagetype: XPS_IMAGE_TYPE, partname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).SetContent)(::windows::core::Vtable::as_raw(self), sourcestream.into().abi(), imagetype, partname.into().abi()).ok()
    }
    pub unsafe fn GetImageType(&self) -> ::windows::core::Result<XPS_IMAGE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetImageType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMImageResource, ::windows::core::IUnknown, IXpsOMPart, IXpsOMResource);
impl ::core::clone::Clone for IXpsOMImageResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMImageResource {
    type Vtable = IXpsOMImageResource_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMImageResource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3db8417d_ae50_485e_9a44_d7758f78a23f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMImageResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readerstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, imagetype: XPS_IMAGE_TYPE, partname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
    pub GetImageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagetype: *mut XPS_IMAGE_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMImageResourceCollection(::windows::core::IUnknown);
impl IXpsOMImageResourceCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, object: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMImageResource>>,
    {
        (::windows::core::Vtable::vtable(self).InsertAt)(::windows::core::Vtable::as_raw(self), index, object.into().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveAt)(::windows::core::Vtable::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, object: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMImageResource>>,
    {
        (::windows::core::Vtable::vtable(self).SetAt)(::windows::core::Vtable::as_raw(self), index, object.into().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, object: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMImageResource>>,
    {
        (::windows::core::Vtable::vtable(self).Append)(::windows::core::Vtable::as_raw(self), object.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<P0>(&self, partname: P0) -> ::windows::core::Result<IXpsOMImageResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetByPartName)(::windows::core::Vtable::as_raw(self), partname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMImageResourceCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMImageResourceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMImageResourceCollection {
    type Vtable = IXpsOMImageResourceCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMImageResourceCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a4a1a71_9cde_4b71_b33f_62de843eabfe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMImageResourceCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMLinearGradientBrush(::windows::core::IUnknown);
impl IXpsOMLinearGradientBrush {
    pub unsafe fn GetStartPoint(&self) -> ::windows::core::Result<XPS_POINT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStartPoint)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStartPoint(&self, startpoint: *const XPS_POINT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetStartPoint)(::windows::core::Vtable::as_raw(self), startpoint).ok()
    }
    pub unsafe fn GetEndPoint(&self) -> ::windows::core::Result<XPS_POINT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetEndPoint)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEndPoint(&self, endpoint: *const XPS_POINT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetEndPoint)(::windows::core::Vtable::as_raw(self), endpoint).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMLinearGradientBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMLinearGradientBrush, ::windows::core::IUnknown, IXpsOMShareable, IXpsOMBrush, IXpsOMGradientBrush);
impl ::core::clone::Clone for IXpsOMLinearGradientBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMLinearGradientBrush {
    type Vtable = IXpsOMLinearGradientBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMLinearGradientBrush {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x005e279f_c30d_40ff_93ec_1950d3c528db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMLinearGradientBrush_Vtbl {
    pub base__: IXpsOMGradientBrush_Vtbl,
    pub GetStartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *mut XPS_POINT) -> ::windows::core::HRESULT,
    pub SetStartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT) -> ::windows::core::HRESULT,
    pub GetEndPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: *mut XPS_POINT) -> ::windows::core::HRESULT,
    pub SetEndPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: *const XPS_POINT) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lineargradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMMatrixTransform(::windows::core::IUnknown);
impl IXpsOMMatrixTransform {
    pub unsafe fn GetMatrix(&self) -> ::windows::core::Result<XPS_MATRIX> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMatrix)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMatrix(&self, matrix: *const XPS_MATRIX) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetMatrix)(::windows::core::Vtable::as_raw(self), matrix).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMMatrixTransform, ::windows::core::IUnknown, IXpsOMShareable);
impl ::core::clone::Clone for IXpsOMMatrixTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMMatrixTransform {
    type Vtable = IXpsOMMatrixTransform_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMMatrixTransform {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb77330ff_bb37_4501_a93e_f1b1e50bfc46);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMMatrixTransform_Vtbl {
    pub base__: IXpsOMShareable_Vtbl,
    pub GetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *mut XPS_MATRIX) -> ::windows::core::HRESULT,
    pub SetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMNameCollection(::windows::core::IUnknown);
impl IXpsOMNameCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMNameCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMNameCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMNameCollection {
    type Vtable = IXpsOMNameCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMNameCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bddf8ec_c915_421b_a166_d173d25653d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMNameCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMObjectFactory(::windows::core::IUnknown);
impl IXpsOMObjectFactory {
    pub unsafe fn CreatePackage(&self) -> ::windows::core::Result<IXpsOMPackage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePackage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePackageFromFile<P0, P1>(&self, filename: P0, reuseobjects: P1) -> ::windows::core::Result<IXpsOMPackage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePackageFromFile)(::windows::core::Vtable::as_raw(self), filename.into().abi(), reuseobjects.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageFromStream<P0, P1>(&self, stream: P0, reuseobjects: P1) -> ::windows::core::Result<IXpsOMPackage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePackageFromStream)(::windows::core::Vtable::as_raw(self), stream.into().abi(), reuseobjects.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateStoryFragmentsResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> ::windows::core::Result<IXpsOMStoryFragmentsResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateStoryFragmentsResource)(::windows::core::Vtable::as_raw(self), acquiredstream.into().abi(), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocumentStructureResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> ::windows::core::Result<IXpsOMDocumentStructureResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDocumentStructureResource)(::windows::core::Vtable::as_raw(self), acquiredstream.into().abi(), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateSignatureBlockResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> ::windows::core::Result<IXpsOMSignatureBlockResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSignatureBlockResource)(::windows::core::Vtable::as_raw(self), acquiredstream.into().abi(), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResource<P0, P1>(&self, dictionary: P0, parturi: P1) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMDictionary>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateRemoteDictionaryResource)(::windows::core::Vtable::as_raw(self), dictionary.into().abi(), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResourceFromStream<P0, P1, P2>(&self, dictionarymarkupstream: P0, dictionaryparturi: P1, resources: P2) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P2: ::std::convert::Into<::windows::core::InParam<IXpsOMPartResources>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateRemoteDictionaryResourceFromStream)(::windows::core::Vtable::as_raw(self), dictionarymarkupstream.into().abi(), dictionaryparturi.into().abi(), resources.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePartResources(&self) -> ::windows::core::Result<IXpsOMPartResources> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePartResources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocumentSequence<P0>(&self, parturi: P0) -> ::windows::core::Result<IXpsOMDocumentSequence>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDocumentSequence)(::windows::core::Vtable::as_raw(self), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocument<P0>(&self, parturi: P0) -> ::windows::core::Result<IXpsOMDocument>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDocument)(::windows::core::Vtable::as_raw(self), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePageReference(&self, advisorypagedimensions: *const XPS_SIZE) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePageReference)(::windows::core::Vtable::as_raw(self), advisorypagedimensions, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePage<P0, P1>(&self, pagedimensions: *const XPS_SIZE, language: P0, parturi: P1) -> ::windows::core::Result<IXpsOMPage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePage)(::windows::core::Vtable::as_raw(self), pagedimensions, language.into().abi(), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePageFromStream<P0, P1, P2, P3>(&self, pagemarkupstream: P0, parturi: P1, resources: P2, reuseobjects: P3) -> ::windows::core::Result<IXpsOMPage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P2: ::std::convert::Into<::windows::core::InParam<IXpsOMPartResources>>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePageFromStream)(::windows::core::Vtable::as_raw(self), pagemarkupstream.into().abi(), parturi.into().abi(), resources.into().abi(), reuseobjects.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCanvas(&self) -> ::windows::core::Result<IXpsOMCanvas> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateCanvas)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateGlyphs<P0>(&self, fontresource: P0) -> ::windows::core::Result<IXpsOMGlyphs>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMFontResource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateGlyphs)(::windows::core::Vtable::as_raw(self), fontresource.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePath(&self) -> ::windows::core::Result<IXpsOMPath> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateGeometry)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateGeometryFigure(&self, startpoint: *const XPS_POINT) -> ::windows::core::Result<IXpsOMGeometryFigure> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateGeometryFigure)(::windows::core::Vtable::as_raw(self), startpoint, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateMatrixTransform(&self, matrix: *const XPS_MATRIX) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateMatrixTransform)(::windows::core::Vtable::as_raw(self), matrix, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateSolidColorBrush<P0>(&self, color: *const XPS_COLOR, colorprofile: P0) -> ::windows::core::Result<IXpsOMSolidColorBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMColorProfileResource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSolidColorBrush)(::windows::core::Vtable::as_raw(self), color, colorprofile.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateColorProfileResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> ::windows::core::Result<IXpsOMColorProfileResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateColorProfileResource)(::windows::core::Vtable::as_raw(self), acquiredstream.into().abi(), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateImageBrush<P0>(&self, image: P0, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows::core::Result<IXpsOMImageBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMImageResource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateImageBrush)(::windows::core::Vtable::as_raw(self), image.into().abi(), viewbox, viewport, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateVisualBrush(&self, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows::core::Result<IXpsOMVisualBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateVisualBrush)(::windows::core::Vtable::as_raw(self), viewbox, viewport, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateImageResource<P0, P1>(&self, acquiredstream: P0, contenttype: XPS_IMAGE_TYPE, parturi: P1) -> ::windows::core::Result<IXpsOMImageResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateImageResource)(::windows::core::Vtable::as_raw(self), acquiredstream.into().abi(), contenttype, parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePrintTicketResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> ::windows::core::Result<IXpsOMPrintTicketResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePrintTicketResource)(::windows::core::Vtable::as_raw(self), acquiredstream.into().abi(), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateFontResource<P0, P1, P2>(&self, acquiredstream: P0, fontembedding: XPS_FONT_EMBEDDING, parturi: P1, isobfsourcestream: P2) -> ::windows::core::Result<IXpsOMFontResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateFontResource)(::windows::core::Vtable::as_raw(self), acquiredstream.into().abi(), fontembedding, parturi.into().abi(), isobfsourcestream.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateGradientStop<P0>(&self, color: *const XPS_COLOR, colorprofile: P0, offset: f32) -> ::windows::core::Result<IXpsOMGradientStop>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMColorProfileResource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateGradientStop)(::windows::core::Vtable::as_raw(self), color, colorprofile.into().abi(), offset, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateLinearGradientBrush<P0, P1>(&self, gradstop1: P0, gradstop2: P1, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT) -> ::windows::core::Result<IXpsOMLinearGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMGradientStop>>,
        P1: ::std::convert::Into<::windows::core::InParam<IXpsOMGradientStop>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateLinearGradientBrush)(::windows::core::Vtable::as_raw(self), gradstop1.into().abi(), gradstop2.into().abi(), startpoint, endpoint, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateRadialGradientBrush<P0, P1>(&self, gradstop1: P0, gradstop2: P1, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE) -> ::windows::core::Result<IXpsOMRadialGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMGradientStop>>,
        P1: ::std::convert::Into<::windows::core::InParam<IXpsOMGradientStop>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateRadialGradientBrush)(::windows::core::Vtable::as_raw(self), gradstop1.into().abi(), gradstop2.into().abi(), centerpoint, gradientorigin, radiisizes, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateCoreProperties<P0>(&self, parturi: P0) -> ::windows::core::Result<IXpsOMCoreProperties>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateCoreProperties)(::windows::core::Vtable::as_raw(self), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateDictionary)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePartUriCollection(&self) -> ::windows::core::Result<IXpsOMPartUriCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePartUriCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnFile<P0, P1, P2, P3, P4, P5, P6>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: P1, interleaving: XPS_INTERLEAVING, documentsequencepartname: P2, coreproperties: P3, packagethumbnail: P4, documentsequenceprintticket: P5, discardcontrolpartname: P6) -> ::windows::core::Result<IXpsOMPackageWriter>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P3: ::std::convert::Into<::windows::core::InParam<IXpsOMCoreProperties>>,
        P4: ::std::convert::Into<::windows::core::InParam<IXpsOMImageResource>>,
        P5: ::std::convert::Into<::windows::core::InParam<IXpsOMPrintTicketResource>>,
        P6: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePackageWriterOnFile)(::windows::core::Vtable::as_raw(self), filename.into().abi(), securityattributes, flagsandattributes, optimizemarkupsize.into(), interleaving, documentsequencepartname.into().abi(), coreproperties.into().abi(), packagethumbnail.into().abi(), documentsequenceprintticket.into().abi(), discardcontrolpartname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnStream<P0, P1, P2, P3, P4, P5, P6>(&self, outputstream: P0, optimizemarkupsize: P1, interleaving: XPS_INTERLEAVING, documentsequencepartname: P2, coreproperties: P3, packagethumbnail: P4, documentsequenceprintticket: P5, discardcontrolpartname: P6) -> ::windows::core::Result<IXpsOMPackageWriter>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::ISequentialStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P3: ::std::convert::Into<::windows::core::InParam<IXpsOMCoreProperties>>,
        P4: ::std::convert::Into<::windows::core::InParam<IXpsOMImageResource>>,
        P5: ::std::convert::Into<::windows::core::InParam<IXpsOMPrintTicketResource>>,
        P6: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePackageWriterOnStream)(::windows::core::Vtable::as_raw(self), outputstream.into().abi(), optimizemarkupsize.into(), interleaving, documentsequencepartname.into().abi(), coreproperties.into().abi(), packagethumbnail.into().abi(), documentsequenceprintticket.into().abi(), discardcontrolpartname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePartUri<P0>(&self, uri: P0) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePartUri)(::windows::core::Vtable::as_raw(self), uri.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateReadOnlyStreamOnFile<P0>(&self, filename: P0) -> ::windows::core::Result<super::super::System::Com::IStream>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateReadOnlyStreamOnFile)(::windows::core::Vtable::as_raw(self), filename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMObjectFactory, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMObjectFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMObjectFactory {
    type Vtable = IXpsOMObjectFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMObjectFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9b2a685_a50d_4fc2_b764_b56e093ea0ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMObjectFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreatePackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreatePackageFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreatePackageFromFile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreatePackageFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreatePackageFromStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateStoryFragmentsResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, storyfragmentsresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateStoryFragmentsResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateDocumentStructureResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, documentstructureresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateDocumentStructureResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateSignatureBlockResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, signatureblockresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateSignatureBlockResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateRemoteDictionaryResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, remotedictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateRemoteDictionaryResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateRemoteDictionaryResourceFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionarymarkupstream: *mut ::core::ffi::c_void, dictionaryparturi: *mut ::core::ffi::c_void, resources: *mut ::core::ffi::c_void, dictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateRemoteDictionaryResourceFromStream: usize,
    pub CreatePartResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partresources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateDocumentSequence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, documentsequence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateDocumentSequence: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateDocument: usize,
    pub CreatePageReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, pagereference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: ::windows::core::PCWSTR, parturi: *mut ::core::ffi::c_void, page: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePageFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagemarkupstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, resources: *mut ::core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, page: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePageFromStream: usize,
    pub CreateCanvas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, canvas: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateGlyphs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontresource: *mut ::core::ffi::c_void, glyphs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreatePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateGeometryFigure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT, figure: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateMatrixTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX, transform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSolidColorBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut ::core::ffi::c_void, solidcolorbrush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateColorProfileResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, colorprofileresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateColorProfileResource: usize,
    pub CreateImageBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, imagebrush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateVisualBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, visualbrush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateImageResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, contenttype: XPS_IMAGE_TYPE, parturi: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateImageResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, printticketresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePrintTicketResource: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateFontResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, fontembedding: XPS_FONT_EMBEDDING, parturi: *mut ::core::ffi::c_void, isobfsourcestream: super::super::Foundation::BOOL, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateFontResource: usize,
    pub CreateGradientStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut ::core::ffi::c_void, offset: f32, gradientstop: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateLinearGradientBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradstop1: *mut ::core::ffi::c_void, gradstop2: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT, lineargradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateRadialGradientBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradstop1: *mut ::core::ffi::c_void, gradstop2: *mut ::core::ffi::c_void, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE, radialgradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateCoreProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, coreproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateCoreProperties: usize,
    pub CreateDictionary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreatePartUriCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturicollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void, packagethumbnail: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnFile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void, packagethumbnail: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePartUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::PCWSTR, parturi: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePartUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateReadOnlyStreamOnFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateReadOnlyStreamOnFile: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMObjectFactory1(::windows::core::IUnknown);
impl IXpsOMObjectFactory1 {
    pub unsafe fn GetDocumentTypeFromFile<P0>(&self, filename: P0) -> ::windows::core::Result<XPS_DOCUMENT_TYPE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDocumentTypeFromFile)(::windows::core::Vtable::as_raw(self), filename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDocumentTypeFromStream<P0>(&self, xpsdocumentstream: P0) -> ::windows::core::Result<XPS_DOCUMENT_TYPE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDocumentTypeFromStream)(::windows::core::Vtable::as_raw(self), xpsdocumentstream.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn ConvertHDPhotoToJpegXR<P0>(&self, imageresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMImageResource>>,
    {
        (::windows::core::Vtable::vtable(self).ConvertHDPhotoToJpegXR)(::windows::core::Vtable::as_raw(self), imageresource.into().abi()).ok()
    }
    pub unsafe fn ConvertJpegXRToHDPhoto<P0>(&self, imageresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMImageResource>>,
    {
        (::windows::core::Vtable::vtable(self).ConvertJpegXRToHDPhoto)(::windows::core::Vtable::as_raw(self), imageresource.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnFile1<P0, P1, P2, P3, P4, P5, P6>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: P1, interleaving: XPS_INTERLEAVING, documentsequencepartname: P2, coreproperties: P3, packagethumbnail: P4, documentsequenceprintticket: P5, discardcontrolpartname: P6, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<IXpsOMPackageWriter>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P3: ::std::convert::Into<::windows::core::InParam<IXpsOMCoreProperties>>,
        P4: ::std::convert::Into<::windows::core::InParam<IXpsOMImageResource>>,
        P5: ::std::convert::Into<::windows::core::InParam<IXpsOMPrintTicketResource>>,
        P6: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePackageWriterOnFile1)(::windows::core::Vtable::as_raw(self), filename.into().abi(), securityattributes, flagsandattributes, optimizemarkupsize.into(), interleaving, documentsequencepartname.into().abi(), coreproperties.into().abi(), packagethumbnail.into().abi(), documentsequenceprintticket.into().abi(), discardcontrolpartname.into().abi(), documenttype, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnStream1<P0, P1, P2, P3, P4, P5, P6>(&self, outputstream: P0, optimizemarkupsize: P1, interleaving: XPS_INTERLEAVING, documentsequencepartname: P2, coreproperties: P3, packagethumbnail: P4, documentsequenceprintticket: P5, discardcontrolpartname: P6, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<IXpsOMPackageWriter>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::ISequentialStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P3: ::std::convert::Into<::windows::core::InParam<IXpsOMCoreProperties>>,
        P4: ::std::convert::Into<::windows::core::InParam<IXpsOMImageResource>>,
        P5: ::std::convert::Into<::windows::core::InParam<IXpsOMPrintTicketResource>>,
        P6: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePackageWriterOnStream1)(::windows::core::Vtable::as_raw(self), outputstream.into().abi(), optimizemarkupsize.into(), interleaving, documentsequencepartname.into().abi(), coreproperties.into().abi(), packagethumbnail.into().abi(), documentsequenceprintticket.into().abi(), discardcontrolpartname.into().abi(), documenttype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePackage1(&self) -> ::windows::core::Result<IXpsOMPackage1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePackage1)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageFromStream1<P0, P1>(&self, stream: P0, reuseobjects: P1) -> ::windows::core::Result<IXpsOMPackage1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePackageFromStream1)(::windows::core::Vtable::as_raw(self), stream.into().abi(), reuseobjects.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePackageFromFile1<P0, P1>(&self, filename: P0, reuseobjects: P1) -> ::windows::core::Result<IXpsOMPackage1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePackageFromFile1)(::windows::core::Vtable::as_raw(self), filename.into().abi(), reuseobjects.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePage1<P0, P1>(&self, pagedimensions: *const XPS_SIZE, language: P0, parturi: P1) -> ::windows::core::Result<IXpsOMPage1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePage1)(::windows::core::Vtable::as_raw(self), pagedimensions, language.into().abi(), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePageFromStream1<P0, P1, P2, P3>(&self, pagemarkupstream: P0, parturi: P1, resources: P2, reuseobjects: P3) -> ::windows::core::Result<IXpsOMPage1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P2: ::std::convert::Into<::windows::core::InParam<IXpsOMPartResources>>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePageFromStream1)(::windows::core::Vtable::as_raw(self), pagemarkupstream.into().abi(), parturi.into().abi(), resources.into().abi(), reuseobjects.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResourceFromStream1<P0, P1, P2>(&self, dictionarymarkupstream: P0, parturi: P1, resources: P2) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P2: ::std::convert::Into<::windows::core::InParam<IXpsOMPartResources>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateRemoteDictionaryResourceFromStream1)(::windows::core::Vtable::as_raw(self), dictionarymarkupstream.into().abi(), parturi.into().abi(), resources.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMObjectFactory1, ::windows::core::IUnknown, IXpsOMObjectFactory);
impl ::core::clone::Clone for IXpsOMObjectFactory1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMObjectFactory1 {
    type Vtable = IXpsOMObjectFactory1_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMObjectFactory1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0a91b617_d612_4181_bf7c_be5824e9cc8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMObjectFactory1_Vtbl {
    pub base__: IXpsOMObjectFactory_Vtbl,
    pub GetDocumentTypeFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDocumentTypeFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpsdocumentstream: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDocumentTypeFromStream: usize,
    pub ConvertHDPhotoToJpegXR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ConvertJpegXRToHDPhoto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnFile1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void, packagethumbnail: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnFile1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnStream1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void, packagethumbnail: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnStream1: usize,
    pub CreatePackage1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreatePackageFromStream1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreatePackageFromStream1: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreatePackageFromFile1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreatePackageFromFile1: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePage1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: ::windows::core::PCWSTR, parturi: *mut ::core::ffi::c_void, page: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePage1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePageFromStream1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagemarkupstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, resources: *mut ::core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, page: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePageFromStream1: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateRemoteDictionaryResourceFromStream1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionarymarkupstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, resources: *mut ::core::ffi::c_void, dictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateRemoteDictionaryResourceFromStream1: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPackage(::windows::core::IUnknown);
impl IXpsOMPackage {
    pub unsafe fn GetDocumentSequence(&self) -> ::windows::core::Result<IXpsOMDocumentSequence> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDocumentSequence)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDocumentSequence<P0>(&self, documentsequence: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMDocumentSequence>>,
    {
        (::windows::core::Vtable::vtable(self).SetDocumentSequence)(::windows::core::Vtable::as_raw(self), documentsequence.into().abi()).ok()
    }
    pub unsafe fn GetCoreProperties(&self) -> ::windows::core::Result<IXpsOMCoreProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCoreProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCoreProperties<P0>(&self, coreproperties: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMCoreProperties>>,
    {
        (::windows::core::Vtable::vtable(self).SetCoreProperties)(::windows::core::Vtable::as_raw(self), coreproperties.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetDiscardControlPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDiscardControlPartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetDiscardControlPartName<P0>(&self, discardcontrolparturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).SetDiscardControlPartName)(::windows::core::Vtable::as_raw(self), discardcontrolparturi.into().abi()).ok()
    }
    pub unsafe fn GetThumbnailResource(&self) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetThumbnailResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetThumbnailResource<P0>(&self, imageresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMImageResource>>,
    {
        (::windows::core::Vtable::vtable(self).SetThumbnailResource)(::windows::core::Vtable::as_raw(self), imageresource.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn WriteToFile<P0, P1>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).WriteToFile)(::windows::core::Vtable::as_raw(self), filename.into().abi(), securityattributes, flagsandattributes, optimizemarkupsize.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn WriteToStream<P0, P1>(&self, stream: P0, optimizemarkupsize: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::ISequentialStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).WriteToStream)(::windows::core::Vtable::as_raw(self), stream.into().abi(), optimizemarkupsize.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMPackage, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMPackage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMPackage {
    type Vtable = IXpsOMPackage_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMPackage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18c3df65_81e1_4674_91dc_fc452f5a416f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackage_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDocumentSequence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDocumentSequence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequence: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCoreProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coreproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCoreProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetDiscardControlPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discardcontrolparturi: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetDiscardControlPartName: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetDiscardControlPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discardcontrolparturi: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetDiscardControlPartName: usize,
    pub GetThumbnailResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetThumbnailResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub WriteToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))]
    WriteToFile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub WriteToStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    WriteToStream: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPackage1(::windows::core::IUnknown);
impl IXpsOMPackage1 {
    pub unsafe fn GetDocumentType(&self) -> ::windows::core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDocumentType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn WriteToFile1<P0, P1>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: P1, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).WriteToFile1)(::windows::core::Vtable::as_raw(self), filename.into().abi(), securityattributes, flagsandattributes, optimizemarkupsize.into(), documenttype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn WriteToStream1<P0, P1>(&self, outputstream: P0, optimizemarkupsize: P1, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::ISequentialStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).WriteToStream1)(::windows::core::Vtable::as_raw(self), outputstream.into().abi(), optimizemarkupsize.into(), documenttype).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMPackage1, ::windows::core::IUnknown, IXpsOMPackage);
impl ::core::clone::Clone for IXpsOMPackage1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMPackage1 {
    type Vtable = IXpsOMPackage1_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMPackage1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95a9435e_12bb_461b_8e7f_c6adb04cd96a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackage1_Vtbl {
    pub base__: IXpsOMPackage_Vtbl,
    pub GetDocumentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub WriteToFile1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))]
    WriteToFile1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub WriteToStream1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    WriteToStream1: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPackageTarget(::windows::core::IUnknown);
impl IXpsOMPackageTarget {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateXpsOMPackageWriter<P0, P1, P2>(&self, documentsequencepartname: P0, documentsequenceprintticket: P1, discardcontrolpartname: P2) -> ::windows::core::Result<IXpsOMPackageWriter>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P1: ::std::convert::Into<::windows::core::InParam<IXpsOMPrintTicketResource>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateXpsOMPackageWriter)(::windows::core::Vtable::as_raw(self), documentsequencepartname.into().abi(), documentsequenceprintticket.into().abi(), discardcontrolpartname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMPackageTarget, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMPackageTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMPackageTarget {
    type Vtable = IXpsOMPackageTarget_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMPackageTarget {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x219a9db0_4959_47d0_8034_b1ce84f41a4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackageTarget_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateXpsOMPackageWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequencepartname: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateXpsOMPackageWriter: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPackageWriter(::windows::core::IUnknown);
impl IXpsOMPackageWriter {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn StartNewDocument<P0, P1, P2, P3, P4>(&self, documentpartname: P0, documentprintticket: P1, documentstructure: P2, signatureblockresources: P3, restrictedfonts: P4) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P1: ::std::convert::Into<::windows::core::InParam<IXpsOMPrintTicketResource>>,
        P2: ::std::convert::Into<::windows::core::InParam<IXpsOMDocumentStructureResource>>,
        P3: ::std::convert::Into<::windows::core::InParam<IXpsOMSignatureBlockResourceCollection>>,
        P4: ::std::convert::Into<::windows::core::InParam<IXpsOMPartUriCollection>>,
    {
        (::windows::core::Vtable::vtable(self).StartNewDocument)(::windows::core::Vtable::as_raw(self), documentpartname.into().abi(), documentprintticket.into().abi(), documentstructure.into().abi(), signatureblockresources.into().abi(), restrictedfonts.into().abi()).ok()
    }
    pub unsafe fn AddPage<P0, P1, P2, P3, P4>(&self, page: P0, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: P1, storyfragments: P2, pageprintticket: P3, pagethumbnail: P4) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMPage>>,
        P1: ::std::convert::Into<::windows::core::InParam<IXpsOMPartUriCollection>>,
        P2: ::std::convert::Into<::windows::core::InParam<IXpsOMStoryFragmentsResource>>,
        P3: ::std::convert::Into<::windows::core::InParam<IXpsOMPrintTicketResource>>,
        P4: ::std::convert::Into<::windows::core::InParam<IXpsOMImageResource>>,
    {
        (::windows::core::Vtable::vtable(self).AddPage)(::windows::core::Vtable::as_raw(self), page.into().abi(), advisorypagedimensions, discardableresourceparts.into().abi(), storyfragments.into().abi(), pageprintticket.into().abi(), pagethumbnail.into().abi()).ok()
    }
    pub unsafe fn AddResource<P0>(&self, resource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMResource>>,
    {
        (::windows::core::Vtable::vtable(self).AddResource)(::windows::core::Vtable::as_raw(self), resource.into().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsClosed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsClosed)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMPackageWriter, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMPackageWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMPackageWriter {
    type Vtable = IXpsOMPackageWriter_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMPackageWriter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e2aa182_a443_42c6_b41b_4f8e9de73ff9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackageWriter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub StartNewDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentpartname: *mut ::core::ffi::c_void, documentprintticket: *mut ::core::ffi::c_void, documentstructure: *mut ::core::ffi::c_void, signatureblockresources: *mut ::core::ffi::c_void, restrictedfonts: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    StartNewDocument: usize,
    pub AddPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: *mut ::core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: *mut ::core::ffi::c_void, storyfragments: *mut ::core::ffi::c_void, pageprintticket: *mut ::core::ffi::c_void, pagethumbnail: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsClosed: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPackageWriter3D(::windows::core::IUnknown);
impl IXpsOMPackageWriter3D {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn AddModelTexture<P0, P1>(&self, texturepartname: P0, texturedata: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).AddModelTexture)(::windows::core::Vtable::as_raw(self), texturepartname.into().abi(), texturedata.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetModelPrintTicket<P0, P1>(&self, printticketpartname: P0, printticketdata: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).SetModelPrintTicket)(::windows::core::Vtable::as_raw(self), printticketpartname.into().abi(), printticketdata.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMPackageWriter3D, ::windows::core::IUnknown, IXpsOMPackageWriter);
impl ::core::clone::Clone for IXpsOMPackageWriter3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMPackageWriter3D {
    type Vtable = IXpsOMPackageWriter3D_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMPackageWriter3D {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8a45033_640e_43fa_9bdf_fddeaa31c6a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackageWriter3D_Vtbl {
    pub base__: IXpsOMPackageWriter_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub AddModelTexture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, texturepartname: *mut ::core::ffi::c_void, texturedata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    AddModelTexture: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetModelPrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketpartname: *mut ::core::ffi::c_void, printticketdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetModelPrintTicket: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPage(::windows::core::IUnknown);
impl IXpsOMPage {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVisuals(&self) -> ::windows::core::Result<IXpsOMVisualCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetVisuals)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPageDimensions(&self) -> ::windows::core::Result<XPS_SIZE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPageDimensions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPageDimensions)(::windows::core::Vtable::as_raw(self), pagedimensions).ok()
    }
    pub unsafe fn GetContentBox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetContentBox)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetContentBox(&self, contentbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetContentBox)(::windows::core::Vtable::as_raw(self), contentbox).ok()
    }
    pub unsafe fn GetBleedBox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBleedBox)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBleedBox(&self, bleedbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetBleedBox)(::windows::core::Vtable::as_raw(self), bleedbox).ok()
    }
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLanguage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLanguage<P0>(&self, language: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetLanguage)(::windows::core::Vtable::as_raw(self), language.into().abi()).ok()
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIsHyperlinkTarget)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<P0>(&self, ishyperlinktarget: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetIsHyperlinkTarget)(::windows::core::Vtable::as_raw(self), ishyperlinktarget.into()).ok()
    }
    pub unsafe fn GetDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDictionary)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDictionaryLocal(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDictionaryLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDictionaryLocal<P0>(&self, resourcedictionary: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMDictionary>>,
    {
        (::windows::core::Vtable::vtable(self).SetDictionaryLocal)(::windows::core::Vtable::as_raw(self), resourcedictionary.into().abi()).ok()
    }
    pub unsafe fn GetDictionaryResource(&self) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDictionaryResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDictionaryResource<P0>(&self, remotedictionaryresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMRemoteDictionaryResource>>,
    {
        (::windows::core::Vtable::vtable(self).SetDictionaryResource)(::windows::core::Vtable::as_raw(self), remotedictionaryresource.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Write<P0, P1>(&self, stream: P0, optimizemarkupsize: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::ISequentialStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Write)(::windows::core::Vtable::as_raw(self), stream.into().abi(), optimizemarkupsize.into()).ok()
    }
    pub unsafe fn GenerateUnusedLookupKey(&self, r#type: XPS_OBJECT_TYPE) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GenerateUnusedLookupKey)(::windows::core::Vtable::as_raw(self), r#type, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMPage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMPage, ::windows::core::IUnknown, IXpsOMPart);
impl ::core::clone::Clone for IXpsOMPage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMPage {
    type Vtable = IXpsOMPage_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMPage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3e18888_f120_4fee_8c68_35296eae91d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPage_Vtbl {
    pub base__: IXpsOMPart_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagereference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetVisuals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visuals: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPageDimensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows::core::HRESULT,
    pub SetPageDimensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows::core::HRESULT,
    pub GetContentBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentbox: *mut XPS_RECT) -> ::windows::core::HRESULT,
    pub SetContentBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentbox: *const XPS_RECT) -> ::windows::core::HRESULT,
    pub GetBleedBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bleedbox: *mut XPS_RECT) -> ::windows::core::HRESULT,
    pub SetBleedBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bleedbox: *const XPS_RECT) -> ::windows::core::HRESULT,
    pub GetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsHyperlinkTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlinktarget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsHyperlinkTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsHyperlinkTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlinktarget: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsHyperlinkTarget: usize,
    pub GetDictionary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDictionaryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDictionaryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDictionaryResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDictionaryResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Write: usize,
    pub GenerateUnusedLookupKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: XPS_OBJECT_TYPE, key: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPage1(::windows::core::IUnknown);
impl IXpsOMPage1 {
    pub unsafe fn GetDocumentType(&self) -> ::windows::core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDocumentType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Write1<P0, P1>(&self, stream: P0, optimizemarkupsize: P1, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::ISequentialStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).Write1)(::windows::core::Vtable::as_raw(self), stream.into().abi(), optimizemarkupsize.into(), documenttype).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMPage1, ::windows::core::IUnknown, IXpsOMPart, IXpsOMPage);
impl ::core::clone::Clone for IXpsOMPage1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMPage1 {
    type Vtable = IXpsOMPage1_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMPage1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x305b60ef_6892_4dda_9cbb_3aa65974508a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPage1_Vtbl {
    pub base__: IXpsOMPage_Vtbl,
    pub GetDocumentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Write1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Write1: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPageReference(::windows::core::IUnknown);
impl IXpsOMPageReference {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMDocument> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPage(&self) -> ::windows::core::Result<IXpsOMPage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPage<P0>(&self, page: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMPage>>,
    {
        (::windows::core::Vtable::vtable(self).SetPage)(::windows::core::Vtable::as_raw(self), page.into().abi()).ok()
    }
    pub unsafe fn DiscardPage(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).DiscardPage)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPageLoaded(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).IsPageLoaded)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAdvisoryPageDimensions(&self) -> ::windows::core::Result<XPS_SIZE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAdvisoryPageDimensions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAdvisoryPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetAdvisoryPageDimensions)(::windows::core::Vtable::as_raw(self), pagedimensions).ok()
    }
    pub unsafe fn GetStoryFragmentsResource(&self) -> ::windows::core::Result<IXpsOMStoryFragmentsResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStoryFragmentsResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStoryFragmentsResource<P0>(&self, storyfragmentsresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMStoryFragmentsResource>>,
    {
        (::windows::core::Vtable::vtable(self).SetStoryFragmentsResource)(::windows::core::Vtable::as_raw(self), storyfragmentsresource.into().abi()).ok()
    }
    pub unsafe fn GetPrintTicketResource(&self) -> ::windows::core::Result<IXpsOMPrintTicketResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPrintTicketResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPrintTicketResource<P0>(&self, printticketresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMPrintTicketResource>>,
    {
        (::windows::core::Vtable::vtable(self).SetPrintTicketResource)(::windows::core::Vtable::as_raw(self), printticketresource.into().abi()).ok()
    }
    pub unsafe fn GetThumbnailResource(&self) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetThumbnailResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetThumbnailResource<P0>(&self, imageresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMImageResource>>,
    {
        (::windows::core::Vtable::vtable(self).SetThumbnailResource)(::windows::core::Vtable::as_raw(self), imageresource.into().abi()).ok()
    }
    pub unsafe fn CollectLinkTargets(&self) -> ::windows::core::Result<IXpsOMNameCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CollectLinkTargets)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CollectPartResources(&self) -> ::windows::core::Result<IXpsOMPartResources> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CollectPartResources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasRestrictedFonts(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).HasRestrictedFonts)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMPageReference, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMPageReference {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMPageReference {
    type Vtable = IXpsOMPageReference_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMPageReference {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed360180_6f92_4998_890d_2f208531a0a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPageReference_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DiscardPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPageLoaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ispageloaded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPageLoaded: usize,
    pub GetAdvisoryPageDimensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows::core::HRESULT,
    pub SetAdvisoryPageDimensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows::core::HRESULT,
    pub GetStoryFragmentsResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyfragmentsresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStoryFragmentsResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyfragmentsresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetThumbnailResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetThumbnailResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CollectLinkTargets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linktargets: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CollectPartResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partresources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasRestrictedFonts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restrictedfonts: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasRestrictedFonts: usize,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagereference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPageReferenceCollection(::windows::core::IUnknown);
impl IXpsOMPageReferenceCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, pagereference: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMPageReference>>,
    {
        (::windows::core::Vtable::vtable(self).InsertAt)(::windows::core::Vtable::as_raw(self), index, pagereference.into().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveAt)(::windows::core::Vtable::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, pagereference: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMPageReference>>,
    {
        (::windows::core::Vtable::vtable(self).SetAt)(::windows::core::Vtable::as_raw(self), index, pagereference.into().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, pagereference: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMPageReference>>,
    {
        (::windows::core::Vtable::vtable(self).Append)(::windows::core::Vtable::as_raw(self), pagereference.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMPageReferenceCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMPageReferenceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMPageReferenceCollection {
    type Vtable = IXpsOMPageReferenceCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMPageReferenceCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca16ba4d_e7b9_45c5_958b_f98022473745);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPageReferenceCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pagereference: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pagereference: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pagereference: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagereference: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPart(::windows::core::IUnknown);
impl IXpsOMPart {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).SetPartName)(::windows::core::Vtable::as_raw(self), parturi.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMPart, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMPart {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMPart {
    type Vtable = IXpsOMPart_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMPart {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x74eb2f0b_a91e_4486_afac_0fabeca3dfc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPart_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetPartName: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetPartName: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPartResources(::windows::core::IUnknown);
impl IXpsOMPartResources {
    pub unsafe fn GetFontResources(&self) -> ::windows::core::Result<IXpsOMFontResourceCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFontResources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetImageResources(&self) -> ::windows::core::Result<IXpsOMImageResourceCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetImageResources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetColorProfileResources(&self) -> ::windows::core::Result<IXpsOMColorProfileResourceCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetColorProfileResources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetRemoteDictionaryResources(&self) -> ::windows::core::Result<IXpsOMRemoteDictionaryResourceCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRemoteDictionaryResources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMPartResources, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMPartResources {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMPartResources {
    type Vtable = IXpsOMPartResources_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMPartResources {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4cf7729_4864_4275_99b3_a8717163ecaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPartResources_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetFontResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontresources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetImageResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetColorProfileResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorprofileresources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRemoteDictionaryResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionaryresources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPartUriCollection(::windows::core::IUnknown);
impl IXpsOMPartUriCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn InsertAt<P0>(&self, index: u32, parturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).InsertAt)(::windows::core::Vtable::as_raw(self), index, parturi.into().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveAt)(::windows::core::Vtable::as_raw(self), index).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetAt<P0>(&self, index: u32, parturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).SetAt)(::windows::core::Vtable::as_raw(self), index, parturi.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn Append<P0>(&self, parturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).Append)(::windows::core::Vtable::as_raw(self), parturi.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMPartUriCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMPartUriCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMPartUriCollection {
    type Vtable = IXpsOMPartUriCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMPartUriCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57c650d4_067c_4893_8c33_f62a0633730f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPartUriCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, parturi: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetAt: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, parturi: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    InsertAt: usize,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, parturi: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetAt: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    Append: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPath(::windows::core::IUnknown);
impl IXpsOMPath {
    pub unsafe fn GetGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGeometry)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetGeometryLocal(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGeometryLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetGeometryLocal<P0>(&self, geometry: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMGeometry>>,
    {
        (::windows::core::Vtable::vtable(self).SetGeometryLocal)(::windows::core::Vtable::as_raw(self), geometry.into().abi()).ok()
    }
    pub unsafe fn GetGeometryLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGeometryLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetGeometryLookup<P0>(&self, lookup: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetGeometryLookup)(::windows::core::Vtable::as_raw(self), lookup.into().abi()).ok()
    }
    pub unsafe fn GetAccessibilityShortDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAccessibilityShortDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAccessibilityShortDescription<P0>(&self, shortdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetAccessibilityShortDescription)(::windows::core::Vtable::as_raw(self), shortdescription.into().abi()).ok()
    }
    pub unsafe fn GetAccessibilityLongDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAccessibilityLongDescription)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetAccessibilityLongDescription<P0>(&self, longdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetAccessibilityLongDescription)(::windows::core::Vtable::as_raw(self), longdescription.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSnapsToPixels(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSnapsToPixels)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSnapsToPixels<P0>(&self, snapstopixels: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetSnapsToPixels)(::windows::core::Vtable::as_raw(self), snapstopixels.into()).ok()
    }
    pub unsafe fn GetStrokeBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStrokeBrush)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStrokeBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStrokeBrushLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStrokeBrushLocal<P0>(&self, brush: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMBrush>>,
    {
        (::windows::core::Vtable::vtable(self).SetStrokeBrushLocal)(::windows::core::Vtable::as_raw(self), brush.into().abi()).ok()
    }
    pub unsafe fn GetStrokeBrushLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStrokeBrushLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStrokeBrushLookup<P0>(&self, lookup: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetStrokeBrushLookup)(::windows::core::Vtable::as_raw(self), lookup.into().abi()).ok()
    }
    pub unsafe fn GetStrokeDashes(&self) -> ::windows::core::Result<IXpsOMDashCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStrokeDashes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStrokeDashCap(&self) -> ::windows::core::Result<XPS_DASH_CAP> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStrokeDashCap)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStrokeDashCap(&self, strokedashcap: XPS_DASH_CAP) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetStrokeDashCap)(::windows::core::Vtable::as_raw(self), strokedashcap).ok()
    }
    pub unsafe fn GetStrokeDashOffset(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStrokeDashOffset)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStrokeDashOffset(&self, strokedashoffset: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetStrokeDashOffset)(::windows::core::Vtable::as_raw(self), strokedashoffset).ok()
    }
    pub unsafe fn GetStrokeStartLineCap(&self) -> ::windows::core::Result<XPS_LINE_CAP> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStrokeStartLineCap)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStrokeStartLineCap(&self, strokestartlinecap: XPS_LINE_CAP) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetStrokeStartLineCap)(::windows::core::Vtable::as_raw(self), strokestartlinecap).ok()
    }
    pub unsafe fn GetStrokeEndLineCap(&self) -> ::windows::core::Result<XPS_LINE_CAP> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStrokeEndLineCap)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStrokeEndLineCap(&self, strokeendlinecap: XPS_LINE_CAP) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetStrokeEndLineCap)(::windows::core::Vtable::as_raw(self), strokeendlinecap).ok()
    }
    pub unsafe fn GetStrokeLineJoin(&self) -> ::windows::core::Result<XPS_LINE_JOIN> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStrokeLineJoin)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStrokeLineJoin(&self, strokelinejoin: XPS_LINE_JOIN) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetStrokeLineJoin)(::windows::core::Vtable::as_raw(self), strokelinejoin).ok()
    }
    pub unsafe fn GetStrokeMiterLimit(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStrokeMiterLimit)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStrokeMiterLimit(&self, strokemiterlimit: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetStrokeMiterLimit)(::windows::core::Vtable::as_raw(self), strokemiterlimit).ok()
    }
    pub unsafe fn GetStrokeThickness(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStrokeThickness)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetStrokeThickness(&self, strokethickness: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetStrokeThickness)(::windows::core::Vtable::as_raw(self), strokethickness).ok()
    }
    pub unsafe fn GetFillBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFillBrush)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFillBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFillBrushLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFillBrushLocal<P0>(&self, brush: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMBrush>>,
    {
        (::windows::core::Vtable::vtable(self).SetFillBrushLocal)(::windows::core::Vtable::as_raw(self), brush.into().abi()).ok()
    }
    pub unsafe fn GetFillBrushLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFillBrushLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFillBrushLookup<P0>(&self, lookup: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetFillBrushLookup)(::windows::core::Vtable::as_raw(self), lookup.into().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMPath> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMPath, ::windows::core::IUnknown, IXpsOMShareable, IXpsOMVisual);
impl ::core::clone::Clone for IXpsOMPath {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMPath {
    type Vtable = IXpsOMPath_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMPath {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x37d38bb6_3ee9_4110_9312_14b194163337);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPath_Vtbl {
    pub base__: IXpsOMVisual_Vtbl,
    pub GetGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetGeometryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetGeometryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetGeometryLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetGeometryLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetAccessibilityShortDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortdescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetAccessibilityShortDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortdescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetAccessibilityLongDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, longdescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetAccessibilityLongDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, longdescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSnapsToPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapstopixels: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSnapsToPixels: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSnapsToPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapstopixels: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSnapsToPixels: usize,
    pub GetStrokeBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStrokeBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStrokeBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStrokeBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetStrokeBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetStrokeDashes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokedashes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStrokeDashCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokedashcap: *mut XPS_DASH_CAP) -> ::windows::core::HRESULT,
    pub SetStrokeDashCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokedashcap: XPS_DASH_CAP) -> ::windows::core::HRESULT,
    pub GetStrokeDashOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokedashoffset: *mut f32) -> ::windows::core::HRESULT,
    pub SetStrokeDashOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokedashoffset: f32) -> ::windows::core::HRESULT,
    pub GetStrokeStartLineCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokestartlinecap: *mut XPS_LINE_CAP) -> ::windows::core::HRESULT,
    pub SetStrokeStartLineCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokestartlinecap: XPS_LINE_CAP) -> ::windows::core::HRESULT,
    pub GetStrokeEndLineCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokeendlinecap: *mut XPS_LINE_CAP) -> ::windows::core::HRESULT,
    pub SetStrokeEndLineCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokeendlinecap: XPS_LINE_CAP) -> ::windows::core::HRESULT,
    pub GetStrokeLineJoin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokelinejoin: *mut XPS_LINE_JOIN) -> ::windows::core::HRESULT,
    pub SetStrokeLineJoin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokelinejoin: XPS_LINE_JOIN) -> ::windows::core::HRESULT,
    pub GetStrokeMiterLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokemiterlimit: *mut f32) -> ::windows::core::HRESULT,
    pub SetStrokeMiterLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokemiterlimit: f32) -> ::windows::core::HRESULT,
    pub GetStrokeThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokethickness: *mut f32) -> ::windows::core::HRESULT,
    pub SetStrokeThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokethickness: f32) -> ::windows::core::HRESULT,
    pub GetFillBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFillBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFillBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFillBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetFillBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMPrintTicketResource(::windows::core::IUnknown);
impl IXpsOMPrintTicketResource {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, partname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).SetContent)(::windows::core::Vtable::as_raw(self), sourcestream.into().abi(), partname.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMPrintTicketResource, ::windows::core::IUnknown, IXpsOMPart, IXpsOMResource);
impl ::core::clone::Clone for IXpsOMPrintTicketResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMPrintTicketResource {
    type Vtable = IXpsOMPrintTicketResource_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMPrintTicketResource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe7ff32d2_34aa_499b_bbe9_9cd4ee6c59f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPrintTicketResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMRadialGradientBrush(::windows::core::IUnknown);
impl IXpsOMRadialGradientBrush {
    pub unsafe fn GetCenter(&self) -> ::windows::core::Result<XPS_POINT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCenter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCenter(&self, center: *const XPS_POINT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetCenter)(::windows::core::Vtable::as_raw(self), center).ok()
    }
    pub unsafe fn GetRadiiSizes(&self) -> ::windows::core::Result<XPS_SIZE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRadiiSizes)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRadiiSizes(&self, radiisizes: *const XPS_SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetRadiiSizes)(::windows::core::Vtable::as_raw(self), radiisizes).ok()
    }
    pub unsafe fn GetGradientOrigin(&self) -> ::windows::core::Result<XPS_POINT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetGradientOrigin)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetGradientOrigin(&self, origin: *const XPS_POINT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetGradientOrigin)(::windows::core::Vtable::as_raw(self), origin).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMRadialGradientBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMRadialGradientBrush, ::windows::core::IUnknown, IXpsOMShareable, IXpsOMBrush, IXpsOMGradientBrush);
impl ::core::clone::Clone for IXpsOMRadialGradientBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMRadialGradientBrush {
    type Vtable = IXpsOMRadialGradientBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMRadialGradientBrush {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75f207e5_08bf_413c_96b1_b82b4064176b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRadialGradientBrush_Vtbl {
    pub base__: IXpsOMGradientBrush_Vtbl,
    pub GetCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, center: *mut XPS_POINT) -> ::windows::core::HRESULT,
    pub SetCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, center: *const XPS_POINT) -> ::windows::core::HRESULT,
    pub GetRadiiSizes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radiisizes: *mut XPS_SIZE) -> ::windows::core::HRESULT,
    pub SetRadiiSizes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radiisizes: *const XPS_SIZE) -> ::windows::core::HRESULT,
    pub GetGradientOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: *mut XPS_POINT) -> ::windows::core::HRESULT,
    pub SetGradientOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: *const XPS_POINT) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radialgradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMRemoteDictionaryResource(::windows::core::IUnknown);
impl IXpsOMRemoteDictionaryResource {
    pub unsafe fn GetDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDictionary)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDictionary<P0>(&self, dictionary: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMDictionary>>,
    {
        (::windows::core::Vtable::vtable(self).SetDictionary)(::windows::core::Vtable::as_raw(self), dictionary.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMRemoteDictionaryResource, ::windows::core::IUnknown, IXpsOMPart, IXpsOMResource);
impl ::core::clone::Clone for IXpsOMRemoteDictionaryResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMRemoteDictionaryResource {
    type Vtable = IXpsOMRemoteDictionaryResource_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMRemoteDictionaryResource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9bd7cd4_e16a_4bf8_8c84_c950af7a3061);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRemoteDictionaryResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    pub GetDictionary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDictionary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMRemoteDictionaryResource1(::windows::core::IUnknown);
impl IXpsOMRemoteDictionaryResource1 {
    pub unsafe fn GetDocumentType(&self) -> ::windows::core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDocumentType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write1<P0>(&self, stream: P0, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::ISequentialStream>>,
    {
        (::windows::core::Vtable::vtable(self).Write1)(::windows::core::Vtable::as_raw(self), stream.into().abi(), documenttype).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMRemoteDictionaryResource1, ::windows::core::IUnknown, IXpsOMPart, IXpsOMResource, IXpsOMRemoteDictionaryResource);
impl ::core::clone::Clone for IXpsOMRemoteDictionaryResource1 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMRemoteDictionaryResource1 {
    type Vtable = IXpsOMRemoteDictionaryResource1_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMRemoteDictionaryResource1 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbf8fc1d4_9d46_4141_ba5f_94bb9250d041);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRemoteDictionaryResource1_Vtbl {
    pub base__: IXpsOMRemoteDictionaryResource_Vtbl,
    pub GetDocumentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Write1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Write1: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMRemoteDictionaryResourceCollection(::windows::core::IUnknown);
impl IXpsOMRemoteDictionaryResourceCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, object: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMRemoteDictionaryResource>>,
    {
        (::windows::core::Vtable::vtable(self).InsertAt)(::windows::core::Vtable::as_raw(self), index, object.into().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveAt)(::windows::core::Vtable::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, object: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMRemoteDictionaryResource>>,
    {
        (::windows::core::Vtable::vtable(self).SetAt)(::windows::core::Vtable::as_raw(self), index, object.into().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, object: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMRemoteDictionaryResource>>,
    {
        (::windows::core::Vtable::vtable(self).Append)(::windows::core::Vtable::as_raw(self), object.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<P0>(&self, partname: P0) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetByPartName)(::windows::core::Vtable::as_raw(self), partname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMRemoteDictionaryResourceCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMRemoteDictionaryResourceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMRemoteDictionaryResourceCollection {
    type Vtable = IXpsOMRemoteDictionaryResourceCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMRemoteDictionaryResourceCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c38db61_7fec_464a_87bd_41e3bef018be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRemoteDictionaryResourceCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, remotedictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMResource(::windows::core::IUnknown);
impl IXpsOMResource {}
::windows::core::interface_hierarchy!(IXpsOMResource, ::windows::core::IUnknown, IXpsOMPart);
impl ::core::clone::Clone for IXpsOMResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMResource {
    type Vtable = IXpsOMResource_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMResource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda2ac0a2_73a2_4975_ad14_74097c3ff3a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMResource_Vtbl {
    pub base__: IXpsOMPart_Vtbl,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMShareable(::windows::core::IUnknown);
impl IXpsOMShareable {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMShareable, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMShareable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMShareable {
    type Vtable = IXpsOMShareable_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMShareable {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7137398f_2fc1_454d_8c6a_2c3115a16ece);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMShareable_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMSignatureBlockResource(::windows::core::IUnknown);
impl IXpsOMSignatureBlockResource {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMDocument> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, partname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).SetContent)(::windows::core::Vtable::as_raw(self), sourcestream.into().abi(), partname.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMSignatureBlockResource, ::windows::core::IUnknown, IXpsOMPart, IXpsOMResource);
impl ::core::clone::Clone for IXpsOMSignatureBlockResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMSignatureBlockResource {
    type Vtable = IXpsOMSignatureBlockResource_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMSignatureBlockResource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4776ad35_2e04_4357_8743_ebf6c171a905);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMSignatureBlockResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMSignatureBlockResourceCollection(::windows::core::IUnknown);
impl IXpsOMSignatureBlockResourceCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMSignatureBlockResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, signatureblockresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMSignatureBlockResource>>,
    {
        (::windows::core::Vtable::vtable(self).InsertAt)(::windows::core::Vtable::as_raw(self), index, signatureblockresource.into().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveAt)(::windows::core::Vtable::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, signatureblockresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMSignatureBlockResource>>,
    {
        (::windows::core::Vtable::vtable(self).SetAt)(::windows::core::Vtable::as_raw(self), index, signatureblockresource.into().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, signatureblockresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMSignatureBlockResource>>,
    {
        (::windows::core::Vtable::vtable(self).Append)(::windows::core::Vtable::as_raw(self), signatureblockresource.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<P0>(&self, partname: P0) -> ::windows::core::Result<IXpsOMSignatureBlockResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetByPartName)(::windows::core::Vtable::as_raw(self), partname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMSignatureBlockResourceCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMSignatureBlockResourceCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMSignatureBlockResourceCollection {
    type Vtable = IXpsOMSignatureBlockResourceCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMSignatureBlockResourceCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab8f5d8e_351b_4d33_aaed_fa56f0022931);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMSignatureBlockResourceCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureblockresource: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, signatureblockresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMSolidColorBrush(::windows::core::IUnknown);
impl IXpsOMSolidColorBrush {
    pub unsafe fn GetColor(&self, color: *mut XPS_COLOR, colorprofile: *mut ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetColor)(::windows::core::Vtable::as_raw(self), color, ::core::mem::transmute(colorprofile)).ok()
    }
    pub unsafe fn SetColor<P0>(&self, color: *const XPS_COLOR, colorprofile: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMColorProfileResource>>,
    {
        (::windows::core::Vtable::vtable(self).SetColor)(::windows::core::Vtable::as_raw(self), color, colorprofile.into().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMSolidColorBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMSolidColorBrush, ::windows::core::IUnknown, IXpsOMShareable, IXpsOMBrush);
impl ::core::clone::Clone for IXpsOMSolidColorBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMSolidColorBrush {
    type Vtable = IXpsOMSolidColorBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMSolidColorBrush {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa06f9f05_3be9_4763_98a8_094fc672e488);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMSolidColorBrush_Vtbl {
    pub base__: IXpsOMBrush_Vtbl,
    pub GetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, solidcolorbrush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMStoryFragmentsResource(::windows::core::IUnknown);
impl IXpsOMStoryFragmentsResource {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, partname: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).SetContent)(::windows::core::Vtable::as_raw(self), sourcestream.into().abi(), partname.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMStoryFragmentsResource, ::windows::core::IUnknown, IXpsOMPart, IXpsOMResource);
impl ::core::clone::Clone for IXpsOMStoryFragmentsResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMStoryFragmentsResource {
    type Vtable = IXpsOMStoryFragmentsResource_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMStoryFragmentsResource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2b3ca09_0473_4282_87ae_1780863223f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMStoryFragmentsResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMThumbnailGenerator(::windows::core::IUnknown);
impl IXpsOMThumbnailGenerator {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GenerateThumbnail<P0, P1>(&self, page: P0, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: P1) -> ::windows::core::Result<IXpsOMImageResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMPage>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GenerateThumbnail)(::windows::core::Vtable::as_raw(self), page.into().abi(), thumbnailtype, thumbnailsize, imageresourcepartname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMThumbnailGenerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMThumbnailGenerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMThumbnailGenerator {
    type Vtable = IXpsOMThumbnailGenerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMThumbnailGenerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15b873d5_1971_41e8_83a3_6578403064c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMThumbnailGenerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GenerateThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: *mut ::core::ffi::c_void, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GenerateThumbnail: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMTileBrush(::windows::core::IUnknown);
impl IXpsOMTileBrush {
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTransform)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTransformLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, transform: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMMatrixTransform>>,
    {
        (::windows::core::Vtable::vtable(self).SetTransformLocal)(::windows::core::Vtable::as_raw(self), transform.into().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTransformLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetTransformLookup)(::windows::core::Vtable::as_raw(self), key.into().abi()).ok()
    }
    pub unsafe fn GetViewbox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetViewbox)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetViewbox(&self, viewbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetViewbox)(::windows::core::Vtable::as_raw(self), viewbox).ok()
    }
    pub unsafe fn GetViewport(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetViewport)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetViewport(&self, viewport: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetViewport)(::windows::core::Vtable::as_raw(self), viewport).ok()
    }
    pub unsafe fn GetTileMode(&self) -> ::windows::core::Result<XPS_TILE_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTileMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTileMode(&self, tilemode: XPS_TILE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetTileMode)(::windows::core::Vtable::as_raw(self), tilemode).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMTileBrush, ::windows::core::IUnknown, IXpsOMShareable, IXpsOMBrush);
impl ::core::clone::Clone for IXpsOMTileBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMTileBrush {
    type Vtable = IXpsOMTileBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMTileBrush {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0fc2328d_d722_4a54_b2ec_be90218a789e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMTileBrush_Vtbl {
    pub base__: IXpsOMBrush_Vtbl,
    pub GetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetViewbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewbox: *mut XPS_RECT) -> ::windows::core::HRESULT,
    pub SetViewbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT) -> ::windows::core::HRESULT,
    pub GetViewport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *mut XPS_RECT) -> ::windows::core::HRESULT,
    pub SetViewport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *const XPS_RECT) -> ::windows::core::HRESULT,
    pub GetTileMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tilemode: *mut XPS_TILE_MODE) -> ::windows::core::HRESULT,
    pub SetTileMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tilemode: XPS_TILE_MODE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMVisual(::windows::core::IUnknown);
impl IXpsOMVisual {
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTransform)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTransformLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, matrixtransform: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMMatrixTransform>>,
    {
        (::windows::core::Vtable::vtable(self).SetTransformLocal)(::windows::core::Vtable::as_raw(self), matrixtransform.into().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTransformLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetTransformLookup)(::windows::core::Vtable::as_raw(self), key.into().abi()).ok()
    }
    pub unsafe fn GetClipGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetClipGeometry)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetClipGeometryLocal(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetClipGeometryLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClipGeometryLocal<P0>(&self, clipgeometry: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMGeometry>>,
    {
        (::windows::core::Vtable::vtable(self).SetClipGeometryLocal)(::windows::core::Vtable::as_raw(self), clipgeometry.into().abi()).ok()
    }
    pub unsafe fn GetClipGeometryLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetClipGeometryLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClipGeometryLookup<P0>(&self, key: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetClipGeometryLookup)(::windows::core::Vtable::as_raw(self), key.into().abi()).ok()
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOpacity)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetOpacity)(::windows::core::Vtable::as_raw(self), opacity).ok()
    }
    pub unsafe fn GetOpacityMaskBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOpacityMaskBrush)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOpacityMaskBrushLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOpacityMaskBrushLocal<P0>(&self, opacitymaskbrush: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMBrush>>,
    {
        (::windows::core::Vtable::vtable(self).SetOpacityMaskBrushLocal)(::windows::core::Vtable::as_raw(self), opacitymaskbrush.into().abi()).ok()
    }
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOpacityMaskBrushLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOpacityMaskBrushLookup<P0>(&self, key: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetOpacityMaskBrushLookup)(::windows::core::Vtable::as_raw(self), key.into().abi()).ok()
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIsHyperlinkTarget)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<P0>(&self, ishyperlink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).SetIsHyperlinkTarget)(::windows::core::Vtable::as_raw(self), ishyperlink.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> ::windows::core::Result<super::super::System::Com::IUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHyperlinkNavigateUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHyperlinkNavigateUri<P0>(&self, hyperlinkuri: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IUri>>,
    {
        (::windows::core::Vtable::vtable(self).SetHyperlinkNavigateUri)(::windows::core::Vtable::as_raw(self), hyperlinkuri.into().abi()).ok()
    }
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLanguage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLanguage<P0>(&self, language: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetLanguage)(::windows::core::Vtable::as_raw(self), language.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMVisual, ::windows::core::IUnknown, IXpsOMShareable);
impl ::core::clone::Clone for IXpsOMVisual {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMVisual {
    type Vtable = IXpsOMVisual_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMVisual {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc3e7333_fb0b_4af3_a819_0b4eaad0d2fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMVisual_Vtbl {
    pub base__: IXpsOMShareable_Vtbl,
    pub GetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetClipGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetClipGeometryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetClipGeometryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetClipGeometryLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetClipGeometryLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows::core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT,
    pub GetOpacityMaskBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOpacityMaskBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetOpacityMaskBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOpacityMaskBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetOpacityMaskBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsHyperlinkTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlink: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsHyperlinkTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsHyperlinkTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlink: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsHyperlinkTarget: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetHyperlinkNavigateUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hyperlinkuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetHyperlinkNavigateUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetHyperlinkNavigateUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hyperlinkuri: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetHyperlinkNavigateUri: usize,
    pub GetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMVisualBrush(::windows::core::IUnknown);
impl IXpsOMVisualBrush {
    pub unsafe fn GetVisual(&self) -> ::windows::core::Result<IXpsOMVisual> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetVisual)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVisualLocal(&self) -> ::windows::core::Result<IXpsOMVisual> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetVisualLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetVisualLocal<P0>(&self, visual: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMVisual>>,
    {
        (::windows::core::Vtable::vtable(self).SetVisualLocal)(::windows::core::Vtable::as_raw(self), visual.into().abi()).ok()
    }
    pub unsafe fn GetVisualLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetVisualLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetVisualLookup<P0>(&self, lookup: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetVisualLookup)(::windows::core::Vtable::as_raw(self), lookup.into().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMVisualBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsOMVisualBrush, ::windows::core::IUnknown, IXpsOMShareable, IXpsOMBrush, IXpsOMTileBrush);
impl ::core::clone::Clone for IXpsOMVisualBrush {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMVisualBrush {
    type Vtable = IXpsOMVisualBrush_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMVisualBrush {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97e294af_5b37_46b4_8057_874d2f64119b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMVisualBrush_Vtbl {
    pub base__: IXpsOMTileBrush_Vtbl,
    pub GetVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetVisualLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetVisualLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetVisualLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetVisualLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visualbrush: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsOMVisualCollection(::windows::core::IUnknown);
impl IXpsOMVisualCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsOMVisual> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, object: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMVisual>>,
    {
        (::windows::core::Vtable::vtable(self).InsertAt)(::windows::core::Vtable::as_raw(self), index, object.into().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveAt)(::windows::core::Vtable::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, object: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMVisual>>,
    {
        (::windows::core::Vtable::vtable(self).SetAt)(::windows::core::Vtable::as_raw(self), index, object.into().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, object: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMVisual>>,
    {
        (::windows::core::Vtable::vtable(self).Append)(::windows::core::Vtable::as_raw(self), object.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsOMVisualCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsOMVisualCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsOMVisualCollection {
    type Vtable = IXpsOMVisualCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsOMVisualCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94d8abde_ab91_46a8_82b7_f5b05ef01a96);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMVisualCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsSignature(::windows::core::IUnknown);
impl IXpsSignature {
    pub unsafe fn GetSignatureId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSignatureId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSignatureValue(&self, signaturehashvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetSignatureValue)(::windows::core::Vtable::as_raw(self), signaturehashvalue, count).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCertificateEnumerator(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcCertificateEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCertificateEnumerator)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSigningTime(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSigningTime)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetSigningTimeFormat(&self) -> ::windows::core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSigningTimeFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetSignaturePartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSignaturePartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn Verify(&self, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<XPS_SIGNATURE_STATUS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Verify)(::windows::core::Vtable::as_raw(self), x509certificate, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPolicy(&self) -> ::windows::core::Result<XPS_SIGN_POLICY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPolicy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomObjectEnumerator(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCustomObjectEnumerator)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomReferenceEnumerator(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcSignatureReferenceEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCustomReferenceEnumerator)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSignatureXml(&self, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetSignatureXml)(::windows::core::Vtable::as_raw(self), signaturexml, count).ok()
    }
    pub unsafe fn SetSignatureXml(&self, signaturexml: &[u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSignatureXml)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(signaturexml.as_ptr()), signaturexml.len() as _).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsSignature, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsSignature {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsSignature {
    type Vtable = IXpsSignature_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsSignature {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ae4c93e_1ade_42fb_898b_3a5658284857);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignature_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSignatureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sigid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetSignatureValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturehashvalue: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCertificateEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificateenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCertificateEnumerator: usize,
    pub GetSigningTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sigdatetimestring: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetSigningTimeFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetSigningTimeFormat: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetSignaturePartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetSignaturePartName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub Verify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, sigstatus: *mut XPS_SIGNATURE_STATUS) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    Verify: usize,
    pub GetPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCustomObjectEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCustomObjectEnumerator: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCustomReferenceEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCustomReferenceEnumerator: usize,
    pub GetSignatureXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT,
    pub SetSignatureXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturexml: *const u8, count: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsSignatureBlock(::windows::core::IUnknown);
impl IXpsSignatureBlock {
    pub unsafe fn GetRequests(&self) -> ::windows::core::Result<IXpsSignatureRequestCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRequests)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDocumentIndex(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDocumentIndex)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetDocumentName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDocumentName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateRequest<P0>(&self, requestid: P0) -> ::windows::core::Result<IXpsSignatureRequest>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateRequest)(::windows::core::Vtable::as_raw(self), requestid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsSignatureBlock, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsSignatureBlock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsSignatureBlock {
    type Vtable = IXpsSignatureBlock_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsSignatureBlock {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x151fac09_0b97_4ac6_a323_5e4297d4322b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureBlock_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetRequests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requests: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetPartName: usize,
    pub GetDocumentIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fixeddocumentindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetDocumentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fixeddocumentname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetDocumentName: usize,
    pub CreateRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: ::windows::core::PCWSTR, signaturerequest: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsSignatureBlockCollection(::windows::core::IUnknown);
impl IXpsSignatureBlockCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsSignatureBlock> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveAt)(::windows::core::Vtable::as_raw(self), index).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsSignatureBlockCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsSignatureBlockCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsSignatureBlockCollection {
    type Vtable = IXpsSignatureBlockCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsSignatureBlockCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23397050_fe99_467a_8dce_9237f074ffe4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureBlockCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signatureblock: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsSignatureCollection(::windows::core::IUnknown);
impl IXpsSignatureCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsSignature> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveAt)(::windows::core::Vtable::as_raw(self), index).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsSignatureCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsSignatureCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsSignatureCollection {
    type Vtable = IXpsSignatureCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsSignatureCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa2d1d95d_add2_4dff_ab27_6b9c645ff322);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsSignatureManager(::windows::core::IUnknown);
impl IXpsSignatureManager {
    pub unsafe fn LoadPackageFile<P0>(&self, filename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).LoadPackageFile)(::windows::core::Vtable::as_raw(self), filename.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadPackageStream<P0>(&self, stream: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).LoadPackageStream)(::windows::core::Vtable::as_raw(self), stream.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn Sign<P0>(&self, signoptions: P0, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::Result<IXpsSignature>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsSigningOptions>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).Sign)(::windows::core::Vtable::as_raw(self), signoptions.into().abi(), x509certificate, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetSignatureOriginPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSignatureOriginPartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetSignatureOriginPartName<P0>(&self, signatureoriginpartname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).SetSignatureOriginPartName)(::windows::core::Vtable::as_raw(self), signatureoriginpartname.into().abi()).ok()
    }
    pub unsafe fn GetSignatures(&self) -> ::windows::core::Result<IXpsSignatureCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSignatures)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn AddSignatureBlock<P0>(&self, partname: P0, fixeddocumentindex: u32) -> ::windows::core::Result<IXpsSignatureBlock>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).AddSignatureBlock)(::windows::core::Vtable::as_raw(self), partname.into().abi(), fixeddocumentindex, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSignatureBlocks(&self) -> ::windows::core::Result<IXpsSignatureBlockCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSignatureBlocks)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateSigningOptions(&self) -> ::windows::core::Result<IXpsSigningOptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSigningOptions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn SavePackageToFile<P0>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SavePackageToFile)(::windows::core::Vtable::as_raw(self), filename.into().abi(), securityattributes, flagsandattributes).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SavePackageToStream<P0>(&self, stream: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).SavePackageToStream)(::windows::core::Vtable::as_raw(self), stream.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsSignatureManager, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsSignatureManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsSignatureManager {
    type Vtable = IXpsSignatureManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsSignatureManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3e8d338_fdc4_4afc_80b5_d532a1782ee1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub LoadPackageFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadPackageStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadPackageStream: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub Sign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signoptions: *mut ::core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, signature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    Sign: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetSignatureOriginPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetSignatureOriginPartName: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetSignatureOriginPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetSignatureOriginPartName: usize,
    pub GetSignatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatures: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub AddSignatureBlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, fixeddocumentindex: u32, signatureblock: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    AddSignatureBlock: usize,
    pub GetSignatureBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureblocks: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateSigningOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signingoptions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub SavePackageToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))]
    SavePackageToFile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SavePackageToStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SavePackageToStream: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsSignatureRequest(::windows::core::IUnknown);
impl IXpsSignatureRequest {
    pub unsafe fn GetIntent(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIntent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetIntent<P0>(&self, intent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetIntent)(::windows::core::Vtable::as_raw(self), intent.into().abi()).ok()
    }
    pub unsafe fn GetRequestedSigner(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRequestedSigner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRequestedSigner<P0>(&self, signername: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetRequestedSigner)(::windows::core::Vtable::as_raw(self), signername.into().abi()).ok()
    }
    pub unsafe fn GetRequestSignByDate(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRequestSignByDate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetRequestSignByDate<P0>(&self, datestring: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetRequestSignByDate)(::windows::core::Vtable::as_raw(self), datestring.into().abi()).ok()
    }
    pub unsafe fn GetSigningLocale(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSigningLocale)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSigningLocale<P0>(&self, place: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetSigningLocale)(::windows::core::Vtable::as_raw(self), place.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetSpotLocation(&self, pageindex: *mut i32, pagepartname: *mut ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, x: *mut f32, y: *mut f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetSpotLocation)(::windows::core::Vtable::as_raw(self), pageindex, ::core::mem::transmute(pagepartname), x, y).ok()
    }
    pub unsafe fn SetSpotLocation(&self, pageindex: i32, x: f32, y: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSpotLocation)(::windows::core::Vtable::as_raw(self), pageindex, x, y).ok()
    }
    pub unsafe fn GetRequestId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRequestId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSignature(&self) -> ::windows::core::Result<IXpsSignature> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSignature)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IXpsSignatureRequest, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsSignatureRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsSignatureRequest {
    type Vtable = IXpsSignatureRequest_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsSignatureRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xac58950b_7208_4b2d_b2c4_951083d3b8eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureRequest_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetIntent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, intent: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetIntent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, intent: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetRequestedSigner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signername: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetRequestedSigner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signername: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetRequestSignByDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datestring: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetRequestSignByDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datestring: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetSigningLocale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, place: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetSigningLocale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, place: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetSpotLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pageindex: *mut i32, pagepartname: *mut *mut ::core::ffi::c_void, x: *mut f32, y: *mut f32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetSpotLocation: usize,
    pub SetSpotLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pageindex: i32, x: f32, y: f32) -> ::windows::core::HRESULT,
    pub GetRequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signature: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsSignatureRequestCollection(::windows::core::IUnknown);
impl IXpsSignatureRequestCollection {
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows::core::Result<IXpsSignatureRequest> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAt)(::windows::core::Vtable::as_raw(self), index, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).RemoveAt)(::windows::core::Vtable::as_raw(self), index).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsSignatureRequestCollection, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsSignatureRequestCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsSignatureRequestCollection {
    type Vtable = IXpsSignatureRequestCollection_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsSignatureRequestCollection {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0253e68_9f19_412e_9b4f_54d3b0ac6cd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureRequestCollection_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signaturerequest: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
pub struct IXpsSigningOptions(::windows::core::IUnknown);
impl IXpsSigningOptions {
    pub unsafe fn GetSignatureId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSignatureId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSignatureId<P0>(&self, signatureid: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetSignatureId)(::windows::core::Vtable::as_raw(self), signatureid.into().abi()).ok()
    }
    pub unsafe fn GetSignatureMethod(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSignatureMethod)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSignatureMethod<P0>(&self, signaturemethod: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetSignatureMethod)(::windows::core::Vtable::as_raw(self), signaturemethod.into().abi()).ok()
    }
    pub unsafe fn GetDigestMethod(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDigestMethod)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDigestMethod<P0>(&self, digestmethod: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetDigestMethod)(::windows::core::Vtable::as_raw(self), digestmethod.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetSignaturePartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSignaturePartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetSignaturePartName<P0>(&self, signaturepartname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).SetSignaturePartName)(::windows::core::Vtable::as_raw(self), signaturepartname.into().abi()).ok()
    }
    pub unsafe fn GetPolicy(&self) -> ::windows::core::Result<XPS_SIGN_POLICY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPolicy)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPolicy(&self, policy: XPS_SIGN_POLICY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetPolicy)(::windows::core::Vtable::as_raw(self), policy).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetSigningTimeFormat(&self) -> ::windows::core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSigningTimeFormat)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetSigningTimeFormat(&self, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetSigningTimeFormat)(::windows::core::Vtable::as_raw(self), timeformat).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomObjects(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectSet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCustomObjects)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomReferences(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcSignatureReferenceSet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCustomReferences)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`*"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCertificateSet(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcCertificateSet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCertificateSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFlags(&self) -> ::windows::core::Result<XPS_SIGN_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFlags)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetFlags(&self, flags: XPS_SIGN_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).SetFlags)(::windows::core::Vtable::as_raw(self), flags).ok()
    }
}
::windows::core::interface_hierarchy!(IXpsSigningOptions, ::windows::core::IUnknown);
impl ::core::clone::Clone for IXpsSigningOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Vtable for IXpsSigningOptions {
    type Vtable = IXpsSigningOptions_Vtbl;
}
unsafe impl ::windows::core::Interface for IXpsSigningOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7718eae4_3215_49be_af5b_594fef7fcfa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSigningOptions_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSignatureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetSignatureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetSignatureMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturemethod: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetSignatureMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturemethod: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetDigestMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetDigestMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestmethod: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetSignaturePartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetSignaturePartName: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetSignaturePartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetSignaturePartName: usize,
    pub GetPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> ::windows::core::HRESULT,
    pub SetPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: XPS_SIGN_POLICY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetSigningTimeFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetSigningTimeFormat: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub SetSigningTimeFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    SetSigningTimeFormat: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCustomObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customobjectset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCustomObjects: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCustomReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customreferenceset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCustomReferences: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCertificateSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificateset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCertificateSet: usize,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut XPS_SIGN_FLAGS) -> ::windows::core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: XPS_SIGN_FLAGS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_ABSOLUTE_REFERENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108159i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_ALREADY_OWNED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108413i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_BLEED_BOX_PAGE_DIMENSIONS_NOT_IN_SYNC: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108407i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_BOTH_PATHFIGURE_AND_ABBR_SYNTAX_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108409i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_BOTH_RESOURCE_AND_SOURCEATTR_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108408i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_CARET_OUTSIDE_STRING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108923i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_CARET_OUT_OF_ORDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108922i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_COLOR_COMPONENT_OUT_OF_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108410i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_DICTIONARY_ITEM_NAMED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108671i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_DUPLICATE_NAMES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109175i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_DUPLICATE_RESOURCE_KEYS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109184i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INDEX_OUT_OF_RANGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108416i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_BLEED_BOX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109692i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_CONTENT_BOX: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109685i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_CONTENT_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109682i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_FLOAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109689i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_FONT_URI: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109686i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_LANGUAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109696i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_LOOKUP_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109690i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_MARKUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109684i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109695i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_NUMBER_OF_COLOR_CHANNELS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108158i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_NUMBER_OF_POINTS_IN_CURVE_SEGMENTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108160i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_OBFUSCATED_FONT_URI: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109681i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_PAGE_SIZE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109693i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_RESOURCE_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109694i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_SIGNATUREBLOCK_MARKUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108789i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_THUMBNAIL_IMAGE_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109691i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_INVALID_XML_ENCODING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109683i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MAPPING_OUTSIDE_INDICES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108924i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MAPPING_OUTSIDE_STRING: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108925i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MAPPING_OUT_OF_ORDER: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108926i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MARKUP_COMPATIBILITY_ELEMENTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108791i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_COLORPROFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109436i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_DISCARDCONTROL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109422i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_DOCUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109431i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_DOCUMENTSEQUENCE_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109432i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_FONTURI: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109433i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_GLYPHS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109438i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_IMAGE_IN_IMAGEBRUSH: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109426i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_LOOKUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109439i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_NAME: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109440i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_PAGE_IN_DOCUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109428i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_PAGE_IN_PAGEREFERENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109427i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_PART_REFERENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109424i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_PART_STREAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109421i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_REFERRED_DOCUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109430i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_REFERRED_PAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109429i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_RELATIONSHIP_TARGET: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109435i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_RESOURCE_KEY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109425i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_RESOURCE_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109434i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_RESTRICTED_FONT_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109423i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MISSING_SEGMENT_DATA: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109437i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_DOCUMENTSEQUENCE_RELATIONSHIPS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109182i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109178i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENTSEQUENCE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109177i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_PAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109179i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_REFERENCES_TO_PART: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109176i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_RESOURCES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109183i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PACKAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109180i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109181i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_NEGATIVE_FLOAT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108918i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_NESTED_REMOTE_DICTIONARY: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108670i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_NOT_ENOUGH_GRADIENT_STOPS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108405i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_NO_CUSTOM_OBJECTS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108414i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_OBJECT_DETACHED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108790i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_ODD_BIDILEVEL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108921i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_ONE_TO_ONE_MAPPING_EXPECTED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108920i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_PACKAGE_ALREADY_OPENED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108793i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_PACKAGE_NOT_OPENED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108794i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_PACKAGE_WRITER_NOT_CLOSED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108404i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_RELATIONSHIP_EXTERNAL: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108406i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_RESOURCE_NOT_OWNED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108412i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_RESTRICTED_FONT_NOT_OBFUSCATED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108919i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_SIGNATUREID_DUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108792i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_SIGREQUESTID_DUP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108795i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_STRING_TOO_LONG: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108928i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_TOO_MANY_INDICES: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108927i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_UNAVAILABLE_PACKAGE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109420i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_UNEXPECTED_COLORPROFILE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108411i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_UNEXPECTED_CONTENT_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109688i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_UNEXPECTED_RELATIONSHIP_TYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109680i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_UNEXPECTED_RESTRICTED_FONT_RELATIONSHIP: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142109679i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_VISUAL_CIRCULAR_REF: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108415i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_E_XKEY_ATTR_PRESENT_OUTSIDE_RES_DICT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2142108672i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XpsOMObjectFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe974d26d_3d9b_4d47_88cc_3872f2dc3585);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XpsOMThumbnailGenerator: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e4a23e2_b969_4761_be35_1a8ced58e323);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XpsSignatureManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb0c43320_2315_44a2_b70a_0943a140a8ee);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DEVICE_CAPABILITIES(pub u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_BINNAMES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(12u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_BINS: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(6u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_COLLATE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(22u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_COLORDEVICE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(32u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_COPIES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(18u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_DRIVER: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(11u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_DUPLEX: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(7u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_ENUMRESOLUTIONS: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(13u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_EXTRA: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(9u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_FIELDS: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(1u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_FILEDEPENDENCIES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(14u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_MAXEXTENT: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(5u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_MEDIAREADY: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(29u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_MEDIATYPENAMES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(34u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_MEDIATYPES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(35u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_MINEXTENT: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(4u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_ORIENTATION: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(17u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_NUP: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(33u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PAPERNAMES: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(16u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PAPERS: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(2u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PAPERSIZE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(3u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PERSONALITY: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(25u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PRINTERMEM: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(28u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PRINTRATE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(26u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PRINTRATEPPM: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(31u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_PRINTRATEUNIT: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(27u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_SIZE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(8u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_STAPLE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(30u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_TRUETYPE: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(15u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const DC_VERSION: DEVICE_CAPABILITIES = DEVICE_CAPABILITIES(10u16);
impl ::core::marker::Copy for DEVICE_CAPABILITIES {}
impl ::core::clone::Clone for DEVICE_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DEVICE_CAPABILITIES {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRINT_WINDOW_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PW_CLIENTONLY: PRINT_WINDOW_FLAGS = PRINT_WINDOW_FLAGS(1u32);
impl ::core::marker::Copy for PRINT_WINDOW_FLAGS {}
impl ::core::clone::Clone for PRINT_WINDOW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PRINT_WINDOW_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PSINJECT_POINT(pub u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_BEGINSTREAM: PSINJECT_POINT = PSINJECT_POINT(1u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PSADOBE: PSINJECT_POINT = PSINJECT_POINT(2u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PAGESATEND: PSINJECT_POINT = PSINJECT_POINT(3u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PAGES: PSINJECT_POINT = PSINJECT_POINT(4u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_DOCNEEDEDRES: PSINJECT_POINT = PSINJECT_POINT(5u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_DOCSUPPLIEDRES: PSINJECT_POINT = PSINJECT_POINT(6u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PAGEORDER: PSINJECT_POINT = PSINJECT_POINT(7u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ORIENTATION: PSINJECT_POINT = PSINJECT_POINT(8u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_BOUNDINGBOX: PSINJECT_POINT = PSINJECT_POINT(9u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_DOCUMENTPROCESSCOLORS: PSINJECT_POINT = PSINJECT_POINT(10u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_COMMENTS: PSINJECT_POINT = PSINJECT_POINT(11u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_BEGINDEFAULTS: PSINJECT_POINT = PSINJECT_POINT(12u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ENDDEFAULTS: PSINJECT_POINT = PSINJECT_POINT(13u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_BEGINPROLOG: PSINJECT_POINT = PSINJECT_POINT(14u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ENDPROLOG: PSINJECT_POINT = PSINJECT_POINT(15u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_BEGINSETUP: PSINJECT_POINT = PSINJECT_POINT(16u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ENDSETUP: PSINJECT_POINT = PSINJECT_POINT(17u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_TRAILER: PSINJECT_POINT = PSINJECT_POINT(18u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_EOF: PSINJECT_POINT = PSINJECT_POINT(19u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ENDSTREAM: PSINJECT_POINT = PSINJECT_POINT(20u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_DOCUMENTPROCESSCOLORSATEND: PSINJECT_POINT = PSINJECT_POINT(21u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PAGENUMBER: PSINJECT_POINT = PSINJECT_POINT(100u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_BEGINPAGESETUP: PSINJECT_POINT = PSINJECT_POINT(101u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ENDPAGESETUP: PSINJECT_POINT = PSINJECT_POINT(102u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PAGETRAILER: PSINJECT_POINT = PSINJECT_POINT(103u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PLATECOLOR: PSINJECT_POINT = PSINJECT_POINT(104u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_SHOWPAGE: PSINJECT_POINT = PSINJECT_POINT(105u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_PAGEBBOX: PSINJECT_POINT = PSINJECT_POINT(106u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_ENDPAGECOMMENTS: PSINJECT_POINT = PSINJECT_POINT(107u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_VMSAVE: PSINJECT_POINT = PSINJECT_POINT(200u16);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const PSINJECT_VMRESTORE: PSINJECT_POINT = PSINJECT_POINT(201u16);
impl ::core::marker::Copy for PSINJECT_POINT {}
impl ::core::clone::Clone for PSINJECT_POINT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PSINJECT_POINT {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_COLOR_INTERPOLATION(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_COLOR_INTERPOLATION_SCRGBLINEAR: XPS_COLOR_INTERPOLATION = XPS_COLOR_INTERPOLATION(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_COLOR_INTERPOLATION_SRGBLINEAR: XPS_COLOR_INTERPOLATION = XPS_COLOR_INTERPOLATION(2i32);
impl ::core::marker::Copy for XPS_COLOR_INTERPOLATION {}
impl ::core::clone::Clone for XPS_COLOR_INTERPOLATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_COLOR_INTERPOLATION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_COLOR_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_COLOR_TYPE_SRGB: XPS_COLOR_TYPE = XPS_COLOR_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_COLOR_TYPE_SCRGB: XPS_COLOR_TYPE = XPS_COLOR_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_COLOR_TYPE_CONTEXT: XPS_COLOR_TYPE = XPS_COLOR_TYPE(3i32);
impl ::core::marker::Copy for XPS_COLOR_TYPE {}
impl ::core::clone::Clone for XPS_COLOR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_COLOR_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_DASH_CAP(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DASH_CAP_FLAT: XPS_DASH_CAP = XPS_DASH_CAP(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DASH_CAP_ROUND: XPS_DASH_CAP = XPS_DASH_CAP(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DASH_CAP_SQUARE: XPS_DASH_CAP = XPS_DASH_CAP(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DASH_CAP_TRIANGLE: XPS_DASH_CAP = XPS_DASH_CAP(4i32);
impl ::core::marker::Copy for XPS_DASH_CAP {}
impl ::core::clone::Clone for XPS_DASH_CAP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_DASH_CAP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_DOCUMENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DOCUMENT_TYPE_UNSPECIFIED: XPS_DOCUMENT_TYPE = XPS_DOCUMENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DOCUMENT_TYPE_XPS: XPS_DOCUMENT_TYPE = XPS_DOCUMENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_DOCUMENT_TYPE_OPENXPS: XPS_DOCUMENT_TYPE = XPS_DOCUMENT_TYPE(3i32);
impl ::core::marker::Copy for XPS_DOCUMENT_TYPE {}
impl ::core::clone::Clone for XPS_DOCUMENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_DOCUMENT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_FILL_RULE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_FILL_RULE_EVENODD: XPS_FILL_RULE = XPS_FILL_RULE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_FILL_RULE_NONZERO: XPS_FILL_RULE = XPS_FILL_RULE(2i32);
impl ::core::marker::Copy for XPS_FILL_RULE {}
impl ::core::clone::Clone for XPS_FILL_RULE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_FILL_RULE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_FONT_EMBEDDING(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_FONT_EMBEDDING_NORMAL: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_FONT_EMBEDDING_OBFUSCATED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_FONT_EMBEDDING_RESTRICTED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_FONT_EMBEDDING_RESTRICTED_UNOBFUSCATED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(4i32);
impl ::core::marker::Copy for XPS_FONT_EMBEDDING {}
impl ::core::clone::Clone for XPS_FONT_EMBEDDING {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_FONT_EMBEDDING {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_IMAGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_IMAGE_TYPE_JPEG: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_IMAGE_TYPE_PNG: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_IMAGE_TYPE_TIFF: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_IMAGE_TYPE_WDP: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_IMAGE_TYPE_JXR: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(5i32);
impl ::core::marker::Copy for XPS_IMAGE_TYPE {}
impl ::core::clone::Clone for XPS_IMAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_IMAGE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_INTERLEAVING(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_INTERLEAVING_OFF: XPS_INTERLEAVING = XPS_INTERLEAVING(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_INTERLEAVING_ON: XPS_INTERLEAVING = XPS_INTERLEAVING(2i32);
impl ::core::marker::Copy for XPS_INTERLEAVING {}
impl ::core::clone::Clone for XPS_INTERLEAVING {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_INTERLEAVING {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_LINE_CAP(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_CAP_FLAT: XPS_LINE_CAP = XPS_LINE_CAP(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_CAP_ROUND: XPS_LINE_CAP = XPS_LINE_CAP(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_CAP_SQUARE: XPS_LINE_CAP = XPS_LINE_CAP(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_CAP_TRIANGLE: XPS_LINE_CAP = XPS_LINE_CAP(4i32);
impl ::core::marker::Copy for XPS_LINE_CAP {}
impl ::core::clone::Clone for XPS_LINE_CAP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_LINE_CAP {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_LINE_JOIN(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_JOIN_MITER: XPS_LINE_JOIN = XPS_LINE_JOIN(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_JOIN_BEVEL: XPS_LINE_JOIN = XPS_LINE_JOIN(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_LINE_JOIN_ROUND: XPS_LINE_JOIN = XPS_LINE_JOIN(3i32);
impl ::core::marker::Copy for XPS_LINE_JOIN {}
impl ::core::clone::Clone for XPS_LINE_JOIN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_LINE_JOIN {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_OBJECT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_CANVAS: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_GLYPHS: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_PATH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_MATRIX_TRANSFORM: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_GEOMETRY: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_SOLID_COLOR_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_IMAGE_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_LINEAR_GRADIENT_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(8i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_RADIAL_GRADIENT_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(9i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_OBJECT_TYPE_VISUAL_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(10i32);
impl ::core::marker::Copy for XPS_OBJECT_TYPE {}
impl ::core::clone::Clone for XPS_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_OBJECT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_SEGMENT_STROKE_PATTERN(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_STROKE_PATTERN_ALL: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_STROKE_PATTERN_NONE: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_STROKE_PATTERN_MIXED: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(3i32);
impl ::core::marker::Copy for XPS_SEGMENT_STROKE_PATTERN {}
impl ::core::clone::Clone for XPS_SEGMENT_STROKE_PATTERN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_SEGMENT_STROKE_PATTERN {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_SEGMENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_ARC_LARGE_CLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_ARC_LARGE_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_ARC_SMALL_CLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_ARC_SMALL_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_BEZIER: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_LINE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SEGMENT_TYPE_QUADRATIC_BEZIER: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(7i32);
impl ::core::marker::Copy for XPS_SEGMENT_TYPE {}
impl ::core::clone::Clone for XPS_SEGMENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_SEGMENT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_SIGNATURE_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGNATURE_STATUS_INCOMPLIANT: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGNATURE_STATUS_INCOMPLETE: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGNATURE_STATUS_BROKEN: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGNATURE_STATUS_QUESTIONABLE: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(4i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGNATURE_STATUS_VALID: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(5i32);
impl ::core::marker::Copy for XPS_SIGNATURE_STATUS {}
impl ::core::clone::Clone for XPS_SIGNATURE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_SIGNATURE_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_SIGN_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_FLAGS_NONE: XPS_SIGN_FLAGS = XPS_SIGN_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_FLAGS_IGNORE_MARKUP_COMPATIBILITY: XPS_SIGN_FLAGS = XPS_SIGN_FLAGS(1i32);
impl ::core::marker::Copy for XPS_SIGN_FLAGS {}
impl ::core::clone::Clone for XPS_SIGN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_SIGN_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_SIGN_POLICY(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_POLICY_NONE: XPS_SIGN_POLICY = XPS_SIGN_POLICY(0i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_POLICY_CORE_PROPERTIES: XPS_SIGN_POLICY = XPS_SIGN_POLICY(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_POLICY_SIGNATURE_RELATIONSHIPS: XPS_SIGN_POLICY = XPS_SIGN_POLICY(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_POLICY_PRINT_TICKET: XPS_SIGN_POLICY = XPS_SIGN_POLICY(4i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_POLICY_DISCARD_CONTROL: XPS_SIGN_POLICY = XPS_SIGN_POLICY(8i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SIGN_POLICY_ALL: XPS_SIGN_POLICY = XPS_SIGN_POLICY(15i32);
impl ::core::marker::Copy for XPS_SIGN_POLICY {}
impl ::core::clone::Clone for XPS_SIGN_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_SIGN_POLICY {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_SPREAD_METHOD(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SPREAD_METHOD_PAD: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SPREAD_METHOD_REFLECT: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_SPREAD_METHOD_REPEAT: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(3i32);
impl ::core::marker::Copy for XPS_SPREAD_METHOD {}
impl ::core::clone::Clone for XPS_SPREAD_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_SPREAD_METHOD {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_STYLE_SIMULATION(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_STYLE_SIMULATION_NONE: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_STYLE_SIMULATION_ITALIC: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_STYLE_SIMULATION_BOLD: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_STYLE_SIMULATION_BOLDITALIC: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(4i32);
impl ::core::marker::Copy for XPS_STYLE_SIMULATION {}
impl ::core::clone::Clone for XPS_STYLE_SIMULATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_STYLE_SIMULATION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_THUMBNAIL_SIZE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_THUMBNAIL_SIZE_VERYSMALL: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_THUMBNAIL_SIZE_SMALL: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_THUMBNAIL_SIZE_MEDIUM: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_THUMBNAIL_SIZE_LARGE: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(4i32);
impl ::core::marker::Copy for XPS_THUMBNAIL_SIZE {}
impl ::core::clone::Clone for XPS_THUMBNAIL_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_THUMBNAIL_SIZE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_TILE_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_TILE_MODE_NONE: XPS_TILE_MODE = XPS_TILE_MODE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_TILE_MODE_TILE: XPS_TILE_MODE = XPS_TILE_MODE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_TILE_MODE_FLIPX: XPS_TILE_MODE = XPS_TILE_MODE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_TILE_MODE_FLIPY: XPS_TILE_MODE = XPS_TILE_MODE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub const XPS_TILE_MODE_FLIPXY: XPS_TILE_MODE = XPS_TILE_MODE(5i32);
impl ::core::marker::Copy for XPS_TILE_MODE {}
impl ::core::clone::Clone for XPS_TILE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_TILE_MODE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct DOCINFOA {
    pub cbSize: i32,
    pub lpszDocName: ::windows::core::PCSTR,
    pub lpszOutput: ::windows::core::PCSTR,
    pub lpszDatatype: ::windows::core::PCSTR,
    pub fwType: u32,
}
impl ::core::marker::Copy for DOCINFOA {}
impl ::core::clone::Clone for DOCINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOCINFOA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct DOCINFOW {
    pub cbSize: i32,
    pub lpszDocName: ::windows::core::PCWSTR,
    pub lpszOutput: ::windows::core::PCWSTR,
    pub lpszDatatype: ::windows::core::PCWSTR,
    pub fwType: u32,
}
impl ::core::marker::Copy for DOCINFOW {}
impl ::core::clone::Clone for DOCINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOCINFOW {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DRAWPATRECT {
    pub ptPosition: super::super::Foundation::POINT,
    pub ptSize: super::super::Foundation::POINT,
    pub wStyle: u16,
    pub wPattern: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DRAWPATRECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DRAWPATRECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DRAWPATRECT {
    type Abi = Self;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HPTPROVIDER(pub isize);
impl HPTPROVIDER {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HPTPROVIDER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HPTPROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HPTPROVIDER {}
impl ::core::fmt::Debug for HPTPROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HPTPROVIDER").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HPTPROVIDER>> for HPTPROVIDER {
    fn from(optional: ::core::option::Option<HPTPROVIDER>) -> HPTPROVIDER {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HPTPROVIDER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct PSFEATURE_CUSTPAPER {
    pub lOrientation: i32,
    pub lWidth: i32,
    pub lHeight: i32,
    pub lWidthOffset: i32,
    pub lHeightOffset: i32,
}
impl ::core::marker::Copy for PSFEATURE_CUSTPAPER {}
impl ::core::clone::Clone for PSFEATURE_CUSTPAPER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PSFEATURE_CUSTPAPER {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PSFEATURE_OUTPUT {
    pub bPageIndependent: super::super::Foundation::BOOL,
    pub bSetPageDevice: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PSFEATURE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PSFEATURE_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PSFEATURE_OUTPUT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct PSINJECTDATA {
    pub DataBytes: u32,
    pub InjectionPoint: PSINJECT_POINT,
    pub PageNumber: u16,
}
impl ::core::marker::Copy for PSINJECTDATA {}
impl ::core::clone::Clone for PSINJECTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PSINJECTDATA {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_COLOR {
    pub colorType: XPS_COLOR_TYPE,
    pub value: XPS_COLOR_0,
}
impl ::core::marker::Copy for XPS_COLOR {}
impl ::core::clone::Clone for XPS_COLOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_COLOR {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub union XPS_COLOR_0 {
    pub sRGB: XPS_COLOR_0_1,
    pub scRGB: XPS_COLOR_0_2,
    pub context: XPS_COLOR_0_0,
}
impl ::core::marker::Copy for XPS_COLOR_0 {}
impl ::core::clone::Clone for XPS_COLOR_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_COLOR_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_COLOR_0_0 {
    pub channelCount: u8,
    pub channels: [f32; 9],
}
impl ::core::marker::Copy for XPS_COLOR_0_0 {}
impl ::core::clone::Clone for XPS_COLOR_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_COLOR_0_0 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_COLOR_0_1 {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
impl ::core::marker::Copy for XPS_COLOR_0_1 {}
impl ::core::clone::Clone for XPS_COLOR_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_COLOR_0_1 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_COLOR_0_2 {
    pub alpha: f32,
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}
impl ::core::marker::Copy for XPS_COLOR_0_2 {}
impl ::core::clone::Clone for XPS_COLOR_0_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_COLOR_0_2 {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_DASH {
    pub length: f32,
    pub gap: f32,
}
impl ::core::marker::Copy for XPS_DASH {}
impl ::core::clone::Clone for XPS_DASH {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_DASH {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_GLYPH_INDEX {
    pub index: i32,
    pub advanceWidth: f32,
    pub horizontalOffset: f32,
    pub verticalOffset: f32,
}
impl ::core::marker::Copy for XPS_GLYPH_INDEX {}
impl ::core::clone::Clone for XPS_GLYPH_INDEX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_GLYPH_INDEX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_GLYPH_MAPPING {
    pub unicodeStringStart: u32,
    pub unicodeStringLength: u16,
    pub glyphIndicesStart: u32,
    pub glyphIndicesLength: u16,
}
impl ::core::marker::Copy for XPS_GLYPH_MAPPING {}
impl ::core::clone::Clone for XPS_GLYPH_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_GLYPH_MAPPING {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_MATRIX {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub m31: f32,
    pub m32: f32,
}
impl ::core::marker::Copy for XPS_MATRIX {}
impl ::core::clone::Clone for XPS_MATRIX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_MATRIX {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_POINT {
    pub x: f32,
    pub y: f32,
}
impl ::core::marker::Copy for XPS_POINT {}
impl ::core::clone::Clone for XPS_POINT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_POINT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_RECT {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
impl ::core::marker::Copy for XPS_RECT {}
impl ::core::clone::Clone for XPS_RECT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_RECT {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Xps\"`*"]
pub struct XPS_SIZE {
    pub width: f32,
    pub height: f32,
}
impl ::core::marker::Copy for XPS_SIZE {}
impl ::core::clone::Clone for XPS_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for XPS_SIZE {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_Storage_Xps\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type ABORTPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Graphics::Gdi::HDC, param1: i32) -> super::super::Foundation::BOOL>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
#[cfg(feature = "default")]
::core::include!("default.rs");
