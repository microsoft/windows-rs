#[cfg(feature = "Win32_Storage_Xps_Printing")]
#[doc = "Required features: `\"Win32_Storage_Xps_Printing\"`"]
pub mod Printing;
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn AbortDoc<P0>(hdc: P0) -> i32
where
    P0: ::windows_core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows_targets::link!("gdi32.dll" "system" fn AbortDoc(hdc : super::super::Graphics::Gdi:: HDC) -> i32);
    AbortDoc(hdc.into_param().abi())
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DeviceCapabilitiesA<P0, P1>(pdevice: P0, pport: P1, fwcapability: PRINTER_DEVICE_CAPABILITIES, poutput: ::windows_core::PSTR, pdevmode: ::core::option::Option<*const super::super::Graphics::Gdi::DEVMODEA>) -> i32
where
    P0: ::windows_core::IntoParam<::windows_core::PCSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCSTR>,
{
    ::windows_targets::link!("winspool.drv" "system" fn DeviceCapabilitiesA(pdevice : ::windows_core::PCSTR, pport : ::windows_core::PCSTR, fwcapability : PRINTER_DEVICE_CAPABILITIES, poutput : ::windows_core::PSTR, pdevmode : *const super::super::Graphics::Gdi:: DEVMODEA) -> i32);
    DeviceCapabilitiesA(pdevice.into_param().abi(), pport.into_param().abi(), fwcapability, ::core::mem::transmute(poutput), ::core::mem::transmute(pdevmode.unwrap_or(::std::ptr::null())))
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DeviceCapabilitiesW<P0, P1>(pdevice: P0, pport: P1, fwcapability: PRINTER_DEVICE_CAPABILITIES, poutput: ::windows_core::PWSTR, pdevmode: ::core::option::Option<*const super::super::Graphics::Gdi::DEVMODEW>) -> i32
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("winspool.drv" "system" fn DeviceCapabilitiesW(pdevice : ::windows_core::PCWSTR, pport : ::windows_core::PCWSTR, fwcapability : PRINTER_DEVICE_CAPABILITIES, poutput : ::windows_core::PWSTR, pdevmode : *const super::super::Graphics::Gdi:: DEVMODEW) -> i32);
    DeviceCapabilitiesW(pdevice.into_param().abi(), pport.into_param().abi(), fwcapability, ::core::mem::transmute(poutput), ::core::mem::transmute(pdevmode.unwrap_or(::std::ptr::null())))
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EndDoc<P0>(hdc: P0) -> i32
where
    P0: ::windows_core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows_targets::link!("gdi32.dll" "system" fn EndDoc(hdc : super::super::Graphics::Gdi:: HDC) -> i32);
    EndDoc(hdc.into_param().abi())
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn EndPage<P0>(hdc: P0) -> i32
where
    P0: ::windows_core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows_targets::link!("gdi32.dll" "system" fn EndPage(hdc : super::super::Graphics::Gdi:: HDC) -> i32);
    EndPage(hdc.into_param().abi())
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn Escape<P0>(hdc: P0, iescape: i32, pvin: ::core::option::Option<&[u8]>, pvout: ::core::option::Option<*mut ::core::ffi::c_void>) -> i32
where
    P0: ::windows_core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows_targets::link!("gdi32.dll" "system" fn Escape(hdc : super::super::Graphics::Gdi:: HDC, iescape : i32, cjin : i32, pvin : ::windows_core::PCSTR, pvout : *mut ::core::ffi::c_void) -> i32);
    Escape(hdc.into_param().abi(), iescape, pvin.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(pvin.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(pvout.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn ExtEscape<P0>(hdc: P0, iescape: i32, lpindata: ::core::option::Option<&[u8]>, lpoutdata: ::core::option::Option<&mut [u8]>) -> i32
where
    P0: ::windows_core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows_targets::link!("gdi32.dll" "system" fn ExtEscape(hdc : super::super::Graphics::Gdi:: HDC, iescape : i32, cjinput : i32, lpindata : ::windows_core::PCSTR, cjoutput : i32, lpoutdata : ::windows_core::PSTR) -> i32);
    ExtEscape(hdc.into_param().abi(), iescape, lpindata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(lpindata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), lpoutdata.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(lpoutdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn PrintWindow<P0, P1>(hwnd: P0, hdcblt: P1, nflags: PRINT_WINDOW_FLAGS) -> super::super::Foundation::BOOL
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HWND>,
    P1: ::windows_core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows_targets::link!("user32.dll" "system" fn PrintWindow(hwnd : super::super::Foundation:: HWND, hdcblt : super::super::Graphics::Gdi:: HDC, nflags : PRINT_WINDOW_FLAGS) -> super::super::Foundation:: BOOL);
    PrintWindow(hwnd.into_param().abi(), hdcblt.into_param().abi(), nflags)
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn SetAbortProc<P0>(hdc: P0, proc: ABORTPROC) -> i32
where
    P0: ::windows_core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows_targets::link!("gdi32.dll" "system" fn SetAbortProc(hdc : super::super::Graphics::Gdi:: HDC, proc : ABORTPROC) -> i32);
    SetAbortProc(hdc.into_param().abi(), proc)
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn StartDocA<P0>(hdc: P0, lpdi: *const DOCINFOA) -> i32
where
    P0: ::windows_core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows_targets::link!("gdi32.dll" "system" fn StartDocA(hdc : super::super::Graphics::Gdi:: HDC, lpdi : *const DOCINFOA) -> i32);
    StartDocA(hdc.into_param().abi(), lpdi)
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn StartDocW<P0>(hdc: P0, lpdi: *const DOCINFOW) -> i32
where
    P0: ::windows_core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows_targets::link!("gdi32.dll" "system" fn StartDocW(hdc : super::super::Graphics::Gdi:: HDC, lpdi : *const DOCINFOW) -> i32);
    StartDocW(hdc.into_param().abi(), lpdi)
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[inline]
pub unsafe fn StartPage<P0>(hdc: P0) -> i32
where
    P0: ::windows_core::IntoParam<super::super::Graphics::Gdi::HDC>,
{
    ::windows_targets::link!("gdi32.dll" "system" fn StartPage(hdc : super::super::Graphics::Gdi:: HDC) -> i32);
    StartPage(hdc.into_param().abi())
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsDocumentPackageTarget(::windows_core::IUnknown);
impl IXpsDocumentPackageTarget {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetXpsOMPackageWriter<P0, P1>(&self, documentsequencepartname: P0, discardcontrolpartname: P1) -> ::windows_core::Result<IXpsOMPackageWriter>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetXpsOMPackageWriter)(::windows_core::Interface::as_raw(self), documentsequencepartname.into_param().abi(), discardcontrolpartname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetXpsOMFactory(&self) -> ::windows_core::Result<IXpsOMObjectFactory> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetXpsOMFactory)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetXpsType(&self) -> ::windows_core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetXpsType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsDocumentPackageTarget, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsDocumentPackageTarget {
    type Vtable = IXpsDocumentPackageTarget_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsDocumentPackageTarget {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3b0b6d38_53ad_41da_b212_d37637a6714e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsDocumentPackageTarget_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetXpsOMPackageWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequencepartname: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetXpsOMPackageWriter: usize,
    pub GetXpsOMFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpsfactory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetXpsType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsDocumentPackageTarget3D(::windows_core::IUnknown);
impl IXpsDocumentPackageTarget3D {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetXpsOMPackageWriter3D<P0, P1, P2, P3>(&self, documentsequencepartname: P0, discardcontrolpartname: P1, modelpartname: P2, modeldata: P3) -> ::windows_core::Result<IXpsOMPackageWriter3D>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P2: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P3: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetXpsOMPackageWriter3D)(::windows_core::Interface::as_raw(self), documentsequencepartname.into_param().abi(), discardcontrolpartname.into_param().abi(), modelpartname.into_param().abi(), modeldata.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetXpsOMFactory(&self) -> ::windows_core::Result<IXpsOMObjectFactory> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetXpsOMFactory)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsDocumentPackageTarget3D, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsDocumentPackageTarget3D {
    type Vtable = IXpsDocumentPackageTarget3D_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsDocumentPackageTarget3D {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x60ba71b8_3101_4984_9199_f4ea775ff01d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsDocumentPackageTarget3D_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetXpsOMPackageWriter3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequencepartname: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, modelpartname: *mut ::core::ffi::c_void, modeldata: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetXpsOMPackageWriter3D: usize,
    pub GetXpsOMFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpsfactory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMBrush(::windows_core::IUnknown);
impl IXpsOMBrush {
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOpacity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOpacity)(::windows_core::Interface::as_raw(self), opacity).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMBrush, ::windows_core::IUnknown, IXpsOMShareable);
unsafe impl ::windows_core::Interface for IXpsOMBrush {
    type Vtable = IXpsOMBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMBrush {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x56a3f80c_ea4c_4187_a57b_a2a473b2b42b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMBrush_Vtbl {
    pub base__: IXpsOMShareable_Vtbl,
    pub GetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows_core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMCanvas(::windows_core::IUnknown);
impl IXpsOMCanvas {
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransform(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransform)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransformLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, matrixtransform: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMMatrixTransform>,
    {
        (::windows_core::Interface::vtable(self).base__.SetTransformLocal)(::windows_core::Interface::as_raw(self), matrixtransform.into_param().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransformLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetTransformLookup)(::windows_core::Interface::as_raw(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetClipGeometry(&self) -> ::windows_core::Result<IXpsOMGeometry> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetClipGeometry)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetClipGeometryLocal(&self) -> ::windows_core::Result<IXpsOMGeometry> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetClipGeometryLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetClipGeometryLocal<P0>(&self, clipgeometry: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMGeometry>,
    {
        (::windows_core::Interface::vtable(self).base__.SetClipGeometryLocal)(::windows_core::Interface::as_raw(self), clipgeometry.into_param().abi()).ok()
    }
    pub unsafe fn GetClipGeometryLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetClipGeometryLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetClipGeometryLookup<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetClipGeometryLookup)(::windows_core::Interface::as_raw(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetOpacity(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOpacity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOpacity)(::windows_core::Interface::as_raw(self), opacity).ok()
    }
    pub unsafe fn GetOpacityMaskBrush(&self) -> ::windows_core::Result<IXpsOMBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOpacityMaskBrush)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> ::windows_core::Result<IXpsOMBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOpacityMaskBrushLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOpacityMaskBrushLocal<P0>(&self, opacitymaskbrush: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMBrush>,
    {
        (::windows_core::Interface::vtable(self).base__.SetOpacityMaskBrushLocal)(::windows_core::Interface::as_raw(self), opacitymaskbrush.into_param().abi()).ok()
    }
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOpacityMaskBrushLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOpacityMaskBrushLookup<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetOpacityMaskBrushLookup)(::windows_core::Interface::as_raw(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetIsHyperlinkTarget)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<P0>(&self, ishyperlink: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetIsHyperlinkTarget)(::windows_core::Interface::as_raw(self), ishyperlink.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> ::windows_core::Result<super::super::System::Com::IUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetHyperlinkNavigateUri)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHyperlinkNavigateUri<P0>(&self, hyperlinkuri: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IUri>,
    {
        (::windows_core::Interface::vtable(self).base__.SetHyperlinkNavigateUri)(::windows_core::Interface::as_raw(self), hyperlinkuri.into_param().abi()).ok()
    }
    pub unsafe fn GetLanguage(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLanguage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLanguage<P0>(&self, language: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetLanguage)(::windows_core::Interface::as_raw(self), language.into_param().abi()).ok()
    }
    pub unsafe fn GetVisuals(&self) -> ::windows_core::Result<IXpsOMVisualCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetVisuals)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUseAliasedEdgeMode(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetUseAliasedEdgeMode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUseAliasedEdgeMode<P0>(&self, usealiasededgemode: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetUseAliasedEdgeMode)(::windows_core::Interface::as_raw(self), usealiasededgemode.into_param().abi()).ok()
    }
    pub unsafe fn GetAccessibilityShortDescription(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAccessibilityShortDescription)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAccessibilityShortDescription<P0>(&self, shortdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetAccessibilityShortDescription)(::windows_core::Interface::as_raw(self), shortdescription.into_param().abi()).ok()
    }
    pub unsafe fn GetAccessibilityLongDescription(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAccessibilityLongDescription)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAccessibilityLongDescription<P0>(&self, longdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetAccessibilityLongDescription)(::windows_core::Interface::as_raw(self), longdescription.into_param().abi()).ok()
    }
    pub unsafe fn GetDictionary(&self) -> ::windows_core::Result<IXpsOMDictionary> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDictionary)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDictionaryLocal(&self) -> ::windows_core::Result<IXpsOMDictionary> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDictionaryLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDictionaryLocal<P0>(&self, resourcedictionary: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMDictionary>,
    {
        (::windows_core::Interface::vtable(self).SetDictionaryLocal)(::windows_core::Interface::as_raw(self), resourcedictionary.into_param().abi()).ok()
    }
    pub unsafe fn GetDictionaryResource(&self) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDictionaryResource)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDictionaryResource<P0>(&self, remotedictionaryresource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMRemoteDictionaryResource>,
    {
        (::windows_core::Interface::vtable(self).SetDictionaryResource)(::windows_core::Interface::as_raw(self), remotedictionaryresource.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IXpsOMCanvas> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMCanvas, ::windows_core::IUnknown, IXpsOMShareable, IXpsOMVisual);
unsafe impl ::windows_core::Interface for IXpsOMCanvas {
    type Vtable = IXpsOMCanvas_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMCanvas {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x221d1452_331e_47c6_87e9_6ccefb9b5ba3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMCanvas_Vtbl {
    pub base__: IXpsOMVisual_Vtbl,
    pub GetVisuals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visuals: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUseAliasedEdgeMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usealiasededgemode: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUseAliasedEdgeMode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUseAliasedEdgeMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usealiasededgemode: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUseAliasedEdgeMode: usize,
    pub GetAccessibilityShortDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetAccessibilityShortDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetAccessibilityLongDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, longdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetAccessibilityLongDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, longdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetDictionary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDictionaryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDictionaryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDictionaryResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDictionaryResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, canvas: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMColorProfileResource(::windows_core::IUnknown);
impl IXpsOMColorProfileResource {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPartName)(::windows_core::Interface::as_raw(self), parturi.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStream)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, partname: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).SetContent)(::windows_core::Interface::as_raw(self), sourcestream.into_param().abi(), partname.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMColorProfileResource, ::windows_core::IUnknown, IXpsOMPart, IXpsOMResource);
unsafe impl ::windows_core::Interface for IXpsOMColorProfileResource {
    type Vtable = IXpsOMColorProfileResource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMColorProfileResource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x67bd7d69_1eef_4bb1_b5e7_6f4f87be8abe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMColorProfileResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMColorProfileResourceCollection(::windows_core::IUnknown);
impl IXpsOMColorProfileResourceCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsOMColorProfileResource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, object: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMColorProfileResource>,
    {
        (::windows_core::Interface::vtable(self).InsertAt)(::windows_core::Interface::as_raw(self), index, object.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, object: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMColorProfileResource>,
    {
        (::windows_core::Interface::vtable(self).SetAt)(::windows_core::Interface::as_raw(self), index, object.into_param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, object: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMColorProfileResource>,
    {
        (::windows_core::Interface::vtable(self).Append)(::windows_core::Interface::as_raw(self), object.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<P0>(&self, partname: P0) -> ::windows_core::Result<IXpsOMColorProfileResource>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetByPartName)(::windows_core::Interface::as_raw(self), partname.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMColorProfileResourceCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMColorProfileResourceCollection {
    type Vtable = IXpsOMColorProfileResourceCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMColorProfileResourceCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x12759630_5fba_4283_8f7d_cca849809edb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMColorProfileResourceCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMCoreProperties(::windows_core::IUnknown);
impl IXpsOMCoreProperties {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPartName)(::windows_core::Interface::as_raw(self), parturi.into_param().abi()).ok()
    }
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<IXpsOMPackage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCategory(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCategory)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCategory<P0>(&self, category: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetCategory)(::windows_core::Interface::as_raw(self), category.into_param().abi()).ok()
    }
    pub unsafe fn GetContentStatus(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetContentStatus)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetContentStatus<P0>(&self, contentstatus: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetContentStatus)(::windows_core::Interface::as_raw(self), contentstatus.into_param().abi()).ok()
    }
    pub unsafe fn GetContentType(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetContentType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetContentType<P0>(&self, contenttype: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetContentType)(::windows_core::Interface::as_raw(self), contenttype.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCreated(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCreated)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCreated(&self, created: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCreated)(::windows_core::Interface::as_raw(self), created).ok()
    }
    pub unsafe fn GetCreator(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCreator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCreator<P0>(&self, creator: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetCreator)(::windows_core::Interface::as_raw(self), creator.into_param().abi()).ok()
    }
    pub unsafe fn GetDescription(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDescription)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDescription<P0>(&self, description: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDescription)(::windows_core::Interface::as_raw(self), description.into_param().abi()).ok()
    }
    pub unsafe fn GetIdentifier(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetIdentifier)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIdentifier<P0>(&self, identifier: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetIdentifier)(::windows_core::Interface::as_raw(self), identifier.into_param().abi()).ok()
    }
    pub unsafe fn GetKeywords(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetKeywords)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetKeywords<P0>(&self, keywords: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetKeywords)(::windows_core::Interface::as_raw(self), keywords.into_param().abi()).ok()
    }
    pub unsafe fn GetLanguage(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLanguage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLanguage<P0>(&self, language: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLanguage)(::windows_core::Interface::as_raw(self), language.into_param().abi()).ok()
    }
    pub unsafe fn GetLastModifiedBy(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLastModifiedBy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLastModifiedBy<P0>(&self, lastmodifiedby: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLastModifiedBy)(::windows_core::Interface::as_raw(self), lastmodifiedby.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLastPrinted(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLastPrinted)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLastPrinted(&self, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLastPrinted)(::windows_core::Interface::as_raw(self), lastprinted).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetModified(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetModified)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetModified(&self, modified: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetModified)(::windows_core::Interface::as_raw(self), modified).ok()
    }
    pub unsafe fn GetRevision(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRevision)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRevision<P0>(&self, revision: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetRevision)(::windows_core::Interface::as_raw(self), revision.into_param().abi()).ok()
    }
    pub unsafe fn GetSubject(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSubject)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSubject<P0>(&self, subject: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSubject)(::windows_core::Interface::as_raw(self), subject.into_param().abi()).ok()
    }
    pub unsafe fn GetTitle(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTitle)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTitle<P0>(&self, title: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTitle)(::windows_core::Interface::as_raw(self), title.into_param().abi()).ok()
    }
    pub unsafe fn GetVersion(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetVersion)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetVersion<P0>(&self, version: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetVersion)(::windows_core::Interface::as_raw(self), version.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IXpsOMCoreProperties> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMCoreProperties, ::windows_core::IUnknown, IXpsOMPart);
unsafe impl ::windows_core::Interface for IXpsOMCoreProperties {
    type Vtable = IXpsOMCoreProperties_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMCoreProperties {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3340fe8f_4027_4aa1_8f5f_d35ae45fe597);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMCoreProperties_Vtbl {
    pub base__: IXpsOMPart_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetCategory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, category: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetContentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentstatus: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetContentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentstatus: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, created: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCreated: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, created: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCreated: usize,
    pub GetCreator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, creator: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetCreator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, creator: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, description: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, identifier: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetKeywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keywords: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetKeywords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, keywords: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetLastModifiedBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodifiedby: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetLastModifiedBy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastmodifiedby: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLastPrinted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastprinted: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLastPrinted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetLastPrinted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetLastPrinted: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modified: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetModified: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetModified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, modified: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetModified: usize,
    pub GetRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, revision: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetRevision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, revision: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subject: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetSubject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subject: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coreproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMDashCollection(::windows_core::IUnknown);
impl IXpsOMDashCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows_core::Result<XPS_DASH> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn InsertAt(&self, index: u32, dash: *const XPS_DASH) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).InsertAt)(::windows_core::Interface::as_raw(self), index, dash).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt(&self, index: u32, dash: *const XPS_DASH) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAt)(::windows_core::Interface::as_raw(self), index, dash).ok()
    }
    pub unsafe fn Append(&self, dash: *const XPS_DASH) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Append)(::windows_core::Interface::as_raw(self), dash).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMDashCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMDashCollection {
    type Vtable = IXpsOMDashCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMDashCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x081613f4_74eb_48f2_83b3_37a9ce2d7dc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDashCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, dash: *mut XPS_DASH) -> ::windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> ::windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dash: *const XPS_DASH) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMDictionary(::windows_core::IUnknown);
impl IXpsOMDictionary {
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32, key: *mut ::windows_core::PWSTR, entry: *mut ::core::option::Option<IXpsOMShareable>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), index, key, ::core::mem::transmute(entry)).ok()
    }
    pub unsafe fn GetByKey<P0, P1>(&self, key: P0, beforeentry: P1) -> ::windows_core::Result<IXpsOMShareable>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IXpsOMShareable>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetByKey)(::windows_core::Interface::as_raw(self), key.into_param().abi(), beforeentry.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIndex<P0>(&self, entry: P0) -> ::windows_core::Result<u32>
    where
        P0: ::windows_core::IntoParam<IXpsOMShareable>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetIndex)(::windows_core::Interface::as_raw(self), entry.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Append<P0, P1>(&self, key: P0, entry: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IXpsOMShareable>,
    {
        (::windows_core::Interface::vtable(self).Append)(::windows_core::Interface::as_raw(self), key.into_param().abi(), entry.into_param().abi()).ok()
    }
    pub unsafe fn InsertAt<P0, P1>(&self, index: u32, key: P0, entry: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IXpsOMShareable>,
    {
        (::windows_core::Interface::vtable(self).InsertAt)(::windows_core::Interface::as_raw(self), index, key.into_param().abi(), entry.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0, P1>(&self, index: u32, key: P0, entry: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<IXpsOMShareable>,
    {
        (::windows_core::Interface::vtable(self).SetAt)(::windows_core::Interface::as_raw(self), index, key.into_param().abi(), entry.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IXpsOMDictionary> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMDictionary, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMDictionary {
    type Vtable = IXpsOMDictionary_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMDictionary {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x897c86b8_8eaf_4ae3_bdde_56419fcf4236);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDictionary_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, key: *mut ::windows_core::PWSTR, entry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetByKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR, beforeentry: *mut ::core::ffi::c_void, entry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, entry: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR, entry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, key: ::windows_core::PCWSTR, entry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, key: ::windows_core::PCWSTR, entry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMDocument(::windows_core::IUnknown);
impl IXpsOMDocument {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPartName)(::windows_core::Interface::as_raw(self), parturi.into_param().abi()).ok()
    }
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<IXpsOMDocumentSequence> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPageReferences(&self) -> ::windows_core::Result<IXpsOMPageReferenceCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPageReferences)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPrintTicketResource(&self) -> ::windows_core::Result<IXpsOMPrintTicketResource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPrintTicketResource)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPrintTicketResource<P0>(&self, printticketresource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMPrintTicketResource>,
    {
        (::windows_core::Interface::vtable(self).SetPrintTicketResource)(::windows_core::Interface::as_raw(self), printticketresource.into_param().abi()).ok()
    }
    pub unsafe fn GetDocumentStructureResource(&self) -> ::windows_core::Result<IXpsOMDocumentStructureResource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDocumentStructureResource)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDocumentStructureResource<P0>(&self, documentstructureresource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMDocumentStructureResource>,
    {
        (::windows_core::Interface::vtable(self).SetDocumentStructureResource)(::windows_core::Interface::as_raw(self), documentstructureresource.into_param().abi()).ok()
    }
    pub unsafe fn GetSignatureBlockResources(&self) -> ::windows_core::Result<IXpsOMSignatureBlockResourceCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignatureBlockResources)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IXpsOMDocument> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMDocument, ::windows_core::IUnknown, IXpsOMPart);
unsafe impl ::windows_core::Interface for IXpsOMDocument {
    type Vtable = IXpsOMDocument_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMDocument {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2c2c94cb_ac5f_4254_8ee9_23948309d9f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocument_Vtbl {
    pub base__: IXpsOMPart_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequence: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPageReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagereferences: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetPrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDocumentStructureResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentstructureresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDocumentStructureResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentstructureresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSignatureBlockResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureblockresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMDocumentCollection(::windows_core::IUnknown);
impl IXpsOMDocumentCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsOMDocument> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, document: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMDocument>,
    {
        (::windows_core::Interface::vtable(self).InsertAt)(::windows_core::Interface::as_raw(self), index, document.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, document: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMDocument>,
    {
        (::windows_core::Interface::vtable(self).SetAt)(::windows_core::Interface::as_raw(self), index, document.into_param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, document: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMDocument>,
    {
        (::windows_core::Interface::vtable(self).Append)(::windows_core::Interface::as_raw(self), document.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMDocumentCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMDocumentCollection {
    type Vtable = IXpsOMDocumentCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMDocumentCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd1c87f0d_e947_4754_8a25_971478f7e83e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocumentCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, document: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, document: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, document: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMDocumentSequence(::windows_core::IUnknown);
impl IXpsOMDocumentSequence {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPartName)(::windows_core::Interface::as_raw(self), parturi.into_param().abi()).ok()
    }
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<IXpsOMPackage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocuments(&self) -> ::windows_core::Result<IXpsOMDocumentCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDocuments)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPrintTicketResource(&self) -> ::windows_core::Result<IXpsOMPrintTicketResource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPrintTicketResource)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPrintTicketResource<P0>(&self, printticketresource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMPrintTicketResource>,
    {
        (::windows_core::Interface::vtable(self).SetPrintTicketResource)(::windows_core::Interface::as_raw(self), printticketresource.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMDocumentSequence, ::windows_core::IUnknown, IXpsOMPart);
unsafe impl ::windows_core::Interface for IXpsOMDocumentSequence {
    type Vtable = IXpsOMDocumentSequence_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMDocumentSequence {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x56492eb4_d8d5_425e_8256_4c2b64ad0264);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocumentSequence_Vtbl {
    pub base__: IXpsOMPart_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDocuments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documents: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetPrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMDocumentStructureResource(::windows_core::IUnknown);
impl IXpsOMDocumentStructureResource {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPartName)(::windows_core::Interface::as_raw(self), parturi.into_param().abi()).ok()
    }
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<IXpsOMDocument> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStream)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, partname: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).SetContent)(::windows_core::Interface::as_raw(self), sourcestream.into_param().abi(), partname.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMDocumentStructureResource, ::windows_core::IUnknown, IXpsOMPart, IXpsOMResource);
unsafe impl ::windows_core::Interface for IXpsOMDocumentStructureResource {
    type Vtable = IXpsOMDocumentStructureResource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMDocumentStructureResource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x85febc8a_6b63_48a9_af07_7064e4ecff30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMDocumentStructureResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMFontResource(::windows_core::IUnknown);
impl IXpsOMFontResource {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPartName)(::windows_core::Interface::as_raw(self), parturi.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStream)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, embeddingoption: XPS_FONT_EMBEDDING, partname: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).SetContent)(::windows_core::Interface::as_raw(self), sourcestream.into_param().abi(), embeddingoption, partname.into_param().abi()).ok()
    }
    pub unsafe fn GetEmbeddingOption(&self) -> ::windows_core::Result<XPS_FONT_EMBEDDING> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEmbeddingOption)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMFontResource, ::windows_core::IUnknown, IXpsOMPart, IXpsOMResource);
unsafe impl ::windows_core::Interface for IXpsOMFontResource {
    type Vtable = IXpsOMFontResource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMFontResource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa8c45708_47d9_4af4_8d20_33b48c9b8485);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMFontResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readerstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, embeddingoption: XPS_FONT_EMBEDDING, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
    pub GetEmbeddingOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, embeddingoption: *mut XPS_FONT_EMBEDDING) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMFontResourceCollection(::windows_core::IUnknown);
impl IXpsOMFontResourceCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsOMFontResource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMFontResource>,
    {
        (::windows_core::Interface::vtable(self).SetAt)(::windows_core::Interface::as_raw(self), index, value.into_param().abi()).ok()
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMFontResource>,
    {
        (::windows_core::Interface::vtable(self).InsertAt)(::windows_core::Interface::as_raw(self), index, value.into_param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMFontResource>,
    {
        (::windows_core::Interface::vtable(self).Append)(::windows_core::Interface::as_raw(self), value.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), index).ok()
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<P0>(&self, partname: P0) -> ::windows_core::Result<IXpsOMFontResource>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetByPartName)(::windows_core::Interface::as_raw(self), partname.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMFontResourceCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMFontResourceCollection {
    type Vtable = IXpsOMFontResourceCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMFontResourceCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x70b4a6bb_88d4_4fa8_aaf9_6d9c596fdbad);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMFontResourceCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMGeometry(::windows_core::IUnknown);
impl IXpsOMGeometry {
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFigures(&self) -> ::windows_core::Result<IXpsOMGeometryFigureCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFigures)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFillRule(&self) -> ::windows_core::Result<XPS_FILL_RULE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFillRule)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFillRule(&self, fillrule: XPS_FILL_RULE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFillRule)(::windows_core::Interface::as_raw(self), fillrule).ok()
    }
    pub unsafe fn GetTransform(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTransform)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTransformLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, transform: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMMatrixTransform>,
    {
        (::windows_core::Interface::vtable(self).SetTransformLocal)(::windows_core::Interface::as_raw(self), transform.into_param().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTransformLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, lookup: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTransformLookup)(::windows_core::Interface::as_raw(self), lookup.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IXpsOMGeometry> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMGeometry, ::windows_core::IUnknown, IXpsOMShareable);
unsafe impl ::windows_core::Interface for IXpsOMGeometry {
    type Vtable = IXpsOMGeometry_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMGeometry {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x64fcf3d7_4d58_44ba_ad73_a13af6492072);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGeometry_Vtbl {
    pub base__: IXpsOMShareable_Vtbl,
    pub GetFigures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, figures: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFillRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillrule: *mut XPS_FILL_RULE) -> ::windows_core::HRESULT,
    pub SetFillRule: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillrule: XPS_FILL_RULE) -> ::windows_core::HRESULT,
    pub GetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMGeometryFigure(::windows_core::IUnknown);
impl IXpsOMGeometryFigure {
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<IXpsOMGeometry> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSegmentData(&self, datacount: *mut u32, segmentdata: *mut f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSegmentData)(::windows_core::Interface::as_raw(self), datacount, segmentdata).ok()
    }
    pub unsafe fn GetSegmentTypes(&self, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSegmentTypes)(::windows_core::Interface::as_raw(self), segmentcount, segmenttypes).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSegmentStrokes(&self, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSegmentStrokes)(::windows_core::Interface::as_raw(self), segmentcount, segmentstrokes).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSegments(&self, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSegments)(::windows_core::Interface::as_raw(self), segmentcount, segmentdatacount, segmenttypes, segmentdata, segmentstrokes).ok()
    }
    pub unsafe fn GetStartPoint(&self) -> ::windows_core::Result<XPS_POINT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStartPoint)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStartPoint(&self, startpoint: *const XPS_POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStartPoint)(::windows_core::Interface::as_raw(self), startpoint).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsClosed(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetIsClosed)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsClosed<P0>(&self, isclosed: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetIsClosed)(::windows_core::Interface::as_raw(self), isclosed.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsFilled(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetIsFilled)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsFilled<P0>(&self, isfilled: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetIsFilled)(::windows_core::Interface::as_raw(self), isfilled.into_param().abi()).ok()
    }
    pub unsafe fn GetSegmentCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSegmentCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSegmentDataCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSegmentDataCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSegmentStrokePattern(&self) -> ::windows_core::Result<XPS_SEGMENT_STROKE_PATTERN> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSegmentStrokePattern)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IXpsOMGeometryFigure> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMGeometryFigure, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMGeometryFigure {
    type Vtable = IXpsOMGeometryFigure_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMGeometryFigure {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd410dc83_908c_443e_8947_b1795d3c165a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGeometryFigure_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSegmentData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datacount: *mut u32, segmentdata: *mut f32) -> ::windows_core::HRESULT,
    pub GetSegmentTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSegmentStrokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSegmentStrokes: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSegments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSegments: usize,
    pub GetStartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *mut XPS_POINT) -> ::windows_core::HRESULT,
    pub SetStartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsClosed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isclosed: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsClosed: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsFilled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isfilled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsFilled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsFilled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isfilled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsFilled: usize,
    pub GetSegmentCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetSegmentDataCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentdatacount: *mut u32) -> ::windows_core::HRESULT,
    pub GetSegmentStrokePattern: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, segmentstrokepattern: *mut XPS_SEGMENT_STROKE_PATTERN) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometryfigure: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMGeometryFigureCollection(::windows_core::IUnknown);
impl IXpsOMGeometryFigureCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsOMGeometryFigure> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, geometryfigure: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMGeometryFigure>,
    {
        (::windows_core::Interface::vtable(self).InsertAt)(::windows_core::Interface::as_raw(self), index, geometryfigure.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, geometryfigure: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMGeometryFigure>,
    {
        (::windows_core::Interface::vtable(self).SetAt)(::windows_core::Interface::as_raw(self), index, geometryfigure.into_param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, geometryfigure: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMGeometryFigure>,
    {
        (::windows_core::Interface::vtable(self).Append)(::windows_core::Interface::as_raw(self), geometryfigure.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMGeometryFigureCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMGeometryFigureCollection {
    type Vtable = IXpsOMGeometryFigureCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMGeometryFigureCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfd48c3f3_a58e_4b5a_8826_1de54abe72b2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGeometryFigureCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometryfigure: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMGlyphs(::windows_core::IUnknown);
impl IXpsOMGlyphs {
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransform(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransform)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransformLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, matrixtransform: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMMatrixTransform>,
    {
        (::windows_core::Interface::vtable(self).base__.SetTransformLocal)(::windows_core::Interface::as_raw(self), matrixtransform.into_param().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransformLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetTransformLookup)(::windows_core::Interface::as_raw(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetClipGeometry(&self) -> ::windows_core::Result<IXpsOMGeometry> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetClipGeometry)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetClipGeometryLocal(&self) -> ::windows_core::Result<IXpsOMGeometry> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetClipGeometryLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetClipGeometryLocal<P0>(&self, clipgeometry: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMGeometry>,
    {
        (::windows_core::Interface::vtable(self).base__.SetClipGeometryLocal)(::windows_core::Interface::as_raw(self), clipgeometry.into_param().abi()).ok()
    }
    pub unsafe fn GetClipGeometryLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetClipGeometryLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetClipGeometryLookup<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetClipGeometryLookup)(::windows_core::Interface::as_raw(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetOpacity(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOpacity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOpacity)(::windows_core::Interface::as_raw(self), opacity).ok()
    }
    pub unsafe fn GetOpacityMaskBrush(&self) -> ::windows_core::Result<IXpsOMBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOpacityMaskBrush)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> ::windows_core::Result<IXpsOMBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOpacityMaskBrushLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOpacityMaskBrushLocal<P0>(&self, opacitymaskbrush: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMBrush>,
    {
        (::windows_core::Interface::vtable(self).base__.SetOpacityMaskBrushLocal)(::windows_core::Interface::as_raw(self), opacitymaskbrush.into_param().abi()).ok()
    }
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOpacityMaskBrushLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOpacityMaskBrushLookup<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetOpacityMaskBrushLookup)(::windows_core::Interface::as_raw(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetIsHyperlinkTarget)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<P0>(&self, ishyperlink: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetIsHyperlinkTarget)(::windows_core::Interface::as_raw(self), ishyperlink.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> ::windows_core::Result<super::super::System::Com::IUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetHyperlinkNavigateUri)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHyperlinkNavigateUri<P0>(&self, hyperlinkuri: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IUri>,
    {
        (::windows_core::Interface::vtable(self).base__.SetHyperlinkNavigateUri)(::windows_core::Interface::as_raw(self), hyperlinkuri.into_param().abi()).ok()
    }
    pub unsafe fn GetLanguage(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLanguage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLanguage<P0>(&self, language: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetLanguage)(::windows_core::Interface::as_raw(self), language.into_param().abi()).ok()
    }
    pub unsafe fn GetUnicodeString(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetUnicodeString)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGlyphIndexCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetGlyphIndexCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGlyphIndices(&self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetGlyphIndices)(::windows_core::Interface::as_raw(self), indexcount, glyphindices).ok()
    }
    pub unsafe fn GetGlyphMappingCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetGlyphMappingCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGlyphMappings(&self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetGlyphMappings)(::windows_core::Interface::as_raw(self), glyphmappingcount, glyphmappings).ok()
    }
    pub unsafe fn GetProhibitedCaretStopCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProhibitedCaretStopCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetProhibitedCaretStops(&self, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProhibitedCaretStops)(::windows_core::Interface::as_raw(self), prohibitedcaretstopcount, prohibitedcaretstops).ok()
    }
    pub unsafe fn GetBidiLevel(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetBidiLevel)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsSideways(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetIsSideways)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDeviceFontName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDeviceFontName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStyleSimulations(&self) -> ::windows_core::Result<XPS_STYLE_SIMULATION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStyleSimulations)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStyleSimulations(&self, stylesimulations: XPS_STYLE_SIMULATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStyleSimulations)(::windows_core::Interface::as_raw(self), stylesimulations).ok()
    }
    pub unsafe fn GetOrigin(&self) -> ::windows_core::Result<XPS_POINT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOrigin)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOrigin(&self, origin: *const XPS_POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOrigin)(::windows_core::Interface::as_raw(self), origin).ok()
    }
    pub unsafe fn GetFontRenderingEmSize(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFontRenderingEmSize)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFontRenderingEmSize(&self, fontrenderingemsize: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFontRenderingEmSize)(::windows_core::Interface::as_raw(self), fontrenderingemsize).ok()
    }
    pub unsafe fn GetFontResource(&self) -> ::windows_core::Result<IXpsOMFontResource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFontResource)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFontResource<P0>(&self, fontresource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMFontResource>,
    {
        (::windows_core::Interface::vtable(self).SetFontResource)(::windows_core::Interface::as_raw(self), fontresource.into_param().abi()).ok()
    }
    pub unsafe fn GetFontFaceIndex(&self) -> ::windows_core::Result<i16> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFontFaceIndex)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFontFaceIndex(&self, fontfaceindex: i16) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFontFaceIndex)(::windows_core::Interface::as_raw(self), fontfaceindex).ok()
    }
    pub unsafe fn GetFillBrush(&self) -> ::windows_core::Result<IXpsOMBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFillBrush)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFillBrushLocal(&self) -> ::windows_core::Result<IXpsOMBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFillBrushLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFillBrushLocal<P0>(&self, fillbrush: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMBrush>,
    {
        (::windows_core::Interface::vtable(self).SetFillBrushLocal)(::windows_core::Interface::as_raw(self), fillbrush.into_param().abi()).ok()
    }
    pub unsafe fn GetFillBrushLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFillBrushLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFillBrushLookup<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetFillBrushLookup)(::windows_core::Interface::as_raw(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetGlyphsEditor(&self) -> ::windows_core::Result<IXpsOMGlyphsEditor> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetGlyphsEditor)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IXpsOMGlyphs> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMGlyphs, ::windows_core::IUnknown, IXpsOMShareable, IXpsOMVisual);
unsafe impl ::windows_core::Interface for IXpsOMGlyphs {
    type Vtable = IXpsOMGlyphs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMGlyphs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x819b3199_0a5a_4b64_bec7_a9e17e780de2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGlyphs_Vtbl {
    pub base__: IXpsOMVisual_Vtbl,
    pub GetUnicodeString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unicodestring: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetGlyphIndexCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetGlyphIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows_core::HRESULT,
    pub GetGlyphMappingCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetGlyphMappings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows_core::HRESULT,
    pub GetProhibitedCaretStopCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetProhibitedCaretStops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows_core::HRESULT,
    pub GetBidiLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bidilevel: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsSideways: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsSideways: usize,
    pub GetDeviceFontName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicefontname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetStyleSimulations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stylesimulations: *mut XPS_STYLE_SIMULATION) -> ::windows_core::HRESULT,
    pub SetStyleSimulations: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stylesimulations: XPS_STYLE_SIMULATION) -> ::windows_core::HRESULT,
    pub GetOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: *mut XPS_POINT) -> ::windows_core::HRESULT,
    pub SetOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: *const XPS_POINT) -> ::windows_core::HRESULT,
    pub GetFontRenderingEmSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontrenderingemsize: *mut f32) -> ::windows_core::HRESULT,
    pub SetFontRenderingEmSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontrenderingemsize: f32) -> ::windows_core::HRESULT,
    pub GetFontResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetFontResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFontFaceIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfaceindex: *mut i16) -> ::windows_core::HRESULT,
    pub SetFontFaceIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontfaceindex: i16) -> ::windows_core::HRESULT,
    pub GetFillBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFillBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetFillBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fillbrush: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFillBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetFillBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetGlyphsEditor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, editor: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMGlyphsEditor(::windows_core::IUnknown);
impl IXpsOMGlyphsEditor {
    pub unsafe fn ApplyEdits(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ApplyEdits)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetUnicodeString(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetUnicodeString)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetUnicodeString<P0>(&self, unicodestring: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetUnicodeString)(::windows_core::Interface::as_raw(self), unicodestring.into_param().abi()).ok()
    }
    pub unsafe fn GetGlyphIndexCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetGlyphIndexCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGlyphIndices(&self, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetGlyphIndices)(::windows_core::Interface::as_raw(self), indexcount, glyphindices).ok()
    }
    pub unsafe fn SetGlyphIndices(&self, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGlyphIndices)(::windows_core::Interface::as_raw(self), indexcount, glyphindices).ok()
    }
    pub unsafe fn GetGlyphMappingCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetGlyphMappingCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGlyphMappings(&self, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetGlyphMappings)(::windows_core::Interface::as_raw(self), glyphmappingcount, glyphmappings).ok()
    }
    pub unsafe fn SetGlyphMappings(&self, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGlyphMappings)(::windows_core::Interface::as_raw(self), glyphmappingcount, glyphmappings).ok()
    }
    pub unsafe fn GetProhibitedCaretStopCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProhibitedCaretStopCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetProhibitedCaretStops(&self, count: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetProhibitedCaretStops)(::windows_core::Interface::as_raw(self), count, prohibitedcaretstops).ok()
    }
    pub unsafe fn SetProhibitedCaretStops(&self, count: u32, prohibitedcaretstops: *const u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetProhibitedCaretStops)(::windows_core::Interface::as_raw(self), count, prohibitedcaretstops).ok()
    }
    pub unsafe fn GetBidiLevel(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetBidiLevel)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBidiLevel(&self, bidilevel: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBidiLevel)(::windows_core::Interface::as_raw(self), bidilevel).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsSideways(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetIsSideways)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsSideways<P0>(&self, issideways: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetIsSideways)(::windows_core::Interface::as_raw(self), issideways.into_param().abi()).ok()
    }
    pub unsafe fn GetDeviceFontName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDeviceFontName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDeviceFontName<P0>(&self, devicefontname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDeviceFontName)(::windows_core::Interface::as_raw(self), devicefontname.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMGlyphsEditor, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMGlyphsEditor {
    type Vtable = IXpsOMGlyphsEditor_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMGlyphsEditor {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa5ab8616_5b16_4b9f_9629_89b323ed7909);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGlyphsEditor_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub ApplyEdits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetUnicodeString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unicodestring: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetUnicodeString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unicodestring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetGlyphIndexCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetGlyphIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows_core::HRESULT,
    pub SetGlyphIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> ::windows_core::HRESULT,
    pub GetGlyphMappingCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetGlyphMappings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows_core::HRESULT,
    pub SetGlyphMappings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> ::windows_core::HRESULT,
    pub GetProhibitedCaretStopCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> ::windows_core::HRESULT,
    pub GetProhibitedCaretStops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows_core::HRESULT,
    pub SetProhibitedCaretStops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, prohibitedcaretstops: *const u32) -> ::windows_core::HRESULT,
    pub GetBidiLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bidilevel: *mut u32) -> ::windows_core::HRESULT,
    pub SetBidiLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bidilevel: u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsSideways: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsSideways: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsSideways: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, issideways: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsSideways: usize,
    pub GetDeviceFontName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicefontname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetDeviceFontName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicefontname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMGradientBrush(::windows_core::IUnknown);
impl IXpsOMGradientBrush {
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOpacity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOpacity)(::windows_core::Interface::as_raw(self), opacity).ok()
    }
    pub unsafe fn GetGradientStops(&self) -> ::windows_core::Result<IXpsOMGradientStopCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetGradientStops)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransform(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTransform)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTransformLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, transform: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMMatrixTransform>,
    {
        (::windows_core::Interface::vtable(self).SetTransformLocal)(::windows_core::Interface::as_raw(self), transform.into_param().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTransformLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTransformLookup)(::windows_core::Interface::as_raw(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetSpreadMethod(&self) -> ::windows_core::Result<XPS_SPREAD_METHOD> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSpreadMethod)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSpreadMethod(&self, spreadmethod: XPS_SPREAD_METHOD) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSpreadMethod)(::windows_core::Interface::as_raw(self), spreadmethod).ok()
    }
    pub unsafe fn GetColorInterpolationMode(&self) -> ::windows_core::Result<XPS_COLOR_INTERPOLATION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetColorInterpolationMode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetColorInterpolationMode(&self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetColorInterpolationMode)(::windows_core::Interface::as_raw(self), colorinterpolationmode).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMGradientBrush, ::windows_core::IUnknown, IXpsOMShareable, IXpsOMBrush);
unsafe impl ::windows_core::Interface for IXpsOMGradientBrush {
    type Vtable = IXpsOMGradientBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMGradientBrush {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xedb59622_61a2_42c3_bace_acf2286c06bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGradientBrush_Vtbl {
    pub base__: IXpsOMBrush_Vtbl,
    pub GetGradientStops: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradientstops: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetSpreadMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spreadmethod: *mut XPS_SPREAD_METHOD) -> ::windows_core::HRESULT,
    pub SetSpreadMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, spreadmethod: XPS_SPREAD_METHOD) -> ::windows_core::HRESULT,
    pub GetColorInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorinterpolationmode: *mut XPS_COLOR_INTERPOLATION) -> ::windows_core::HRESULT,
    pub SetColorInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMGradientStop(::windows_core::IUnknown);
impl IXpsOMGradientStop {
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<IXpsOMGradientBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOffset(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOffset)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOffset(&self, offset: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOffset)(::windows_core::Interface::as_raw(self), offset).ok()
    }
    pub unsafe fn GetColor(&self, color: *mut XPS_COLOR, colorprofile: *mut ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetColor)(::windows_core::Interface::as_raw(self), color, ::core::mem::transmute(colorprofile)).ok()
    }
    pub unsafe fn SetColor<P0>(&self, color: *const XPS_COLOR, colorprofile: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMColorProfileResource>,
    {
        (::windows_core::Interface::vtable(self).SetColor)(::windows_core::Interface::as_raw(self), color, colorprofile.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IXpsOMGradientStop> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMGradientStop, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMGradientStop {
    type Vtable = IXpsOMGradientStop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMGradientStop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5cf4f5cc_3969_49b5_a70a_5550b618fe49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGradientStop_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: *mut f32) -> ::windows_core::HRESULT,
    pub SetOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: f32) -> ::windows_core::HRESULT,
    pub GetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradientstop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMGradientStopCollection(::windows_core::IUnknown);
impl IXpsOMGradientStopCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsOMGradientStop> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, stop: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMGradientStop>,
    {
        (::windows_core::Interface::vtable(self).InsertAt)(::windows_core::Interface::as_raw(self), index, stop.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, stop: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMGradientStop>,
    {
        (::windows_core::Interface::vtable(self).SetAt)(::windows_core::Interface::as_raw(self), index, stop.into_param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, stop: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMGradientStop>,
    {
        (::windows_core::Interface::vtable(self).Append)(::windows_core::Interface::as_raw(self), stop.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMGradientStopCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMGradientStopCollection {
    type Vtable = IXpsOMGradientStopCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMGradientStopCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9174c3a_3cd3_4319_bda4_11a39392ceef);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMGradientStopCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, stop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, stop: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, stop: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stop: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMImageBrush(::windows_core::IUnknown);
impl IXpsOMImageBrush {
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetOpacity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetOpacity)(::windows_core::Interface::as_raw(self), opacity).ok()
    }
    pub unsafe fn GetTransform(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransform)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransformLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, transform: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMMatrixTransform>,
    {
        (::windows_core::Interface::vtable(self).base__.SetTransformLocal)(::windows_core::Interface::as_raw(self), transform.into_param().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransformLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetTransformLookup)(::windows_core::Interface::as_raw(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetViewbox(&self) -> ::windows_core::Result<XPS_RECT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetViewbox)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetViewbox(&self, viewbox: *const XPS_RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetViewbox)(::windows_core::Interface::as_raw(self), viewbox).ok()
    }
    pub unsafe fn GetViewport(&self) -> ::windows_core::Result<XPS_RECT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetViewport)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetViewport(&self, viewport: *const XPS_RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetViewport)(::windows_core::Interface::as_raw(self), viewport).ok()
    }
    pub unsafe fn GetTileMode(&self) -> ::windows_core::Result<XPS_TILE_MODE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTileMode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTileMode(&self, tilemode: XPS_TILE_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetTileMode)(::windows_core::Interface::as_raw(self), tilemode).ok()
    }
    pub unsafe fn GetImageResource(&self) -> ::windows_core::Result<IXpsOMImageResource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetImageResource)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetImageResource<P0>(&self, imageresource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMImageResource>,
    {
        (::windows_core::Interface::vtable(self).SetImageResource)(::windows_core::Interface::as_raw(self), imageresource.into_param().abi()).ok()
    }
    pub unsafe fn GetColorProfileResource(&self) -> ::windows_core::Result<IXpsOMColorProfileResource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetColorProfileResource)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetColorProfileResource<P0>(&self, colorprofileresource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMColorProfileResource>,
    {
        (::windows_core::Interface::vtable(self).SetColorProfileResource)(::windows_core::Interface::as_raw(self), colorprofileresource.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IXpsOMImageBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMImageBrush, ::windows_core::IUnknown, IXpsOMShareable, IXpsOMBrush, IXpsOMTileBrush);
unsafe impl ::windows_core::Interface for IXpsOMImageBrush {
    type Vtable = IXpsOMImageBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMImageBrush {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3df0b466_d382_49ef_8550_dd94c80242e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMImageBrush_Vtbl {
    pub base__: IXpsOMTileBrush_Vtbl,
    pub GetImageResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetImageResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetColorProfileResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorprofileresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetColorProfileResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorprofileresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagebrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMImageResource(::windows_core::IUnknown);
impl IXpsOMImageResource {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPartName)(::windows_core::Interface::as_raw(self), parturi.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStream)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, imagetype: XPS_IMAGE_TYPE, partname: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).SetContent)(::windows_core::Interface::as_raw(self), sourcestream.into_param().abi(), imagetype, partname.into_param().abi()).ok()
    }
    pub unsafe fn GetImageType(&self) -> ::windows_core::Result<XPS_IMAGE_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetImageType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMImageResource, ::windows_core::IUnknown, IXpsOMPart, IXpsOMResource);
unsafe impl ::windows_core::Interface for IXpsOMImageResource {
    type Vtable = IXpsOMImageResource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMImageResource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3db8417d_ae50_485e_9a44_d7758f78a23f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMImageResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, readerstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, imagetype: XPS_IMAGE_TYPE, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
    pub GetImageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imagetype: *mut XPS_IMAGE_TYPE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMImageResourceCollection(::windows_core::IUnknown);
impl IXpsOMImageResourceCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsOMImageResource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, object: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMImageResource>,
    {
        (::windows_core::Interface::vtable(self).InsertAt)(::windows_core::Interface::as_raw(self), index, object.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, object: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMImageResource>,
    {
        (::windows_core::Interface::vtable(self).SetAt)(::windows_core::Interface::as_raw(self), index, object.into_param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, object: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMImageResource>,
    {
        (::windows_core::Interface::vtable(self).Append)(::windows_core::Interface::as_raw(self), object.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<P0>(&self, partname: P0) -> ::windows_core::Result<IXpsOMImageResource>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetByPartName)(::windows_core::Interface::as_raw(self), partname.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMImageResourceCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMImageResourceCollection {
    type Vtable = IXpsOMImageResourceCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMImageResourceCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7a4a1a71_9cde_4b71_b33f_62de843eabfe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMImageResourceCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMLinearGradientBrush(::windows_core::IUnknown);
impl IXpsOMLinearGradientBrush {
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetOpacity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetOpacity)(::windows_core::Interface::as_raw(self), opacity).ok()
    }
    pub unsafe fn GetGradientStops(&self) -> ::windows_core::Result<IXpsOMGradientStopCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetGradientStops)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransform(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransform)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransformLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, transform: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMMatrixTransform>,
    {
        (::windows_core::Interface::vtable(self).base__.SetTransformLocal)(::windows_core::Interface::as_raw(self), transform.into_param().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransformLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetTransformLookup)(::windows_core::Interface::as_raw(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetSpreadMethod(&self) -> ::windows_core::Result<XPS_SPREAD_METHOD> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSpreadMethod)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSpreadMethod(&self, spreadmethod: XPS_SPREAD_METHOD) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSpreadMethod)(::windows_core::Interface::as_raw(self), spreadmethod).ok()
    }
    pub unsafe fn GetColorInterpolationMode(&self) -> ::windows_core::Result<XPS_COLOR_INTERPOLATION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetColorInterpolationMode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetColorInterpolationMode(&self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetColorInterpolationMode)(::windows_core::Interface::as_raw(self), colorinterpolationmode).ok()
    }
    pub unsafe fn GetStartPoint(&self) -> ::windows_core::Result<XPS_POINT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStartPoint)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStartPoint(&self, startpoint: *const XPS_POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStartPoint)(::windows_core::Interface::as_raw(self), startpoint).ok()
    }
    pub unsafe fn GetEndPoint(&self) -> ::windows_core::Result<XPS_POINT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEndPoint)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetEndPoint(&self, endpoint: *const XPS_POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetEndPoint)(::windows_core::Interface::as_raw(self), endpoint).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IXpsOMLinearGradientBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMLinearGradientBrush, ::windows_core::IUnknown, IXpsOMShareable, IXpsOMBrush, IXpsOMGradientBrush);
unsafe impl ::windows_core::Interface for IXpsOMLinearGradientBrush {
    type Vtable = IXpsOMLinearGradientBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMLinearGradientBrush {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x005e279f_c30d_40ff_93ec_1950d3c528db);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMLinearGradientBrush_Vtbl {
    pub base__: IXpsOMGradientBrush_Vtbl,
    pub GetStartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *mut XPS_POINT) -> ::windows_core::HRESULT,
    pub SetStartPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT) -> ::windows_core::HRESULT,
    pub GetEndPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: *mut XPS_POINT) -> ::windows_core::HRESULT,
    pub SetEndPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: *const XPS_POINT) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lineargradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMMatrixTransform(::windows_core::IUnknown);
impl IXpsOMMatrixTransform {
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMatrix(&self) -> ::windows_core::Result<XPS_MATRIX> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetMatrix)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMatrix(&self, matrix: *const XPS_MATRIX) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetMatrix)(::windows_core::Interface::as_raw(self), matrix).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMMatrixTransform, ::windows_core::IUnknown, IXpsOMShareable);
unsafe impl ::windows_core::Interface for IXpsOMMatrixTransform {
    type Vtable = IXpsOMMatrixTransform_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMMatrixTransform {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb77330ff_bb37_4501_a93e_f1b1e50bfc46);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMMatrixTransform_Vtbl {
    pub base__: IXpsOMShareable_Vtbl,
    pub GetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *mut XPS_MATRIX) -> ::windows_core::HRESULT,
    pub SetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMNameCollection(::windows_core::IUnknown);
impl IXpsOMNameCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMNameCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMNameCollection {
    type Vtable = IXpsOMNameCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMNameCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4bddf8ec_c915_421b_a166_d173d25653d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMNameCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, name: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMObjectFactory(::windows_core::IUnknown);
impl IXpsOMObjectFactory {
    pub unsafe fn CreatePackage(&self) -> ::windows_core::Result<IXpsOMPackage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePackage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePackageFromFile<P0, P1>(&self, filename: P0, reuseobjects: P1) -> ::windows_core::Result<IXpsOMPackage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePackageFromFile)(::windows_core::Interface::as_raw(self), filename.into_param().abi(), reuseobjects.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageFromStream<P0, P1>(&self, stream: P0, reuseobjects: P1) -> ::windows_core::Result<IXpsOMPackage>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePackageFromStream)(::windows_core::Interface::as_raw(self), stream.into_param().abi(), reuseobjects.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateStoryFragmentsResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> ::windows_core::Result<IXpsOMStoryFragmentsResource>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateStoryFragmentsResource)(::windows_core::Interface::as_raw(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocumentStructureResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> ::windows_core::Result<IXpsOMDocumentStructureResource>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateDocumentStructureResource)(::windows_core::Interface::as_raw(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateSignatureBlockResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> ::windows_core::Result<IXpsOMSignatureBlockResource>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSignatureBlockResource)(::windows_core::Interface::as_raw(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResource<P0, P1>(&self, dictionary: P0, parturi: P1) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource>
    where
        P0: ::windows_core::IntoParam<IXpsOMDictionary>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateRemoteDictionaryResource)(::windows_core::Interface::as_raw(self), dictionary.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResourceFromStream<P0, P1, P2>(&self, dictionarymarkupstream: P0, dictionaryparturi: P1, resources: P2) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P2: ::windows_core::IntoParam<IXpsOMPartResources>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateRemoteDictionaryResourceFromStream)(::windows_core::Interface::as_raw(self), dictionarymarkupstream.into_param().abi(), dictionaryparturi.into_param().abi(), resources.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreatePartResources(&self) -> ::windows_core::Result<IXpsOMPartResources> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePartResources)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocumentSequence<P0>(&self, parturi: P0) -> ::windows_core::Result<IXpsOMDocumentSequence>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateDocumentSequence)(::windows_core::Interface::as_raw(self), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocument<P0>(&self, parturi: P0) -> ::windows_core::Result<IXpsOMDocument>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateDocument)(::windows_core::Interface::as_raw(self), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreatePageReference(&self, advisorypagedimensions: *const XPS_SIZE) -> ::windows_core::Result<IXpsOMPageReference> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePageReference)(::windows_core::Interface::as_raw(self), advisorypagedimensions, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePage<P0, P1>(&self, pagedimensions: *const XPS_SIZE, language: P0, parturi: P1) -> ::windows_core::Result<IXpsOMPage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePage)(::windows_core::Interface::as_raw(self), pagedimensions, language.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePageFromStream<P0, P1, P2, P3>(&self, pagemarkupstream: P0, parturi: P1, resources: P2, reuseobjects: P3) -> ::windows_core::Result<IXpsOMPage>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P2: ::windows_core::IntoParam<IXpsOMPartResources>,
        P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePageFromStream)(::windows_core::Interface::as_raw(self), pagemarkupstream.into_param().abi(), parturi.into_param().abi(), resources.into_param().abi(), reuseobjects.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateCanvas(&self) -> ::windows_core::Result<IXpsOMCanvas> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateCanvas)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateGlyphs<P0>(&self, fontresource: P0) -> ::windows_core::Result<IXpsOMGlyphs>
    where
        P0: ::windows_core::IntoParam<IXpsOMFontResource>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateGlyphs)(::windows_core::Interface::as_raw(self), fontresource.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreatePath(&self) -> ::windows_core::Result<IXpsOMPath> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateGeometry(&self) -> ::windows_core::Result<IXpsOMGeometry> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateGeometry)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateGeometryFigure(&self, startpoint: *const XPS_POINT) -> ::windows_core::Result<IXpsOMGeometryFigure> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateGeometryFigure)(::windows_core::Interface::as_raw(self), startpoint, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateMatrixTransform(&self, matrix: *const XPS_MATRIX) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateMatrixTransform)(::windows_core::Interface::as_raw(self), matrix, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSolidColorBrush<P0>(&self, color: *const XPS_COLOR, colorprofile: P0) -> ::windows_core::Result<IXpsOMSolidColorBrush>
    where
        P0: ::windows_core::IntoParam<IXpsOMColorProfileResource>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSolidColorBrush)(::windows_core::Interface::as_raw(self), color, colorprofile.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateColorProfileResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> ::windows_core::Result<IXpsOMColorProfileResource>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateColorProfileResource)(::windows_core::Interface::as_raw(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateImageBrush<P0>(&self, image: P0, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows_core::Result<IXpsOMImageBrush>
    where
        P0: ::windows_core::IntoParam<IXpsOMImageResource>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateImageBrush)(::windows_core::Interface::as_raw(self), image.into_param().abi(), viewbox, viewport, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateVisualBrush(&self, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows_core::Result<IXpsOMVisualBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateVisualBrush)(::windows_core::Interface::as_raw(self), viewbox, viewport, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateImageResource<P0, P1>(&self, acquiredstream: P0, contenttype: XPS_IMAGE_TYPE, parturi: P1) -> ::windows_core::Result<IXpsOMImageResource>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateImageResource)(::windows_core::Interface::as_raw(self), acquiredstream.into_param().abi(), contenttype, parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePrintTicketResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> ::windows_core::Result<IXpsOMPrintTicketResource>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePrintTicketResource)(::windows_core::Interface::as_raw(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateFontResource<P0, P1, P2>(&self, acquiredstream: P0, fontembedding: XPS_FONT_EMBEDDING, parturi: P1, isobfsourcestream: P2) -> ::windows_core::Result<IXpsOMFontResource>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateFontResource)(::windows_core::Interface::as_raw(self), acquiredstream.into_param().abi(), fontembedding, parturi.into_param().abi(), isobfsourcestream.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateGradientStop<P0>(&self, color: *const XPS_COLOR, colorprofile: P0, offset: f32) -> ::windows_core::Result<IXpsOMGradientStop>
    where
        P0: ::windows_core::IntoParam<IXpsOMColorProfileResource>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateGradientStop)(::windows_core::Interface::as_raw(self), color, colorprofile.into_param().abi(), offset, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateLinearGradientBrush<P0, P1>(&self, gradstop1: P0, gradstop2: P1, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT) -> ::windows_core::Result<IXpsOMLinearGradientBrush>
    where
        P0: ::windows_core::IntoParam<IXpsOMGradientStop>,
        P1: ::windows_core::IntoParam<IXpsOMGradientStop>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateLinearGradientBrush)(::windows_core::Interface::as_raw(self), gradstop1.into_param().abi(), gradstop2.into_param().abi(), startpoint, endpoint, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRadialGradientBrush<P0, P1>(&self, gradstop1: P0, gradstop2: P1, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE) -> ::windows_core::Result<IXpsOMRadialGradientBrush>
    where
        P0: ::windows_core::IntoParam<IXpsOMGradientStop>,
        P1: ::windows_core::IntoParam<IXpsOMGradientStop>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateRadialGradientBrush)(::windows_core::Interface::as_raw(self), gradstop1.into_param().abi(), gradstop2.into_param().abi(), centerpoint, gradientorigin, radiisizes, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateCoreProperties<P0>(&self, parturi: P0) -> ::windows_core::Result<IXpsOMCoreProperties>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateCoreProperties)(::windows_core::Interface::as_raw(self), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateDictionary(&self) -> ::windows_core::Result<IXpsOMDictionary> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateDictionary)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreatePartUriCollection(&self) -> ::windows_core::Result<IXpsOMPartUriCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePartUriCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnFile<P0, P1, P2, P3, P4, P5, P6>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: P1, interleaving: XPS_INTERLEAVING, documentsequencepartname: P2, coreproperties: P3, packagethumbnail: P4, documentsequenceprintticket: P5, discardcontrolpartname: P6) -> ::windows_core::Result<IXpsOMPackageWriter>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P3: ::windows_core::IntoParam<IXpsOMCoreProperties>,
        P4: ::windows_core::IntoParam<IXpsOMImageResource>,
        P5: ::windows_core::IntoParam<IXpsOMPrintTicketResource>,
        P6: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePackageWriterOnFile)(::windows_core::Interface::as_raw(self), filename.into_param().abi(), securityattributes, flagsandattributes, optimizemarkupsize.into_param().abi(), interleaving, documentsequencepartname.into_param().abi(), coreproperties.into_param().abi(), packagethumbnail.into_param().abi(), documentsequenceprintticket.into_param().abi(), discardcontrolpartname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnStream<P0, P1, P2, P3, P4, P5, P6>(&self, outputstream: P0, optimizemarkupsize: P1, interleaving: XPS_INTERLEAVING, documentsequencepartname: P2, coreproperties: P3, packagethumbnail: P4, documentsequenceprintticket: P5, discardcontrolpartname: P6) -> ::windows_core::Result<IXpsOMPackageWriter>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::ISequentialStream>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P3: ::windows_core::IntoParam<IXpsOMCoreProperties>,
        P4: ::windows_core::IntoParam<IXpsOMImageResource>,
        P5: ::windows_core::IntoParam<IXpsOMPrintTicketResource>,
        P6: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePackageWriterOnStream)(::windows_core::Interface::as_raw(self), outputstream.into_param().abi(), optimizemarkupsize.into_param().abi(), interleaving, documentsequencepartname.into_param().abi(), coreproperties.into_param().abi(), packagethumbnail.into_param().abi(), documentsequenceprintticket.into_param().abi(), discardcontrolpartname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePartUri<P0>(&self, uri: P0) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePartUri)(::windows_core::Interface::as_raw(self), uri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateReadOnlyStreamOnFile<P0>(&self, filename: P0) -> ::windows_core::Result<super::super::System::Com::IStream>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateReadOnlyStreamOnFile)(::windows_core::Interface::as_raw(self), filename.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMObjectFactory, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMObjectFactory {
    type Vtable = IXpsOMObjectFactory_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMObjectFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf9b2a685_a50d_4fc2_b764_b56e093ea0ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMObjectFactory_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub CreatePackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreatePackageFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreatePackageFromFile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreatePackageFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreatePackageFromStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateStoryFragmentsResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, storyfragmentsresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateStoryFragmentsResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateDocumentStructureResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, documentstructureresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateDocumentStructureResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateSignatureBlockResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, signatureblockresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateSignatureBlockResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateRemoteDictionaryResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, remotedictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateRemoteDictionaryResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateRemoteDictionaryResourceFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionarymarkupstream: *mut ::core::ffi::c_void, dictionaryparturi: *mut ::core::ffi::c_void, resources: *mut ::core::ffi::c_void, dictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateRemoteDictionaryResourceFromStream: usize,
    pub CreatePartResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateDocumentSequence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, documentsequence: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateDocumentSequence: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateDocument: usize,
    pub CreatePageReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, pagereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: ::windows_core::PCWSTR, parturi: *mut ::core::ffi::c_void, page: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePageFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagemarkupstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, resources: *mut ::core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, page: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePageFromStream: usize,
    pub CreateCanvas: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, canvas: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateGlyphs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontresource: *mut ::core::ffi::c_void, glyphs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreatePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateGeometryFigure: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT, figure: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateMatrixTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSolidColorBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut ::core::ffi::c_void, solidcolorbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateColorProfileResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, colorprofileresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateColorProfileResource: usize,
    pub CreateImageBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, imagebrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateVisualBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, visualbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateImageResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, contenttype: XPS_IMAGE_TYPE, parturi: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateImageResource: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, printticketresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePrintTicketResource: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateFontResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, fontembedding: XPS_FONT_EMBEDDING, parturi: *mut ::core::ffi::c_void, isobfsourcestream: super::super::Foundation::BOOL, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateFontResource: usize,
    pub CreateGradientStop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut ::core::ffi::c_void, offset: f32, gradientstop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateLinearGradientBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradstop1: *mut ::core::ffi::c_void, gradstop2: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT, lineargradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateRadialGradientBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gradstop1: *mut ::core::ffi::c_void, gradstop2: *mut ::core::ffi::c_void, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE, radialgradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateCoreProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, coreproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateCoreProperties: usize,
    pub CreateDictionary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreatePartUriCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturicollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void, packagethumbnail: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnFile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void, packagethumbnail: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePartUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows_core::PCWSTR, parturi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePartUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateReadOnlyStreamOnFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateReadOnlyStreamOnFile: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMObjectFactory1(::windows_core::IUnknown);
impl IXpsOMObjectFactory1 {
    pub unsafe fn CreatePackage(&self) -> ::windows_core::Result<IXpsOMPackage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreatePackage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePackageFromFile<P0, P1>(&self, filename: P0, reuseobjects: P1) -> ::windows_core::Result<IXpsOMPackage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreatePackageFromFile)(::windows_core::Interface::as_raw(self), filename.into_param().abi(), reuseobjects.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageFromStream<P0, P1>(&self, stream: P0, reuseobjects: P1) -> ::windows_core::Result<IXpsOMPackage>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreatePackageFromStream)(::windows_core::Interface::as_raw(self), stream.into_param().abi(), reuseobjects.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateStoryFragmentsResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> ::windows_core::Result<IXpsOMStoryFragmentsResource>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateStoryFragmentsResource)(::windows_core::Interface::as_raw(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocumentStructureResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> ::windows_core::Result<IXpsOMDocumentStructureResource>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateDocumentStructureResource)(::windows_core::Interface::as_raw(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateSignatureBlockResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> ::windows_core::Result<IXpsOMSignatureBlockResource>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateSignatureBlockResource)(::windows_core::Interface::as_raw(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResource<P0, P1>(&self, dictionary: P0, parturi: P1) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource>
    where
        P0: ::windows_core::IntoParam<IXpsOMDictionary>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateRemoteDictionaryResource)(::windows_core::Interface::as_raw(self), dictionary.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResourceFromStream<P0, P1, P2>(&self, dictionarymarkupstream: P0, dictionaryparturi: P1, resources: P2) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P2: ::windows_core::IntoParam<IXpsOMPartResources>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateRemoteDictionaryResourceFromStream)(::windows_core::Interface::as_raw(self), dictionarymarkupstream.into_param().abi(), dictionaryparturi.into_param().abi(), resources.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreatePartResources(&self) -> ::windows_core::Result<IXpsOMPartResources> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreatePartResources)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocumentSequence<P0>(&self, parturi: P0) -> ::windows_core::Result<IXpsOMDocumentSequence>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateDocumentSequence)(::windows_core::Interface::as_raw(self), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocument<P0>(&self, parturi: P0) -> ::windows_core::Result<IXpsOMDocument>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateDocument)(::windows_core::Interface::as_raw(self), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreatePageReference(&self, advisorypagedimensions: *const XPS_SIZE) -> ::windows_core::Result<IXpsOMPageReference> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreatePageReference)(::windows_core::Interface::as_raw(self), advisorypagedimensions, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePage<P0, P1>(&self, pagedimensions: *const XPS_SIZE, language: P0, parturi: P1) -> ::windows_core::Result<IXpsOMPage>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreatePage)(::windows_core::Interface::as_raw(self), pagedimensions, language.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePageFromStream<P0, P1, P2, P3>(&self, pagemarkupstream: P0, parturi: P1, resources: P2, reuseobjects: P3) -> ::windows_core::Result<IXpsOMPage>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P2: ::windows_core::IntoParam<IXpsOMPartResources>,
        P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreatePageFromStream)(::windows_core::Interface::as_raw(self), pagemarkupstream.into_param().abi(), parturi.into_param().abi(), resources.into_param().abi(), reuseobjects.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateCanvas(&self) -> ::windows_core::Result<IXpsOMCanvas> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateCanvas)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateGlyphs<P0>(&self, fontresource: P0) -> ::windows_core::Result<IXpsOMGlyphs>
    where
        P0: ::windows_core::IntoParam<IXpsOMFontResource>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateGlyphs)(::windows_core::Interface::as_raw(self), fontresource.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreatePath(&self) -> ::windows_core::Result<IXpsOMPath> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreatePath)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateGeometry(&self) -> ::windows_core::Result<IXpsOMGeometry> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateGeometry)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateGeometryFigure(&self, startpoint: *const XPS_POINT) -> ::windows_core::Result<IXpsOMGeometryFigure> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateGeometryFigure)(::windows_core::Interface::as_raw(self), startpoint, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateMatrixTransform(&self, matrix: *const XPS_MATRIX) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateMatrixTransform)(::windows_core::Interface::as_raw(self), matrix, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSolidColorBrush<P0>(&self, color: *const XPS_COLOR, colorprofile: P0) -> ::windows_core::Result<IXpsOMSolidColorBrush>
    where
        P0: ::windows_core::IntoParam<IXpsOMColorProfileResource>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateSolidColorBrush)(::windows_core::Interface::as_raw(self), color, colorprofile.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateColorProfileResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> ::windows_core::Result<IXpsOMColorProfileResource>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateColorProfileResource)(::windows_core::Interface::as_raw(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateImageBrush<P0>(&self, image: P0, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows_core::Result<IXpsOMImageBrush>
    where
        P0: ::windows_core::IntoParam<IXpsOMImageResource>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateImageBrush)(::windows_core::Interface::as_raw(self), image.into_param().abi(), viewbox, viewport, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateVisualBrush(&self, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows_core::Result<IXpsOMVisualBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateVisualBrush)(::windows_core::Interface::as_raw(self), viewbox, viewport, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateImageResource<P0, P1>(&self, acquiredstream: P0, contenttype: XPS_IMAGE_TYPE, parturi: P1) -> ::windows_core::Result<IXpsOMImageResource>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateImageResource)(::windows_core::Interface::as_raw(self), acquiredstream.into_param().abi(), contenttype, parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePrintTicketResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> ::windows_core::Result<IXpsOMPrintTicketResource>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreatePrintTicketResource)(::windows_core::Interface::as_raw(self), acquiredstream.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateFontResource<P0, P1, P2>(&self, acquiredstream: P0, fontembedding: XPS_FONT_EMBEDDING, parturi: P1, isobfsourcestream: P2) -> ::windows_core::Result<IXpsOMFontResource>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateFontResource)(::windows_core::Interface::as_raw(self), acquiredstream.into_param().abi(), fontembedding, parturi.into_param().abi(), isobfsourcestream.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateGradientStop<P0>(&self, color: *const XPS_COLOR, colorprofile: P0, offset: f32) -> ::windows_core::Result<IXpsOMGradientStop>
    where
        P0: ::windows_core::IntoParam<IXpsOMColorProfileResource>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateGradientStop)(::windows_core::Interface::as_raw(self), color, colorprofile.into_param().abi(), offset, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateLinearGradientBrush<P0, P1>(&self, gradstop1: P0, gradstop2: P1, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT) -> ::windows_core::Result<IXpsOMLinearGradientBrush>
    where
        P0: ::windows_core::IntoParam<IXpsOMGradientStop>,
        P1: ::windows_core::IntoParam<IXpsOMGradientStop>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateLinearGradientBrush)(::windows_core::Interface::as_raw(self), gradstop1.into_param().abi(), gradstop2.into_param().abi(), startpoint, endpoint, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRadialGradientBrush<P0, P1>(&self, gradstop1: P0, gradstop2: P1, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE) -> ::windows_core::Result<IXpsOMRadialGradientBrush>
    where
        P0: ::windows_core::IntoParam<IXpsOMGradientStop>,
        P1: ::windows_core::IntoParam<IXpsOMGradientStop>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateRadialGradientBrush)(::windows_core::Interface::as_raw(self), gradstop1.into_param().abi(), gradstop2.into_param().abi(), centerpoint, gradientorigin, radiisizes, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateCoreProperties<P0>(&self, parturi: P0) -> ::windows_core::Result<IXpsOMCoreProperties>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateCoreProperties)(::windows_core::Interface::as_raw(self), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateDictionary(&self) -> ::windows_core::Result<IXpsOMDictionary> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateDictionary)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreatePartUriCollection(&self) -> ::windows_core::Result<IXpsOMPartUriCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreatePartUriCollection)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnFile<P0, P1, P2, P3, P4, P5, P6>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: P1, interleaving: XPS_INTERLEAVING, documentsequencepartname: P2, coreproperties: P3, packagethumbnail: P4, documentsequenceprintticket: P5, discardcontrolpartname: P6) -> ::windows_core::Result<IXpsOMPackageWriter>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P3: ::windows_core::IntoParam<IXpsOMCoreProperties>,
        P4: ::windows_core::IntoParam<IXpsOMImageResource>,
        P5: ::windows_core::IntoParam<IXpsOMPrintTicketResource>,
        P6: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreatePackageWriterOnFile)(::windows_core::Interface::as_raw(self), filename.into_param().abi(), securityattributes, flagsandattributes, optimizemarkupsize.into_param().abi(), interleaving, documentsequencepartname.into_param().abi(), coreproperties.into_param().abi(), packagethumbnail.into_param().abi(), documentsequenceprintticket.into_param().abi(), discardcontrolpartname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnStream<P0, P1, P2, P3, P4, P5, P6>(&self, outputstream: P0, optimizemarkupsize: P1, interleaving: XPS_INTERLEAVING, documentsequencepartname: P2, coreproperties: P3, packagethumbnail: P4, documentsequenceprintticket: P5, discardcontrolpartname: P6) -> ::windows_core::Result<IXpsOMPackageWriter>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::ISequentialStream>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P3: ::windows_core::IntoParam<IXpsOMCoreProperties>,
        P4: ::windows_core::IntoParam<IXpsOMImageResource>,
        P5: ::windows_core::IntoParam<IXpsOMPrintTicketResource>,
        P6: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreatePackageWriterOnStream)(::windows_core::Interface::as_raw(self), outputstream.into_param().abi(), optimizemarkupsize.into_param().abi(), interleaving, documentsequencepartname.into_param().abi(), coreproperties.into_param().abi(), packagethumbnail.into_param().abi(), documentsequenceprintticket.into_param().abi(), discardcontrolpartname.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePartUri<P0>(&self, uri: P0) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreatePartUri)(::windows_core::Interface::as_raw(self), uri.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateReadOnlyStreamOnFile<P0>(&self, filename: P0) -> ::windows_core::Result<super::super::System::Com::IStream>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.CreateReadOnlyStreamOnFile)(::windows_core::Interface::as_raw(self), filename.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocumentTypeFromFile<P0>(&self, filename: P0) -> ::windows_core::Result<XPS_DOCUMENT_TYPE>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDocumentTypeFromFile)(::windows_core::Interface::as_raw(self), filename.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetDocumentTypeFromStream<P0>(&self, xpsdocumentstream: P0) -> ::windows_core::Result<XPS_DOCUMENT_TYPE>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDocumentTypeFromStream)(::windows_core::Interface::as_raw(self), xpsdocumentstream.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn ConvertHDPhotoToJpegXR<P0>(&self, imageresource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMImageResource>,
    {
        (::windows_core::Interface::vtable(self).ConvertHDPhotoToJpegXR)(::windows_core::Interface::as_raw(self), imageresource.into_param().abi()).ok()
    }
    pub unsafe fn ConvertJpegXRToHDPhoto<P0>(&self, imageresource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMImageResource>,
    {
        (::windows_core::Interface::vtable(self).ConvertJpegXRToHDPhoto)(::windows_core::Interface::as_raw(self), imageresource.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnFile1<P0, P1, P2, P3, P4, P5, P6>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: P1, interleaving: XPS_INTERLEAVING, documentsequencepartname: P2, coreproperties: P3, packagethumbnail: P4, documentsequenceprintticket: P5, discardcontrolpartname: P6, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::Result<IXpsOMPackageWriter>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P3: ::windows_core::IntoParam<IXpsOMCoreProperties>,
        P4: ::windows_core::IntoParam<IXpsOMImageResource>,
        P5: ::windows_core::IntoParam<IXpsOMPrintTicketResource>,
        P6: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePackageWriterOnFile1)(::windows_core::Interface::as_raw(self), filename.into_param().abi(), securityattributes, flagsandattributes, optimizemarkupsize.into_param().abi(), interleaving, documentsequencepartname.into_param().abi(), coreproperties.into_param().abi(), packagethumbnail.into_param().abi(), documentsequenceprintticket.into_param().abi(), discardcontrolpartname.into_param().abi(), documenttype, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnStream1<P0, P1, P2, P3, P4, P5, P6>(&self, outputstream: P0, optimizemarkupsize: P1, interleaving: XPS_INTERLEAVING, documentsequencepartname: P2, coreproperties: P3, packagethumbnail: P4, documentsequenceprintticket: P5, discardcontrolpartname: P6, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::Result<IXpsOMPackageWriter>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::ISequentialStream>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P3: ::windows_core::IntoParam<IXpsOMCoreProperties>,
        P4: ::windows_core::IntoParam<IXpsOMImageResource>,
        P5: ::windows_core::IntoParam<IXpsOMPrintTicketResource>,
        P6: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePackageWriterOnStream1)(::windows_core::Interface::as_raw(self), outputstream.into_param().abi(), optimizemarkupsize.into_param().abi(), interleaving, documentsequencepartname.into_param().abi(), coreproperties.into_param().abi(), packagethumbnail.into_param().abi(), documentsequenceprintticket.into_param().abi(), discardcontrolpartname.into_param().abi(), documenttype, &mut result__).from_abi(result__)
    }
    pub unsafe fn CreatePackage1(&self) -> ::windows_core::Result<IXpsOMPackage1> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePackage1)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageFromStream1<P0, P1>(&self, stream: P0, reuseobjects: P1) -> ::windows_core::Result<IXpsOMPackage1>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePackageFromStream1)(::windows_core::Interface::as_raw(self), stream.into_param().abi(), reuseobjects.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePackageFromFile1<P0, P1>(&self, filename: P0, reuseobjects: P1) -> ::windows_core::Result<IXpsOMPackage1>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePackageFromFile1)(::windows_core::Interface::as_raw(self), filename.into_param().abi(), reuseobjects.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePage1<P0, P1>(&self, pagedimensions: *const XPS_SIZE, language: P0, parturi: P1) -> ::windows_core::Result<IXpsOMPage1>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePage1)(::windows_core::Interface::as_raw(self), pagedimensions, language.into_param().abi(), parturi.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePageFromStream1<P0, P1, P2, P3>(&self, pagemarkupstream: P0, parturi: P1, resources: P2, reuseobjects: P3) -> ::windows_core::Result<IXpsOMPage1>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P2: ::windows_core::IntoParam<IXpsOMPartResources>,
        P3: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePageFromStream1)(::windows_core::Interface::as_raw(self), pagemarkupstream.into_param().abi(), parturi.into_param().abi(), resources.into_param().abi(), reuseobjects.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResourceFromStream1<P0, P1, P2>(&self, dictionarymarkupstream: P0, parturi: P1, resources: P2) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P2: ::windows_core::IntoParam<IXpsOMPartResources>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateRemoteDictionaryResourceFromStream1)(::windows_core::Interface::as_raw(self), dictionarymarkupstream.into_param().abi(), parturi.into_param().abi(), resources.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMObjectFactory1, ::windows_core::IUnknown, IXpsOMObjectFactory);
unsafe impl ::windows_core::Interface for IXpsOMObjectFactory1 {
    type Vtable = IXpsOMObjectFactory1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMObjectFactory1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0a91b617_d612_4181_bf7c_be5824e9cc8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMObjectFactory1_Vtbl {
    pub base__: IXpsOMObjectFactory_Vtbl,
    pub GetDocumentTypeFromFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetDocumentTypeFromStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, xpsdocumentstream: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetDocumentTypeFromStream: usize,
    pub ConvertHDPhotoToJpegXR: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ConvertJpegXRToHDPhoto: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnFile1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void, packagethumbnail: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnFile1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePackageWriterOnStream1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void, packagethumbnail: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePackageWriterOnStream1: usize,
    pub CreatePackage1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreatePackageFromStream1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreatePackageFromStream1: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreatePackageFromFile1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreatePackageFromFile1: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePage1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: ::windows_core::PCWSTR, parturi: *mut ::core::ffi::c_void, page: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePage1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreatePageFromStream1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagemarkupstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, resources: *mut ::core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, page: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreatePageFromStream1: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateRemoteDictionaryResourceFromStream1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionarymarkupstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, resources: *mut ::core::ffi::c_void, dictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateRemoteDictionaryResourceFromStream1: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMPackage(::windows_core::IUnknown);
impl IXpsOMPackage {
    pub unsafe fn GetDocumentSequence(&self) -> ::windows_core::Result<IXpsOMDocumentSequence> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDocumentSequence)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDocumentSequence<P0>(&self, documentsequence: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMDocumentSequence>,
    {
        (::windows_core::Interface::vtable(self).SetDocumentSequence)(::windows_core::Interface::as_raw(self), documentsequence.into_param().abi()).ok()
    }
    pub unsafe fn GetCoreProperties(&self) -> ::windows_core::Result<IXpsOMCoreProperties> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCoreProperties)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCoreProperties<P0>(&self, coreproperties: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMCoreProperties>,
    {
        (::windows_core::Interface::vtable(self).SetCoreProperties)(::windows_core::Interface::as_raw(self), coreproperties.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetDiscardControlPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDiscardControlPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetDiscardControlPartName<P0>(&self, discardcontrolparturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).SetDiscardControlPartName)(::windows_core::Interface::as_raw(self), discardcontrolparturi.into_param().abi()).ok()
    }
    pub unsafe fn GetThumbnailResource(&self) -> ::windows_core::Result<IXpsOMImageResource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetThumbnailResource)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetThumbnailResource<P0>(&self, imageresource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMImageResource>,
    {
        (::windows_core::Interface::vtable(self).SetThumbnailResource)(::windows_core::Interface::as_raw(self), imageresource.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn WriteToFile<P0, P1>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).WriteToFile)(::windows_core::Interface::as_raw(self), filename.into_param().abi(), securityattributes, flagsandattributes, optimizemarkupsize.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn WriteToStream<P0, P1>(&self, stream: P0, optimizemarkupsize: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::ISequentialStream>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).WriteToStream)(::windows_core::Interface::as_raw(self), stream.into_param().abi(), optimizemarkupsize.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMPackage, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMPackage {
    type Vtable = IXpsOMPackage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMPackage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x18c3df65_81e1_4674_91dc_fc452f5a416f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackage_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetDocumentSequence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequence: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDocumentSequence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequence: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetCoreProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coreproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetCoreProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetDiscardControlPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discardcontrolparturi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetDiscardControlPartName: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetDiscardControlPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, discardcontrolparturi: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetDiscardControlPartName: usize,
    pub GetThumbnailResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetThumbnailResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub WriteToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))]
    WriteToFile: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub WriteToStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    WriteToStream: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMPackage1(::windows_core::IUnknown);
impl IXpsOMPackage1 {
    pub unsafe fn GetDocumentSequence(&self) -> ::windows_core::Result<IXpsOMDocumentSequence> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDocumentSequence)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDocumentSequence<P0>(&self, documentsequence: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMDocumentSequence>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDocumentSequence)(::windows_core::Interface::as_raw(self), documentsequence.into_param().abi()).ok()
    }
    pub unsafe fn GetCoreProperties(&self) -> ::windows_core::Result<IXpsOMCoreProperties> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetCoreProperties)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCoreProperties<P0>(&self, coreproperties: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMCoreProperties>,
    {
        (::windows_core::Interface::vtable(self).base__.SetCoreProperties)(::windows_core::Interface::as_raw(self), coreproperties.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetDiscardControlPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDiscardControlPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetDiscardControlPartName<P0>(&self, discardcontrolparturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDiscardControlPartName)(::windows_core::Interface::as_raw(self), discardcontrolparturi.into_param().abi()).ok()
    }
    pub unsafe fn GetThumbnailResource(&self) -> ::windows_core::Result<IXpsOMImageResource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetThumbnailResource)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetThumbnailResource<P0>(&self, imageresource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMImageResource>,
    {
        (::windows_core::Interface::vtable(self).base__.SetThumbnailResource)(::windows_core::Interface::as_raw(self), imageresource.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn WriteToFile<P0, P1>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.WriteToFile)(::windows_core::Interface::as_raw(self), filename.into_param().abi(), securityattributes, flagsandattributes, optimizemarkupsize.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn WriteToStream<P0, P1>(&self, stream: P0, optimizemarkupsize: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::ISequentialStream>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.WriteToStream)(::windows_core::Interface::as_raw(self), stream.into_param().abi(), optimizemarkupsize.into_param().abi()).ok()
    }
    pub unsafe fn GetDocumentType(&self) -> ::windows_core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDocumentType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn WriteToFile1<P0, P1>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: P1, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).WriteToFile1)(::windows_core::Interface::as_raw(self), filename.into_param().abi(), securityattributes, flagsandattributes, optimizemarkupsize.into_param().abi(), documenttype).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn WriteToStream1<P0, P1>(&self, outputstream: P0, optimizemarkupsize: P1, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::ISequentialStream>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).WriteToStream1)(::windows_core::Interface::as_raw(self), outputstream.into_param().abi(), optimizemarkupsize.into_param().abi(), documenttype).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMPackage1, ::windows_core::IUnknown, IXpsOMPackage);
unsafe impl ::windows_core::Interface for IXpsOMPackage1 {
    type Vtable = IXpsOMPackage1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMPackage1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x95a9435e_12bb_461b_8e7f_c6adb04cd96a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackage1_Vtbl {
    pub base__: IXpsOMPackage_Vtbl,
    pub GetDocumentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub WriteToFile1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))]
    WriteToFile1: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub WriteToStream1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    WriteToStream1: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMPackageTarget(::windows_core::IUnknown);
impl IXpsOMPackageTarget {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateXpsOMPackageWriter<P0, P1, P2>(&self, documentsequencepartname: P0, documentsequenceprintticket: P1, discardcontrolpartname: P2) -> ::windows_core::Result<IXpsOMPackageWriter>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P1: ::windows_core::IntoParam<IXpsOMPrintTicketResource>,
        P2: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateXpsOMPackageWriter)(::windows_core::Interface::as_raw(self), documentsequencepartname.into_param().abi(), documentsequenceprintticket.into_param().abi(), discardcontrolpartname.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMPackageTarget, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMPackageTarget {
    type Vtable = IXpsOMPackageTarget_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMPackageTarget {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x219a9db0_4959_47d0_8034_b1ce84f41a4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackageTarget_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub CreateXpsOMPackageWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentsequencepartname: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    CreateXpsOMPackageWriter: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMPackageWriter(::windows_core::IUnknown);
impl IXpsOMPackageWriter {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn StartNewDocument<P0, P1, P2, P3, P4>(&self, documentpartname: P0, documentprintticket: P1, documentstructure: P2, signatureblockresources: P3, restrictedfonts: P4) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P1: ::windows_core::IntoParam<IXpsOMPrintTicketResource>,
        P2: ::windows_core::IntoParam<IXpsOMDocumentStructureResource>,
        P3: ::windows_core::IntoParam<IXpsOMSignatureBlockResourceCollection>,
        P4: ::windows_core::IntoParam<IXpsOMPartUriCollection>,
    {
        (::windows_core::Interface::vtable(self).StartNewDocument)(::windows_core::Interface::as_raw(self), documentpartname.into_param().abi(), documentprintticket.into_param().abi(), documentstructure.into_param().abi(), signatureblockresources.into_param().abi(), restrictedfonts.into_param().abi()).ok()
    }
    pub unsafe fn AddPage<P0, P1, P2, P3, P4>(&self, page: P0, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: P1, storyfragments: P2, pageprintticket: P3, pagethumbnail: P4) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMPage>,
        P1: ::windows_core::IntoParam<IXpsOMPartUriCollection>,
        P2: ::windows_core::IntoParam<IXpsOMStoryFragmentsResource>,
        P3: ::windows_core::IntoParam<IXpsOMPrintTicketResource>,
        P4: ::windows_core::IntoParam<IXpsOMImageResource>,
    {
        (::windows_core::Interface::vtable(self).AddPage)(::windows_core::Interface::as_raw(self), page.into_param().abi(), advisorypagedimensions, discardableresourceparts.into_param().abi(), storyfragments.into_param().abi(), pageprintticket.into_param().abi(), pagethumbnail.into_param().abi()).ok()
    }
    pub unsafe fn AddResource<P0>(&self, resource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMResource>,
    {
        (::windows_core::Interface::vtable(self).AddResource)(::windows_core::Interface::as_raw(self), resource.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsClosed(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsClosed)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMPackageWriter, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMPackageWriter {
    type Vtable = IXpsOMPackageWriter_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMPackageWriter {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e2aa182_a443_42c6_b41b_4f8e9de73ff9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackageWriter_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub StartNewDocument: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documentpartname: *mut ::core::ffi::c_void, documentprintticket: *mut ::core::ffi::c_void, documentstructure: *mut ::core::ffi::c_void, signatureblockresources: *mut ::core::ffi::c_void, restrictedfonts: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    StartNewDocument: usize,
    pub AddPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: *mut ::core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: *mut ::core::ffi::c_void, storyfragments: *mut ::core::ffi::c_void, pageprintticket: *mut ::core::ffi::c_void, pagethumbnail: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsClosed: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMPackageWriter3D(::windows_core::IUnknown);
impl IXpsOMPackageWriter3D {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn StartNewDocument<P0, P1, P2, P3, P4>(&self, documentpartname: P0, documentprintticket: P1, documentstructure: P2, signatureblockresources: P3, restrictedfonts: P4) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P1: ::windows_core::IntoParam<IXpsOMPrintTicketResource>,
        P2: ::windows_core::IntoParam<IXpsOMDocumentStructureResource>,
        P3: ::windows_core::IntoParam<IXpsOMSignatureBlockResourceCollection>,
        P4: ::windows_core::IntoParam<IXpsOMPartUriCollection>,
    {
        (::windows_core::Interface::vtable(self).base__.StartNewDocument)(::windows_core::Interface::as_raw(self), documentpartname.into_param().abi(), documentprintticket.into_param().abi(), documentstructure.into_param().abi(), signatureblockresources.into_param().abi(), restrictedfonts.into_param().abi()).ok()
    }
    pub unsafe fn AddPage<P0, P1, P2, P3, P4>(&self, page: P0, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: P1, storyfragments: P2, pageprintticket: P3, pagethumbnail: P4) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMPage>,
        P1: ::windows_core::IntoParam<IXpsOMPartUriCollection>,
        P2: ::windows_core::IntoParam<IXpsOMStoryFragmentsResource>,
        P3: ::windows_core::IntoParam<IXpsOMPrintTicketResource>,
        P4: ::windows_core::IntoParam<IXpsOMImageResource>,
    {
        (::windows_core::Interface::vtable(self).base__.AddPage)(::windows_core::Interface::as_raw(self), page.into_param().abi(), advisorypagedimensions, discardableresourceparts.into_param().abi(), storyfragments.into_param().abi(), pageprintticket.into_param().abi(), pagethumbnail.into_param().abi()).ok()
    }
    pub unsafe fn AddResource<P0>(&self, resource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMResource>,
    {
        (::windows_core::Interface::vtable(self).base__.AddResource)(::windows_core::Interface::as_raw(self), resource.into_param().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.Close)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsClosed(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsClosed)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn AddModelTexture<P0, P1>(&self, texturepartname: P0, texturedata: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P1: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).AddModelTexture)(::windows_core::Interface::as_raw(self), texturepartname.into_param().abi(), texturedata.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetModelPrintTicket<P0, P1>(&self, printticketpartname: P0, printticketdata: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
        P1: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).SetModelPrintTicket)(::windows_core::Interface::as_raw(self), printticketpartname.into_param().abi(), printticketdata.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMPackageWriter3D, ::windows_core::IUnknown, IXpsOMPackageWriter);
unsafe impl ::windows_core::Interface for IXpsOMPackageWriter3D {
    type Vtable = IXpsOMPackageWriter3D_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMPackageWriter3D {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe8a45033_640e_43fa_9bdf_fddeaa31c6a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPackageWriter3D_Vtbl {
    pub base__: IXpsOMPackageWriter_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub AddModelTexture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, texturepartname: *mut ::core::ffi::c_void, texturedata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    AddModelTexture: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetModelPrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketpartname: *mut ::core::ffi::c_void, printticketdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetModelPrintTicket: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMPage(::windows_core::IUnknown);
impl IXpsOMPage {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPartName)(::windows_core::Interface::as_raw(self), parturi.into_param().abi()).ok()
    }
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<IXpsOMPageReference> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetVisuals(&self) -> ::windows_core::Result<IXpsOMVisualCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetVisuals)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPageDimensions(&self) -> ::windows_core::Result<XPS_SIZE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPageDimensions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPageDimensions)(::windows_core::Interface::as_raw(self), pagedimensions).ok()
    }
    pub unsafe fn GetContentBox(&self) -> ::windows_core::Result<XPS_RECT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetContentBox)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetContentBox(&self, contentbox: *const XPS_RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetContentBox)(::windows_core::Interface::as_raw(self), contentbox).ok()
    }
    pub unsafe fn GetBleedBox(&self) -> ::windows_core::Result<XPS_RECT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetBleedBox)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBleedBox(&self, bleedbox: *const XPS_RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetBleedBox)(::windows_core::Interface::as_raw(self), bleedbox).ok()
    }
    pub unsafe fn GetLanguage(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLanguage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLanguage<P0>(&self, language: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLanguage)(::windows_core::Interface::as_raw(self), language.into_param().abi()).ok()
    }
    pub unsafe fn GetName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetIsHyperlinkTarget)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<P0>(&self, ishyperlinktarget: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetIsHyperlinkTarget)(::windows_core::Interface::as_raw(self), ishyperlinktarget.into_param().abi()).ok()
    }
    pub unsafe fn GetDictionary(&self) -> ::windows_core::Result<IXpsOMDictionary> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDictionary)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDictionaryLocal(&self) -> ::windows_core::Result<IXpsOMDictionary> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDictionaryLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDictionaryLocal<P0>(&self, resourcedictionary: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMDictionary>,
    {
        (::windows_core::Interface::vtable(self).SetDictionaryLocal)(::windows_core::Interface::as_raw(self), resourcedictionary.into_param().abi()).ok()
    }
    pub unsafe fn GetDictionaryResource(&self) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDictionaryResource)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDictionaryResource<P0>(&self, remotedictionaryresource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMRemoteDictionaryResource>,
    {
        (::windows_core::Interface::vtable(self).SetDictionaryResource)(::windows_core::Interface::as_raw(self), remotedictionaryresource.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Write<P0, P1>(&self, stream: P0, optimizemarkupsize: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::ISequentialStream>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).Write)(::windows_core::Interface::as_raw(self), stream.into_param().abi(), optimizemarkupsize.into_param().abi()).ok()
    }
    pub unsafe fn GenerateUnusedLookupKey(&self, r#type: XPS_OBJECT_TYPE) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GenerateUnusedLookupKey)(::windows_core::Interface::as_raw(self), r#type, &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IXpsOMPage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMPage, ::windows_core::IUnknown, IXpsOMPart);
unsafe impl ::windows_core::Interface for IXpsOMPage {
    type Vtable = IXpsOMPage_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMPage {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd3e18888_f120_4fee_8c68_35296eae91d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPage_Vtbl {
    pub base__: IXpsOMPart_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetVisuals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visuals: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPageDimensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows_core::HRESULT,
    pub SetPageDimensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows_core::HRESULT,
    pub GetContentBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentbox: *mut XPS_RECT) -> ::windows_core::HRESULT,
    pub SetContentBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentbox: *const XPS_RECT) -> ::windows_core::HRESULT,
    pub GetBleedBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bleedbox: *mut XPS_RECT) -> ::windows_core::HRESULT,
    pub SetBleedBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bleedbox: *const XPS_RECT) -> ::windows_core::HRESULT,
    pub GetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsHyperlinkTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlinktarget: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsHyperlinkTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsHyperlinkTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlinktarget: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsHyperlinkTarget: usize,
    pub GetDictionary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDictionaryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDictionaryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetDictionaryResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDictionaryResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Write: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Write: usize,
    pub GenerateUnusedLookupKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: XPS_OBJECT_TYPE, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMPage1(::windows_core::IUnknown);
impl IXpsOMPage1 {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPartName)(::windows_core::Interface::as_raw(self), parturi.into_param().abi()).ok()
    }
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<IXpsOMPageReference> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetVisuals(&self) -> ::windows_core::Result<IXpsOMVisualCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetVisuals)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPageDimensions(&self) -> ::windows_core::Result<XPS_SIZE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPageDimensions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetPageDimensions)(::windows_core::Interface::as_raw(self), pagedimensions).ok()
    }
    pub unsafe fn GetContentBox(&self) -> ::windows_core::Result<XPS_RECT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetContentBox)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetContentBox(&self, contentbox: *const XPS_RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetContentBox)(::windows_core::Interface::as_raw(self), contentbox).ok()
    }
    pub unsafe fn GetBleedBox(&self) -> ::windows_core::Result<XPS_RECT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetBleedBox)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBleedBox(&self, bleedbox: *const XPS_RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetBleedBox)(::windows_core::Interface::as_raw(self), bleedbox).ok()
    }
    pub unsafe fn GetLanguage(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLanguage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLanguage<P0>(&self, language: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetLanguage)(::windows_core::Interface::as_raw(self), language.into_param().abi()).ok()
    }
    pub unsafe fn GetName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetIsHyperlinkTarget)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<P0>(&self, ishyperlinktarget: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetIsHyperlinkTarget)(::windows_core::Interface::as_raw(self), ishyperlinktarget.into_param().abi()).ok()
    }
    pub unsafe fn GetDictionary(&self) -> ::windows_core::Result<IXpsOMDictionary> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDictionary)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDictionaryLocal(&self) -> ::windows_core::Result<IXpsOMDictionary> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDictionaryLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDictionaryLocal<P0>(&self, resourcedictionary: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMDictionary>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDictionaryLocal)(::windows_core::Interface::as_raw(self), resourcedictionary.into_param().abi()).ok()
    }
    pub unsafe fn GetDictionaryResource(&self) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDictionaryResource)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDictionaryResource<P0>(&self, remotedictionaryresource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMRemoteDictionaryResource>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDictionaryResource)(::windows_core::Interface::as_raw(self), remotedictionaryresource.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Write<P0, P1>(&self, stream: P0, optimizemarkupsize: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::ISequentialStream>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.Write)(::windows_core::Interface::as_raw(self), stream.into_param().abi(), optimizemarkupsize.into_param().abi()).ok()
    }
    pub unsafe fn GenerateUnusedLookupKey(&self, r#type: XPS_OBJECT_TYPE) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GenerateUnusedLookupKey)(::windows_core::Interface::as_raw(self), r#type, &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IXpsOMPage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocumentType(&self) -> ::windows_core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDocumentType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Write1<P0, P1>(&self, stream: P0, optimizemarkupsize: P1, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::ISequentialStream>,
        P1: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).Write1)(::windows_core::Interface::as_raw(self), stream.into_param().abi(), optimizemarkupsize.into_param().abi(), documenttype).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMPage1, ::windows_core::IUnknown, IXpsOMPart, IXpsOMPage);
unsafe impl ::windows_core::Interface for IXpsOMPage1 {
    type Vtable = IXpsOMPage1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMPage1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x305b60ef_6892_4dda_9cbb_3aa65974508a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPage1_Vtbl {
    pub base__: IXpsOMPage_Vtbl,
    pub GetDocumentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Write1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Write1: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMPageReference(::windows_core::IUnknown);
impl IXpsOMPageReference {
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<IXpsOMDocument> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPage(&self) -> ::windows_core::Result<IXpsOMPage> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPage<P0>(&self, page: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMPage>,
    {
        (::windows_core::Interface::vtable(self).SetPage)(::windows_core::Interface::as_raw(self), page.into_param().abi()).ok()
    }
    pub unsafe fn DiscardPage(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).DiscardPage)(::windows_core::Interface::as_raw(self)).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsPageLoaded(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsPageLoaded)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAdvisoryPageDimensions(&self) -> ::windows_core::Result<XPS_SIZE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAdvisoryPageDimensions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAdvisoryPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAdvisoryPageDimensions)(::windows_core::Interface::as_raw(self), pagedimensions).ok()
    }
    pub unsafe fn GetStoryFragmentsResource(&self) -> ::windows_core::Result<IXpsOMStoryFragmentsResource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStoryFragmentsResource)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStoryFragmentsResource<P0>(&self, storyfragmentsresource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMStoryFragmentsResource>,
    {
        (::windows_core::Interface::vtable(self).SetStoryFragmentsResource)(::windows_core::Interface::as_raw(self), storyfragmentsresource.into_param().abi()).ok()
    }
    pub unsafe fn GetPrintTicketResource(&self) -> ::windows_core::Result<IXpsOMPrintTicketResource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPrintTicketResource)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPrintTicketResource<P0>(&self, printticketresource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMPrintTicketResource>,
    {
        (::windows_core::Interface::vtable(self).SetPrintTicketResource)(::windows_core::Interface::as_raw(self), printticketresource.into_param().abi()).ok()
    }
    pub unsafe fn GetThumbnailResource(&self) -> ::windows_core::Result<IXpsOMImageResource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetThumbnailResource)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetThumbnailResource<P0>(&self, imageresource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMImageResource>,
    {
        (::windows_core::Interface::vtable(self).SetThumbnailResource)(::windows_core::Interface::as_raw(self), imageresource.into_param().abi()).ok()
    }
    pub unsafe fn CollectLinkTargets(&self) -> ::windows_core::Result<IXpsOMNameCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CollectLinkTargets)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CollectPartResources(&self) -> ::windows_core::Result<IXpsOMPartResources> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CollectPartResources)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HasRestrictedFonts(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).HasRestrictedFonts)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IXpsOMPageReference> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMPageReference, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMPageReference {
    type Vtable = IXpsOMPageReference_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMPageReference {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xed360180_6f92_4998_890d_2f208531a0a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPageReference_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub DiscardPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsPageLoaded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ispageloaded: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsPageLoaded: usize,
    pub GetAdvisoryPageDimensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows_core::HRESULT,
    pub SetAdvisoryPageDimensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows_core::HRESULT,
    pub GetStoryFragmentsResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyfragmentsresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetStoryFragmentsResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, storyfragmentsresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetPrintTicketResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printticketresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetThumbnailResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetThumbnailResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CollectLinkTargets: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, linktargets: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CollectPartResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HasRestrictedFonts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, restrictedfonts: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HasRestrictedFonts: usize,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMPageReferenceCollection(::windows_core::IUnknown);
impl IXpsOMPageReferenceCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsOMPageReference> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, pagereference: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMPageReference>,
    {
        (::windows_core::Interface::vtable(self).InsertAt)(::windows_core::Interface::as_raw(self), index, pagereference.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, pagereference: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMPageReference>,
    {
        (::windows_core::Interface::vtable(self).SetAt)(::windows_core::Interface::as_raw(self), index, pagereference.into_param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, pagereference: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMPageReference>,
    {
        (::windows_core::Interface::vtable(self).Append)(::windows_core::Interface::as_raw(self), pagereference.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMPageReferenceCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMPageReferenceCollection {
    type Vtable = IXpsOMPageReferenceCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMPageReferenceCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xca16ba4d_e7b9_45c5_958b_f98022473745);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPageReferenceCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pagereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pagereference: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, pagereference: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagereference: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMPart(::windows_core::IUnknown);
impl IXpsOMPart {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).SetPartName)(::windows_core::Interface::as_raw(self), parturi.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMPart, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMPart {
    type Vtable = IXpsOMPart_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMPart {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x74eb2f0b_a91e_4486_afac_0fabeca3dfc6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPart_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetPartName: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetPartName: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMPartResources(::windows_core::IUnknown);
impl IXpsOMPartResources {
    pub unsafe fn GetFontResources(&self) -> ::windows_core::Result<IXpsOMFontResourceCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFontResources)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetImageResources(&self) -> ::windows_core::Result<IXpsOMImageResourceCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetImageResources)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColorProfileResources(&self) -> ::windows_core::Result<IXpsOMColorProfileResourceCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetColorProfileResources)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetRemoteDictionaryResources(&self) -> ::windows_core::Result<IXpsOMRemoteDictionaryResourceCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRemoteDictionaryResources)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMPartResources, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMPartResources {
    type Vtable = IXpsOMPartResources_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMPartResources {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf4cf7729_4864_4275_99b3_a8717163ecaf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPartResources_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetFontResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fontresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetImageResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, imageresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetColorProfileResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorprofileresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetRemoteDictionaryResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionaryresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMPartUriCollection(::windows_core::IUnknown);
impl IXpsOMPartUriCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetAt(&self, index: u32) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn InsertAt<P0>(&self, index: u32, parturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).InsertAt)(::windows_core::Interface::as_raw(self), index, parturi.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), index).ok()
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetAt<P0>(&self, index: u32, parturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).SetAt)(::windows_core::Interface::as_raw(self), index, parturi.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn Append<P0>(&self, parturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).Append)(::windows_core::Interface::as_raw(self), parturi.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMPartUriCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMPartUriCollection {
    type Vtable = IXpsOMPartUriCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMPartUriCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x57c650d4_067c_4893_8c33_f62a0633730f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPartUriCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, parturi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetAt: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, parturi: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    InsertAt: usize,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, parturi: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetAt: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    Append: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMPath(::windows_core::IUnknown);
impl IXpsOMPath {
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransform(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransform)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransformLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, matrixtransform: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMMatrixTransform>,
    {
        (::windows_core::Interface::vtable(self).base__.SetTransformLocal)(::windows_core::Interface::as_raw(self), matrixtransform.into_param().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransformLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetTransformLookup)(::windows_core::Interface::as_raw(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetClipGeometry(&self) -> ::windows_core::Result<IXpsOMGeometry> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetClipGeometry)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetClipGeometryLocal(&self) -> ::windows_core::Result<IXpsOMGeometry> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetClipGeometryLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetClipGeometryLocal<P0>(&self, clipgeometry: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMGeometry>,
    {
        (::windows_core::Interface::vtable(self).base__.SetClipGeometryLocal)(::windows_core::Interface::as_raw(self), clipgeometry.into_param().abi()).ok()
    }
    pub unsafe fn GetClipGeometryLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetClipGeometryLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetClipGeometryLookup<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetClipGeometryLookup)(::windows_core::Interface::as_raw(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetOpacity(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOpacity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOpacity)(::windows_core::Interface::as_raw(self), opacity).ok()
    }
    pub unsafe fn GetOpacityMaskBrush(&self) -> ::windows_core::Result<IXpsOMBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOpacityMaskBrush)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> ::windows_core::Result<IXpsOMBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOpacityMaskBrushLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOpacityMaskBrushLocal<P0>(&self, opacitymaskbrush: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMBrush>,
    {
        (::windows_core::Interface::vtable(self).base__.SetOpacityMaskBrushLocal)(::windows_core::Interface::as_raw(self), opacitymaskbrush.into_param().abi()).ok()
    }
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOpacityMaskBrushLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOpacityMaskBrushLookup<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetOpacityMaskBrushLookup)(::windows_core::Interface::as_raw(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetIsHyperlinkTarget)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<P0>(&self, ishyperlink: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.SetIsHyperlinkTarget)(::windows_core::Interface::as_raw(self), ishyperlink.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> ::windows_core::Result<super::super::System::Com::IUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetHyperlinkNavigateUri)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHyperlinkNavigateUri<P0>(&self, hyperlinkuri: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IUri>,
    {
        (::windows_core::Interface::vtable(self).base__.SetHyperlinkNavigateUri)(::windows_core::Interface::as_raw(self), hyperlinkuri.into_param().abi()).ok()
    }
    pub unsafe fn GetLanguage(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetLanguage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLanguage<P0>(&self, language: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetLanguage)(::windows_core::Interface::as_raw(self), language.into_param().abi()).ok()
    }
    pub unsafe fn GetGeometry(&self) -> ::windows_core::Result<IXpsOMGeometry> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetGeometry)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetGeometryLocal(&self) -> ::windows_core::Result<IXpsOMGeometry> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetGeometryLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetGeometryLocal<P0>(&self, geometry: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMGeometry>,
    {
        (::windows_core::Interface::vtable(self).SetGeometryLocal)(::windows_core::Interface::as_raw(self), geometry.into_param().abi()).ok()
    }
    pub unsafe fn GetGeometryLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetGeometryLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetGeometryLookup<P0>(&self, lookup: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetGeometryLookup)(::windows_core::Interface::as_raw(self), lookup.into_param().abi()).ok()
    }
    pub unsafe fn GetAccessibilityShortDescription(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAccessibilityShortDescription)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAccessibilityShortDescription<P0>(&self, shortdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetAccessibilityShortDescription)(::windows_core::Interface::as_raw(self), shortdescription.into_param().abi()).ok()
    }
    pub unsafe fn GetAccessibilityLongDescription(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAccessibilityLongDescription)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetAccessibilityLongDescription<P0>(&self, longdescription: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetAccessibilityLongDescription)(::windows_core::Interface::as_raw(self), longdescription.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSnapsToPixels(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSnapsToPixels)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSnapsToPixels<P0>(&self, snapstopixels: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetSnapsToPixels)(::windows_core::Interface::as_raw(self), snapstopixels.into_param().abi()).ok()
    }
    pub unsafe fn GetStrokeBrush(&self) -> ::windows_core::Result<IXpsOMBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStrokeBrush)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStrokeBrushLocal(&self) -> ::windows_core::Result<IXpsOMBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStrokeBrushLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStrokeBrushLocal<P0>(&self, brush: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMBrush>,
    {
        (::windows_core::Interface::vtable(self).SetStrokeBrushLocal)(::windows_core::Interface::as_raw(self), brush.into_param().abi()).ok()
    }
    pub unsafe fn GetStrokeBrushLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStrokeBrushLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStrokeBrushLookup<P0>(&self, lookup: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetStrokeBrushLookup)(::windows_core::Interface::as_raw(self), lookup.into_param().abi()).ok()
    }
    pub unsafe fn GetStrokeDashes(&self) -> ::windows_core::Result<IXpsOMDashCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStrokeDashes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStrokeDashCap(&self) -> ::windows_core::Result<XPS_DASH_CAP> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStrokeDashCap)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStrokeDashCap(&self, strokedashcap: XPS_DASH_CAP) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStrokeDashCap)(::windows_core::Interface::as_raw(self), strokedashcap).ok()
    }
    pub unsafe fn GetStrokeDashOffset(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStrokeDashOffset)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStrokeDashOffset(&self, strokedashoffset: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStrokeDashOffset)(::windows_core::Interface::as_raw(self), strokedashoffset).ok()
    }
    pub unsafe fn GetStrokeStartLineCap(&self) -> ::windows_core::Result<XPS_LINE_CAP> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStrokeStartLineCap)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStrokeStartLineCap(&self, strokestartlinecap: XPS_LINE_CAP) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStrokeStartLineCap)(::windows_core::Interface::as_raw(self), strokestartlinecap).ok()
    }
    pub unsafe fn GetStrokeEndLineCap(&self) -> ::windows_core::Result<XPS_LINE_CAP> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStrokeEndLineCap)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStrokeEndLineCap(&self, strokeendlinecap: XPS_LINE_CAP) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStrokeEndLineCap)(::windows_core::Interface::as_raw(self), strokeendlinecap).ok()
    }
    pub unsafe fn GetStrokeLineJoin(&self) -> ::windows_core::Result<XPS_LINE_JOIN> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStrokeLineJoin)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStrokeLineJoin(&self, strokelinejoin: XPS_LINE_JOIN) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStrokeLineJoin)(::windows_core::Interface::as_raw(self), strokelinejoin).ok()
    }
    pub unsafe fn GetStrokeMiterLimit(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStrokeMiterLimit)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStrokeMiterLimit(&self, strokemiterlimit: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStrokeMiterLimit)(::windows_core::Interface::as_raw(self), strokemiterlimit).ok()
    }
    pub unsafe fn GetStrokeThickness(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStrokeThickness)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetStrokeThickness(&self, strokethickness: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetStrokeThickness)(::windows_core::Interface::as_raw(self), strokethickness).ok()
    }
    pub unsafe fn GetFillBrush(&self) -> ::windows_core::Result<IXpsOMBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFillBrush)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFillBrushLocal(&self) -> ::windows_core::Result<IXpsOMBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFillBrushLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFillBrushLocal<P0>(&self, brush: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMBrush>,
    {
        (::windows_core::Interface::vtable(self).SetFillBrushLocal)(::windows_core::Interface::as_raw(self), brush.into_param().abi()).ok()
    }
    pub unsafe fn GetFillBrushLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFillBrushLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFillBrushLookup<P0>(&self, lookup: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetFillBrushLookup)(::windows_core::Interface::as_raw(self), lookup.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IXpsOMPath> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMPath, ::windows_core::IUnknown, IXpsOMShareable, IXpsOMVisual);
unsafe impl ::windows_core::Interface for IXpsOMPath {
    type Vtable = IXpsOMPath_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMPath {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x37d38bb6_3ee9_4110_9312_14b194163337);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPath_Vtbl {
    pub base__: IXpsOMVisual_Vtbl,
    pub GetGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetGeometryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetGeometryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, geometry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetGeometryLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetGeometryLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetAccessibilityShortDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetAccessibilityShortDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shortdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetAccessibilityLongDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, longdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetAccessibilityLongDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, longdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSnapsToPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapstopixels: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSnapsToPixels: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSnapsToPixels: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapstopixels: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSnapsToPixels: usize,
    pub GetStrokeBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetStrokeBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetStrokeBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetStrokeBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetStrokeBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetStrokeDashes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokedashes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetStrokeDashCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokedashcap: *mut XPS_DASH_CAP) -> ::windows_core::HRESULT,
    pub SetStrokeDashCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokedashcap: XPS_DASH_CAP) -> ::windows_core::HRESULT,
    pub GetStrokeDashOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokedashoffset: *mut f32) -> ::windows_core::HRESULT,
    pub SetStrokeDashOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokedashoffset: f32) -> ::windows_core::HRESULT,
    pub GetStrokeStartLineCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokestartlinecap: *mut XPS_LINE_CAP) -> ::windows_core::HRESULT,
    pub SetStrokeStartLineCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokestartlinecap: XPS_LINE_CAP) -> ::windows_core::HRESULT,
    pub GetStrokeEndLineCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokeendlinecap: *mut XPS_LINE_CAP) -> ::windows_core::HRESULT,
    pub SetStrokeEndLineCap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokeendlinecap: XPS_LINE_CAP) -> ::windows_core::HRESULT,
    pub GetStrokeLineJoin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokelinejoin: *mut XPS_LINE_JOIN) -> ::windows_core::HRESULT,
    pub SetStrokeLineJoin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokelinejoin: XPS_LINE_JOIN) -> ::windows_core::HRESULT,
    pub GetStrokeMiterLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokemiterlimit: *mut f32) -> ::windows_core::HRESULT,
    pub SetStrokeMiterLimit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokemiterlimit: f32) -> ::windows_core::HRESULT,
    pub GetStrokeThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokethickness: *mut f32) -> ::windows_core::HRESULT,
    pub SetStrokeThickness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokethickness: f32) -> ::windows_core::HRESULT,
    pub GetFillBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFillBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetFillBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetFillBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetFillBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMPrintTicketResource(::windows_core::IUnknown);
impl IXpsOMPrintTicketResource {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPartName)(::windows_core::Interface::as_raw(self), parturi.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStream)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, partname: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).SetContent)(::windows_core::Interface::as_raw(self), sourcestream.into_param().abi(), partname.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMPrintTicketResource, ::windows_core::IUnknown, IXpsOMPart, IXpsOMResource);
unsafe impl ::windows_core::Interface for IXpsOMPrintTicketResource {
    type Vtable = IXpsOMPrintTicketResource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMPrintTicketResource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe7ff32d2_34aa_499b_bbe9_9cd4ee6c59f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMPrintTicketResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMRadialGradientBrush(::windows_core::IUnknown);
impl IXpsOMRadialGradientBrush {
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetOpacity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetOpacity)(::windows_core::Interface::as_raw(self), opacity).ok()
    }
    pub unsafe fn GetGradientStops(&self) -> ::windows_core::Result<IXpsOMGradientStopCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetGradientStops)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransform(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransform)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransformLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, transform: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMMatrixTransform>,
    {
        (::windows_core::Interface::vtable(self).base__.SetTransformLocal)(::windows_core::Interface::as_raw(self), transform.into_param().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransformLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetTransformLookup)(::windows_core::Interface::as_raw(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetSpreadMethod(&self) -> ::windows_core::Result<XPS_SPREAD_METHOD> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetSpreadMethod)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSpreadMethod(&self, spreadmethod: XPS_SPREAD_METHOD) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetSpreadMethod)(::windows_core::Interface::as_raw(self), spreadmethod).ok()
    }
    pub unsafe fn GetColorInterpolationMode(&self) -> ::windows_core::Result<XPS_COLOR_INTERPOLATION> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetColorInterpolationMode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetColorInterpolationMode(&self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetColorInterpolationMode)(::windows_core::Interface::as_raw(self), colorinterpolationmode).ok()
    }
    pub unsafe fn GetCenter(&self) -> ::windows_core::Result<XPS_POINT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCenter)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetCenter(&self, center: *const XPS_POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetCenter)(::windows_core::Interface::as_raw(self), center).ok()
    }
    pub unsafe fn GetRadiiSizes(&self) -> ::windows_core::Result<XPS_SIZE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRadiiSizes)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRadiiSizes(&self, radiisizes: *const XPS_SIZE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetRadiiSizes)(::windows_core::Interface::as_raw(self), radiisizes).ok()
    }
    pub unsafe fn GetGradientOrigin(&self) -> ::windows_core::Result<XPS_POINT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetGradientOrigin)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetGradientOrigin(&self, origin: *const XPS_POINT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetGradientOrigin)(::windows_core::Interface::as_raw(self), origin).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IXpsOMRadialGradientBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMRadialGradientBrush, ::windows_core::IUnknown, IXpsOMShareable, IXpsOMBrush, IXpsOMGradientBrush);
unsafe impl ::windows_core::Interface for IXpsOMRadialGradientBrush {
    type Vtable = IXpsOMRadialGradientBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMRadialGradientBrush {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x75f207e5_08bf_413c_96b1_b82b4064176b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRadialGradientBrush_Vtbl {
    pub base__: IXpsOMGradientBrush_Vtbl,
    pub GetCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, center: *mut XPS_POINT) -> ::windows_core::HRESULT,
    pub SetCenter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, center: *const XPS_POINT) -> ::windows_core::HRESULT,
    pub GetRadiiSizes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radiisizes: *mut XPS_SIZE) -> ::windows_core::HRESULT,
    pub SetRadiiSizes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radiisizes: *const XPS_SIZE) -> ::windows_core::HRESULT,
    pub GetGradientOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: *mut XPS_POINT) -> ::windows_core::HRESULT,
    pub SetGradientOrigin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, origin: *const XPS_POINT) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radialgradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMRemoteDictionaryResource(::windows_core::IUnknown);
impl IXpsOMRemoteDictionaryResource {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPartName)(::windows_core::Interface::as_raw(self), parturi.into_param().abi()).ok()
    }
    pub unsafe fn GetDictionary(&self) -> ::windows_core::Result<IXpsOMDictionary> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDictionary)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDictionary<P0>(&self, dictionary: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMDictionary>,
    {
        (::windows_core::Interface::vtable(self).SetDictionary)(::windows_core::Interface::as_raw(self), dictionary.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMRemoteDictionaryResource, ::windows_core::IUnknown, IXpsOMPart, IXpsOMResource);
unsafe impl ::windows_core::Interface for IXpsOMRemoteDictionaryResource {
    type Vtable = IXpsOMRemoteDictionaryResource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMRemoteDictionaryResource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9bd7cd4_e16a_4bf8_8c84_c950af7a3061);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRemoteDictionaryResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    pub GetDictionary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDictionary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionary: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMRemoteDictionaryResource1(::windows_core::IUnknown);
impl IXpsOMRemoteDictionaryResource1 {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.base__.SetPartName)(::windows_core::Interface::as_raw(self), parturi.into_param().abi()).ok()
    }
    pub unsafe fn GetDictionary(&self) -> ::windows_core::Result<IXpsOMDictionary> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetDictionary)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDictionary<P0>(&self, dictionary: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMDictionary>,
    {
        (::windows_core::Interface::vtable(self).base__.SetDictionary)(::windows_core::Interface::as_raw(self), dictionary.into_param().abi()).ok()
    }
    pub unsafe fn GetDocumentType(&self) -> ::windows_core::Result<XPS_DOCUMENT_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDocumentType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Write1<P0>(&self, stream: P0, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::ISequentialStream>,
    {
        (::windows_core::Interface::vtable(self).Write1)(::windows_core::Interface::as_raw(self), stream.into_param().abi(), documenttype).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMRemoteDictionaryResource1, ::windows_core::IUnknown, IXpsOMPart, IXpsOMResource, IXpsOMRemoteDictionaryResource);
unsafe impl ::windows_core::Interface for IXpsOMRemoteDictionaryResource1 {
    type Vtable = IXpsOMRemoteDictionaryResource1_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMRemoteDictionaryResource1 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbf8fc1d4_9d46_4141_ba5f_94bb9250d041);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRemoteDictionaryResource1_Vtbl {
    pub base__: IXpsOMRemoteDictionaryResource_Vtbl,
    pub GetDocumentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Write1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Write1: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMRemoteDictionaryResourceCollection(::windows_core::IUnknown);
impl IXpsOMRemoteDictionaryResourceCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, object: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMRemoteDictionaryResource>,
    {
        (::windows_core::Interface::vtable(self).InsertAt)(::windows_core::Interface::as_raw(self), index, object.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, object: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMRemoteDictionaryResource>,
    {
        (::windows_core::Interface::vtable(self).SetAt)(::windows_core::Interface::as_raw(self), index, object.into_param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, object: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMRemoteDictionaryResource>,
    {
        (::windows_core::Interface::vtable(self).Append)(::windows_core::Interface::as_raw(self), object.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<P0>(&self, partname: P0) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetByPartName)(::windows_core::Interface::as_raw(self), partname.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMRemoteDictionaryResourceCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMRemoteDictionaryResourceCollection {
    type Vtable = IXpsOMRemoteDictionaryResourceCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMRemoteDictionaryResourceCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5c38db61_7fec_464a_87bd_41e3bef018be);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMRemoteDictionaryResourceCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, remotedictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMResource(::windows_core::IUnknown);
impl IXpsOMResource {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).base__.SetPartName)(::windows_core::Interface::as_raw(self), parturi.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMResource, ::windows_core::IUnknown, IXpsOMPart);
unsafe impl ::windows_core::Interface for IXpsOMResource {
    type Vtable = IXpsOMResource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMResource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda2ac0a2_73a2_4975_ad14_74097c3ff3a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMResource_Vtbl {
    pub base__: IXpsOMPart_Vtbl,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMShareable(::windows_core::IUnknown);
impl IXpsOMShareable {
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMShareable, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMShareable {
    type Vtable = IXpsOMShareable_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMShareable {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7137398f_2fc1_454d_8c6a_2c3115a16ece);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMShareable_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMSignatureBlockResource(::windows_core::IUnknown);
impl IXpsOMSignatureBlockResource {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPartName)(::windows_core::Interface::as_raw(self), parturi.into_param().abi()).ok()
    }
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<IXpsOMDocument> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStream)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, partname: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).SetContent)(::windows_core::Interface::as_raw(self), sourcestream.into_param().abi(), partname.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMSignatureBlockResource, ::windows_core::IUnknown, IXpsOMPart, IXpsOMResource);
unsafe impl ::windows_core::Interface for IXpsOMSignatureBlockResource {
    type Vtable = IXpsOMSignatureBlockResource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMSignatureBlockResource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4776ad35_2e04_4357_8743_ebf6c171a905);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMSignatureBlockResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMSignatureBlockResourceCollection(::windows_core::IUnknown);
impl IXpsOMSignatureBlockResourceCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsOMSignatureBlockResource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, signatureblockresource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMSignatureBlockResource>,
    {
        (::windows_core::Interface::vtable(self).InsertAt)(::windows_core::Interface::as_raw(self), index, signatureblockresource.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, signatureblockresource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMSignatureBlockResource>,
    {
        (::windows_core::Interface::vtable(self).SetAt)(::windows_core::Interface::as_raw(self), index, signatureblockresource.into_param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, signatureblockresource: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMSignatureBlockResource>,
    {
        (::windows_core::Interface::vtable(self).Append)(::windows_core::Interface::as_raw(self), signatureblockresource.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetByPartName<P0>(&self, partname: P0) -> ::windows_core::Result<IXpsOMSignatureBlockResource>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetByPartName)(::windows_core::Interface::as_raw(self), partname.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMSignatureBlockResourceCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMSignatureBlockResourceCollection {
    type Vtable = IXpsOMSignatureBlockResourceCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMSignatureBlockResourceCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab8f5d8e_351b_4d33_aaed_fa56f0022931);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMSignatureBlockResourceCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureblockresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetByPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, signatureblockresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetByPartName: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMSolidColorBrush(::windows_core::IUnknown);
impl IXpsOMSolidColorBrush {
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOpacity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOpacity)(::windows_core::Interface::as_raw(self), opacity).ok()
    }
    pub unsafe fn GetColor(&self, color: *mut XPS_COLOR, colorprofile: *mut ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetColor)(::windows_core::Interface::as_raw(self), color, ::core::mem::transmute(colorprofile)).ok()
    }
    pub unsafe fn SetColor<P0>(&self, color: *const XPS_COLOR, colorprofile: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMColorProfileResource>,
    {
        (::windows_core::Interface::vtable(self).SetColor)(::windows_core::Interface::as_raw(self), color, colorprofile.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IXpsOMSolidColorBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMSolidColorBrush, ::windows_core::IUnknown, IXpsOMShareable, IXpsOMBrush);
unsafe impl ::windows_core::Interface for IXpsOMSolidColorBrush {
    type Vtable = IXpsOMSolidColorBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMSolidColorBrush {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa06f9f05_3be9_4763_98a8_094fc672e488);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMSolidColorBrush_Vtbl {
    pub base__: IXpsOMBrush_Vtbl,
    pub GetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, solidcolorbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMStoryFragmentsResource(::windows_core::IUnknown);
impl IXpsOMStoryFragmentsResource {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).base__.base__.SetPartName)(::windows_core::Interface::as_raw(self), parturi.into_param().abi()).ok()
    }
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<IXpsOMPageReference> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows_core::Result<super::super::System::Com::IStream> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetStream)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetContent<P0, P1>(&self, sourcestream: P0, partname: P1) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).SetContent)(::windows_core::Interface::as_raw(self), sourcestream.into_param().abi(), partname.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMStoryFragmentsResource, ::windows_core::IUnknown, IXpsOMPart, IXpsOMResource);
unsafe impl ::windows_core::Interface for IXpsOMStoryFragmentsResource {
    type Vtable = IXpsOMStoryFragmentsResource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMStoryFragmentsResource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc2b3ca09_0473_4282_87ae_1780863223f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMStoryFragmentsResource_Vtbl {
    pub base__: IXpsOMResource_Vtbl,
    pub GetOwner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetContent: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMThumbnailGenerator(::windows_core::IUnknown);
impl IXpsOMThumbnailGenerator {
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GenerateThumbnail<P0, P1>(&self, page: P0, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: P1) -> ::windows_core::Result<IXpsOMImageResource>
    where
        P0: ::windows_core::IntoParam<IXpsOMPage>,
        P1: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GenerateThumbnail)(::windows_core::Interface::as_raw(self), page.into_param().abi(), thumbnailtype, thumbnailsize, imageresourcepartname.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMThumbnailGenerator, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMThumbnailGenerator {
    type Vtable = IXpsOMThumbnailGenerator_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMThumbnailGenerator {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x15b873d5_1971_41e8_83a3_6578403064c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMThumbnailGenerator_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GenerateThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, page: *mut ::core::ffi::c_void, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GenerateThumbnail: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMTileBrush(::windows_core::IUnknown);
impl IXpsOMTileBrush {
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOpacity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetOpacity)(::windows_core::Interface::as_raw(self), opacity).ok()
    }
    pub unsafe fn GetTransform(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTransform)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTransformLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, transform: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMMatrixTransform>,
    {
        (::windows_core::Interface::vtable(self).SetTransformLocal)(::windows_core::Interface::as_raw(self), transform.into_param().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTransformLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTransformLookup)(::windows_core::Interface::as_raw(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetViewbox(&self) -> ::windows_core::Result<XPS_RECT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetViewbox)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetViewbox(&self, viewbox: *const XPS_RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetViewbox)(::windows_core::Interface::as_raw(self), viewbox).ok()
    }
    pub unsafe fn GetViewport(&self) -> ::windows_core::Result<XPS_RECT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetViewport)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetViewport(&self, viewport: *const XPS_RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetViewport)(::windows_core::Interface::as_raw(self), viewport).ok()
    }
    pub unsafe fn GetTileMode(&self) -> ::windows_core::Result<XPS_TILE_MODE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTileMode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTileMode(&self, tilemode: XPS_TILE_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTileMode)(::windows_core::Interface::as_raw(self), tilemode).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMTileBrush, ::windows_core::IUnknown, IXpsOMShareable, IXpsOMBrush);
unsafe impl ::windows_core::Interface for IXpsOMTileBrush {
    type Vtable = IXpsOMTileBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMTileBrush {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0fc2328d_d722_4a54_b2ec_be90218a789e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMTileBrush_Vtbl {
    pub base__: IXpsOMBrush_Vtbl,
    pub GetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetViewbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewbox: *mut XPS_RECT) -> ::windows_core::HRESULT,
    pub SetViewbox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT) -> ::windows_core::HRESULT,
    pub GetViewport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *mut XPS_RECT) -> ::windows_core::HRESULT,
    pub SetViewport: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewport: *const XPS_RECT) -> ::windows_core::HRESULT,
    pub GetTileMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tilemode: *mut XPS_TILE_MODE) -> ::windows_core::HRESULT,
    pub SetTileMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tilemode: XPS_TILE_MODE) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMVisual(::windows_core::IUnknown);
impl IXpsOMVisual {
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransform(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTransform)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTransformLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, matrixtransform: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMMatrixTransform>,
    {
        (::windows_core::Interface::vtable(self).SetTransformLocal)(::windows_core::Interface::as_raw(self), matrixtransform.into_param().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetTransformLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetTransformLookup)(::windows_core::Interface::as_raw(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetClipGeometry(&self) -> ::windows_core::Result<IXpsOMGeometry> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetClipGeometry)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetClipGeometryLocal(&self) -> ::windows_core::Result<IXpsOMGeometry> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetClipGeometryLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetClipGeometryLocal<P0>(&self, clipgeometry: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMGeometry>,
    {
        (::windows_core::Interface::vtable(self).SetClipGeometryLocal)(::windows_core::Interface::as_raw(self), clipgeometry.into_param().abi()).ok()
    }
    pub unsafe fn GetClipGeometryLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetClipGeometryLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetClipGeometryLookup<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetClipGeometryLookup)(::windows_core::Interface::as_raw(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetOpacity(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOpacity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetOpacity)(::windows_core::Interface::as_raw(self), opacity).ok()
    }
    pub unsafe fn GetOpacityMaskBrush(&self) -> ::windows_core::Result<IXpsOMBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOpacityMaskBrush)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> ::windows_core::Result<IXpsOMBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOpacityMaskBrushLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOpacityMaskBrushLocal<P0>(&self, opacitymaskbrush: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMBrush>,
    {
        (::windows_core::Interface::vtable(self).SetOpacityMaskBrushLocal)(::windows_core::Interface::as_raw(self), opacitymaskbrush.into_param().abi()).ok()
    }
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOpacityMaskBrushLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOpacityMaskBrushLookup<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetOpacityMaskBrushLookup)(::windows_core::Interface::as_raw(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetName(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetName)(::windows_core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetIsHyperlinkTarget)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<P0>(&self, ishyperlink: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).SetIsHyperlinkTarget)(::windows_core::Interface::as_raw(self), ishyperlink.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> ::windows_core::Result<super::super::System::Com::IUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetHyperlinkNavigateUri)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHyperlinkNavigateUri<P0>(&self, hyperlinkuri: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IUri>,
    {
        (::windows_core::Interface::vtable(self).SetHyperlinkNavigateUri)(::windows_core::Interface::as_raw(self), hyperlinkuri.into_param().abi()).ok()
    }
    pub unsafe fn GetLanguage(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLanguage)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLanguage<P0>(&self, language: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetLanguage)(::windows_core::Interface::as_raw(self), language.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMVisual, ::windows_core::IUnknown, IXpsOMShareable);
unsafe impl ::windows_core::Interface for IXpsOMVisual {
    type Vtable = IXpsOMVisual_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMVisual {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc3e7333_fb0b_4af3_a819_0b4eaad0d2fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMVisual_Vtbl {
    pub base__: IXpsOMShareable_Vtbl,
    pub GetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetTransformLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetTransformLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetClipGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetClipGeometryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetClipGeometryLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetClipGeometryLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetClipGeometryLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows_core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows_core::HRESULT,
    pub GetOpacityMaskBrush: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetOpacityMaskBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetOpacityMaskBrushLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetOpacityMaskBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetOpacityMaskBrushLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsHyperlinkTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlink: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsHyperlinkTarget: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetIsHyperlinkTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ishyperlink: super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetIsHyperlinkTarget: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetHyperlinkNavigateUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hyperlinkuri: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetHyperlinkNavigateUri: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetHyperlinkNavigateUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hyperlinkuri: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetHyperlinkNavigateUri: usize,
    pub GetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMVisualBrush(::windows_core::IUnknown);
impl IXpsOMVisualBrush {
    pub unsafe fn GetOwner(&self) -> ::windows_core::Result<::windows_core::IUnknown> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetOwner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows_core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.base__.GetType)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows_core::Result<f32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.base__.GetOpacity)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.base__.SetOpacity)(::windows_core::Interface::as_raw(self), opacity).ok()
    }
    pub unsafe fn GetTransform(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransform)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows_core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransformLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, transform: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMMatrixTransform>,
    {
        (::windows_core::Interface::vtable(self).base__.SetTransformLocal)(::windows_core::Interface::as_raw(self), transform.into_param().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTransformLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.SetTransformLookup)(::windows_core::Interface::as_raw(self), key.into_param().abi()).ok()
    }
    pub unsafe fn GetViewbox(&self) -> ::windows_core::Result<XPS_RECT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetViewbox)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetViewbox(&self, viewbox: *const XPS_RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetViewbox)(::windows_core::Interface::as_raw(self), viewbox).ok()
    }
    pub unsafe fn GetViewport(&self) -> ::windows_core::Result<XPS_RECT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetViewport)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetViewport(&self, viewport: *const XPS_RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetViewport)(::windows_core::Interface::as_raw(self), viewport).ok()
    }
    pub unsafe fn GetTileMode(&self) -> ::windows_core::Result<XPS_TILE_MODE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetTileMode)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTileMode(&self, tilemode: XPS_TILE_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.SetTileMode)(::windows_core::Interface::as_raw(self), tilemode).ok()
    }
    pub unsafe fn GetVisual(&self) -> ::windows_core::Result<IXpsOMVisual> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetVisual)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetVisualLocal(&self) -> ::windows_core::Result<IXpsOMVisual> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetVisualLocal)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetVisualLocal<P0>(&self, visual: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMVisual>,
    {
        (::windows_core::Interface::vtable(self).SetVisualLocal)(::windows_core::Interface::as_raw(self), visual.into_param().abi()).ok()
    }
    pub unsafe fn GetVisualLookup(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetVisualLookup)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetVisualLookup<P0>(&self, lookup: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetVisualLookup)(::windows_core::Interface::as_raw(self), lookup.into_param().abi()).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows_core::Result<IXpsOMVisualBrush> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Clone)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMVisualBrush, ::windows_core::IUnknown, IXpsOMShareable, IXpsOMBrush, IXpsOMTileBrush);
unsafe impl ::windows_core::Interface for IXpsOMVisualBrush {
    type Vtable = IXpsOMVisualBrush_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMVisualBrush {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x97e294af_5b37_46b4_8057_874d2f64119b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMVisualBrush_Vtbl {
    pub base__: IXpsOMTileBrush_Vtbl,
    pub GetVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetVisualLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetVisualLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetVisualLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetVisualLookup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lookup: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visualbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsOMVisualCollection(::windows_core::IUnknown);
impl IXpsOMVisualCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsOMVisual> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn InsertAt<P0>(&self, index: u32, object: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMVisual>,
    {
        (::windows_core::Interface::vtable(self).InsertAt)(::windows_core::Interface::as_raw(self), index, object.into_param().abi()).ok()
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), index).ok()
    }
    pub unsafe fn SetAt<P0>(&self, index: u32, object: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMVisual>,
    {
        (::windows_core::Interface::vtable(self).SetAt)(::windows_core::Interface::as_raw(self), index, object.into_param().abi()).ok()
    }
    pub unsafe fn Append<P0>(&self, object: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IXpsOMVisual>,
    {
        (::windows_core::Interface::vtable(self).Append)(::windows_core::Interface::as_raw(self), object.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsOMVisualCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsOMVisualCollection {
    type Vtable = IXpsOMVisualCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsOMVisualCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x94d8abde_ab91_46a8_82b7_f5b05ef01a96);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsOMVisualCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub InsertAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT,
    pub SetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsSignature(::windows_core::IUnknown);
impl IXpsSignature {
    pub unsafe fn GetSignatureId(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignatureId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSignatureValue(&self, signaturehashvalue: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSignatureValue)(::windows_core::Interface::as_raw(self), signaturehashvalue, count).ok()
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCertificateEnumerator(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcCertificateEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCertificateEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSigningTime(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSigningTime)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetSigningTimeFormat(&self) -> ::windows_core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSigningTimeFormat)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetSignaturePartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignaturePartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn Verify(&self, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::Result<XPS_SIGNATURE_STATUS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Verify)(::windows_core::Interface::as_raw(self), x509certificate, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPolicy(&self) -> ::windows_core::Result<XPS_SIGN_POLICY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPolicy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomObjectEnumerator(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCustomObjectEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomReferenceEnumerator(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcSignatureReferenceEnumerator> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCustomReferenceEnumerator)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSignatureXml(&self, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSignatureXml)(::windows_core::Interface::as_raw(self), signaturexml, count).ok()
    }
    pub unsafe fn SetSignatureXml(&self, signaturexml: &[u8]) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSignatureXml)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(signaturexml.as_ptr()), signaturexml.len().try_into().unwrap()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsSignature, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsSignature {
    type Vtable = IXpsSignature_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsSignature {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x6ae4c93e_1ade_42fb_898b_3a5658284857);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignature_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetSignatureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sigid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetSignatureValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturehashvalue: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCertificateEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificateenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCertificateEnumerator: usize,
    pub GetSigningTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sigdatetimestring: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetSigningTimeFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetSigningTimeFormat: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetSignaturePartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetSignaturePartName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub Verify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, sigstatus: *mut XPS_SIGNATURE_STATUS) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    Verify: usize,
    pub GetPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCustomObjectEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCustomObjectEnumerator: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCustomReferenceEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCustomReferenceEnumerator: usize,
    pub GetSignatureXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT,
    pub SetSignatureXml: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturexml: *const u8, count: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsSignatureBlock(::windows_core::IUnknown);
impl IXpsSignatureBlock {
    pub unsafe fn GetRequests(&self) -> ::windows_core::Result<IXpsSignatureRequestCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRequests)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDocumentIndex(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDocumentIndex)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetDocumentName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDocumentName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateRequest<P0>(&self, requestid: P0) -> ::windows_core::Result<IXpsSignatureRequest>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateRequest)(::windows_core::Interface::as_raw(self), requestid.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsSignatureBlock, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsSignatureBlock {
    type Vtable = IXpsSignatureBlock_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsSignatureBlock {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x151fac09_0b97_4ac6_a323_5e4297d4322b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureBlock_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetRequests: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requests: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetPartName: usize,
    pub GetDocumentIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fixeddocumentindex: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetDocumentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fixeddocumentname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetDocumentName: usize,
    pub CreateRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: ::windows_core::PCWSTR, signaturerequest: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsSignatureBlockCollection(::windows_core::IUnknown);
impl IXpsSignatureBlockCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsSignatureBlock> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), index).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsSignatureBlockCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsSignatureBlockCollection {
    type Vtable = IXpsSignatureBlockCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsSignatureBlockCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x23397050_fe99_467a_8dce_9237f074ffe4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureBlockCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signatureblock: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsSignatureCollection(::windows_core::IUnknown);
impl IXpsSignatureCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsSignature> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), index).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsSignatureCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsSignatureCollection {
    type Vtable = IXpsSignatureCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsSignatureCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa2d1d95d_add2_4dff_ab27_6b9c645ff322);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsSignatureManager(::windows_core::IUnknown);
impl IXpsSignatureManager {
    pub unsafe fn LoadPackageFile<P0>(&self, filename: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).LoadPackageFile)(::windows_core::Interface::as_raw(self), filename.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn LoadPackageStream<P0>(&self, stream: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).LoadPackageStream)(::windows_core::Interface::as_raw(self), stream.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub unsafe fn Sign<P0>(&self, signoptions: P0, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::Result<IXpsSignature>
    where
        P0: ::windows_core::IntoParam<IXpsSigningOptions>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).Sign)(::windows_core::Interface::as_raw(self), signoptions.into_param().abi(), x509certificate, &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetSignatureOriginPartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignatureOriginPartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetSignatureOriginPartName<P0>(&self, signatureoriginpartname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).SetSignatureOriginPartName)(::windows_core::Interface::as_raw(self), signatureoriginpartname.into_param().abi()).ok()
    }
    pub unsafe fn GetSignatures(&self) -> ::windows_core::Result<IXpsSignatureCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignatures)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn AddSignatureBlock<P0>(&self, partname: P0, fixeddocumentindex: u32) -> ::windows_core::Result<IXpsSignatureBlock>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AddSignatureBlock)(::windows_core::Interface::as_raw(self), partname.into_param().abi(), fixeddocumentindex, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSignatureBlocks(&self) -> ::windows_core::Result<IXpsSignatureBlockCollection> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignatureBlocks)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn CreateSigningOptions(&self) -> ::windows_core::Result<IXpsSigningOptions> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreateSigningOptions)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn SavePackageToFile<P0>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SavePackageToFile)(::windows_core::Interface::as_raw(self), filename.into_param().abi(), securityattributes, flagsandattributes).ok()
    }
    #[doc = "Required features: `\"Win32_System_Com\"`"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SavePackageToStream<P0>(&self, stream: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::System::Com::IStream>,
    {
        (::windows_core::Interface::vtable(self).SavePackageToStream)(::windows_core::Interface::as_raw(self), stream.into_param().abi()).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsSignatureManager, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsSignatureManager {
    type Vtable = IXpsSignatureManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsSignatureManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd3e8d338_fdc4_4afc_80b5_d532a1782ee1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub LoadPackageFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub LoadPackageStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    LoadPackageStream: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
    pub Sign: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signoptions: *mut ::core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, signature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography")))]
    Sign: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetSignatureOriginPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetSignatureOriginPartName: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetSignatureOriginPartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetSignatureOriginPartName: usize,
    pub GetSignatures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatures: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub AddSignatureBlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, fixeddocumentindex: u32, signatureblock: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    AddSignatureBlock: usize,
    pub GetSignatureBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureblocks: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CreateSigningOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signingoptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub SavePackageToFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))]
    SavePackageToFile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SavePackageToStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SavePackageToStream: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsSignatureRequest(::windows_core::IUnknown);
impl IXpsSignatureRequest {
    pub unsafe fn GetIntent(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetIntent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetIntent<P0>(&self, intent: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetIntent)(::windows_core::Interface::as_raw(self), intent.into_param().abi()).ok()
    }
    pub unsafe fn GetRequestedSigner(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRequestedSigner)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRequestedSigner<P0>(&self, signername: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetRequestedSigner)(::windows_core::Interface::as_raw(self), signername.into_param().abi()).ok()
    }
    pub unsafe fn GetRequestSignByDate(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRequestSignByDate)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRequestSignByDate<P0>(&self, datestring: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetRequestSignByDate)(::windows_core::Interface::as_raw(self), datestring.into_param().abi()).ok()
    }
    pub unsafe fn GetSigningLocale(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSigningLocale)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSigningLocale<P0>(&self, place: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSigningLocale)(::windows_core::Interface::as_raw(self), place.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetSpotLocation(&self, pageindex: *mut i32, pagepartname: *mut ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, x: *mut f32, y: *mut f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).GetSpotLocation)(::windows_core::Interface::as_raw(self), pageindex, ::core::mem::transmute(pagepartname), x, y).ok()
    }
    pub unsafe fn SetSpotLocation(&self, pageindex: i32, x: f32, y: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSpotLocation)(::windows_core::Interface::as_raw(self), pageindex, x, y).ok()
    }
    pub unsafe fn GetRequestId(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetRequestId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSignature(&self) -> ::windows_core::Result<IXpsSignature> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignature)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IXpsSignatureRequest, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsSignatureRequest {
    type Vtable = IXpsSignatureRequest_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsSignatureRequest {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xac58950b_7208_4b2d_b2c4_951083d3b8eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureRequest_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetIntent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, intent: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetIntent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, intent: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetRequestedSigner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signername: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetRequestedSigner: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signername: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetRequestSignByDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datestring: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetRequestSignByDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, datestring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetSigningLocale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, place: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetSigningLocale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, place: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetSpotLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pageindex: *mut i32, pagepartname: *mut *mut ::core::ffi::c_void, x: *mut f32, y: *mut f32) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetSpotLocation: usize,
    pub SetSpotLocation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pageindex: i32, x: f32, y: f32) -> ::windows_core::HRESULT,
    pub GetRequestId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub GetSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsSignatureRequestCollection(::windows_core::IUnknown);
impl IXpsSignatureRequestCollection {
    pub unsafe fn GetCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetAt(&self, index: u32) -> ::windows_core::Result<IXpsSignatureRequest> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAt)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn RemoveAt(&self, index: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).RemoveAt)(::windows_core::Interface::as_raw(self), index).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsSignatureRequestCollection, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsSignatureRequestCollection {
    type Vtable = IXpsSignatureRequestCollection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsSignatureRequestCollection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf0253e68_9f19_412e_9b4f_54d3b0ac6cd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSignatureRequestCollection_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
    pub GetAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, signaturerequest: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub RemoveAt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IXpsSigningOptions(::windows_core::IUnknown);
impl IXpsSigningOptions {
    pub unsafe fn GetSignatureId(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignatureId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSignatureId<P0>(&self, signatureid: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSignatureId)(::windows_core::Interface::as_raw(self), signatureid.into_param().abi()).ok()
    }
    pub unsafe fn GetSignatureMethod(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignatureMethod)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetSignatureMethod<P0>(&self, signaturemethod: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetSignatureMethod)(::windows_core::Interface::as_raw(self), signaturemethod.into_param().abi()).ok()
    }
    pub unsafe fn GetDigestMethod(&self) -> ::windows_core::Result<::windows_core::PWSTR> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDigestMethod)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDigestMethod<P0>(&self, digestmethod: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).SetDigestMethod)(::windows_core::Interface::as_raw(self), digestmethod.into_param().abi()).ok()
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetSignaturePartName(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSignaturePartName)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetSignaturePartName<P0>(&self, signaturepartname: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::Packaging::Opc::IOpcPartUri>,
    {
        (::windows_core::Interface::vtable(self).SetSignaturePartName)(::windows_core::Interface::as_raw(self), signaturepartname.into_param().abi()).ok()
    }
    pub unsafe fn GetPolicy(&self) -> ::windows_core::Result<XPS_SIGN_POLICY> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPolicy)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetPolicy(&self, policy: XPS_SIGN_POLICY) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPolicy)(::windows_core::Interface::as_raw(self), policy).ok()
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetSigningTimeFormat(&self) -> ::windows_core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSigningTimeFormat)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn SetSigningTimeFormat(&self, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSigningTimeFormat)(::windows_core::Interface::as_raw(self), timeformat).ok()
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomObjects(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectSet> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCustomObjects)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCustomReferences(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcSignatureReferenceSet> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCustomReferences)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`"]
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub unsafe fn GetCertificateSet(&self) -> ::windows_core::Result<super::Packaging::Opc::IOpcCertificateSet> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetCertificateSet)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetFlags(&self) -> ::windows_core::Result<XPS_SIGN_FLAGS> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetFlags)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetFlags(&self, flags: XPS_SIGN_FLAGS) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetFlags)(::windows_core::Interface::as_raw(self), flags).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IXpsSigningOptions, ::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IXpsSigningOptions {
    type Vtable = IXpsSigningOptions_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IXpsSigningOptions {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7718eae4_3215_49be_af5b_594fef7fcfa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXpsSigningOptions_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetSignatureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetSignatureId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signatureid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetSignatureMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturemethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetSignatureMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturemethod: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub GetDigestMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT,
    pub SetDigestMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, digestmethod: ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub GetSignaturePartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    GetSignaturePartName: usize,
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub SetSignaturePartName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com")))]
    SetSignaturePartName: usize,
    pub GetPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> ::windows_core::HRESULT,
    pub SetPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, policy: XPS_SIGN_POLICY) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetSigningTimeFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetSigningTimeFormat: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub SetSigningTimeFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    SetSigningTimeFormat: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCustomObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCustomObjects: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCustomReferences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, customreferenceset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCustomReferences: usize,
    #[cfg(feature = "Win32_Storage_Packaging_Opc")]
    pub GetCertificateSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, certificateset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Packaging_Opc"))]
    GetCertificateSet: usize,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: *mut XPS_SIGN_FLAGS) -> ::windows_core::HRESULT,
    pub SetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, flags: XPS_SIGN_FLAGS) -> ::windows_core::HRESULT,
}
pub const DC_BINNAMES: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(12u16);
pub const DC_BINS: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(6u16);
pub const DC_COLLATE: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(22u16);
pub const DC_COLORDEVICE: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(32u16);
pub const DC_COPIES: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(18u16);
pub const DC_DRIVER: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(11u16);
pub const DC_DUPLEX: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(7u16);
pub const DC_ENUMRESOLUTIONS: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(13u16);
pub const DC_EXTRA: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(9u16);
pub const DC_FIELDS: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(1u16);
pub const DC_FILEDEPENDENCIES: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(14u16);
pub const DC_MAXEXTENT: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(5u16);
pub const DC_MEDIAREADY: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(29u16);
pub const DC_MEDIATYPENAMES: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(34u16);
pub const DC_MEDIATYPES: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(35u16);
pub const DC_MINEXTENT: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(4u16);
pub const DC_NUP: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(33u16);
pub const DC_ORIENTATION: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(17u16);
pub const DC_PAPERNAMES: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(16u16);
pub const DC_PAPERS: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(2u16);
pub const DC_PAPERSIZE: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(3u16);
pub const DC_PERSONALITY: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(25u16);
pub const DC_PRINTERMEM: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(28u16);
pub const DC_PRINTRATE: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(26u16);
pub const DC_PRINTRATEPPM: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(31u16);
pub const DC_PRINTRATEUNIT: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(27u16);
pub const DC_SIZE: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(8u16);
pub const DC_STAPLE: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(30u16);
pub const DC_TRUETYPE: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(15u16);
pub const DC_VERSION: PRINTER_DEVICE_CAPABILITIES = PRINTER_DEVICE_CAPABILITIES(10u16);
pub const PSINJECT_BEGINDEFAULTS: PSINJECT_POINT = PSINJECT_POINT(12u16);
pub const PSINJECT_BEGINPAGESETUP: PSINJECT_POINT = PSINJECT_POINT(101u16);
pub const PSINJECT_BEGINPROLOG: PSINJECT_POINT = PSINJECT_POINT(14u16);
pub const PSINJECT_BEGINSETUP: PSINJECT_POINT = PSINJECT_POINT(16u16);
pub const PSINJECT_BEGINSTREAM: PSINJECT_POINT = PSINJECT_POINT(1u16);
pub const PSINJECT_BOUNDINGBOX: PSINJECT_POINT = PSINJECT_POINT(9u16);
pub const PSINJECT_COMMENTS: PSINJECT_POINT = PSINJECT_POINT(11u16);
pub const PSINJECT_DOCNEEDEDRES: PSINJECT_POINT = PSINJECT_POINT(5u16);
pub const PSINJECT_DOCSUPPLIEDRES: PSINJECT_POINT = PSINJECT_POINT(6u16);
pub const PSINJECT_DOCUMENTPROCESSCOLORS: PSINJECT_POINT = PSINJECT_POINT(10u16);
pub const PSINJECT_DOCUMENTPROCESSCOLORSATEND: PSINJECT_POINT = PSINJECT_POINT(21u16);
pub const PSINJECT_ENDDEFAULTS: PSINJECT_POINT = PSINJECT_POINT(13u16);
pub const PSINJECT_ENDPAGECOMMENTS: PSINJECT_POINT = PSINJECT_POINT(107u16);
pub const PSINJECT_ENDPAGESETUP: PSINJECT_POINT = PSINJECT_POINT(102u16);
pub const PSINJECT_ENDPROLOG: PSINJECT_POINT = PSINJECT_POINT(15u16);
pub const PSINJECT_ENDSETUP: PSINJECT_POINT = PSINJECT_POINT(17u16);
pub const PSINJECT_ENDSTREAM: PSINJECT_POINT = PSINJECT_POINT(20u16);
pub const PSINJECT_EOF: PSINJECT_POINT = PSINJECT_POINT(19u16);
pub const PSINJECT_ORIENTATION: PSINJECT_POINT = PSINJECT_POINT(8u16);
pub const PSINJECT_PAGEBBOX: PSINJECT_POINT = PSINJECT_POINT(106u16);
pub const PSINJECT_PAGENUMBER: PSINJECT_POINT = PSINJECT_POINT(100u16);
pub const PSINJECT_PAGEORDER: PSINJECT_POINT = PSINJECT_POINT(7u16);
pub const PSINJECT_PAGES: PSINJECT_POINT = PSINJECT_POINT(4u16);
pub const PSINJECT_PAGESATEND: PSINJECT_POINT = PSINJECT_POINT(3u16);
pub const PSINJECT_PAGETRAILER: PSINJECT_POINT = PSINJECT_POINT(103u16);
pub const PSINJECT_PLATECOLOR: PSINJECT_POINT = PSINJECT_POINT(104u16);
pub const PSINJECT_PSADOBE: PSINJECT_POINT = PSINJECT_POINT(2u16);
pub const PSINJECT_SHOWPAGE: PSINJECT_POINT = PSINJECT_POINT(105u16);
pub const PSINJECT_TRAILER: PSINJECT_POINT = PSINJECT_POINT(18u16);
pub const PSINJECT_VMRESTORE: PSINJECT_POINT = PSINJECT_POINT(201u16);
pub const PSINJECT_VMSAVE: PSINJECT_POINT = PSINJECT_POINT(200u16);
pub const PW_CLIENTONLY: PRINT_WINDOW_FLAGS = PRINT_WINDOW_FLAGS(1u32);
pub const XPS_COLOR_INTERPOLATION_SCRGBLINEAR: XPS_COLOR_INTERPOLATION = XPS_COLOR_INTERPOLATION(1i32);
pub const XPS_COLOR_INTERPOLATION_SRGBLINEAR: XPS_COLOR_INTERPOLATION = XPS_COLOR_INTERPOLATION(2i32);
pub const XPS_COLOR_TYPE_CONTEXT: XPS_COLOR_TYPE = XPS_COLOR_TYPE(3i32);
pub const XPS_COLOR_TYPE_SCRGB: XPS_COLOR_TYPE = XPS_COLOR_TYPE(2i32);
pub const XPS_COLOR_TYPE_SRGB: XPS_COLOR_TYPE = XPS_COLOR_TYPE(1i32);
pub const XPS_DASH_CAP_FLAT: XPS_DASH_CAP = XPS_DASH_CAP(1i32);
pub const XPS_DASH_CAP_ROUND: XPS_DASH_CAP = XPS_DASH_CAP(2i32);
pub const XPS_DASH_CAP_SQUARE: XPS_DASH_CAP = XPS_DASH_CAP(3i32);
pub const XPS_DASH_CAP_TRIANGLE: XPS_DASH_CAP = XPS_DASH_CAP(4i32);
pub const XPS_DOCUMENT_TYPE_OPENXPS: XPS_DOCUMENT_TYPE = XPS_DOCUMENT_TYPE(3i32);
pub const XPS_DOCUMENT_TYPE_UNSPECIFIED: XPS_DOCUMENT_TYPE = XPS_DOCUMENT_TYPE(1i32);
pub const XPS_DOCUMENT_TYPE_XPS: XPS_DOCUMENT_TYPE = XPS_DOCUMENT_TYPE(2i32);
pub const XPS_E_ABSOLUTE_REFERENCE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108159i32);
pub const XPS_E_ALREADY_OWNED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108413i32);
pub const XPS_E_BLEED_BOX_PAGE_DIMENSIONS_NOT_IN_SYNC: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108407i32);
pub const XPS_E_BOTH_PATHFIGURE_AND_ABBR_SYNTAX_PRESENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108409i32);
pub const XPS_E_BOTH_RESOURCE_AND_SOURCEATTR_PRESENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108408i32);
pub const XPS_E_CARET_OUTSIDE_STRING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108923i32);
pub const XPS_E_CARET_OUT_OF_ORDER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108922i32);
pub const XPS_E_COLOR_COMPONENT_OUT_OF_RANGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108410i32);
pub const XPS_E_DICTIONARY_ITEM_NAMED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108671i32);
pub const XPS_E_DUPLICATE_NAMES: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109175i32);
pub const XPS_E_DUPLICATE_RESOURCE_KEYS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109184i32);
pub const XPS_E_INDEX_OUT_OF_RANGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108416i32);
pub const XPS_E_INVALID_BLEED_BOX: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109692i32);
pub const XPS_E_INVALID_CONTENT_BOX: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109685i32);
pub const XPS_E_INVALID_CONTENT_TYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109682i32);
pub const XPS_E_INVALID_FLOAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109689i32);
pub const XPS_E_INVALID_FONT_URI: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109686i32);
pub const XPS_E_INVALID_LANGUAGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109696i32);
pub const XPS_E_INVALID_LOOKUP_TYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109690i32);
pub const XPS_E_INVALID_MARKUP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109684i32);
pub const XPS_E_INVALID_NAME: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109695i32);
pub const XPS_E_INVALID_NUMBER_OF_COLOR_CHANNELS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108158i32);
pub const XPS_E_INVALID_NUMBER_OF_POINTS_IN_CURVE_SEGMENTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108160i32);
pub const XPS_E_INVALID_OBFUSCATED_FONT_URI: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109681i32);
pub const XPS_E_INVALID_PAGE_SIZE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109693i32);
pub const XPS_E_INVALID_RESOURCE_KEY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109694i32);
pub const XPS_E_INVALID_SIGNATUREBLOCK_MARKUP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108789i32);
pub const XPS_E_INVALID_THUMBNAIL_IMAGE_TYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109691i32);
pub const XPS_E_INVALID_XML_ENCODING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109683i32);
pub const XPS_E_MAPPING_OUTSIDE_INDICES: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108924i32);
pub const XPS_E_MAPPING_OUTSIDE_STRING: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108925i32);
pub const XPS_E_MAPPING_OUT_OF_ORDER: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108926i32);
pub const XPS_E_MARKUP_COMPATIBILITY_ELEMENTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108791i32);
pub const XPS_E_MISSING_COLORPROFILE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109436i32);
pub const XPS_E_MISSING_DISCARDCONTROL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109422i32);
pub const XPS_E_MISSING_DOCUMENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109431i32);
pub const XPS_E_MISSING_DOCUMENTSEQUENCE_RELATIONSHIP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109432i32);
pub const XPS_E_MISSING_FONTURI: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109433i32);
pub const XPS_E_MISSING_GLYPHS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109438i32);
pub const XPS_E_MISSING_IMAGE_IN_IMAGEBRUSH: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109426i32);
pub const XPS_E_MISSING_LOOKUP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109439i32);
pub const XPS_E_MISSING_NAME: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109440i32);
pub const XPS_E_MISSING_PAGE_IN_DOCUMENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109428i32);
pub const XPS_E_MISSING_PAGE_IN_PAGEREFERENCE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109427i32);
pub const XPS_E_MISSING_PART_REFERENCE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109424i32);
pub const XPS_E_MISSING_PART_STREAM: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109421i32);
pub const XPS_E_MISSING_REFERRED_DOCUMENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109430i32);
pub const XPS_E_MISSING_REFERRED_PAGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109429i32);
pub const XPS_E_MISSING_RELATIONSHIP_TARGET: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109435i32);
pub const XPS_E_MISSING_RESOURCE_KEY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109425i32);
pub const XPS_E_MISSING_RESOURCE_RELATIONSHIP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109434i32);
pub const XPS_E_MISSING_RESTRICTED_FONT_RELATIONSHIP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109423i32);
pub const XPS_E_MISSING_SEGMENT_DATA: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109437i32);
pub const XPS_E_MULTIPLE_DOCUMENTSEQUENCE_RELATIONSHIPS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109182i32);
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109178i32);
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_DOCUMENTSEQUENCE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109177i32);
pub const XPS_E_MULTIPLE_PRINTTICKETS_ON_PAGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109179i32);
pub const XPS_E_MULTIPLE_REFERENCES_TO_PART: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109176i32);
pub const XPS_E_MULTIPLE_RESOURCES: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109183i32);
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PACKAGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109180i32);
pub const XPS_E_MULTIPLE_THUMBNAILS_ON_PAGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109181i32);
pub const XPS_E_NEGATIVE_FLOAT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108918i32);
pub const XPS_E_NESTED_REMOTE_DICTIONARY: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108670i32);
pub const XPS_E_NOT_ENOUGH_GRADIENT_STOPS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108405i32);
pub const XPS_E_NO_CUSTOM_OBJECTS: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108414i32);
pub const XPS_E_OBJECT_DETACHED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108790i32);
pub const XPS_E_ODD_BIDILEVEL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108921i32);
pub const XPS_E_ONE_TO_ONE_MAPPING_EXPECTED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108920i32);
pub const XPS_E_PACKAGE_ALREADY_OPENED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108793i32);
pub const XPS_E_PACKAGE_NOT_OPENED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108794i32);
pub const XPS_E_PACKAGE_WRITER_NOT_CLOSED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108404i32);
pub const XPS_E_RELATIONSHIP_EXTERNAL: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108406i32);
pub const XPS_E_RESOURCE_NOT_OWNED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108412i32);
pub const XPS_E_RESTRICTED_FONT_NOT_OBFUSCATED: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108919i32);
pub const XPS_E_SIGNATUREID_DUP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108792i32);
pub const XPS_E_SIGREQUESTID_DUP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108795i32);
pub const XPS_E_STRING_TOO_LONG: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108928i32);
pub const XPS_E_TOO_MANY_INDICES: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108927i32);
pub const XPS_E_UNAVAILABLE_PACKAGE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109420i32);
pub const XPS_E_UNEXPECTED_COLORPROFILE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108411i32);
pub const XPS_E_UNEXPECTED_CONTENT_TYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109688i32);
pub const XPS_E_UNEXPECTED_RELATIONSHIP_TYPE: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109680i32);
pub const XPS_E_UNEXPECTED_RESTRICTED_FONT_RELATIONSHIP: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142109679i32);
pub const XPS_E_VISUAL_CIRCULAR_REF: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108415i32);
pub const XPS_E_XKEY_ATTR_PRESENT_OUTSIDE_RES_DICT: ::windows_core::HRESULT = ::windows_core::HRESULT(-2142108672i32);
pub const XPS_FILL_RULE_EVENODD: XPS_FILL_RULE = XPS_FILL_RULE(1i32);
pub const XPS_FILL_RULE_NONZERO: XPS_FILL_RULE = XPS_FILL_RULE(2i32);
pub const XPS_FONT_EMBEDDING_NORMAL: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(1i32);
pub const XPS_FONT_EMBEDDING_OBFUSCATED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(2i32);
pub const XPS_FONT_EMBEDDING_RESTRICTED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(3i32);
pub const XPS_FONT_EMBEDDING_RESTRICTED_UNOBFUSCATED: XPS_FONT_EMBEDDING = XPS_FONT_EMBEDDING(4i32);
pub const XPS_IMAGE_TYPE_JPEG: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(1i32);
pub const XPS_IMAGE_TYPE_JXR: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(5i32);
pub const XPS_IMAGE_TYPE_PNG: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(2i32);
pub const XPS_IMAGE_TYPE_TIFF: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(3i32);
pub const XPS_IMAGE_TYPE_WDP: XPS_IMAGE_TYPE = XPS_IMAGE_TYPE(4i32);
pub const XPS_INTERLEAVING_OFF: XPS_INTERLEAVING = XPS_INTERLEAVING(1i32);
pub const XPS_INTERLEAVING_ON: XPS_INTERLEAVING = XPS_INTERLEAVING(2i32);
pub const XPS_LINE_CAP_FLAT: XPS_LINE_CAP = XPS_LINE_CAP(1i32);
pub const XPS_LINE_CAP_ROUND: XPS_LINE_CAP = XPS_LINE_CAP(2i32);
pub const XPS_LINE_CAP_SQUARE: XPS_LINE_CAP = XPS_LINE_CAP(3i32);
pub const XPS_LINE_CAP_TRIANGLE: XPS_LINE_CAP = XPS_LINE_CAP(4i32);
pub const XPS_LINE_JOIN_BEVEL: XPS_LINE_JOIN = XPS_LINE_JOIN(2i32);
pub const XPS_LINE_JOIN_MITER: XPS_LINE_JOIN = XPS_LINE_JOIN(1i32);
pub const XPS_LINE_JOIN_ROUND: XPS_LINE_JOIN = XPS_LINE_JOIN(3i32);
pub const XPS_OBJECT_TYPE_CANVAS: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(1i32);
pub const XPS_OBJECT_TYPE_GEOMETRY: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(5i32);
pub const XPS_OBJECT_TYPE_GLYPHS: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(2i32);
pub const XPS_OBJECT_TYPE_IMAGE_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(7i32);
pub const XPS_OBJECT_TYPE_LINEAR_GRADIENT_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(8i32);
pub const XPS_OBJECT_TYPE_MATRIX_TRANSFORM: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(4i32);
pub const XPS_OBJECT_TYPE_PATH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(3i32);
pub const XPS_OBJECT_TYPE_RADIAL_GRADIENT_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(9i32);
pub const XPS_OBJECT_TYPE_SOLID_COLOR_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(6i32);
pub const XPS_OBJECT_TYPE_VISUAL_BRUSH: XPS_OBJECT_TYPE = XPS_OBJECT_TYPE(10i32);
pub const XPS_SEGMENT_STROKE_PATTERN_ALL: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(1i32);
pub const XPS_SEGMENT_STROKE_PATTERN_MIXED: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(3i32);
pub const XPS_SEGMENT_STROKE_PATTERN_NONE: XPS_SEGMENT_STROKE_PATTERN = XPS_SEGMENT_STROKE_PATTERN(2i32);
pub const XPS_SEGMENT_TYPE_ARC_LARGE_CLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(1i32);
pub const XPS_SEGMENT_TYPE_ARC_LARGE_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(2i32);
pub const XPS_SEGMENT_TYPE_ARC_SMALL_CLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(3i32);
pub const XPS_SEGMENT_TYPE_ARC_SMALL_COUNTERCLOCKWISE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(4i32);
pub const XPS_SEGMENT_TYPE_BEZIER: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(5i32);
pub const XPS_SEGMENT_TYPE_LINE: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(6i32);
pub const XPS_SEGMENT_TYPE_QUADRATIC_BEZIER: XPS_SEGMENT_TYPE = XPS_SEGMENT_TYPE(7i32);
pub const XPS_SIGNATURE_STATUS_BROKEN: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(3i32);
pub const XPS_SIGNATURE_STATUS_INCOMPLETE: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(2i32);
pub const XPS_SIGNATURE_STATUS_INCOMPLIANT: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(1i32);
pub const XPS_SIGNATURE_STATUS_QUESTIONABLE: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(4i32);
pub const XPS_SIGNATURE_STATUS_VALID: XPS_SIGNATURE_STATUS = XPS_SIGNATURE_STATUS(5i32);
pub const XPS_SIGN_FLAGS_IGNORE_MARKUP_COMPATIBILITY: XPS_SIGN_FLAGS = XPS_SIGN_FLAGS(1i32);
pub const XPS_SIGN_FLAGS_NONE: XPS_SIGN_FLAGS = XPS_SIGN_FLAGS(0i32);
pub const XPS_SIGN_POLICY_ALL: XPS_SIGN_POLICY = XPS_SIGN_POLICY(15i32);
pub const XPS_SIGN_POLICY_CORE_PROPERTIES: XPS_SIGN_POLICY = XPS_SIGN_POLICY(1i32);
pub const XPS_SIGN_POLICY_DISCARD_CONTROL: XPS_SIGN_POLICY = XPS_SIGN_POLICY(8i32);
pub const XPS_SIGN_POLICY_NONE: XPS_SIGN_POLICY = XPS_SIGN_POLICY(0i32);
pub const XPS_SIGN_POLICY_PRINT_TICKET: XPS_SIGN_POLICY = XPS_SIGN_POLICY(4i32);
pub const XPS_SIGN_POLICY_SIGNATURE_RELATIONSHIPS: XPS_SIGN_POLICY = XPS_SIGN_POLICY(2i32);
pub const XPS_SPREAD_METHOD_PAD: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(1i32);
pub const XPS_SPREAD_METHOD_REFLECT: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(2i32);
pub const XPS_SPREAD_METHOD_REPEAT: XPS_SPREAD_METHOD = XPS_SPREAD_METHOD(3i32);
pub const XPS_STYLE_SIMULATION_BOLD: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(3i32);
pub const XPS_STYLE_SIMULATION_BOLDITALIC: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(4i32);
pub const XPS_STYLE_SIMULATION_ITALIC: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(2i32);
pub const XPS_STYLE_SIMULATION_NONE: XPS_STYLE_SIMULATION = XPS_STYLE_SIMULATION(1i32);
pub const XPS_THUMBNAIL_SIZE_LARGE: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(4i32);
pub const XPS_THUMBNAIL_SIZE_MEDIUM: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(3i32);
pub const XPS_THUMBNAIL_SIZE_SMALL: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(2i32);
pub const XPS_THUMBNAIL_SIZE_VERYSMALL: XPS_THUMBNAIL_SIZE = XPS_THUMBNAIL_SIZE(1i32);
pub const XPS_TILE_MODE_FLIPX: XPS_TILE_MODE = XPS_TILE_MODE(3i32);
pub const XPS_TILE_MODE_FLIPXY: XPS_TILE_MODE = XPS_TILE_MODE(5i32);
pub const XPS_TILE_MODE_FLIPY: XPS_TILE_MODE = XPS_TILE_MODE(4i32);
pub const XPS_TILE_MODE_NONE: XPS_TILE_MODE = XPS_TILE_MODE(1i32);
pub const XPS_TILE_MODE_TILE: XPS_TILE_MODE = XPS_TILE_MODE(2i32);
pub const XpsOMObjectFactory: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xe974d26d_3d9b_4d47_88cc_3872f2dc3585);
pub const XpsOMThumbnailGenerator: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e4a23e2_b969_4761_be35_1a8ced58e323);
pub const XpsSignatureManager: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb0c43320_2315_44a2_b70a_0943a140a8ee);
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRINTER_DEVICE_CAPABILITIES(pub u16);
impl ::core::marker::Copy for PRINTER_DEVICE_CAPABILITIES {}
impl ::core::clone::Clone for PRINTER_DEVICE_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PRINTER_DEVICE_CAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PRINTER_DEVICE_CAPABILITIES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PRINTER_DEVICE_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRINTER_DEVICE_CAPABILITIES").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRINT_WINDOW_FLAGS(pub u32);
impl ::core::marker::Copy for PRINT_WINDOW_FLAGS {}
impl ::core::clone::Clone for PRINT_WINDOW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PRINT_WINDOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PRINT_WINDOW_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PRINT_WINDOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRINT_WINDOW_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PSINJECT_POINT(pub u16);
impl ::core::marker::Copy for PSINJECT_POINT {}
impl ::core::clone::Clone for PSINJECT_POINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PSINJECT_POINT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for PSINJECT_POINT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PSINJECT_POINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSINJECT_POINT").field(&self.0).finish()
    }
}
impl PSINJECT_POINT {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PSINJECT_POINT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PSINJECT_POINT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PSINJECT_POINT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PSINJECT_POINT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PSINJECT_POINT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_COLOR_INTERPOLATION(pub i32);
impl ::core::marker::Copy for XPS_COLOR_INTERPOLATION {}
impl ::core::clone::Clone for XPS_COLOR_INTERPOLATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_COLOR_INTERPOLATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_COLOR_INTERPOLATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_COLOR_INTERPOLATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_COLOR_INTERPOLATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_COLOR_TYPE(pub i32);
impl ::core::marker::Copy for XPS_COLOR_TYPE {}
impl ::core::clone::Clone for XPS_COLOR_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_COLOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_COLOR_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_COLOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_COLOR_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_DASH_CAP(pub i32);
impl ::core::marker::Copy for XPS_DASH_CAP {}
impl ::core::clone::Clone for XPS_DASH_CAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_DASH_CAP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_DASH_CAP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_DASH_CAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_DASH_CAP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_DOCUMENT_TYPE(pub i32);
impl ::core::marker::Copy for XPS_DOCUMENT_TYPE {}
impl ::core::clone::Clone for XPS_DOCUMENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_DOCUMENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_DOCUMENT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_DOCUMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_DOCUMENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_FILL_RULE(pub i32);
impl ::core::marker::Copy for XPS_FILL_RULE {}
impl ::core::clone::Clone for XPS_FILL_RULE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_FILL_RULE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_FILL_RULE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_FILL_RULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_FILL_RULE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_FONT_EMBEDDING(pub i32);
impl ::core::marker::Copy for XPS_FONT_EMBEDDING {}
impl ::core::clone::Clone for XPS_FONT_EMBEDDING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_FONT_EMBEDDING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_FONT_EMBEDDING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_FONT_EMBEDDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_FONT_EMBEDDING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_IMAGE_TYPE(pub i32);
impl ::core::marker::Copy for XPS_IMAGE_TYPE {}
impl ::core::clone::Clone for XPS_IMAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_IMAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_IMAGE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_IMAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_IMAGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_INTERLEAVING(pub i32);
impl ::core::marker::Copy for XPS_INTERLEAVING {}
impl ::core::clone::Clone for XPS_INTERLEAVING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_INTERLEAVING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_INTERLEAVING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_INTERLEAVING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_INTERLEAVING").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_LINE_CAP(pub i32);
impl ::core::marker::Copy for XPS_LINE_CAP {}
impl ::core::clone::Clone for XPS_LINE_CAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_LINE_CAP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_LINE_CAP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_LINE_CAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_LINE_CAP").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_LINE_JOIN(pub i32);
impl ::core::marker::Copy for XPS_LINE_JOIN {}
impl ::core::clone::Clone for XPS_LINE_JOIN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_LINE_JOIN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_LINE_JOIN {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_LINE_JOIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_LINE_JOIN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_OBJECT_TYPE(pub i32);
impl ::core::marker::Copy for XPS_OBJECT_TYPE {}
impl ::core::clone::Clone for XPS_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_OBJECT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_SEGMENT_STROKE_PATTERN(pub i32);
impl ::core::marker::Copy for XPS_SEGMENT_STROKE_PATTERN {}
impl ::core::clone::Clone for XPS_SEGMENT_STROKE_PATTERN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_SEGMENT_STROKE_PATTERN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_SEGMENT_STROKE_PATTERN {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_SEGMENT_STROKE_PATTERN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_SEGMENT_STROKE_PATTERN").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_SEGMENT_TYPE(pub i32);
impl ::core::marker::Copy for XPS_SEGMENT_TYPE {}
impl ::core::clone::Clone for XPS_SEGMENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_SEGMENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_SEGMENT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_SEGMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_SEGMENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_SIGNATURE_STATUS(pub i32);
impl ::core::marker::Copy for XPS_SIGNATURE_STATUS {}
impl ::core::clone::Clone for XPS_SIGNATURE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_SIGNATURE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_SIGNATURE_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_SIGNATURE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_SIGNATURE_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_SIGN_FLAGS(pub i32);
impl ::core::marker::Copy for XPS_SIGN_FLAGS {}
impl ::core::clone::Clone for XPS_SIGN_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_SIGN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_SIGN_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_SIGN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_SIGN_FLAGS").field(&self.0).finish()
    }
}
impl XPS_SIGN_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for XPS_SIGN_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for XPS_SIGN_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for XPS_SIGN_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for XPS_SIGN_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for XPS_SIGN_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_SIGN_POLICY(pub i32);
impl ::core::marker::Copy for XPS_SIGN_POLICY {}
impl ::core::clone::Clone for XPS_SIGN_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_SIGN_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_SIGN_POLICY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_SIGN_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_SIGN_POLICY").field(&self.0).finish()
    }
}
impl XPS_SIGN_POLICY {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for XPS_SIGN_POLICY {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for XPS_SIGN_POLICY {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for XPS_SIGN_POLICY {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for XPS_SIGN_POLICY {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for XPS_SIGN_POLICY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_SPREAD_METHOD(pub i32);
impl ::core::marker::Copy for XPS_SPREAD_METHOD {}
impl ::core::clone::Clone for XPS_SPREAD_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_SPREAD_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_SPREAD_METHOD {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_SPREAD_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_SPREAD_METHOD").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_STYLE_SIMULATION(pub i32);
impl ::core::marker::Copy for XPS_STYLE_SIMULATION {}
impl ::core::clone::Clone for XPS_STYLE_SIMULATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_STYLE_SIMULATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_STYLE_SIMULATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_STYLE_SIMULATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_STYLE_SIMULATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_THUMBNAIL_SIZE(pub i32);
impl ::core::marker::Copy for XPS_THUMBNAIL_SIZE {}
impl ::core::clone::Clone for XPS_THUMBNAIL_SIZE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_THUMBNAIL_SIZE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_THUMBNAIL_SIZE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_THUMBNAIL_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_THUMBNAIL_SIZE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct XPS_TILE_MODE(pub i32);
impl ::core::marker::Copy for XPS_TILE_MODE {}
impl ::core::clone::Clone for XPS_TILE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XPS_TILE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for XPS_TILE_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for XPS_TILE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_TILE_MODE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct DOCINFOA {
    pub cbSize: i32,
    pub lpszDocName: ::windows_core::PCSTR,
    pub lpszOutput: ::windows_core::PCSTR,
    pub lpszDatatype: ::windows_core::PCSTR,
    pub fwType: u32,
}
impl ::core::marker::Copy for DOCINFOA {}
impl ::core::clone::Clone for DOCINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOCINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOCINFOA").field("cbSize", &self.cbSize).field("lpszDocName", &self.lpszDocName).field("lpszOutput", &self.lpszOutput).field("lpszDatatype", &self.lpszDatatype).field("fwType", &self.fwType).finish()
    }
}
impl ::windows_core::TypeKind for DOCINFOA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DOCINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.lpszDocName == other.lpszDocName && self.lpszOutput == other.lpszOutput && self.lpszDatatype == other.lpszDatatype && self.fwType == other.fwType
    }
}
impl ::core::cmp::Eq for DOCINFOA {}
impl ::core::default::Default for DOCINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOCINFOW {
    pub cbSize: i32,
    pub lpszDocName: ::windows_core::PCWSTR,
    pub lpszOutput: ::windows_core::PCWSTR,
    pub lpszDatatype: ::windows_core::PCWSTR,
    pub fwType: u32,
}
impl ::core::marker::Copy for DOCINFOW {}
impl ::core::clone::Clone for DOCINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOCINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOCINFOW").field("cbSize", &self.cbSize).field("lpszDocName", &self.lpszDocName).field("lpszOutput", &self.lpszOutput).field("lpszDatatype", &self.lpszDatatype).field("fwType", &self.fwType).finish()
    }
}
impl ::windows_core::TypeKind for DOCINFOW {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for DOCINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.lpszDocName == other.lpszDocName && self.lpszOutput == other.lpszOutput && self.lpszDatatype == other.lpszDatatype && self.fwType == other.fwType
    }
}
impl ::core::cmp::Eq for DOCINFOW {}
impl ::core::default::Default for DOCINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
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
impl ::core::fmt::Debug for DRAWPATRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRAWPATRECT").field("ptPosition", &self.ptPosition).field("ptSize", &self.ptSize).field("wStyle", &self.wStyle).field("wPattern", &self.wPattern).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for DRAWPATRECT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DRAWPATRECT {
    fn eq(&self, other: &Self) -> bool {
        self.ptPosition == other.ptPosition && self.ptSize == other.ptSize && self.wStyle == other.wStyle && self.wPattern == other.wPattern
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DRAWPATRECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRAWPATRECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
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
impl ::windows_core::TypeKind for HPTPROVIDER {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
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
impl ::core::fmt::Debug for PSFEATURE_CUSTPAPER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSFEATURE_CUSTPAPER").field("lOrientation", &self.lOrientation).field("lWidth", &self.lWidth).field("lHeight", &self.lHeight).field("lWidthOffset", &self.lWidthOffset).field("lHeightOffset", &self.lHeightOffset).finish()
    }
}
impl ::windows_core::TypeKind for PSFEATURE_CUSTPAPER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PSFEATURE_CUSTPAPER {
    fn eq(&self, other: &Self) -> bool {
        self.lOrientation == other.lOrientation && self.lWidth == other.lWidth && self.lHeight == other.lHeight && self.lWidthOffset == other.lWidthOffset && self.lHeightOffset == other.lHeightOffset
    }
}
impl ::core::cmp::Eq for PSFEATURE_CUSTPAPER {}
impl ::core::default::Default for PSFEATURE_CUSTPAPER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "Required features: `\"Win32_Foundation\"`"]
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
impl ::core::fmt::Debug for PSFEATURE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSFEATURE_OUTPUT").field("bPageIndependent", &self.bPageIndependent).field("bSetPageDevice", &self.bSetPageDevice).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::TypeKind for PSFEATURE_OUTPUT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PSFEATURE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.bPageIndependent == other.bPageIndependent && self.bSetPageDevice == other.bSetPageDevice
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PSFEATURE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PSFEATURE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::core::fmt::Debug for PSINJECTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSINJECTDATA").field("DataBytes", &self.DataBytes).field("InjectionPoint", &self.InjectionPoint).field("PageNumber", &self.PageNumber).finish()
    }
}
impl ::windows_core::TypeKind for PSINJECTDATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PSINJECTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.DataBytes == other.DataBytes && self.InjectionPoint == other.InjectionPoint && self.PageNumber == other.PageNumber
    }
}
impl ::core::cmp::Eq for PSINJECTDATA {}
impl ::core::default::Default for PSINJECTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::windows_core::TypeKind for XPS_COLOR {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for XPS_COLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::windows_core::TypeKind for XPS_COLOR_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for XPS_COLOR_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::core::fmt::Debug for XPS_COLOR_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_COLOR_0_0").field("channelCount", &self.channelCount).field("channels", &self.channels).finish()
    }
}
impl ::windows_core::TypeKind for XPS_COLOR_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for XPS_COLOR_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.channelCount == other.channelCount && self.channels == other.channels
    }
}
impl ::core::cmp::Eq for XPS_COLOR_0_0 {}
impl ::core::default::Default for XPS_COLOR_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::core::fmt::Debug for XPS_COLOR_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_COLOR_0_1").field("alpha", &self.alpha).field("red", &self.red).field("green", &self.green).field("blue", &self.blue).finish()
    }
}
impl ::windows_core::TypeKind for XPS_COLOR_0_1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for XPS_COLOR_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.alpha == other.alpha && self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}
impl ::core::cmp::Eq for XPS_COLOR_0_1 {}
impl ::core::default::Default for XPS_COLOR_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::core::fmt::Debug for XPS_COLOR_0_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_COLOR_0_2").field("alpha", &self.alpha).field("red", &self.red).field("green", &self.green).field("blue", &self.blue).finish()
    }
}
impl ::windows_core::TypeKind for XPS_COLOR_0_2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for XPS_COLOR_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.alpha == other.alpha && self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}
impl ::core::cmp::Eq for XPS_COLOR_0_2 {}
impl ::core::default::Default for XPS_COLOR_0_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::core::fmt::Debug for XPS_DASH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_DASH").field("length", &self.length).field("gap", &self.gap).finish()
    }
}
impl ::windows_core::TypeKind for XPS_DASH {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for XPS_DASH {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.gap == other.gap
    }
}
impl ::core::cmp::Eq for XPS_DASH {}
impl ::core::default::Default for XPS_DASH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::core::fmt::Debug for XPS_GLYPH_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_GLYPH_INDEX").field("index", &self.index).field("advanceWidth", &self.advanceWidth).field("horizontalOffset", &self.horizontalOffset).field("verticalOffset", &self.verticalOffset).finish()
    }
}
impl ::windows_core::TypeKind for XPS_GLYPH_INDEX {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for XPS_GLYPH_INDEX {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.advanceWidth == other.advanceWidth && self.horizontalOffset == other.horizontalOffset && self.verticalOffset == other.verticalOffset
    }
}
impl ::core::cmp::Eq for XPS_GLYPH_INDEX {}
impl ::core::default::Default for XPS_GLYPH_INDEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::core::fmt::Debug for XPS_GLYPH_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_GLYPH_MAPPING").field("unicodeStringStart", &self.unicodeStringStart).field("unicodeStringLength", &self.unicodeStringLength).field("glyphIndicesStart", &self.glyphIndicesStart).field("glyphIndicesLength", &self.glyphIndicesLength).finish()
    }
}
impl ::windows_core::TypeKind for XPS_GLYPH_MAPPING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for XPS_GLYPH_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        self.unicodeStringStart == other.unicodeStringStart && self.unicodeStringLength == other.unicodeStringLength && self.glyphIndicesStart == other.glyphIndicesStart && self.glyphIndicesLength == other.glyphIndicesLength
    }
}
impl ::core::cmp::Eq for XPS_GLYPH_MAPPING {}
impl ::core::default::Default for XPS_GLYPH_MAPPING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::core::fmt::Debug for XPS_MATRIX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_MATRIX").field("m11", &self.m11).field("m12", &self.m12).field("m21", &self.m21).field("m22", &self.m22).field("m31", &self.m31).field("m32", &self.m32).finish()
    }
}
impl ::windows_core::TypeKind for XPS_MATRIX {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for XPS_MATRIX {
    fn eq(&self, other: &Self) -> bool {
        self.m11 == other.m11 && self.m12 == other.m12 && self.m21 == other.m21 && self.m22 == other.m22 && self.m31 == other.m31 && self.m32 == other.m32
    }
}
impl ::core::cmp::Eq for XPS_MATRIX {}
impl ::core::default::Default for XPS_MATRIX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::core::fmt::Debug for XPS_POINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_POINT").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::windows_core::TypeKind for XPS_POINT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for XPS_POINT {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for XPS_POINT {}
impl ::core::default::Default for XPS_POINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::core::fmt::Debug for XPS_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_RECT").field("x", &self.x).field("y", &self.y).field("width", &self.width).field("height", &self.height).finish()
    }
}
impl ::windows_core::TypeKind for XPS_RECT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for XPS_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.width == other.width && self.height == other.height
    }
}
impl ::core::cmp::Eq for XPS_RECT {}
impl ::core::default::Default for XPS_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
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
impl ::core::fmt::Debug for XPS_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_SIZE").field("width", &self.width).field("height", &self.height).finish()
    }
}
impl ::windows_core::TypeKind for XPS_SIZE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for XPS_SIZE {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}
impl ::core::cmp::Eq for XPS_SIZE {}
impl ::core::default::Default for XPS_SIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type ABORTPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Graphics::Gdi::HDC, param1: i32) -> super::super::Foundation::BOOL>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
