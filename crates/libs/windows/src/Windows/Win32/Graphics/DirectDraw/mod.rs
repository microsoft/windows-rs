#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[inline]
pub unsafe fn DirectDrawCreate<P0>(lpguid: *mut ::windows::core::GUID, lplpdd: *mut ::core::option::Option<IDirectDraw>, punkouter: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows_targets::link ! ( "ddraw.dll""system" fn DirectDrawCreate ( lpguid : *mut ::windows::core::GUID , lplpdd : *mut * mut::core::ffi::c_void , punkouter : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    DirectDrawCreate(lpguid, ::core::mem::transmute(lplpdd), punkouter.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[inline]
pub unsafe fn DirectDrawCreateClipper<P0>(dwflags: u32, lplpddclipper: *mut ::core::option::Option<IDirectDrawClipper>, punkouter: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows_targets::link ! ( "ddraw.dll""system" fn DirectDrawCreateClipper ( dwflags : u32 , lplpddclipper : *mut * mut::core::ffi::c_void , punkouter : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    DirectDrawCreateClipper(dwflags, ::core::mem::transmute(lplpddclipper), punkouter.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[inline]
pub unsafe fn DirectDrawCreateEx<P0>(lpguid: *mut ::windows::core::GUID, lplpdd: *mut *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, punkouter: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
{
    ::windows_targets::link ! ( "ddraw.dll""system" fn DirectDrawCreateEx ( lpguid : *mut ::windows::core::GUID , lplpdd : *mut *mut ::core::ffi::c_void , iid : *const ::windows::core::GUID , punkouter : * mut::core::ffi::c_void ) -> ::windows::core::HRESULT );
    DirectDrawCreateEx(lpguid, lplpdd, iid, punkouter.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectDrawEnumerateA(lpcallback: LPDDENUMCALLBACKA, lpcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "ddraw.dll""system" fn DirectDrawEnumerateA ( lpcallback : LPDDENUMCALLBACKA , lpcontext : *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    DirectDrawEnumerateA(lpcallback, lpcontext).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DirectDrawEnumerateExA(lpcallback: LPDDENUMCALLBACKEXA, lpcontext: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "ddraw.dll""system" fn DirectDrawEnumerateExA ( lpcallback : LPDDENUMCALLBACKEXA , lpcontext : *mut ::core::ffi::c_void , dwflags : u32 ) -> ::windows::core::HRESULT );
    DirectDrawEnumerateExA(lpcallback, lpcontext, dwflags).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn DirectDrawEnumerateExW(lpcallback: LPDDENUMCALLBACKEXW, lpcontext: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "ddraw.dll""system" fn DirectDrawEnumerateExW ( lpcallback : LPDDENUMCALLBACKEXW , lpcontext : *mut ::core::ffi::c_void , dwflags : u32 ) -> ::windows::core::HRESULT );
    DirectDrawEnumerateExW(lpcallback, lpcontext, dwflags).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DirectDrawEnumerateW(lpcallback: LPDDENUMCALLBACKW, lpcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "ddraw.dll""system" fn DirectDrawEnumerateW ( lpcallback : LPDDENUMCALLBACKW , lpcontext : *mut ::core::ffi::c_void ) -> ::windows::core::HRESULT );
    DirectDrawEnumerateW(lpcallback, lpcontext).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[repr(transparent)]
pub struct IDDVideoPortContainer(::windows::core::IUnknown);
impl IDDVideoPortContainer {
    pub unsafe fn CreateVideoPort<P0>(&self, param0: u32, param1: *mut DDVIDEOPORTDESC, param2: *mut ::core::option::Option<IDirectDrawVideoPort>, param3: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).CreateVideoPort)(::windows::core::Interface::as_raw(self), param0, param1, ::core::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    pub unsafe fn EnumVideoPorts(&self, param0: u32, param1: *mut DDVIDEOPORTCAPS, param2: *mut ::core::ffi::c_void, param3: LPDDENUMVIDEOCALLBACK) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumVideoPorts)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn GetVideoPortConnectInfo(&self, param0: u32, pcinfo: *mut u32, param2: ::core::option::Option<*mut DDVIDEOPORTCONNECT>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVideoPortConnectInfo)(::windows::core::Interface::as_raw(self), param0, pcinfo, ::core::mem::transmute(param2.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryVideoPortStatus(&self, param0: u32, param1: *mut DDVIDEOPORTSTATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).QueryVideoPortStatus)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
}
::windows::imp::interface_hierarchy!(IDDVideoPortContainer, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDDVideoPortContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDDVideoPortContainer {}
impl ::core::fmt::Debug for IDDVideoPortContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDDVideoPortContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDDVideoPortContainer {
    type Vtable = IDDVideoPortContainer_Vtbl;
}
impl ::core::clone::Clone for IDDVideoPortContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDDVideoPortContainer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c142760_a733_11ce_a521_0020af0be560);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDDVideoPortContainer_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateVideoPort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTDESC, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumVideoPorts: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTCAPS, param2: *mut ::core::ffi::c_void, param3: LPDDENUMVIDEOCALLBACK) -> ::windows::core::HRESULT,
    pub GetVideoPortConnectInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, pcinfo: *mut u32, param2: *mut DDVIDEOPORTCONNECT) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryVideoPortStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDVIDEOPORTSTATUS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryVideoPortStatus: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[repr(transparent)]
pub struct IDirectDraw(::windows::core::IUnknown);
impl IDirectDraw {
    pub unsafe fn Compact(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Compact)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateClipper<P0>(&self, param0: u32, param1: *mut ::core::option::Option<IDirectDrawClipper>, param2: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).CreateClipper)(::windows::core::Interface::as_raw(self), param0, ::core::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreatePalette<P0>(&self, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::core::option::Option<IDirectDrawPalette>, param3: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).CreatePalette)(::windows::core::Interface::as_raw(self), param0, param1, ::core::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    pub unsafe fn CreateSurface<P0>(&self, param0: *mut DDSURFACEDESC, param1: *mut ::core::option::Option<IDirectDrawSurface>, param2: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).CreateSurface)(::windows::core::Interface::as_raw(self), param0, ::core::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    pub unsafe fn DuplicateSurface<P0>(&self, param0: P0) -> ::windows::core::Result<IDirectDrawSurface>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface>,
    {
        let mut result__ = ::windows::core::zeroed::<IDirectDrawSurface>();
        (::windows::core::Interface::vtable(self).DuplicateSurface)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumDisplayModes)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumSurfaces)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn FlipToGDISurface(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FlipToGDISurface)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetCaps(&self, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCaps)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayMode)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFourCCCodes)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetGDISurface(&self) -> ::windows::core::Result<IDirectDrawSurface> {
        let mut result__ = ::windows::core::zeroed::<IDirectDrawSurface>();
        (::windows::core::Interface::vtable(self).GetGDISurface)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMonitorFrequency(&self, param0: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMonitorFrequency)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetScanLine(&self, param0: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetScanLine)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetVerticalBlankStatus(&self, param0: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVerticalBlankStatus)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn Initialize(&self, param0: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn RestoreDisplayMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RestoreDisplayMode)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).SetCooperativeLevel)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisplayMode)(::windows::core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WaitForVerticalBlank<P0>(&self, param0: u32, param1: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).WaitForVerticalBlank)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDirectDraw, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDirectDraw {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDraw {}
impl ::core::fmt::Debug for IDirectDraw {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDraw").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectDraw {
    type Vtable = IDirectDraw_Vtbl;
}
impl ::core::clone::Clone for IDirectDraw {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDirectDraw {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c14db80_a733_11ce_a521_0020af0be560);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDraw_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Compact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateClipper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreatePalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreatePalette: usize,
    pub CreateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DuplicateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumDisplayModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> ::windows::core::HRESULT,
    pub EnumSurfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> ::windows::core::HRESULT,
    pub FlipToGDISurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::HRESULT,
    pub GetDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT,
    pub GetFourCCCodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows::core::HRESULT,
    pub GetGDISurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMonitorFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT,
    pub GetScanLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT,
    pub GetVerticalBlankStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RestoreDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCooperativeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCooperativeLevel: usize,
    pub SetDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WaitForVerticalBlank: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WaitForVerticalBlank: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[repr(transparent)]
pub struct IDirectDraw2(::windows::core::IUnknown);
impl IDirectDraw2 {
    pub unsafe fn Compact(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Compact)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateClipper<P0>(&self, param0: u32, param1: *mut ::core::option::Option<IDirectDrawClipper>, param2: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).CreateClipper)(::windows::core::Interface::as_raw(self), param0, ::core::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreatePalette<P0>(&self, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::core::option::Option<IDirectDrawPalette>, param3: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).CreatePalette)(::windows::core::Interface::as_raw(self), param0, param1, ::core::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    pub unsafe fn CreateSurface<P0>(&self, param0: *mut DDSURFACEDESC, param1: *mut ::core::option::Option<IDirectDrawSurface>, param2: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).CreateSurface)(::windows::core::Interface::as_raw(self), param0, ::core::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    pub unsafe fn DuplicateSurface<P0>(&self, param0: P0) -> ::windows::core::Result<IDirectDrawSurface>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface>,
    {
        let mut result__ = ::windows::core::zeroed::<IDirectDrawSurface>();
        (::windows::core::Interface::vtable(self).DuplicateSurface)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumDisplayModes)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumSurfaces)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn FlipToGDISurface(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FlipToGDISurface)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetCaps(&self, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCaps)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayMode)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFourCCCodes)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetGDISurface(&self) -> ::windows::core::Result<IDirectDrawSurface> {
        let mut result__ = ::windows::core::zeroed::<IDirectDrawSurface>();
        (::windows::core::Interface::vtable(self).GetGDISurface)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMonitorFrequency(&self, param0: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMonitorFrequency)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetScanLine(&self, param0: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetScanLine)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetVerticalBlankStatus(&self, param0: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVerticalBlankStatus)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn Initialize(&self, param0: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn RestoreDisplayMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RestoreDisplayMode)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).SetCooperativeLevel)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisplayMode)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3, param4).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WaitForVerticalBlank<P0>(&self, param0: u32, param1: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).WaitForVerticalBlank)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi()).ok()
    }
    pub unsafe fn GetAvailableVidMem(&self, param0: *mut DDSCAPS, param1: *mut u32, param2: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAvailableVidMem)(::windows::core::Interface::as_raw(self), param0, param1, param2).ok()
    }
}
::windows::imp::interface_hierarchy!(IDirectDraw2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDirectDraw2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDraw2 {}
impl ::core::fmt::Debug for IDirectDraw2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDraw2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectDraw2 {
    type Vtable = IDirectDraw2_Vtbl;
}
impl ::core::clone::Clone for IDirectDraw2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDirectDraw2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb3a6f3e0_2b43_11cf_a2de_00aa00b93356);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDraw2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Compact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateClipper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreatePalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreatePalette: usize,
    pub CreateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DuplicateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumDisplayModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK) -> ::windows::core::HRESULT,
    pub EnumSurfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK) -> ::windows::core::HRESULT,
    pub FlipToGDISurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::HRESULT,
    pub GetDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT,
    pub GetFourCCCodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows::core::HRESULT,
    pub GetGDISurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMonitorFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT,
    pub GetScanLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT,
    pub GetVerticalBlankStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RestoreDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCooperativeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCooperativeLevel: usize,
    pub SetDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WaitForVerticalBlank: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WaitForVerticalBlank: usize,
    pub GetAvailableVidMem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut u32, param2: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[repr(transparent)]
pub struct IDirectDraw4(::windows::core::IUnknown);
impl IDirectDraw4 {
    pub unsafe fn Compact(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Compact)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateClipper<P0>(&self, param0: u32, param1: *mut ::core::option::Option<IDirectDrawClipper>, param2: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).CreateClipper)(::windows::core::Interface::as_raw(self), param0, ::core::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreatePalette<P0>(&self, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::core::option::Option<IDirectDrawPalette>, param3: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).CreatePalette)(::windows::core::Interface::as_raw(self), param0, param1, ::core::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    pub unsafe fn CreateSurface<P0>(&self, param0: *mut DDSURFACEDESC2, param1: *mut ::core::option::Option<IDirectDrawSurface4>, param2: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).CreateSurface)(::windows::core::Interface::as_raw(self), param0, ::core::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    pub unsafe fn DuplicateSurface<P0>(&self, param0: P0) -> ::windows::core::Result<IDirectDrawSurface4>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface4>,
    {
        let mut result__ = ::windows::core::zeroed::<IDirectDrawSurface4>();
        (::windows::core::Interface::vtable(self).DuplicateSurface)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumDisplayModes)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumSurfaces)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn FlipToGDISurface(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FlipToGDISurface)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetCaps(&self, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCaps)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayMode)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFourCCCodes)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetGDISurface(&self) -> ::windows::core::Result<IDirectDrawSurface4> {
        let mut result__ = ::windows::core::zeroed::<IDirectDrawSurface4>();
        (::windows::core::Interface::vtable(self).GetGDISurface)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMonitorFrequency(&self, param0: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMonitorFrequency)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetScanLine(&self, param0: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetScanLine)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetVerticalBlankStatus(&self, param0: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVerticalBlankStatus)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn Initialize(&self, param0: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn RestoreDisplayMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RestoreDisplayMode)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).SetCooperativeLevel)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisplayMode)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3, param4).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WaitForVerticalBlank<P0>(&self, param0: u32, param1: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).WaitForVerticalBlank)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi()).ok()
    }
    pub unsafe fn GetAvailableVidMem(&self, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAvailableVidMem)(::windows::core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetSurfaceFromDC<P0>(&self, param0: P0) -> ::windows::core::Result<IDirectDrawSurface4>
    where
        P0: ::windows::core::IntoParam<super::Gdi::HDC>,
    {
        let mut result__ = ::windows::core::zeroed::<IDirectDrawSurface4>();
        (::windows::core::Interface::vtable(self).GetSurfaceFromDC)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RestoreAllSurfaces(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RestoreAllSurfaces)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TestCooperativeLevel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TestCooperativeLevel)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDeviceIdentifier(&self, param0: *mut DDDEVICEIDENTIFIER, param1: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeviceIdentifier)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
}
::windows::imp::interface_hierarchy!(IDirectDraw4, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDirectDraw4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDraw4 {}
impl ::core::fmt::Debug for IDirectDraw4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDraw4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectDraw4 {
    type Vtable = IDirectDraw4_Vtbl;
}
impl ::core::clone::Clone for IDirectDraw4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDirectDraw4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c59509a_39bd_11d1_8c4a_00c04fd930c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDraw4_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Compact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateClipper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreatePalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreatePalette: usize,
    pub CreateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DuplicateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumDisplayModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> ::windows::core::HRESULT,
    pub EnumSurfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK2) -> ::windows::core::HRESULT,
    pub FlipToGDISurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::HRESULT,
    pub GetDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT,
    pub GetFourCCCodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows::core::HRESULT,
    pub GetGDISurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMonitorFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT,
    pub GetScanLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT,
    pub GetVerticalBlankStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RestoreDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCooperativeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCooperativeLevel: usize,
    pub SetDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WaitForVerticalBlank: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WaitForVerticalBlank: usize,
    pub GetAvailableVidMem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetSurfaceFromDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC, param1: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetSurfaceFromDC: usize,
    pub RestoreAllSurfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TestCooperativeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDDEVICEIDENTIFIER, param1: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[repr(transparent)]
pub struct IDirectDraw7(::windows::core::IUnknown);
impl IDirectDraw7 {
    pub unsafe fn Compact(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Compact)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn CreateClipper<P0>(&self, param0: u32, param1: *mut ::core::option::Option<IDirectDrawClipper>, param2: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).CreateClipper)(::windows::core::Interface::as_raw(self), param0, ::core::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn CreatePalette<P0>(&self, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut ::core::option::Option<IDirectDrawPalette>, param3: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).CreatePalette)(::windows::core::Interface::as_raw(self), param0, param1, ::core::mem::transmute(param2), param3.into_param().abi()).ok()
    }
    pub unsafe fn CreateSurface<P0>(&self, param0: *mut DDSURFACEDESC2, param1: *mut ::core::option::Option<IDirectDrawSurface7>, param2: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).CreateSurface)(::windows::core::Interface::as_raw(self), param0, ::core::mem::transmute(param1), param2.into_param().abi()).ok()
    }
    pub unsafe fn DuplicateSurface<P0>(&self, param0: P0) -> ::windows::core::Result<IDirectDrawSurface7>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface7>,
    {
        let mut result__ = ::windows::core::zeroed::<IDirectDrawSurface7>();
        (::windows::core::Interface::vtable(self).DuplicateSurface)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnumDisplayModes(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumDisplayModes)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn EnumSurfaces(&self, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK7) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumSurfaces)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn FlipToGDISurface(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FlipToGDISurface)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetCaps(&self, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCaps)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetDisplayMode(&self, param0: *mut DDSURFACEDESC2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayMode)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetFourCCCodes(&self, param0: *mut u32, param1: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFourCCCodes)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetGDISurface(&self) -> ::windows::core::Result<IDirectDrawSurface7> {
        let mut result__ = ::windows::core::zeroed::<IDirectDrawSurface7>();
        (::windows::core::Interface::vtable(self).GetGDISurface)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetMonitorFrequency(&self, param0: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMonitorFrequency)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetScanLine(&self, param0: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetScanLine)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetVerticalBlankStatus(&self, param0: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVerticalBlankStatus)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn Initialize(&self, param0: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn RestoreDisplayMode(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RestoreDisplayMode)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCooperativeLevel<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).SetCooperativeLevel)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn SetDisplayMode(&self, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisplayMode)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3, param4).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn WaitForVerticalBlank<P0>(&self, param0: u32, param1: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).WaitForVerticalBlank)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi()).ok()
    }
    pub unsafe fn GetAvailableVidMem(&self, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAvailableVidMem)(::windows::core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetSurfaceFromDC<P0>(&self, param0: P0) -> ::windows::core::Result<IDirectDrawSurface7>
    where
        P0: ::windows::core::IntoParam<super::Gdi::HDC>,
    {
        let mut result__ = ::windows::core::zeroed::<IDirectDrawSurface7>();
        (::windows::core::Interface::vtable(self).GetSurfaceFromDC)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn RestoreAllSurfaces(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RestoreAllSurfaces)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn TestCooperativeLevel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).TestCooperativeLevel)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetDeviceIdentifier(&self, param0: *mut DDDEVICEIDENTIFIER2, param1: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeviceIdentifier)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartModeTest(&self, param0: *mut super::super::Foundation::SIZE, param1: u32, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartModeTest)(::windows::core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn EvaluateMode(&self, param0: u32, param1: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EvaluateMode)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
}
::windows::imp::interface_hierarchy!(IDirectDraw7, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDirectDraw7 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDraw7 {}
impl ::core::fmt::Debug for IDirectDraw7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDraw7").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectDraw7 {
    type Vtable = IDirectDraw7_Vtbl;
}
impl ::core::clone::Clone for IDirectDraw7 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDirectDraw7 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15e65ec0_3b9c_11d2_b92f_00609797ea5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDraw7_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Compact: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateClipper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub CreatePalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut super::Gdi::PALETTEENTRY, param2: *mut *mut ::core::ffi::c_void, param3: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    CreatePalette: usize,
    pub CreateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: *mut *mut ::core::ffi::c_void, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DuplicateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumDisplayModes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMMODESCALLBACK2) -> ::windows::core::HRESULT,
    pub EnumSurfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void, param3: LPDDENUMSURFACESCALLBACK7) -> ::windows::core::HRESULT,
    pub FlipToGDISurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDCAPS_DX7, param1: *mut DDCAPS_DX7) -> ::windows::core::HRESULT,
    pub GetDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT,
    pub GetFourCCCodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32, param1: *mut u32) -> ::windows::core::HRESULT,
    pub GetGDISurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMonitorFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT,
    pub GetScanLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT,
    pub GetVerticalBlankStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub RestoreDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCooperativeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HWND, param1: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCooperativeLevel: usize,
    pub SetDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: u32, param4: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub WaitForVerticalBlank: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    WaitForVerticalBlank: usize,
    pub GetAvailableVidMem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut u32, param2: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetSurfaceFromDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC, param1: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetSurfaceFromDC: usize,
    pub RestoreAllSurfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TestCooperativeLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceIdentifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDDEVICEIDENTIFIER2, param1: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StartModeTest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::SIZE, param1: u32, param2: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartModeTest: usize,
    pub EvaluateMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[repr(transparent)]
pub struct IDirectDrawClipper(::windows::core::IUnknown);
impl IDirectDrawClipper {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetClipList(&self, param0: *mut super::super::Foundation::RECT, param1: *mut super::Gdi::RGNDATA, param2: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClipList)(::windows::core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHWnd(&self, param0: *mut super::super::Foundation::HWND) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetHWnd)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDraw>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsClipListChanged(&self, param0: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsClipListChanged)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn SetClipList(&self, param0: *mut super::Gdi::RGNDATA, param1: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClipList)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHWnd<P0>(&self, param0: u32, param1: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        (::windows::core::Interface::vtable(self).SetHWnd)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDirectDrawClipper, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDirectDrawClipper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawClipper {}
impl ::core::fmt::Debug for IDirectDrawClipper {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawClipper").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectDrawClipper {
    type Vtable = IDirectDrawClipper_Vtbl;
}
impl ::core::clone::Clone for IDirectDrawClipper {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDirectDrawClipper {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c14db85_a733_11ce_a521_0020af0be560);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawClipper_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub GetClipList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut super::Gdi::RGNDATA, param2: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    GetClipList: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHWnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHWnd: usize,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsClipListChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsClipListChanged: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub SetClipList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::RGNDATA, param1: u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    SetClipList: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHWnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHWnd: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[repr(transparent)]
pub struct IDirectDrawColorControl(::windows::core::IUnknown);
impl IDirectDrawColorControl {
    pub unsafe fn GetColorControls(&self, param0: *mut DDCOLORCONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColorControls)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn SetColorControls(&self, param0: *mut DDCOLORCONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColorControls)(::windows::core::Interface::as_raw(self), param0).ok()
    }
}
::windows::imp::interface_hierarchy!(IDirectDrawColorControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDirectDrawColorControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawColorControl {}
impl ::core::fmt::Debug for IDirectDrawColorControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawColorControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectDrawColorControl {
    type Vtable = IDirectDrawColorControl_Vtbl;
}
impl ::core::clone::Clone for IDirectDrawColorControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDirectDrawColorControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b9f0ee0_0d7e_11d0_9b06_00a0c903a3b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawColorControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetColorControls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows::core::HRESULT,
    pub SetColorControls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[repr(transparent)]
pub struct IDirectDrawGammaControl(::windows::core::IUnknown);
impl IDirectDrawGammaControl {
    pub unsafe fn GetGammaRamp(&self, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetGammaRamp)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetGammaRamp(&self, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGammaRamp)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
}
::windows::imp::interface_hierarchy!(IDirectDrawGammaControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDirectDrawGammaControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawGammaControl {}
impl ::core::fmt::Debug for IDirectDrawGammaControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawGammaControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectDrawGammaControl {
    type Vtable = IDirectDrawGammaControl_Vtbl;
}
impl ::core::clone::Clone for IDirectDrawGammaControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDirectDrawGammaControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69c11c3e_b46b_11d1_ad7a_00c04fc29b4e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawGammaControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetGammaRamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows::core::HRESULT,
    pub SetGammaRamp: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDGAMMARAMP) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[repr(transparent)]
pub struct IDirectDrawKernel(::windows::core::IUnknown);
impl IDirectDrawKernel {
    pub unsafe fn GetCaps(&self, param0: *mut DDKERNELCAPS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCaps)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetKernelHandle(&self, param0: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetKernelHandle)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn ReleaseKernelHandle(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseKernelHandle)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDirectDrawKernel, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDirectDrawKernel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawKernel {}
impl ::core::fmt::Debug for IDirectDrawKernel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawKernel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectDrawKernel {
    type Vtable = IDirectDrawKernel_Vtbl;
}
impl ::core::clone::Clone for IDirectDrawKernel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDirectDrawKernel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d56c120_6a08_11d0_9b06_00a0c903a3b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawKernel_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDKERNELCAPS) -> ::windows::core::HRESULT,
    pub GetKernelHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut usize) -> ::windows::core::HRESULT,
    pub ReleaseKernelHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[repr(transparent)]
pub struct IDirectDrawPalette(::windows::core::IUnknown);
impl IDirectDrawPalette {
    pub unsafe fn GetCaps(&self, param0: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCaps)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetEntries(&self, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetEntries)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: u32, param2: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDraw>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), param1, param2).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn SetEntries(&self, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEntries)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
}
::windows::imp::interface_hierarchy!(IDirectDrawPalette, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDirectDrawPalette {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawPalette {}
impl ::core::fmt::Debug for IDirectDrawPalette {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawPalette").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectDrawPalette {
    type Vtable = IDirectDrawPalette_Vtbl;
}
impl ::core::clone::Clone for IDirectDrawPalette {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDirectDrawPalette {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c14db84_a733_11ce_a521_0020af0be560);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawPalette_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetEntries: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32, param2: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    Initialize: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub SetEntries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32, param3: *mut super::Gdi::PALETTEENTRY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    SetEntries: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[repr(transparent)]
pub struct IDirectDrawSurface(::windows::core::IUnknown);
impl IDirectDrawSurface {
    pub unsafe fn AddAttachedSurface<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface>,
    {
        (::windows::core::Interface::vtable(self).AddAttachedSurface)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddOverlayDirtyRect(&self, param0: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddOverlayDirtyRect)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Blt<P0>(&self, param0: *mut super::super::Foundation::RECT, param1: P0, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface>,
    {
        (::windows::core::Interface::vtable(self).Blt)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi(), param2, param3, param4).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BltBatch)(::windows::core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BltFast<P0>(&self, param0: u32, param1: u32, param2: P0, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface>,
    {
        (::windows::core::Interface::vtable(self).BltFast)(::windows::core::Interface::as_raw(self), param0, param1, param2.into_param().abi(), param3, param4).ok()
    }
    pub unsafe fn DeleteAttachedSurface<P0>(&self, param0: u32, param1: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface>,
    {
        (::windows::core::Interface::vtable(self).DeleteAttachedSurface)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi()).ok()
    }
    pub unsafe fn EnumAttachedSurfaces(&self, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumAttachedSurfaces)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn EnumOverlayZOrders(&self, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumOverlayZOrders)(::windows::core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn Flip<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface>,
    {
        (::windows::core::Interface::vtable(self).Flip)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn GetAttachedSurface(&self, param0: *mut DDSCAPS, param1: *mut ::core::option::Option<IDirectDrawSurface>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttachedSurface)(::windows::core::Interface::as_raw(self), param0, ::core::mem::transmute(param1)).ok()
    }
    pub unsafe fn GetBltStatus(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBltStatus)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetCaps(&self, param0: *mut DDSCAPS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCaps)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetClipper(&self) -> ::windows::core::Result<IDirectDrawClipper> {
        let mut result__ = ::windows::core::zeroed::<IDirectDrawClipper>();
        (::windows::core::Interface::vtable(self).GetClipper)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColorKey)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetDC(&self, param0: *mut super::Gdi::HDC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDC)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetFlipStatus(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFlipStatus)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOverlayPosition)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetPalette(&self) -> ::windows::core::Result<IDirectDrawPalette> {
        let mut result__ = ::windows::core::zeroed::<IDirectDrawPalette>();
        (::windows::core::Interface::vtable(self).GetPalette)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPixelFormat(&self, param0: *mut DDPIXELFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPixelFormat)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSurfaceDesc)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: *mut DDSURFACEDESC) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDraw>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn IsLost(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsLost)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Lock<P0>(&self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).Lock)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn ReleaseDC<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Gdi::HDC>,
    {
        (::windows::core::Interface::vtable(self).ReleaseDC)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    pub unsafe fn Restore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Restore)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetClipper<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawClipper>,
    {
        (::windows::core::Interface::vtable(self).SetClipper)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    pub unsafe fn SetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColorKey)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetOverlayPosition(&self, param0: i32, param1: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOverlayPosition)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetPalette<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawPalette>,
    {
        (::windows::core::Interface::vtable(self).SetPalette)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    pub unsafe fn Unlock(&self, param0: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unlock)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateOverlay<P0>(&self, param0: *mut super::super::Foundation::RECT, param1: P0, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface>,
    {
        (::windows::core::Interface::vtable(self).UpdateOverlay)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi(), param2, param3, param4).ok()
    }
    pub unsafe fn UpdateOverlayDisplay(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateOverlayDisplay)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn UpdateOverlayZOrder<P0>(&self, param0: u32, param1: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface>,
    {
        (::windows::core::Interface::vtable(self).UpdateOverlayZOrder)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDirectDrawSurface, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDirectDrawSurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawSurface {}
impl ::core::fmt::Debug for IDirectDrawSurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawSurface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectDrawSurface {
    type Vtable = IDirectDrawSurface_Vtbl;
}
impl ::core::clone::Clone for IDirectDrawSurface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDirectDrawSurface {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6c14db81_a733_11ce_a521_0020af0be560);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawSurface_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddAttachedSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddOverlayDirtyRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddOverlayDirtyRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Blt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Blt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BltBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BltBatch: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BltFast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut ::core::ffi::c_void, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BltFast: usize,
    pub DeleteAttachedSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumAttachedSurfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> ::windows::core::HRESULT,
    pub EnumOverlayZOrders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> ::windows::core::HRESULT,
    pub Flip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows::core::HRESULT,
    pub GetAttachedSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBltStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows::core::HRESULT,
    pub GetClipper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetColorKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetDC: usize,
    pub GetFlipStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub GetOverlayPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT,
    pub GetPalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT,
    pub GetSurfaceDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut DDSURFACEDESC) -> ::windows::core::HRESULT,
    pub IsLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Lock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Lock: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub ReleaseDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    ReleaseDC: usize,
    pub Restore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetClipper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetColorKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT,
    pub SetOverlayPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateOverlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateOverlay: usize,
    pub UpdateOverlayDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub UpdateOverlayZOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[repr(transparent)]
pub struct IDirectDrawSurface2(::windows::core::IUnknown);
impl IDirectDrawSurface2 {
    pub unsafe fn AddAttachedSurface<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface2>,
    {
        (::windows::core::Interface::vtable(self).AddAttachedSurface)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddOverlayDirtyRect(&self, param0: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddOverlayDirtyRect)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Blt<P0>(&self, param0: *mut super::super::Foundation::RECT, param1: P0, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface2>,
    {
        (::windows::core::Interface::vtable(self).Blt)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi(), param2, param3, param4).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BltBatch)(::windows::core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BltFast<P0>(&self, param0: u32, param1: u32, param2: P0, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface2>,
    {
        (::windows::core::Interface::vtable(self).BltFast)(::windows::core::Interface::as_raw(self), param0, param1, param2.into_param().abi(), param3, param4).ok()
    }
    pub unsafe fn DeleteAttachedSurface<P0>(&self, param0: u32, param1: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface2>,
    {
        (::windows::core::Interface::vtable(self).DeleteAttachedSurface)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi()).ok()
    }
    pub unsafe fn EnumAttachedSurfaces(&self, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumAttachedSurfaces)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn EnumOverlayZOrders(&self, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumOverlayZOrders)(::windows::core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn Flip<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface2>,
    {
        (::windows::core::Interface::vtable(self).Flip)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn GetAttachedSurface(&self, param0: *mut DDSCAPS, param1: *mut ::core::option::Option<IDirectDrawSurface2>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttachedSurface)(::windows::core::Interface::as_raw(self), param0, ::core::mem::transmute(param1)).ok()
    }
    pub unsafe fn GetBltStatus(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBltStatus)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetCaps(&self, param0: *mut DDSCAPS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCaps)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetClipper(&self) -> ::windows::core::Result<IDirectDrawClipper> {
        let mut result__ = ::windows::core::zeroed::<IDirectDrawClipper>();
        (::windows::core::Interface::vtable(self).GetClipper)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColorKey)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetDC(&self, param0: *mut super::Gdi::HDC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDC)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetFlipStatus(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFlipStatus)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOverlayPosition)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetPalette(&self) -> ::windows::core::Result<IDirectDrawPalette> {
        let mut result__ = ::windows::core::zeroed::<IDirectDrawPalette>();
        (::windows::core::Interface::vtable(self).GetPalette)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPixelFormat(&self, param0: *mut DDPIXELFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPixelFormat)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSurfaceDesc)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: *mut DDSURFACEDESC) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDraw>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn IsLost(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsLost)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Lock<P0>(&self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).Lock)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn ReleaseDC<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Gdi::HDC>,
    {
        (::windows::core::Interface::vtable(self).ReleaseDC)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    pub unsafe fn Restore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Restore)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetClipper<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawClipper>,
    {
        (::windows::core::Interface::vtable(self).SetClipper)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    pub unsafe fn SetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColorKey)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetOverlayPosition(&self, param0: i32, param1: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOverlayPosition)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetPalette<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawPalette>,
    {
        (::windows::core::Interface::vtable(self).SetPalette)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    pub unsafe fn Unlock(&self, param0: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unlock)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateOverlay<P0>(&self, param0: *mut super::super::Foundation::RECT, param1: P0, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface2>,
    {
        (::windows::core::Interface::vtable(self).UpdateOverlay)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi(), param2, param3, param4).ok()
    }
    pub unsafe fn UpdateOverlayDisplay(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateOverlayDisplay)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn UpdateOverlayZOrder<P0>(&self, param0: u32, param1: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface2>,
    {
        (::windows::core::Interface::vtable(self).UpdateOverlayZOrder)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi()).ok()
    }
    pub unsafe fn GetDDInterface(&self, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDDInterface)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn PageLock(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PageLock)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn PageUnlock(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PageUnlock)(::windows::core::Interface::as_raw(self), param0).ok()
    }
}
::windows::imp::interface_hierarchy!(IDirectDrawSurface2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDirectDrawSurface2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawSurface2 {}
impl ::core::fmt::Debug for IDirectDrawSurface2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawSurface2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectDrawSurface2 {
    type Vtable = IDirectDrawSurface2_Vtbl;
}
impl ::core::clone::Clone for IDirectDrawSurface2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDirectDrawSurface2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x57805885_6eec_11cf_9441_a82303c10e27);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawSurface2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddAttachedSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddOverlayDirtyRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddOverlayDirtyRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Blt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Blt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BltBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BltBatch: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BltFast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut ::core::ffi::c_void, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BltFast: usize,
    pub DeleteAttachedSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumAttachedSurfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> ::windows::core::HRESULT,
    pub EnumOverlayZOrders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> ::windows::core::HRESULT,
    pub Flip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows::core::HRESULT,
    pub GetAttachedSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBltStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows::core::HRESULT,
    pub GetClipper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetColorKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetDC: usize,
    pub GetFlipStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub GetOverlayPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT,
    pub GetPalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT,
    pub GetSurfaceDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut DDSURFACEDESC) -> ::windows::core::HRESULT,
    pub IsLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Lock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Lock: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub ReleaseDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    ReleaseDC: usize,
    pub Restore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetClipper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetColorKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT,
    pub SetOverlayPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateOverlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateOverlay: usize,
    pub UpdateOverlayDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub UpdateOverlayZOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDDInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PageLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub PageUnlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[repr(transparent)]
pub struct IDirectDrawSurface3(::windows::core::IUnknown);
impl IDirectDrawSurface3 {
    pub unsafe fn AddAttachedSurface<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface3>,
    {
        (::windows::core::Interface::vtable(self).AddAttachedSurface)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddOverlayDirtyRect(&self, param0: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddOverlayDirtyRect)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Blt<P0>(&self, param0: *mut super::super::Foundation::RECT, param1: P0, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface3>,
    {
        (::windows::core::Interface::vtable(self).Blt)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi(), param2, param3, param4).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BltBatch)(::windows::core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BltFast<P0>(&self, param0: u32, param1: u32, param2: P0, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface3>,
    {
        (::windows::core::Interface::vtable(self).BltFast)(::windows::core::Interface::as_raw(self), param0, param1, param2.into_param().abi(), param3, param4).ok()
    }
    pub unsafe fn DeleteAttachedSurface<P0>(&self, param0: u32, param1: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface3>,
    {
        (::windows::core::Interface::vtable(self).DeleteAttachedSurface)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi()).ok()
    }
    pub unsafe fn EnumAttachedSurfaces(&self, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumAttachedSurfaces)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn EnumOverlayZOrders(&self, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumOverlayZOrders)(::windows::core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn Flip<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface3>,
    {
        (::windows::core::Interface::vtable(self).Flip)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn GetAttachedSurface(&self, param0: *mut DDSCAPS, param1: *mut ::core::option::Option<IDirectDrawSurface3>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttachedSurface)(::windows::core::Interface::as_raw(self), param0, ::core::mem::transmute(param1)).ok()
    }
    pub unsafe fn GetBltStatus(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBltStatus)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetCaps(&self, param0: *mut DDSCAPS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCaps)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetClipper(&self) -> ::windows::core::Result<IDirectDrawClipper> {
        let mut result__ = ::windows::core::zeroed::<IDirectDrawClipper>();
        (::windows::core::Interface::vtable(self).GetClipper)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColorKey)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetDC(&self, param0: *mut super::Gdi::HDC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDC)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetFlipStatus(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFlipStatus)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOverlayPosition)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetPalette(&self) -> ::windows::core::Result<IDirectDrawPalette> {
        let mut result__ = ::windows::core::zeroed::<IDirectDrawPalette>();
        (::windows::core::Interface::vtable(self).GetPalette)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPixelFormat(&self, param0: *mut DDPIXELFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPixelFormat)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSurfaceDesc)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: *mut DDSURFACEDESC) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDraw>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn IsLost(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsLost)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Lock<P0>(&self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).Lock)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn ReleaseDC<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Gdi::HDC>,
    {
        (::windows::core::Interface::vtable(self).ReleaseDC)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    pub unsafe fn Restore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Restore)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetClipper<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawClipper>,
    {
        (::windows::core::Interface::vtable(self).SetClipper)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    pub unsafe fn SetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColorKey)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetOverlayPosition(&self, param0: i32, param1: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOverlayPosition)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetPalette<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawPalette>,
    {
        (::windows::core::Interface::vtable(self).SetPalette)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    pub unsafe fn Unlock(&self, param0: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unlock)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateOverlay<P0>(&self, param0: *mut super::super::Foundation::RECT, param1: P0, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface3>,
    {
        (::windows::core::Interface::vtable(self).UpdateOverlay)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi(), param2, param3, param4).ok()
    }
    pub unsafe fn UpdateOverlayDisplay(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateOverlayDisplay)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn UpdateOverlayZOrder<P0>(&self, param0: u32, param1: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface3>,
    {
        (::windows::core::Interface::vtable(self).UpdateOverlayZOrder)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi()).ok()
    }
    pub unsafe fn GetDDInterface(&self, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDDInterface)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn PageLock(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PageLock)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn PageUnlock(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PageUnlock)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn SetSurfaceDesc(&self, param0: *mut DDSURFACEDESC, param1: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSurfaceDesc)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
}
::windows::imp::interface_hierarchy!(IDirectDrawSurface3, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDirectDrawSurface3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawSurface3 {}
impl ::core::fmt::Debug for IDirectDrawSurface3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawSurface3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectDrawSurface3 {
    type Vtable = IDirectDrawSurface3_Vtbl;
}
impl ::core::clone::Clone for IDirectDrawSurface3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDirectDrawSurface3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda044e00_69b2_11d0_a1d5_00aa00b8dfbb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawSurface3_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddAttachedSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddOverlayDirtyRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddOverlayDirtyRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Blt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Blt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BltBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BltBatch: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BltFast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut ::core::ffi::c_void, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BltFast: usize,
    pub DeleteAttachedSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumAttachedSurfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK) -> ::windows::core::HRESULT,
    pub EnumOverlayZOrders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK) -> ::windows::core::HRESULT,
    pub Flip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows::core::HRESULT,
    pub GetAttachedSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS, param1: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBltStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS) -> ::windows::core::HRESULT,
    pub GetClipper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetColorKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetDC: usize,
    pub GetFlipStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub GetOverlayPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT,
    pub GetPalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT,
    pub GetSurfaceDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut DDSURFACEDESC) -> ::windows::core::HRESULT,
    pub IsLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Lock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Lock: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub ReleaseDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    ReleaseDC: usize,
    pub Restore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetClipper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetColorKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT,
    pub SetOverlayPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Unlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateOverlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateOverlay: usize,
    pub UpdateOverlayDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub UpdateOverlayZOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDDInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PageLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub PageUnlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub SetSurfaceDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC, param1: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[repr(transparent)]
pub struct IDirectDrawSurface4(::windows::core::IUnknown);
impl IDirectDrawSurface4 {
    pub unsafe fn AddAttachedSurface<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface4>,
    {
        (::windows::core::Interface::vtable(self).AddAttachedSurface)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddOverlayDirtyRect(&self, param0: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddOverlayDirtyRect)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Blt<P0>(&self, param0: *mut super::super::Foundation::RECT, param1: P0, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface4>,
    {
        (::windows::core::Interface::vtable(self).Blt)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi(), param2, param3, param4).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BltBatch)(::windows::core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BltFast<P0>(&self, param0: u32, param1: u32, param2: P0, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface4>,
    {
        (::windows::core::Interface::vtable(self).BltFast)(::windows::core::Interface::as_raw(self), param0, param1, param2.into_param().abi(), param3, param4).ok()
    }
    pub unsafe fn DeleteAttachedSurface<P0>(&self, param0: u32, param1: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface4>,
    {
        (::windows::core::Interface::vtable(self).DeleteAttachedSurface)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi()).ok()
    }
    pub unsafe fn EnumAttachedSurfaces(&self, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumAttachedSurfaces)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn EnumOverlayZOrders(&self, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumOverlayZOrders)(::windows::core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn Flip<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface4>,
    {
        (::windows::core::Interface::vtable(self).Flip)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn GetAttachedSurface(&self, param0: *mut DDSCAPS2, param1: *mut ::core::option::Option<IDirectDrawSurface4>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttachedSurface)(::windows::core::Interface::as_raw(self), param0, ::core::mem::transmute(param1)).ok()
    }
    pub unsafe fn GetBltStatus(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBltStatus)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetCaps(&self, param0: *mut DDSCAPS2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCaps)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetClipper(&self) -> ::windows::core::Result<IDirectDrawClipper> {
        let mut result__ = ::windows::core::zeroed::<IDirectDrawClipper>();
        (::windows::core::Interface::vtable(self).GetClipper)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColorKey)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetDC(&self, param0: *mut super::Gdi::HDC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDC)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetFlipStatus(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFlipStatus)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOverlayPosition)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetPalette(&self) -> ::windows::core::Result<IDirectDrawPalette> {
        let mut result__ = ::windows::core::zeroed::<IDirectDrawPalette>();
        (::windows::core::Interface::vtable(self).GetPalette)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPixelFormat(&self, param0: *mut DDPIXELFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPixelFormat)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSurfaceDesc)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: *mut DDSURFACEDESC2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDraw>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn IsLost(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsLost)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Lock<P0>(&self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).Lock)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn ReleaseDC<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Gdi::HDC>,
    {
        (::windows::core::Interface::vtable(self).ReleaseDC)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    pub unsafe fn Restore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Restore)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetClipper<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawClipper>,
    {
        (::windows::core::Interface::vtable(self).SetClipper)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    pub unsafe fn SetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColorKey)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetOverlayPosition(&self, param0: i32, param1: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOverlayPosition)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetPalette<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawPalette>,
    {
        (::windows::core::Interface::vtable(self).SetPalette)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unlock(&self, param0: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unlock)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateOverlay<P0>(&self, param0: *mut super::super::Foundation::RECT, param1: P0, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface4>,
    {
        (::windows::core::Interface::vtable(self).UpdateOverlay)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi(), param2, param3, param4).ok()
    }
    pub unsafe fn UpdateOverlayDisplay(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateOverlayDisplay)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn UpdateOverlayZOrder<P0>(&self, param0: u32, param1: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface4>,
    {
        (::windows::core::Interface::vtable(self).UpdateOverlayZOrder)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi()).ok()
    }
    pub unsafe fn GetDDInterface(&self, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDDInterface)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn PageLock(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PageLock)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn PageUnlock(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PageUnlock)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn SetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSurfaceDesc)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetPrivateData(&self, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrivateData)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn GetPrivateData(&self, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPrivateData)(::windows::core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn FreePrivateData(&self, param0: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FreePrivateData)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetUniquenessValue(&self, param0: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUniquenessValue)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn ChangeUniquenessValue(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChangeUniquenessValue)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDirectDrawSurface4, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDirectDrawSurface4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawSurface4 {}
impl ::core::fmt::Debug for IDirectDrawSurface4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawSurface4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectDrawSurface4 {
    type Vtable = IDirectDrawSurface4_Vtbl;
}
impl ::core::clone::Clone for IDirectDrawSurface4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDirectDrawSurface4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b2b8630_ad35_11d0_8ea6_00609797ea5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawSurface4_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddAttachedSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddOverlayDirtyRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddOverlayDirtyRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Blt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Blt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BltBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BltBatch: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BltFast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut ::core::ffi::c_void, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BltFast: usize,
    pub DeleteAttachedSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumAttachedSurfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK2) -> ::windows::core::HRESULT,
    pub EnumOverlayZOrders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK2) -> ::windows::core::HRESULT,
    pub Flip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows::core::HRESULT,
    pub GetAttachedSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBltStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2) -> ::windows::core::HRESULT,
    pub GetClipper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetColorKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetDC: usize,
    pub GetFlipStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub GetOverlayPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT,
    pub GetPalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT,
    pub GetSurfaceDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT,
    pub IsLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Lock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Lock: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub ReleaseDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    ReleaseDC: usize,
    pub Restore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetClipper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetColorKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT,
    pub SetOverlayPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Unlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Unlock: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateOverlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateOverlay: usize,
    pub UpdateOverlayDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub UpdateOverlayZOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDDInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PageLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub PageUnlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub SetSurfaceDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows::core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows::core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows::core::HRESULT,
    pub FreePrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetUniquenessValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT,
    pub ChangeUniquenessValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[repr(transparent)]
pub struct IDirectDrawSurface7(::windows::core::IUnknown);
impl IDirectDrawSurface7 {
    pub unsafe fn AddAttachedSurface<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface7>,
    {
        (::windows::core::Interface::vtable(self).AddAttachedSurface)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddOverlayDirtyRect(&self, param0: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddOverlayDirtyRect)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Blt<P0>(&self, param0: *mut super::super::Foundation::RECT, param1: P0, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface7>,
    {
        (::windows::core::Interface::vtable(self).Blt)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi(), param2, param3, param4).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BltBatch(&self, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BltBatch)(::windows::core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BltFast<P0>(&self, param0: u32, param1: u32, param2: P0, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface7>,
    {
        (::windows::core::Interface::vtable(self).BltFast)(::windows::core::Interface::as_raw(self), param0, param1, param2.into_param().abi(), param3, param4).ok()
    }
    pub unsafe fn DeleteAttachedSurface<P0>(&self, param0: u32, param1: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface7>,
    {
        (::windows::core::Interface::vtable(self).DeleteAttachedSurface)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi()).ok()
    }
    pub unsafe fn EnumAttachedSurfaces(&self, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK7) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumAttachedSurfaces)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn EnumOverlayZOrders(&self, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK7) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumOverlayZOrders)(::windows::core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn Flip<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface7>,
    {
        (::windows::core::Interface::vtable(self).Flip)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn GetAttachedSurface(&self, param0: *mut DDSCAPS2, param1: *mut ::core::option::Option<IDirectDrawSurface7>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAttachedSurface)(::windows::core::Interface::as_raw(self), param0, ::core::mem::transmute(param1)).ok()
    }
    pub unsafe fn GetBltStatus(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBltStatus)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetCaps(&self, param0: *mut DDSCAPS2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCaps)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetClipper(&self) -> ::windows::core::Result<IDirectDrawClipper> {
        let mut result__ = ::windows::core::zeroed::<IDirectDrawClipper>();
        (::windows::core::Interface::vtable(self).GetClipper)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColorKey)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn GetDC(&self, param0: *mut super::Gdi::HDC) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDC)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetFlipStatus(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFlipStatus)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetOverlayPosition(&self, param0: *mut i32, param1: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOverlayPosition)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn GetPalette(&self) -> ::windows::core::Result<IDirectDrawPalette> {
        let mut result__ = ::windows::core::zeroed::<IDirectDrawPalette>();
        (::windows::core::Interface::vtable(self).GetPalette)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetPixelFormat(&self, param0: *mut DDPIXELFORMAT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPixelFormat)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSurfaceDesc)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn Initialize<P0>(&self, param0: P0, param1: *mut DDSURFACEDESC2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDraw>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn IsLost(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsLost)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Lock<P0>(&self, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).Lock)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub unsafe fn ReleaseDC<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Gdi::HDC>,
    {
        (::windows::core::Interface::vtable(self).ReleaseDC)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    pub unsafe fn Restore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Restore)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetClipper<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawClipper>,
    {
        (::windows::core::Interface::vtable(self).SetClipper)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    pub unsafe fn SetColorKey(&self, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColorKey)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetOverlayPosition(&self, param0: i32, param1: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOverlayPosition)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetPalette<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawPalette>,
    {
        (::windows::core::Interface::vtable(self).SetPalette)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Unlock(&self, param0: *mut super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unlock)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateOverlay<P0>(&self, param0: *mut super::super::Foundation::RECT, param1: P0, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface7>,
    {
        (::windows::core::Interface::vtable(self).UpdateOverlay)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi(), param2, param3, param4).ok()
    }
    pub unsafe fn UpdateOverlayDisplay(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateOverlayDisplay)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn UpdateOverlayZOrder<P0>(&self, param0: u32, param1: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface7>,
    {
        (::windows::core::Interface::vtable(self).UpdateOverlayZOrder)(::windows::core::Interface::as_raw(self), param0, param1.into_param().abi()).ok()
    }
    pub unsafe fn GetDDInterface(&self, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDDInterface)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn PageLock(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PageLock)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn PageUnlock(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PageUnlock)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn SetSurfaceDesc(&self, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSurfaceDesc)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    pub unsafe fn SetPrivateData(&self, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPrivateData)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3).ok()
    }
    pub unsafe fn GetPrivateData(&self, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPrivateData)(::windows::core::Interface::as_raw(self), param0, param1, param2).ok()
    }
    pub unsafe fn FreePrivateData(&self, param0: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FreePrivateData)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetUniquenessValue(&self, param0: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUniquenessValue)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn ChangeUniquenessValue(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChangeUniquenessValue)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetPriority(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPriority)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetPriority(&self, param0: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPriority)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn SetLOD(&self, param0: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLOD)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetLOD(&self, param0: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetLOD)(::windows::core::Interface::as_raw(self), param0).ok()
    }
}
::windows::imp::interface_hierarchy!(IDirectDrawSurface7, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDirectDrawSurface7 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawSurface7 {}
impl ::core::fmt::Debug for IDirectDrawSurface7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawSurface7").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectDrawSurface7 {
    type Vtable = IDirectDrawSurface7_Vtbl;
}
impl ::core::clone::Clone for IDirectDrawSurface7 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDirectDrawSurface7 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06675a80_3b9b_11d2_b92f_00609797ea5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawSurface7_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddAttachedSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddOverlayDirtyRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddOverlayDirtyRect: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Blt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDBLTFX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Blt: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BltBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDBLTBATCH, param1: u32, param2: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BltBatch: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub BltFast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: *mut ::core::ffi::c_void, param3: *mut super::super::Foundation::RECT, param4: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BltFast: usize,
    pub DeleteAttachedSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnumAttachedSurfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: LPDDENUMSURFACESCALLBACK7) -> ::windows::core::HRESULT,
    pub EnumOverlayZOrders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void, param2: LPDDENUMSURFACESCALLBACK7) -> ::windows::core::HRESULT,
    pub Flip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows::core::HRESULT,
    pub GetAttachedSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2, param1: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBltStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub GetCaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSCAPS2) -> ::windows::core::HRESULT,
    pub GetClipper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetColorKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub GetDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    GetDC: usize,
    pub GetFlipStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub GetOverlayPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut i32, param1: *mut i32) -> ::windows::core::HRESULT,
    pub GetPalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPixelFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT) -> ::windows::core::HRESULT,
    pub GetSurfaceDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: *mut DDSURFACEDESC2) -> ::windows::core::HRESULT,
    pub IsLost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Lock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut DDSURFACEDESC2, param2: u32, param3: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Lock: usize,
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub ReleaseDC: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::Gdi::HDC) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))]
    ReleaseDC: usize,
    pub Restore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetClipper: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetColorKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut DDCOLORKEY) -> ::windows::core::HRESULT,
    pub SetOverlayPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: i32, param1: i32) -> ::windows::core::HRESULT,
    pub SetPalette: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Unlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Unlock: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateOverlay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::RECT, param1: *mut ::core::ffi::c_void, param2: *mut super::super::Foundation::RECT, param3: u32, param4: *mut DDOVERLAYFX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateOverlay: usize,
    pub UpdateOverlayDisplay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub UpdateOverlayZOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDDInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub PageLock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub PageUnlock: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub SetSurfaceDesc: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDSURFACEDESC2, param1: u32) -> ::windows::core::HRESULT,
    pub SetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: u32, param3: u32) -> ::windows::core::HRESULT,
    pub GetPrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> ::windows::core::HRESULT,
    pub FreePrivateData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetUniquenessValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT,
    pub ChangeUniquenessValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub GetPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT,
    pub SetLOD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32) -> ::windows::core::HRESULT,
    pub GetLOD: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[repr(transparent)]
pub struct IDirectDrawSurfaceKernel(::windows::core::IUnknown);
impl IDirectDrawSurfaceKernel {
    pub unsafe fn GetKernelHandle(&self, param0: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetKernelHandle)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn ReleaseKernelHandle(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReleaseKernelHandle)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDirectDrawSurfaceKernel, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDirectDrawSurfaceKernel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawSurfaceKernel {}
impl ::core::fmt::Debug for IDirectDrawSurfaceKernel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawSurfaceKernel").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectDrawSurfaceKernel {
    type Vtable = IDirectDrawSurfaceKernel_Vtbl;
}
impl ::core::clone::Clone for IDirectDrawSurfaceKernel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDirectDrawSurfaceKernel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60755da0_6a40_11d0_9b06_00a0c903a3b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawSurfaceKernel_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetKernelHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut usize) -> ::windows::core::HRESULT,
    pub ReleaseKernelHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[repr(transparent)]
pub struct IDirectDrawVideoPort(::windows::core::IUnknown);
impl IDirectDrawVideoPort {
    pub unsafe fn Flip<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface>,
    {
        (::windows::core::Interface::vtable(self).Flip)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    pub unsafe fn GetBandwidthInfo(&self, param0: *mut DDPIXELFORMAT, param1: u32, param2: u32, param3: u32, param4: *mut DDVIDEOPORTBANDWIDTH) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBandwidthInfo)(::windows::core::Interface::as_raw(self), param0, param1, param2, param3, param4).ok()
    }
    pub unsafe fn GetColorControls(&self, param0: *mut DDCOLORCONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColorControls)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetInputFormats(&self, lpnumformats: *mut u32, param1: ::core::option::Option<*mut DDPIXELFORMAT>, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInputFormats)(::windows::core::Interface::as_raw(self), lpnumformats, ::core::mem::transmute(param1.unwrap_or(::std::ptr::null_mut())), param2).ok()
    }
    pub unsafe fn GetOutputFormats(&self, param0: *mut DDPIXELFORMAT, lpnumformats: *mut u32, param2: ::core::option::Option<*mut DDPIXELFORMAT>, param3: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOutputFormats)(::windows::core::Interface::as_raw(self), param0, lpnumformats, ::core::mem::transmute(param2.unwrap_or(::std::ptr::null_mut())), param3).ok()
    }
    pub unsafe fn GetFieldPolarity(&self, param0: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFieldPolarity)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetVideoLine(&self, param0: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVideoLine)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn GetVideoSignalStatus(&self, param0: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVideoSignalStatus)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn SetColorControls(&self, param0: *mut DDCOLORCONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColorControls)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn SetTargetSurface<P0>(&self, param0: P0, param1: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDirectDrawSurface>,
    {
        (::windows::core::Interface::vtable(self).SetTargetSurface)(::windows::core::Interface::as_raw(self), param0.into_param().abi(), param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn StartVideo(&self, param0: *mut DDVIDEOPORTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartVideo)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn StopVideo(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopVideo)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UpdateVideo(&self, param0: *mut DDVIDEOPORTINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateVideo)(::windows::core::Interface::as_raw(self), param0).ok()
    }
    pub unsafe fn WaitForSync(&self, param0: u32, param1: u32, param2: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WaitForSync)(::windows::core::Interface::as_raw(self), param0, param1, param2).ok()
    }
}
::windows::imp::interface_hierarchy!(IDirectDrawVideoPort, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDirectDrawVideoPort {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawVideoPort {}
impl ::core::fmt::Debug for IDirectDrawVideoPort {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawVideoPort").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectDrawVideoPort {
    type Vtable = IDirectDrawVideoPort_Vtbl;
}
impl ::core::clone::Clone for IDirectDrawVideoPort {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDirectDrawVideoPort {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb36d93e0_2b43_11cf_a2de_00aa00b93356);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawVideoPort_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Flip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows::core::HRESULT,
    pub GetBandwidthInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT, param1: u32, param2: u32, param3: u32, param4: *mut DDVIDEOPORTBANDWIDTH) -> ::windows::core::HRESULT,
    pub GetColorControls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows::core::HRESULT,
    pub GetInputFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpnumformats: *mut u32, param1: *mut DDPIXELFORMAT, param2: u32) -> ::windows::core::HRESULT,
    pub GetOutputFormats: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDPIXELFORMAT, lpnumformats: *mut u32, param2: *mut DDPIXELFORMAT, param3: u32) -> ::windows::core::HRESULT,
    pub GetFieldPolarity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut i32) -> ::windows::core::HRESULT,
    pub GetVideoLine: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT,
    pub GetVideoSignalStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut u32) -> ::windows::core::HRESULT,
    pub SetColorControls: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDCOLORCONTROL) -> ::windows::core::HRESULT,
    pub SetTargetSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut ::core::ffi::c_void, param1: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub StartVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDVIDEOPORTINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    StartVideo: usize,
    pub StopVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UpdateVideo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut DDVIDEOPORTINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UpdateVideo: usize,
    pub WaitForSync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: u32, param1: u32, param2: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
#[repr(transparent)]
pub struct IDirectDrawVideoPortNotify(::windows::core::IUnknown);
impl IDirectDrawVideoPortNotify {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AcquireNotification(&self, param0: *mut super::super::Foundation::HANDLE, param1: *mut DDVIDEOPORTNOTIFY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AcquireNotification)(::windows::core::Interface::as_raw(self), param0, param1).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseNotification<P0>(&self, param0: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).ReleaseNotification)(::windows::core::Interface::as_raw(self), param0.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDirectDrawVideoPortNotify, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDirectDrawVideoPortNotify {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirectDrawVideoPortNotify {}
impl ::core::fmt::Debug for IDirectDrawVideoPortNotify {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirectDrawVideoPortNotify").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirectDrawVideoPortNotify {
    type Vtable = IDirectDrawVideoPortNotify_Vtbl;
}
impl ::core::clone::Clone for IDirectDrawVideoPortNotify {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDirectDrawVideoPortNotify {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa655fb94_0589_4e57_b333_567a89468c88);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawVideoPortNotify_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AcquireNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: *mut super::super::Foundation::HANDLE, param1: *mut DDVIDEOPORTNOTIFY) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AcquireNotification: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ReleaseNotification: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, param0: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ReleaseNotification: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const ACCESSRECT_BROKEN: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const ACCESSRECT_NOTHOLDINGWIN16LOCK: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const ACCESSRECT_VRAMSTYLE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const CCHDEVICENAME: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const CLSID_DirectDraw: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd7b70ee0_4340_11cf_b063_0020afc2cd35);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const CLSID_DirectDraw7: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3c305196_50db_11d3_9cfe_00c04fd930c5);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const CLSID_DirectDrawClipper: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x593817a0_7db3_11cf_a2de_00aa00b93356);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFMT_INTERNAL_D15S1: u32 = 73u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFMT_INTERNAL_D24S8: u32 = 75u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFMT_INTERNAL_D24X8: u32 = 77u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFMT_INTERNAL_D32: u32 = 71u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFMT_INTERNAL_S1D15: u32 = 72u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFMT_INTERNAL_S8D24: u32 = 74u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFMT_INTERNAL_X8D24: u32 = 76u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_MEMBEROFGROUP_ARGB: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_3DACCELERATION: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_AUTOGENMIPMAP: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_BUMPMAP: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_CONVERT_TO_ARGB: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_CUBETEXTURE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_DISPLAYMODE: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_DMAP: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_NOALPHABLEND: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_NOFILTER: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_NOTEXCOORDWRAPNORMIP: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_OFFSCREENPLAIN: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_OFFSCREEN_RENDERTARGET: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_PIXELSIZE: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_SAME_FORMAT_RENDERTARGET: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_SAME_FORMAT_UP_TO_ALPHA_RENDERTARGET: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_SRGBREAD: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_SRGBWRITE: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_TEXTURE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_VERTEXTEXTURE: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_VOLUMETEXTURE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_ZSTENCIL: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const D3DFORMAT_OP_ZSTENCIL_WITH_ARBITRARY_COLOR_DEPTH: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DCICOMMAND: u32 = 3075u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDABLT_SRCOVERDEST: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDAL_IMPLICIT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBD_1: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBD_16: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBD_2: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBD_24: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBD_32: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBD_4: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBD_8: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLTFAST_DESTCOLORKEY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLTFAST_DONOTWAIT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLTFAST_NOCOLORKEY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLTFAST_SRCCOLORKEY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLTFAST_WAIT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLTFX_ARITHSTRETCHY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLTFX_MIRRORLEFTRIGHT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLTFX_MIRRORUPDOWN: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLTFX_NOTEARING: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLTFX_ROTATE180: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLTFX_ROTATE270: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLTFX_ROTATE90: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLTFX_ZBUFFERBASEDEST: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLTFX_ZBUFFERRANGE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_AFLAGS: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_ALPHADEST: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_ALPHADESTCONSTOVERRIDE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_ALPHADESTNEG: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_ALPHADESTSURFACEOVERRIDE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_ALPHAEDGEBLEND: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_ALPHASRC: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_ALPHASRCCONSTOVERRIDE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_ALPHASRCNEG: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_ALPHASRCSURFACEOVERRIDE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_ASYNC: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_COLORFILL: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_DDFX: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_DDROPS: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_DEPTHFILL: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_DONOTWAIT: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_EXTENDED_FLAGS: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_EXTENDED_LINEAR_CONTENT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_KEYDEST: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_KEYDESTOVERRIDE: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_KEYSRC: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_KEYSRCOVERRIDE: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_LAST_PRESENTATION: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_PRESENTATION: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_ROP: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_ROTATIONANGLE: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_WAIT: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_ZBUFFER: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_ZBUFFERDESTCONSTOVERRIDE: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_ZBUFFERDESTOVERRIDE: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_ZBUFFERSRCCONSTOVERRIDE: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDBLT_ZBUFFERSRCOVERRIDE: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_AUTOFLIPOVERLAY: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_CANAUTOGENMIPMAP: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_CANBOBHARDWARE: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_CANBOBINTERLEAVED: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_CANBOBNONINTERLEAVED: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_CANCALIBRATEGAMMA: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_CANDROPZ16BIT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_CANFLIPODDEVEN: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_CANMANAGERESOURCE: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_CANMANAGETEXTURE: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_CANRENDERWINDOWED: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_CANSHARERESOURCE: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_CERTIFIED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_COLORCONTROLOVERLAY: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_COLORCONTROLPRIMARY: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_COPYFOURCC: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_DYNAMICTEXTURES: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_FLIPINTERVAL: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_FLIPNOVSYNC: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_NO2DDURING3DSCENE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_NONLOCALVIDMEM: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_NONLOCALVIDMEMCAPS: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_NOPAGELOCKREQUIRED: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_PRIMARYGAMMA: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_RESERVED1: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_STEREO: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_SYSTONONLOCAL_AS_SYSTOLOCAL: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_TEXMANINNONLOCALVIDMEM: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_VIDEOPORT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS2_WIDESURFACES: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_3D: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_ALIGNBOUNDARYDEST: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_ALIGNBOUNDARYSRC: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_ALIGNSIZEDEST: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_ALIGNSIZESRC: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_ALIGNSTRIDE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_ALPHA: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_BANKSWITCHED: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_BLT: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_BLTCOLORFILL: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_BLTDEPTHFILL: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_BLTFOURCC: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_BLTQUEUE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_BLTSTRETCH: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_CANBLTSYSMEM: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_CANCLIP: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_CANCLIPSTRETCHED: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_COLORKEY: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_COLORKEYHWASSIST: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_GDI: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_NOHARDWARE: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_OVERLAY: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_OVERLAYCANTCLIP: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_OVERLAYFOURCC: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_OVERLAYSTRETCH: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_PALETTE: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_PALETTEVSYNC: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_READSCANLINE: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_RESERVED1: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_VBI: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_ZBLTS: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCAPS_ZOVERLAYS: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEYCAPS_DESTBLT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEYCAPS_DESTBLTCLRSPACE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEYCAPS_DESTBLTCLRSPACEYUV: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEYCAPS_DESTBLTYUV: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEYCAPS_DESTOVERLAY: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEYCAPS_DESTOVERLAYCLRSPACE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEYCAPS_DESTOVERLAYCLRSPACEYUV: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEYCAPS_DESTOVERLAYONEACTIVE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEYCAPS_DESTOVERLAYYUV: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEYCAPS_NOCOSTOVERLAY: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEYCAPS_SRCBLT: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEYCAPS_SRCBLTCLRSPACE: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEYCAPS_SRCBLTCLRSPACEYUV: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEYCAPS_SRCBLTYUV: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEYCAPS_SRCOVERLAY: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEYCAPS_SRCOVERLAYCLRSPACE: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEYCAPS_SRCOVERLAYCLRSPACEYUV: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEYCAPS_SRCOVERLAYONEACTIVE: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEYCAPS_SRCOVERLAYYUV: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEY_COLORSPACE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEY_DESTBLT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEY_DESTOVERLAY: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEY_SRCBLT: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCKEY_SRCOVERLAY: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCOLOR_BRIGHTNESS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCOLOR_COLORENABLE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCOLOR_CONTRAST: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCOLOR_GAMMA: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCOLOR_HUE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCOLOR_SATURATION: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCOLOR_SHARPNESS: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCREATEDRIVEROBJECT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCREATE_EMULATIONONLY: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDCREATE_HARDWAREONLY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDEDM_REFRESHRATES: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDEDM_STANDARDVGAMODES: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDEM_MODEFAILED: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDEM_MODEPASSED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDENUMOVERLAYZ_BACKTOFRONT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDENUMOVERLAYZ_FRONTTOBACK: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDENUMRET_CANCEL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDENUMRET_OK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDENUMSURFACES_ALL: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDENUMSURFACES_CANBECREATED: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDENUMSURFACES_DOESEXIST: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDENUMSURFACES_MATCH: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDENUMSURFACES_NOMATCH: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDENUM_ATTACHEDSECONDARYDEVICES: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDENUM_DETACHEDSECONDARYDEVICES: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDENUM_NONDISPLAYDEVICES: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDERR_NOTINITIALIZED: i32 = -2147221008i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFLIP_DONOTWAIT: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFLIP_EVEN: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFLIP_INTERVAL2: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFLIP_INTERVAL3: i32 = 50331648i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFLIP_INTERVAL4: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFLIP_NOVSYNC: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFLIP_ODD: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFLIP_STEREO: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFLIP_WAIT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXALPHACAPS_BLTALPHAEDGEBLEND: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXALPHACAPS_BLTALPHAPIXELS: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXALPHACAPS_BLTALPHAPIXELSNEG: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXALPHACAPS_BLTALPHASURFACES: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXALPHACAPS_BLTALPHASURFACESNEG: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXALPHACAPS_OVERLAYALPHAEDGEBLEND: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXALPHACAPS_OVERLAYALPHAPIXELS: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXALPHACAPS_OVERLAYALPHAPIXELSNEG: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXALPHACAPS_OVERLAYALPHASURFACES: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXALPHACAPS_OVERLAYALPHASURFACESNEG: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_BLTALPHA: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_BLTARITHSTRETCHY: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_BLTARITHSTRETCHYN: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_BLTFILTER: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_BLTMIRRORLEFTRIGHT: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_BLTMIRRORUPDOWN: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_BLTROTATION: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_BLTROTATION90: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_BLTSHRINKX: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_BLTSHRINKXN: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_BLTSHRINKY: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_BLTSHRINKYN: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_BLTSTRETCHX: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_BLTSTRETCHXN: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_BLTSTRETCHY: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_BLTSTRETCHYN: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_OVERLAYALPHA: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_OVERLAYARITHSTRETCHY: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_OVERLAYARITHSTRETCHYN: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_OVERLAYDEINTERLACE: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_OVERLAYFILTER: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_OVERLAYMIRRORLEFTRIGHT: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_OVERLAYMIRRORUPDOWN: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_OVERLAYSHRINKX: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_OVERLAYSHRINKXN: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_OVERLAYSHRINKY: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_OVERLAYSHRINKYN: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_OVERLAYSTRETCHX: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_OVERLAYSTRETCHXN: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_OVERLAYSTRETCHY: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDFXCAPS_OVERLAYSTRETCHYN: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDGBS_CANBLT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDGBS_ISBLTDONE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDGDI_GETHOSTIDENTIFIER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDGET32BITDRIVERNAME: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDGFS_CANFLIP: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDGFS_ISFLIPDONE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHALINFO_GETDRIVERINFO2: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHALINFO_GETDRIVERINFOSET: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHALINFO_ISPRIMARYDISPLAY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHALINFO_MODEXILLEGAL: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_APP_DLLNAME: ::windows::core::PCSTR = ::windows::core::s!("DDRAW.DLL");
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_CB32_CANCREATESURFACE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_CB32_CREATEPALETTE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_CB32_CREATESURFACE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_CB32_DESTROYDRIVER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_CB32_FLIPTOGDISURFACE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_CB32_GETSCANLINE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_CB32_MAPMEMORY: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_CB32_SETCOLORKEY: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_CB32_SETEXCLUSIVEMODE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_CB32_SETMODE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_CB32_WAITFORVERTICALBLANK: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_COLOR_COLORCONTROL: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_CREATESURFACEEX_SWAPHANDLES: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_D3DBUFCB32_CANCREATED3DBUF: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_D3DBUFCB32_CREATED3DBUF: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_D3DBUFCB32_DESTROYD3DBUF: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_D3DBUFCB32_LOCKD3DBUF: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_D3DBUFCB32_UNLOCKD3DBUF: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_DRIVER_DLLNAME: ::windows::core::PCSTR = ::windows::core::s!("DDRAW16.DLL");
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_DRIVER_HANDLED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_DRIVER_NOCKEYHW: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_DRIVER_NOTHANDLED: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_EXEBUFCB32_CANCREATEEXEBUF: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_EXEBUFCB32_CREATEEXEBUF: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_EXEBUFCB32_DESTROYEXEBUF: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_EXEBUFCB32_LOCKEXEBUF: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_EXEBUFCB32_UNLOCKEXEBUF: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_KERNEL_SYNCSURFACEDATA: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_KERNEL_SYNCVIDEOPORTDATA: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_MISC2CB32_ALPHABLT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_MISC2CB32_CREATESURFACEEX: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_MISC2CB32_DESTROYDDLOCAL: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_MISC2CB32_GETDRIVERSTATE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_MISCCB32_GETAVAILDRIVERMEMORY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_MISCCB32_GETHEAPALIGNMENT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_MISCCB32_GETSYSMEMBLTSTATUS: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_MISCCB32_UPDATENONLOCALHEAP: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_MOCOMP32_BEGINFRAME: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_MOCOMP32_CREATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_MOCOMP32_DESTROY: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_MOCOMP32_ENDFRAME: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_MOCOMP32_GETCOMPBUFFINFO: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_MOCOMP32_GETFORMATS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_MOCOMP32_GETGUIDS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_MOCOMP32_GETINTERNALINFO: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_MOCOMP32_QUERYSTATUS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_MOCOMP32_RENDER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_NTCB32_FLIPTOGDISURFACE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_NTCB32_FREEDRIVERMEMORY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_NTCB32_SETEXCLUSIVEMODE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_PALCB32_DESTROYPALETTE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_PALCB32_SETENTRIES: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_PLEASEALLOC_BLOCKSIZE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_PLEASEALLOC_LINEARSIZE: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_PLEASEALLOC_USERMEM: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_PRIVATECAP_ATOMICSURFACECREATION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_PRIVATECAP_NOTIFYPRIMARYCREATION: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_PRIVATECAP_RESERVED1: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_SURFCB32_ADDATTACHEDSURFACE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_SURFCB32_BLT: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_SURFCB32_DESTROYSURFACE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_SURFCB32_FLIP: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_SURFCB32_GETBLTSTATUS: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_SURFCB32_GETFLIPSTATUS: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_SURFCB32_LOCK: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_SURFCB32_RESERVED4: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_SURFCB32_SETCLIPLIST: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_SURFCB32_SETCOLORKEY: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_SURFCB32_SETOVERLAYPOSITION: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_SURFCB32_SETPALETTE: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_SURFCB32_UNLOCK: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_SURFCB32_UPDATEOVERLAY: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_VPORT32_CANCREATEVIDEOPORT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_VPORT32_COLORCONTROL: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_VPORT32_CREATEVIDEOPORT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_VPORT32_DESTROY: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_VPORT32_FLIP: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_VPORT32_GETAUTOFLIPSURF: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_VPORT32_GETBANDWIDTH: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_VPORT32_GETCONNECT: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_VPORT32_GETFIELD: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_VPORT32_GETFLIPSTATUS: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_VPORT32_GETINPUTFORMATS: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_VPORT32_GETLINE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_VPORT32_GETOUTPUTFORMATS: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_VPORT32_GETSIGNALSTATUS: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_VPORT32_UPDATE: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDHAL_VPORT32_WAITFORSYNC: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_BUSMASTER: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_DISPLAY_VSYNC: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_RESERVED1: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_VPORT0_LINE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_VPORT0_VSYNC: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_VPORT1_LINE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_VPORT1_VSYNC: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_VPORT2_LINE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_VPORT2_VSYNC: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_VPORT3_LINE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_VPORT3_VSYNC: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_VPORT4_LINE: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_VPORT4_VSYNC: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_VPORT5_LINE: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_VPORT5_VSYNC: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_VPORT6_LINE: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_VPORT6_VSYNC: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_VPORT7_LINE: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_VPORT7_VSYNC: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_VPORT8_LINE: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_VPORT8_VSYNC: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_VPORT9_LINE: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDIRQ_VPORT9_VSYNC: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDKERNELCAPS_AUTOFLIP: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDKERNELCAPS_CAPTURE_INVERTED: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDKERNELCAPS_CAPTURE_NONLOCALVIDMEM: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDKERNELCAPS_CAPTURE_SYSMEM: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDKERNELCAPS_FIELDPOLARITY: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDKERNELCAPS_FLIPOVERLAY: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDKERNELCAPS_FLIPVIDEOPORT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDKERNELCAPS_LOCK: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDKERNELCAPS_SETSTATE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDKERNELCAPS_SKIPFIELDS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDLOCK_DISCARDCONTENTS: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDLOCK_DONOTWAIT: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDLOCK_EVENT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDLOCK_HASVOLUMETEXTUREBOXRECT: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDLOCK_NODIRTYUPDATE: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDLOCK_NOOVERWRITE: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDLOCK_NOSYSLOCK: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDLOCK_OKTOSWAP: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDLOCK_READONLY: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDLOCK_SURFACEMEMORYPTR: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDLOCK_WAIT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDLOCK_WRITEONLY: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDMCQUERY_READ: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDMODEINFO_MAXREFRESH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDMODEINFO_MODEX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDMODEINFO_PALETTIZED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDMODEINFO_STANDARDVGA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDMODEINFO_STEREO: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDMODEINFO_UNSUPPORTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDNEWCALLBACKFNS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOSDCAPS_MONOLITHICMIPMAP: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOSDCAPS_OPTCOMPRESSED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOSDCAPS_OPTREORDERED: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOSDCAPS_VALIDOSCAPS: i32 = 7i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOSDCAPS_VALIDSCAPS: i32 = 805324800i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOSD_ALL: i32 = 15i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOSD_COMPRESSION_RATIO: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOSD_GUID: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOSD_OSCAPS: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOSD_SCAPS: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVERFX_ARITHSTRETCHY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVERFX_DEINTERLACE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVERFX_MIRRORLEFTRIGHT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVERFX_MIRRORUPDOWN: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVERZ_INSERTINBACKOF: i32 = 5i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVERZ_INSERTINFRONTOF: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVERZ_MOVEBACKWARD: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVERZ_MOVEFORWARD: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVERZ_SENDTOBACK: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVERZ_SENDTOFRONT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_ADDDIRTYRECT: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_ALPHADEST: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_ALPHADESTCONSTOVERRIDE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_ALPHADESTNEG: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_ALPHADESTSURFACEOVERRIDE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_ALPHAEDGEBLEND: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_ALPHASRC: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_ALPHASRCCONSTOVERRIDE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_ALPHASRCNEG: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_ALPHASRCSURFACEOVERRIDE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_ARGBSCALEFACTORS: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_AUTOFLIP: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_BOB: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_BOBHARDWARE: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_DDFX: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_DEGRADEARGBSCALING: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_HIDE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_INTERLEAVED: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_KEYDEST: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_KEYDESTOVERRIDE: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_KEYSRC: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_KEYSRCOVERRIDE: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_OVERRIDEBOBWEAVE: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_REFRESHALL: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_REFRESHDIRTYRECTS: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDOVER_SHOW: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPCAPS_1BIT: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPCAPS_2BIT: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPCAPS_4BIT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPCAPS_8BIT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPCAPS_8BITENTRIES: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPCAPS_ALLOW256: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPCAPS_ALPHA: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPCAPS_INITIALIZE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPCAPS_PRIMARYSURFACE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPCAPS_PRIMARYSURFACELEFT: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPCAPS_VSYNC: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_ALPHA: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_ALPHAPIXELS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_ALPHAPREMULT: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_BUMPDUDV: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_BUMPLUMINANCE: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_COMPRESSED: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_D3DFORMAT: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_FOURCC: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_LUMINANCE: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_NOVEL_TEXTURE_FORMAT: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_PALETTEINDEXED1: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_PALETTEINDEXED2: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_PALETTEINDEXED4: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_PALETTEINDEXED8: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_PALETTEINDEXEDTO8: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_RGB: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_RGBTOYUV: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_STENCILBUFFER: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_YUV: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_ZBUFFER: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDPF_ZPIXELS: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWICLIP_INMASTERSPRITELIST: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWICLIP_ISINITIALIZED: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWICLIP_WATCHWINDOW: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_ACTIVENO: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_ACTIVEYES: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_ALLOWMODEX: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_ATTEMPTEDD3DCONTEXT: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_CREATEDWINDOW: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_CURSORCLIPPED: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_DIRECTDRAW7: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_DIRECTDRAW8: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_DIRTYDC: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_DISABLEINACTIVATE: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_DX8DRIVER: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_EXPLICITMONITOR: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_FPUPRESERVE: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_FPUSETUP: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_HASEXCLUSIVEMODE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_HOOKEDHWND: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_ISFULLSCREEN: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_MODEHASBEENCHANGED: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_MULTITHREADED: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_POWEREDDOWN: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_SETCOOPCALLED: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWILCL_V1SCLBEHAVIOUR: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIPAL_16: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIPAL_2: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIPAL_256: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIPAL_4: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIPAL_ALLOW256: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIPAL_ALPHA: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIPAL_DIRTY: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIPAL_EXCLUSIVE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIPAL_GDI: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIPAL_INHEL: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIPAL_STORED_16: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIPAL_STORED_24: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIPAL_STORED_8: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIPAL_STORED_8INDEX: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_DDHELDONTFREE: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_DX8SURFACE: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_FASTLOCKHELD: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_HARDWAREOPDEST: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_HARDWAREOPSOURCE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_IMPLICITHANDLE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_ISCLIENTMEM: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_ISGDISURFACE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_LATEALLOCATELINEAR: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_LOCKBROKEN: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_LOCKNOTHOLDINGWIN16LOCK: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_LOCKVRAMSTYLE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_MEMFREE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_NOTIFYWHENUNLOCKED: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_READONLYLOCKHELD: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_RESERVED0: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_SOFTWAREAUTOFLIP: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_SYSMEMEXECUTEBUFFER: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_SYSMEMREQUESTED: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_VPORTDATA: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURFGBL_VPORTINTERLEAVED: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_ATTACHED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_ATTACHED_FROM: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_BACKBUFFER: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_DATAISALIASED: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_DCIBUSY: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_DCILOCK: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_DRIVERMANAGED: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_FRONTBUFFER: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_GETDCNULL: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_HASCKEYDESTBLT: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_HASCKEYDESTOVERLAY: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_HASCKEYSRCBLT: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_HASCKEYSRCOVERLAY: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_HASDC: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_HASOVERLAYDATA: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_HASPIXELFORMAT: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_HELCB: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_HW_CKEYDESTBLT: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_HW_CKEYDESTOVERLAY: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_HW_CKEYSRCBLT: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_HW_CKEYSRCOVERLAY: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_IMPLICITCREATE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_IMPLICITROOT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_INMASTERSPRITELIST: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_INVALID: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_ISFREE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_LOCKEXCLUDEDCURSOR: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_PARTOFPRIMARYCHAIN: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_SETGAMMA: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_STEREOSURFACELEFT: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_SW_CKEYDESTBLT: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_SW_CKEYDESTOVERLAY: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_SW_CKEYSRCBLT: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWISURF_SW_CKEYSRCOVERLAY: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIVPORT_COLORKEYANDINTERP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIVPORT_NOKERNELHANDLES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIVPORT_ON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIVPORT_SOFTWARE_AUTOFLIP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIVPORT_SOFTWARE_BOB: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIVPORT_VBION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWIVPORT_VIDEOON: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_ATTACHEDTODESKTOP: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_BADPDEV: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_CHANGINGMODE: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_DDRAWDATANOTFETCHED: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_DISPLAYDRV: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_DRIVERINFO2: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_EMULATIONINITIALIZED: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_EXTENDEDALIGNMENT: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_FLIPPEDTOGDI: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_FULLSCREEN: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_GDIDRV: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_GETCOLOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_HASCKEYDESTOVERLAY: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_HASCKEYSRCOVERLAY: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_HASGDIPALETTE: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_HASGDIPALETTE_EXCLUSIVE: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_MODECHANGED: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_MODEX: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_MODEXILLEGAL: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_NEEDSWIN16FORVRAMLOCK: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_NOEMULATION: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_NOHARDWARE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_PALETTEINIT: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_PDEVICEVRAMBITCLEARED: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_SECONDARYDRIVERLOADED: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_SETCOLOR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_STANDARDVGA: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_TESTINGMODES: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_UMODELOADED: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_VIRTUALDESKTOP: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_VPORTGETCOLOR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_VPORTSETCOLOR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_VPORTSTART: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_VPORTSTOP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_VPORTUPDATE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_xxxxxxxxx1: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDRAWI_xxxxxxxxx2: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_ADDITIONALPRIMARY: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_COMMANDBUFFER: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_CUBEMAP: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_CUBEMAP_NEGATIVEX: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_CUBEMAP_NEGATIVEY: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_CUBEMAP_NEGATIVEZ: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_CUBEMAP_POSITIVEX: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_CUBEMAP_POSITIVEY: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_CUBEMAP_POSITIVEZ: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_D3DTEXTUREMANAGE: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_DISCARDBACKBUFFER: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_DONOTPERSIST: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_ENABLEALPHACHANNEL: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_EXTENDEDFORMATPRIMARY: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_HARDWAREDEINTERLACE: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_HINTANTIALIASING: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_HINTDYNAMIC: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_HINTSTATIC: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_INDEXBUFFER: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_MIPMAPSUBLEVEL: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_NOTUSERLOCKABLE: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_NPATCHES: i32 = 33554432i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_OPAQUE: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_POINTS: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_RESERVED1: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_RESERVED2: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_RESERVED3: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_RESERVED4: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_RTPATCHES: i32 = 16777216i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_STEREOSURFACELEFT: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_TEXTUREMANAGE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_VERTEXBUFFER: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS2_VOLUME: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS3_AUTOGENMIPMAP: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS3_CREATESHAREDRESOURCE: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS3_DMAP: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS3_LIGHTWEIGHTMIPMAP: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS3_MULTISAMPLE_MASK: i32 = 31i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS3_MULTISAMPLE_QUALITY_MASK: i32 = 224i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS3_MULTISAMPLE_QUALITY_SHIFT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS3_OPENSHAREDRESOURCE: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS3_READONLYRESOURCE: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS3_RESERVED1: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS3_RESERVED2: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS3_VIDEO: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_3DDEVICE: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_ALLOCONLOAD: i32 = 67108864i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_ALPHA: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_BACKBUFFER: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_COMMANDBUFFER: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_COMPLEX: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_EXECUTEBUFFER: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_FLIP: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_FRONTBUFFER: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_HWCODEC: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_LIVEVIDEO: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_LOCALVIDMEM: i32 = 268435456i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_MIPMAP: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_MODEX: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_NONLOCALVIDMEM: i32 = 536870912i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_OFFSCREENPLAIN: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_OPTIMIZED: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_OVERLAY: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_OWNDC: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_PALETTE: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_PRIMARYSURFACE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_PRIMARYSURFACELEFT: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_RESERVED1: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_RESERVED2: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_RESERVED3: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_STANDARDVGAMODE: i32 = 1073741824i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_SYSTEMMEMORY: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_TEXTURE: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_VIDEOMEMORY: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_VIDEOPORT: i32 = 134217728i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_VISIBLE: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_WRITEONLY: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCAPS_ZBUFFER: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCL_ALLOWMODEX: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCL_ALLOWREBOOT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCL_CREATEDEVICEWINDOW: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCL_EXCLUSIVE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCL_FPUPRESERVE: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCL_FPUSETUP: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCL_FULLSCREEN: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCL_MULTITHREADED: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCL_NORMAL: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCL_NOWINDOWCHANGES: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCL_SETDEVICEWINDOW: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSCL_SETFOCUSWINDOW: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSDM_STANDARDVGAMODE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_ALL: i32 = 16775662i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_ALPHABITDEPTH: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_BACKBUFFERCOUNT: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_CAPS: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_CKDESTBLT: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_CKDESTOVERLAY: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_CKSRCBLT: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_CKSRCOVERLAY: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_DEPTH: i32 = 8388608i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_FVF: i32 = 2097152i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_HEIGHT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_LINEARSIZE: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_LPSURFACE: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_MIPMAPCOUNT: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_PITCH: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_PIXELFORMAT: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_REFRESHRATE: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_SRCVBHANDLE: i32 = 4194304i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_TEXTURESTAGE: i32 = 1048576i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_WIDTH: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSD_ZBUFFERBITDEPTH: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSETSURFACEDESC_PRESERVEDC: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSETSURFACEDESC_RECREATEDC: i32 = 0i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSGR_CALIBRATE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSKIP_ENABLENEXT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSKIP_SKIPNEXT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSMT_ISTESTREQUIRED: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSPD_IUNKNOWNPOINTER: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSPD_VOLATILE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSVCAPS_RESERVED1: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSVCAPS_RESERVED2: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSVCAPS_RESERVED3: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSVCAPS_RESERVED4: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDSVCAPS_STEREOSEQUENTIAL: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDTRANSFER_CANCEL: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDTRANSFER_HALFLINES: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDTRANSFER_INVERT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDTRANSFER_NONLOCALVIDMEM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDTRANSFER_SYSTEMMEMORY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDUNSUPPORTEDMODE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVERSIONINFO: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPBCAPS_DESTINATION: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPBCAPS_SOURCE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPB_OVERLAY: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPB_TYPE: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPB_VIDEOPORT: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCAPS_AUTOFLIP: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCAPS_COLORCONTROL: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCAPS_HARDWAREDEINTERLACE: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCAPS_INTERLACED: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCAPS_NONINTERLACED: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCAPS_OVERSAMPLEDVBI: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCAPS_READBACKFIELD: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCAPS_READBACKLINE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCAPS_SHAREABLE: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCAPS_SKIPEVENFIELDS: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCAPS_SKIPODDFIELDS: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCAPS_SYNCMASTER: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCAPS_SYSTEMMEMORY: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCAPS_VBIANDVIDEOINDEPENDENT: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCAPS_VBISURFACE: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCONNECT_DISCARDSVREFDATA: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCONNECT_DOUBLECLOCK: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCONNECT_HALFLINE: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCONNECT_INTERLACED: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCONNECT_INVERTPOLARITY: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCONNECT_SHAREEVEN: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCONNECT_SHAREODD: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCONNECT_VACT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCREATE_VBIONLY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPCREATE_VIDEOONLY: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPD_ALIGN: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPD_AUTOFLIP: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPD_CAPS: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPD_FILTERQUALITY: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPD_FX: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPD_HEIGHT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPD_ID: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPD_PREFERREDAUTOFLIP: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPD_WIDTH: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFLIP_VBI: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFLIP_VIDEO: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFORMAT_VBI: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFORMAT_VIDEO: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFX_CROPTOPDATA: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFX_CROPX: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFX_CROPY: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFX_IGNOREVBIXCROP: i32 = 262144i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFX_INTERLEAVE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFX_MIRRORLEFTRIGHT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFX_MIRRORUPDOWN: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFX_PRESHRINKX: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFX_PRESHRINKXB: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFX_PRESHRINKXS: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFX_PRESHRINKY: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFX_PRESHRINKYB: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFX_PRESHRINKYS: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFX_PRESTRETCHX: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFX_PRESTRETCHXN: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFX_PRESTRETCHY: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFX_PRESTRETCHYN: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFX_VBICONVERT: i32 = 65536i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFX_VBINOINTERLEAVE: i32 = 524288i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPFX_VBINOSCALE: i32 = 131072i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPSQ_NOSIGNAL: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPSQ_SIGNALOK: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPSTATUS_VBIONLY: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPSTATUS_VIDEOONLY: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPTARGET_VBI: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPTARGET_VIDEO: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPTYPE_BROOKTREE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1352a560_da61_11cf_9b06_00a0c903a3b8);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPTYPE_CCIR656: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfca326a0_da60_11cf_9b06_00a0c903a3b8);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPTYPE_E_HREFH_VREFH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54f39980_da60_11cf_9b06_00a0c903a3b8);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPTYPE_E_HREFH_VREFL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x92783220_da60_11cf_9b06_00a0c903a3b8);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPTYPE_E_HREFL_VREFH: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa07a02e0_da60_11cf_9b06_00a0c903a3b8);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPTYPE_E_HREFL_VREFL: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe09c77e0_da60_11cf_9b06_00a0c903a3b8);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPTYPE_PHILIPS: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x332cf160_da61_11cf_9b06_00a0c903a3b8);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPWAIT_BEGIN: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPWAIT_END: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVPWAIT_LINE: i32 = 3i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVP_AUTOFLIP: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVP_CONVERT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVP_CROP: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVP_HARDWAREDEINTERLACE: i32 = 32768i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVP_IGNOREVBIXCROP: i32 = 8192i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVP_INTERLEAVE: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVP_MIRRORLEFTRIGHT: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVP_MIRRORUPDOWN: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVP_OVERRIDEBOBWEAVE: i32 = 4096i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVP_PRESCALE: i32 = 64i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVP_SKIPEVENFIELDS: i32 = 128i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVP_SKIPODDFIELDS: i32 = 256i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVP_SYNCMASTER: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVP_VBICONVERT: i32 = 1024i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVP_VBINOINTERLEAVE: i32 = 16384i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDVP_VBINOSCALE: i32 = 2048i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDWAITVB_BLOCKBEGIN: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDWAITVB_BLOCKBEGINEVENT: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDWAITVB_BLOCKEND: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DDWAITVB_I_TESTVB: i32 = -2147483642i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DD_HAL_VERSION: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DD_RUNTIME_VERSION: i32 = 2306i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DD_VERSION: i32 = 512i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DELETED_LASTONE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DELETED_NOTFOUND: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DELETED_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DIRECTDRAW_VERSION: u32 = 1792u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DXAPI_HALVERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DXERR_GENERIC: u32 = 2147500037u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DXERR_OUTOFCAPS: u32 = 2289434984u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DXERR_UNSUPPORTED: u32 = 2147500033u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const DX_OK: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_ColorControlCallbacks: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefd60cc2_49e7_11d0_889d_00aa00bbb76a);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_D3DCallbacks: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bf06990_8794_11d0_9139_080036d2ef02);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_D3DCallbacks2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ba584e1_70b6_11d0_889d_00aa00bbb76a);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_D3DCallbacks3: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddf41230_ec0a_11d0_a9b6_00aa00c0993e);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_D3DCaps: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7bf06991_8794_11d0_9139_080036d2ef02);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_D3DExtendedCaps: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7de41f80_9d93_11d0_89ab_00a0c9054129);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_D3DParseUnknownCommandCallback: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e04ffa0_98e4_11d1_8ce1_00a0c90629a8);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_DDMoreCaps: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x880baf30_b030_11d0_8ea7_00609797ea5b);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_DDMoreSurfaceCaps: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b8a0466_f269_11d1_880b_00c04fd930c5);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_DDStereoMode: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf828169c_a8e8_11d2_a1f2_00a0c983eaf6);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_DxApi: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8a79bef0_b915_11d0_9144_080036d2ef02);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_GetHeapAlignment: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42e02f16_7b41_11d2_8bff_00a0c983eaf6);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_KernelCallbacks: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80863800_6b06_11d0_9b06_00a0c903a3b8);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_KernelCaps: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffaa7540_7aa8_11d0_9b06_00a0c903a3b8);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_Miscellaneous2Callbacks: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x406b2f00_3e5a_11d1_b640_00aa00a1f96a);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_MiscellaneousCallbacks: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefd60cc0_49e7_11d0_889d_00aa00bbb76a);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_MotionCompCallbacks: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1122b40_5da5_11d1_8fcf_00c04fc29b4e);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_NTCallbacks: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fe9ecde_df89_11d1_9db0_0060082771ba);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_NTPrivateDriverCaps: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfad16a23_7b66_11d2_83d7_00c04f7ce58c);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_NonLocalVidMemCaps: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86c4fa80_8d84_11d0_94e8_00c04fc34137);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_OptSurfaceKmodeInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe05c8472_51d4_11d1_8cce_00a0c90629a8);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_OptSurfaceUmodeInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9d792804_5fa8_11d1_8cd0_00a0c90629a8);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_UpdateNonLocalHeap: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x42e02f17_7b41_11d2_8bff_00a0c983eaf6);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_UserModeDriverInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0b0e8e2_5f97_11d1_8cd0_00a0c90629a8);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_UserModeDriverPassword: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97f861b6_60a1_11d1_8cd0_00a0c90629a8);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_VPE2Callbacks: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52882147_2d47_469a_a0d1_03455890f6c8);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_VideoPortCallbacks: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefd60cc1_49e7_11d0_889d_00aa00bbb76a);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_VideoPortCaps: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefd60cc3_49e7_11d0_889d_00aa00bbb76a);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const GUID_ZPixelFormats: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93869880_36cf_11d1_9b1b_00aa00bbb8ae);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const HEAPALIASINFO_MAPPEDDUMMY: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const HEAPALIASINFO_MAPPEDREAL: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const IRQINFO_HANDLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const IRQINFO_NOTHANDLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const MAX_AUTOFLIP_BUFFERS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const MAX_DDDEVICEID_STRING: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const MAX_DRIVER_NAME: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const MAX_PALETTE_SIZE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const MDL_64_BIT_VA: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const MDL_ALLOCATED_FIXED_SIZE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const MDL_ALLOCATED_MUST_SUCCEED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const MDL_IO_PAGE_READ: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const MDL_IO_SPACE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const MDL_LOCK_HELD: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const MDL_MAPPED_TO_SYSTEM_VA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const MDL_MAPPING_CAN_FAIL: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const MDL_NETWORK_HEADER: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const MDL_PAGES_LOCKED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const MDL_PARENT_MAPPED_SYSTEM_VA: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const MDL_PARTIAL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const MDL_PARTIAL_HAS_BEEN_MAPPED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const MDL_SCATTER_GATHER_VA: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const MDL_SOURCE_IS_NONPAGED_POOL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const MDL_WRITE_OPERATION: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const OBJECT_ISROOT: i32 = -2147483648i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const PFINDEX_UNINITIALIZED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const REGSTR_KEY_DDHW_DESCRIPTION: ::windows::core::PCSTR = ::windows::core::s!("Description");
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const REGSTR_KEY_DDHW_DRIVERNAME: ::windows::core::PCSTR = ::windows::core::s!("DriverName");
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const REGSTR_PATH_DDHW: ::windows::core::PCSTR = ::windows::core::s!("Hardware\\DirectDrawDrivers");
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const ROP_HAS_PATTERN: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const ROP_HAS_SOURCE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const SURFACEALIGN_DISCARDABLE: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const VIDMEM_HEAPDISABLED: i32 = 32i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const VIDMEM_ISHEAP: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const VIDMEM_ISLINEAR: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const VIDMEM_ISNONLOCAL: i32 = 8i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const VIDMEM_ISRECTANGULAR: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const VIDMEM_ISWC: i32 = 16i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const VMEMHEAP_ALIGNMENT: i32 = 4i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const VMEMHEAP_LINEAR: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const VMEMHEAP_RECTANGULAR: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub const _FACDD: u32 = 2166u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct ACCESSRECTLIST {
    pub lpLink: *mut ACCESSRECTLIST,
    pub rDest: super::super::Foundation::RECT,
    pub lpOwner: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpSurfaceData: *mut ::core::ffi::c_void,
    pub dwFlags: u32,
    pub lpHeapAliasInfo: *mut HEAPALIASINFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for ACCESSRECTLIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for ACCESSRECTLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for ACCESSRECTLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ACCESSRECTLIST").field("lpLink", &self.lpLink).field("rDest", &self.rDest).field("lpOwner", &self.lpOwner).field("lpSurfaceData", &self.lpSurfaceData).field("dwFlags", &self.dwFlags).field("lpHeapAliasInfo", &self.lpHeapAliasInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for ACCESSRECTLIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for ACCESSRECTLIST {
    fn eq(&self, other: &Self) -> bool {
        self.lpLink == other.lpLink && self.rDest == other.rDest && self.lpOwner == other.lpOwner && self.lpSurfaceData == other.lpSurfaceData && self.dwFlags == other.dwFlags && self.lpHeapAliasInfo == other.lpHeapAliasInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for ACCESSRECTLIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for ACCESSRECTLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct ATTACHLIST {
    pub dwFlags: u32,
    pub lpLink: *mut ATTACHLIST,
    pub lpAttached: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub lpIAttached: *mut DDRAWI_DDRAWSURFACE_INT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for ATTACHLIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for ATTACHLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for ATTACHLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ATTACHLIST").field("dwFlags", &self.dwFlags).field("lpLink", &self.lpLink).field("lpAttached", &self.lpAttached).field("lpIAttached", &self.lpIAttached).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for ATTACHLIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for ATTACHLIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.lpLink == other.lpLink && self.lpAttached == other.lpAttached && self.lpIAttached == other.lpIAttached
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for ATTACHLIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for ATTACHLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DBLNODE {
    pub next: *mut DBLNODE,
    pub prev: *mut DBLNODE,
    pub object: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub object_int: *mut DDRAWI_DDRAWSURFACE_INT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DBLNODE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DBLNODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DBLNODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DBLNODE").field("next", &self.next).field("prev", &self.prev).field("object", &self.object).field("object_int", &self.object_int).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DBLNODE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DBLNODE {
    fn eq(&self, other: &Self) -> bool {
        self.next == other.next && self.prev == other.prev && self.object == other.object && self.object_int == other.object_int
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DBLNODE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DBLNODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD32BITDRIVERDATA {
    pub szName: [u8; 260],
    pub szEntryPoint: [u8; 64],
    pub dwContext: u32,
}
impl ::core::marker::Copy for DD32BITDRIVERDATA {}
impl ::core::clone::Clone for DD32BITDRIVERDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD32BITDRIVERDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD32BITDRIVERDATA").field("szName", &self.szName).field("szEntryPoint", &self.szEntryPoint).field("dwContext", &self.dwContext).finish()
    }
}
impl ::windows::core::TypeKind for DD32BITDRIVERDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD32BITDRIVERDATA {
    fn eq(&self, other: &Self) -> bool {
        self.szName == other.szName && self.szEntryPoint == other.szEntryPoint && self.dwContext == other.dwContext
    }
}
impl ::core::cmp::Eq for DD32BITDRIVERDATA {}
impl ::core::default::Default for DD32BITDRIVERDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDARGB {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub alpha: u8,
}
impl ::core::marker::Copy for DDARGB {}
impl ::core::clone::Clone for DDARGB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDARGB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDARGB").field("blue", &self.blue).field("green", &self.green).field("red", &self.red).field("alpha", &self.alpha).finish()
    }
}
impl ::windows::core::TypeKind for DDARGB {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDARGB {
    fn eq(&self, other: &Self) -> bool {
        self.blue == other.blue && self.green == other.green && self.red == other.red && self.alpha == other.alpha
    }
}
impl ::core::cmp::Eq for DDARGB {}
impl ::core::default::Default for DDARGB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DDBLTBATCH {
    pub lprDest: *mut super::super::Foundation::RECT,
    pub lpDDSSrc: ::std::mem::ManuallyDrop<::core::option::Option<IDirectDrawSurface>>,
    pub lprSrc: *mut super::super::Foundation::RECT,
    pub dwFlags: u32,
    pub lpDDBltFx: *mut DDBLTFX,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DDBLTBATCH {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DDBLTBATCH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDBLTBATCH").field("lprDest", &self.lprDest).field("lpDDSSrc", &self.lpDDSSrc).field("lprSrc", &self.lprSrc).field("dwFlags", &self.dwFlags).field("lpDDBltFx", &self.lpDDBltFx).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DDBLTBATCH {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DDBLTBATCH {
    fn eq(&self, other: &Self) -> bool {
        self.lprDest == other.lprDest && self.lpDDSSrc == other.lpDDSSrc && self.lprSrc == other.lprSrc && self.dwFlags == other.dwFlags && self.lpDDBltFx == other.lpDDBltFx
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DDBLTBATCH {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DDBLTBATCH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDBLTFX {
    pub dwSize: u32,
    pub dwDDFX: u32,
    pub dwROP: u32,
    pub dwDDROP: u32,
    pub dwRotationAngle: u32,
    pub dwZBufferOpCode: u32,
    pub dwZBufferLow: u32,
    pub dwZBufferHigh: u32,
    pub dwZBufferBaseDest: u32,
    pub dwZDestConstBitDepth: u32,
    pub Anonymous1: DDBLTFX_0,
    pub dwZSrcConstBitDepth: u32,
    pub Anonymous2: DDBLTFX_1,
    pub dwAlphaEdgeBlendBitDepth: u32,
    pub dwAlphaEdgeBlend: u32,
    pub dwReserved: u32,
    pub dwAlphaDestConstBitDepth: u32,
    pub Anonymous3: DDBLTFX_2,
    pub dwAlphaSrcConstBitDepth: u32,
    pub Anonymous4: DDBLTFX_3,
    pub Anonymous5: DDBLTFX_4,
    pub ddckDestColorkey: DDCOLORKEY,
    pub ddckSrcColorkey: DDCOLORKEY,
}
impl ::core::clone::Clone for DDBLTFX {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows::core::TypeKind for DDBLTFX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDBLTFX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDBLTFX_0 {
    pub dwZDestConst: u32,
    pub lpDDSZBufferDest: ::std::mem::ManuallyDrop<::core::option::Option<IDirectDrawSurface>>,
}
impl ::core::clone::Clone for DDBLTFX_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows::core::TypeKind for DDBLTFX_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDBLTFX_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDBLTFX_1 {
    pub dwZSrcConst: u32,
    pub lpDDSZBufferSrc: ::std::mem::ManuallyDrop<::core::option::Option<IDirectDrawSurface>>,
}
impl ::core::clone::Clone for DDBLTFX_1 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows::core::TypeKind for DDBLTFX_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDBLTFX_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDBLTFX_2 {
    pub dwAlphaDestConst: u32,
    pub lpDDSAlphaDest: ::std::mem::ManuallyDrop<::core::option::Option<IDirectDrawSurface>>,
}
impl ::core::clone::Clone for DDBLTFX_2 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows::core::TypeKind for DDBLTFX_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDBLTFX_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDBLTFX_3 {
    pub dwAlphaSrcConst: u32,
    pub lpDDSAlphaSrc: ::std::mem::ManuallyDrop<::core::option::Option<IDirectDrawSurface>>,
}
impl ::core::clone::Clone for DDBLTFX_3 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows::core::TypeKind for DDBLTFX_3 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDBLTFX_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDBLTFX_4 {
    pub dwFillColor: u32,
    pub dwFillDepth: u32,
    pub dwFillPixel: u32,
    pub lpDDSPattern: ::std::mem::ManuallyDrop<::core::option::Option<IDirectDrawSurface>>,
}
impl ::core::clone::Clone for DDBLTFX_4 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows::core::TypeKind for DDBLTFX_4 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDBLTFX_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDBOBNEXTFIELDINFO {
    pub lpSurface: *mut DDSURFACEDATA,
}
impl ::core::marker::Copy for DDBOBNEXTFIELDINFO {}
impl ::core::clone::Clone for DDBOBNEXTFIELDINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDBOBNEXTFIELDINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDBOBNEXTFIELDINFO").field("lpSurface", &self.lpSurface).finish()
    }
}
impl ::windows::core::TypeKind for DDBOBNEXTFIELDINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDBOBNEXTFIELDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpSurface == other.lpSurface
    }
}
impl ::core::cmp::Eq for DDBOBNEXTFIELDINFO {}
impl ::core::default::Default for DDBOBNEXTFIELDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDCAPS_DX1 {
    pub dwSize: u32,
    pub dwCaps: u32,
    pub dwCaps2: u32,
    pub dwCKeyCaps: u32,
    pub dwFXCaps: u32,
    pub dwFXAlphaCaps: u32,
    pub dwPalCaps: u32,
    pub dwSVCaps: u32,
    pub dwAlphaBltConstBitDepths: u32,
    pub dwAlphaBltPixelBitDepths: u32,
    pub dwAlphaBltSurfaceBitDepths: u32,
    pub dwAlphaOverlayConstBitDepths: u32,
    pub dwAlphaOverlayPixelBitDepths: u32,
    pub dwAlphaOverlaySurfaceBitDepths: u32,
    pub dwZBufferBitDepths: u32,
    pub dwVidMemTotal: u32,
    pub dwVidMemFree: u32,
    pub dwMaxVisibleOverlays: u32,
    pub dwCurrVisibleOverlays: u32,
    pub dwNumFourCCCodes: u32,
    pub dwAlignBoundarySrc: u32,
    pub dwAlignSizeSrc: u32,
    pub dwAlignBoundaryDest: u32,
    pub dwAlignSizeDest: u32,
    pub dwAlignStrideAlign: u32,
    pub dwRops: [u32; 8],
    pub ddsCaps: DDSCAPS,
    pub dwMinOverlayStretch: u32,
    pub dwMaxOverlayStretch: u32,
    pub dwMinLiveVideoStretch: u32,
    pub dwMaxLiveVideoStretch: u32,
    pub dwMinHwCodecStretch: u32,
    pub dwMaxHwCodecStretch: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
}
impl ::core::marker::Copy for DDCAPS_DX1 {}
impl ::core::clone::Clone for DDCAPS_DX1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDCAPS_DX1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDCAPS_DX1")
            .field("dwSize", &self.dwSize)
            .field("dwCaps", &self.dwCaps)
            .field("dwCaps2", &self.dwCaps2)
            .field("dwCKeyCaps", &self.dwCKeyCaps)
            .field("dwFXCaps", &self.dwFXCaps)
            .field("dwFXAlphaCaps", &self.dwFXAlphaCaps)
            .field("dwPalCaps", &self.dwPalCaps)
            .field("dwSVCaps", &self.dwSVCaps)
            .field("dwAlphaBltConstBitDepths", &self.dwAlphaBltConstBitDepths)
            .field("dwAlphaBltPixelBitDepths", &self.dwAlphaBltPixelBitDepths)
            .field("dwAlphaBltSurfaceBitDepths", &self.dwAlphaBltSurfaceBitDepths)
            .field("dwAlphaOverlayConstBitDepths", &self.dwAlphaOverlayConstBitDepths)
            .field("dwAlphaOverlayPixelBitDepths", &self.dwAlphaOverlayPixelBitDepths)
            .field("dwAlphaOverlaySurfaceBitDepths", &self.dwAlphaOverlaySurfaceBitDepths)
            .field("dwZBufferBitDepths", &self.dwZBufferBitDepths)
            .field("dwVidMemTotal", &self.dwVidMemTotal)
            .field("dwVidMemFree", &self.dwVidMemFree)
            .field("dwMaxVisibleOverlays", &self.dwMaxVisibleOverlays)
            .field("dwCurrVisibleOverlays", &self.dwCurrVisibleOverlays)
            .field("dwNumFourCCCodes", &self.dwNumFourCCCodes)
            .field("dwAlignBoundarySrc", &self.dwAlignBoundarySrc)
            .field("dwAlignSizeSrc", &self.dwAlignSizeSrc)
            .field("dwAlignBoundaryDest", &self.dwAlignBoundaryDest)
            .field("dwAlignSizeDest", &self.dwAlignSizeDest)
            .field("dwAlignStrideAlign", &self.dwAlignStrideAlign)
            .field("dwRops", &self.dwRops)
            .field("ddsCaps", &self.ddsCaps)
            .field("dwMinOverlayStretch", &self.dwMinOverlayStretch)
            .field("dwMaxOverlayStretch", &self.dwMaxOverlayStretch)
            .field("dwMinLiveVideoStretch", &self.dwMinLiveVideoStretch)
            .field("dwMaxLiveVideoStretch", &self.dwMaxLiveVideoStretch)
            .field("dwMinHwCodecStretch", &self.dwMinHwCodecStretch)
            .field("dwMaxHwCodecStretch", &self.dwMaxHwCodecStretch)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .finish()
    }
}
impl ::windows::core::TypeKind for DDCAPS_DX1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDCAPS_DX1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwCaps == other.dwCaps
            && self.dwCaps2 == other.dwCaps2
            && self.dwCKeyCaps == other.dwCKeyCaps
            && self.dwFXCaps == other.dwFXCaps
            && self.dwFXAlphaCaps == other.dwFXAlphaCaps
            && self.dwPalCaps == other.dwPalCaps
            && self.dwSVCaps == other.dwSVCaps
            && self.dwAlphaBltConstBitDepths == other.dwAlphaBltConstBitDepths
            && self.dwAlphaBltPixelBitDepths == other.dwAlphaBltPixelBitDepths
            && self.dwAlphaBltSurfaceBitDepths == other.dwAlphaBltSurfaceBitDepths
            && self.dwAlphaOverlayConstBitDepths == other.dwAlphaOverlayConstBitDepths
            && self.dwAlphaOverlayPixelBitDepths == other.dwAlphaOverlayPixelBitDepths
            && self.dwAlphaOverlaySurfaceBitDepths == other.dwAlphaOverlaySurfaceBitDepths
            && self.dwZBufferBitDepths == other.dwZBufferBitDepths
            && self.dwVidMemTotal == other.dwVidMemTotal
            && self.dwVidMemFree == other.dwVidMemFree
            && self.dwMaxVisibleOverlays == other.dwMaxVisibleOverlays
            && self.dwCurrVisibleOverlays == other.dwCurrVisibleOverlays
            && self.dwNumFourCCCodes == other.dwNumFourCCCodes
            && self.dwAlignBoundarySrc == other.dwAlignBoundarySrc
            && self.dwAlignSizeSrc == other.dwAlignSizeSrc
            && self.dwAlignBoundaryDest == other.dwAlignBoundaryDest
            && self.dwAlignSizeDest == other.dwAlignSizeDest
            && self.dwAlignStrideAlign == other.dwAlignStrideAlign
            && self.dwRops == other.dwRops
            && self.ddsCaps == other.ddsCaps
            && self.dwMinOverlayStretch == other.dwMinOverlayStretch
            && self.dwMaxOverlayStretch == other.dwMaxOverlayStretch
            && self.dwMinLiveVideoStretch == other.dwMinLiveVideoStretch
            && self.dwMaxLiveVideoStretch == other.dwMaxLiveVideoStretch
            && self.dwMinHwCodecStretch == other.dwMinHwCodecStretch
            && self.dwMaxHwCodecStretch == other.dwMaxHwCodecStretch
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.dwReserved3 == other.dwReserved3
    }
}
impl ::core::cmp::Eq for DDCAPS_DX1 {}
impl ::core::default::Default for DDCAPS_DX1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDCAPS_DX3 {
    pub dwSize: u32,
    pub dwCaps: u32,
    pub dwCaps2: u32,
    pub dwCKeyCaps: u32,
    pub dwFXCaps: u32,
    pub dwFXAlphaCaps: u32,
    pub dwPalCaps: u32,
    pub dwSVCaps: u32,
    pub dwAlphaBltConstBitDepths: u32,
    pub dwAlphaBltPixelBitDepths: u32,
    pub dwAlphaBltSurfaceBitDepths: u32,
    pub dwAlphaOverlayConstBitDepths: u32,
    pub dwAlphaOverlayPixelBitDepths: u32,
    pub dwAlphaOverlaySurfaceBitDepths: u32,
    pub dwZBufferBitDepths: u32,
    pub dwVidMemTotal: u32,
    pub dwVidMemFree: u32,
    pub dwMaxVisibleOverlays: u32,
    pub dwCurrVisibleOverlays: u32,
    pub dwNumFourCCCodes: u32,
    pub dwAlignBoundarySrc: u32,
    pub dwAlignSizeSrc: u32,
    pub dwAlignBoundaryDest: u32,
    pub dwAlignSizeDest: u32,
    pub dwAlignStrideAlign: u32,
    pub dwRops: [u32; 8],
    pub ddsCaps: DDSCAPS,
    pub dwMinOverlayStretch: u32,
    pub dwMaxOverlayStretch: u32,
    pub dwMinLiveVideoStretch: u32,
    pub dwMaxLiveVideoStretch: u32,
    pub dwMinHwCodecStretch: u32,
    pub dwMaxHwCodecStretch: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub dwSVBCaps: u32,
    pub dwSVBCKeyCaps: u32,
    pub dwSVBFXCaps: u32,
    pub dwSVBRops: [u32; 8],
    pub dwVSBCaps: u32,
    pub dwVSBCKeyCaps: u32,
    pub dwVSBFXCaps: u32,
    pub dwVSBRops: [u32; 8],
    pub dwSSBCaps: u32,
    pub dwSSBCKeyCaps: u32,
    pub dwSSBFXCaps: u32,
    pub dwSSBRops: [u32; 8],
    pub dwReserved4: u32,
    pub dwReserved5: u32,
    pub dwReserved6: u32,
}
impl ::core::marker::Copy for DDCAPS_DX3 {}
impl ::core::clone::Clone for DDCAPS_DX3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDCAPS_DX3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDCAPS_DX3")
            .field("dwSize", &self.dwSize)
            .field("dwCaps", &self.dwCaps)
            .field("dwCaps2", &self.dwCaps2)
            .field("dwCKeyCaps", &self.dwCKeyCaps)
            .field("dwFXCaps", &self.dwFXCaps)
            .field("dwFXAlphaCaps", &self.dwFXAlphaCaps)
            .field("dwPalCaps", &self.dwPalCaps)
            .field("dwSVCaps", &self.dwSVCaps)
            .field("dwAlphaBltConstBitDepths", &self.dwAlphaBltConstBitDepths)
            .field("dwAlphaBltPixelBitDepths", &self.dwAlphaBltPixelBitDepths)
            .field("dwAlphaBltSurfaceBitDepths", &self.dwAlphaBltSurfaceBitDepths)
            .field("dwAlphaOverlayConstBitDepths", &self.dwAlphaOverlayConstBitDepths)
            .field("dwAlphaOverlayPixelBitDepths", &self.dwAlphaOverlayPixelBitDepths)
            .field("dwAlphaOverlaySurfaceBitDepths", &self.dwAlphaOverlaySurfaceBitDepths)
            .field("dwZBufferBitDepths", &self.dwZBufferBitDepths)
            .field("dwVidMemTotal", &self.dwVidMemTotal)
            .field("dwVidMemFree", &self.dwVidMemFree)
            .field("dwMaxVisibleOverlays", &self.dwMaxVisibleOverlays)
            .field("dwCurrVisibleOverlays", &self.dwCurrVisibleOverlays)
            .field("dwNumFourCCCodes", &self.dwNumFourCCCodes)
            .field("dwAlignBoundarySrc", &self.dwAlignBoundarySrc)
            .field("dwAlignSizeSrc", &self.dwAlignSizeSrc)
            .field("dwAlignBoundaryDest", &self.dwAlignBoundaryDest)
            .field("dwAlignSizeDest", &self.dwAlignSizeDest)
            .field("dwAlignStrideAlign", &self.dwAlignStrideAlign)
            .field("dwRops", &self.dwRops)
            .field("ddsCaps", &self.ddsCaps)
            .field("dwMinOverlayStretch", &self.dwMinOverlayStretch)
            .field("dwMaxOverlayStretch", &self.dwMaxOverlayStretch)
            .field("dwMinLiveVideoStretch", &self.dwMinLiveVideoStretch)
            .field("dwMaxLiveVideoStretch", &self.dwMaxLiveVideoStretch)
            .field("dwMinHwCodecStretch", &self.dwMinHwCodecStretch)
            .field("dwMaxHwCodecStretch", &self.dwMaxHwCodecStretch)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .field("dwSVBCaps", &self.dwSVBCaps)
            .field("dwSVBCKeyCaps", &self.dwSVBCKeyCaps)
            .field("dwSVBFXCaps", &self.dwSVBFXCaps)
            .field("dwSVBRops", &self.dwSVBRops)
            .field("dwVSBCaps", &self.dwVSBCaps)
            .field("dwVSBCKeyCaps", &self.dwVSBCKeyCaps)
            .field("dwVSBFXCaps", &self.dwVSBFXCaps)
            .field("dwVSBRops", &self.dwVSBRops)
            .field("dwSSBCaps", &self.dwSSBCaps)
            .field("dwSSBCKeyCaps", &self.dwSSBCKeyCaps)
            .field("dwSSBFXCaps", &self.dwSSBFXCaps)
            .field("dwSSBRops", &self.dwSSBRops)
            .field("dwReserved4", &self.dwReserved4)
            .field("dwReserved5", &self.dwReserved5)
            .field("dwReserved6", &self.dwReserved6)
            .finish()
    }
}
impl ::windows::core::TypeKind for DDCAPS_DX3 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDCAPS_DX3 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwCaps == other.dwCaps
            && self.dwCaps2 == other.dwCaps2
            && self.dwCKeyCaps == other.dwCKeyCaps
            && self.dwFXCaps == other.dwFXCaps
            && self.dwFXAlphaCaps == other.dwFXAlphaCaps
            && self.dwPalCaps == other.dwPalCaps
            && self.dwSVCaps == other.dwSVCaps
            && self.dwAlphaBltConstBitDepths == other.dwAlphaBltConstBitDepths
            && self.dwAlphaBltPixelBitDepths == other.dwAlphaBltPixelBitDepths
            && self.dwAlphaBltSurfaceBitDepths == other.dwAlphaBltSurfaceBitDepths
            && self.dwAlphaOverlayConstBitDepths == other.dwAlphaOverlayConstBitDepths
            && self.dwAlphaOverlayPixelBitDepths == other.dwAlphaOverlayPixelBitDepths
            && self.dwAlphaOverlaySurfaceBitDepths == other.dwAlphaOverlaySurfaceBitDepths
            && self.dwZBufferBitDepths == other.dwZBufferBitDepths
            && self.dwVidMemTotal == other.dwVidMemTotal
            && self.dwVidMemFree == other.dwVidMemFree
            && self.dwMaxVisibleOverlays == other.dwMaxVisibleOverlays
            && self.dwCurrVisibleOverlays == other.dwCurrVisibleOverlays
            && self.dwNumFourCCCodes == other.dwNumFourCCCodes
            && self.dwAlignBoundarySrc == other.dwAlignBoundarySrc
            && self.dwAlignSizeSrc == other.dwAlignSizeSrc
            && self.dwAlignBoundaryDest == other.dwAlignBoundaryDest
            && self.dwAlignSizeDest == other.dwAlignSizeDest
            && self.dwAlignStrideAlign == other.dwAlignStrideAlign
            && self.dwRops == other.dwRops
            && self.ddsCaps == other.ddsCaps
            && self.dwMinOverlayStretch == other.dwMinOverlayStretch
            && self.dwMaxOverlayStretch == other.dwMaxOverlayStretch
            && self.dwMinLiveVideoStretch == other.dwMinLiveVideoStretch
            && self.dwMaxLiveVideoStretch == other.dwMaxLiveVideoStretch
            && self.dwMinHwCodecStretch == other.dwMinHwCodecStretch
            && self.dwMaxHwCodecStretch == other.dwMaxHwCodecStretch
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.dwReserved3 == other.dwReserved3
            && self.dwSVBCaps == other.dwSVBCaps
            && self.dwSVBCKeyCaps == other.dwSVBCKeyCaps
            && self.dwSVBFXCaps == other.dwSVBFXCaps
            && self.dwSVBRops == other.dwSVBRops
            && self.dwVSBCaps == other.dwVSBCaps
            && self.dwVSBCKeyCaps == other.dwVSBCKeyCaps
            && self.dwVSBFXCaps == other.dwVSBFXCaps
            && self.dwVSBRops == other.dwVSBRops
            && self.dwSSBCaps == other.dwSSBCaps
            && self.dwSSBCKeyCaps == other.dwSSBCKeyCaps
            && self.dwSSBFXCaps == other.dwSSBFXCaps
            && self.dwSSBRops == other.dwSSBRops
            && self.dwReserved4 == other.dwReserved4
            && self.dwReserved5 == other.dwReserved5
            && self.dwReserved6 == other.dwReserved6
    }
}
impl ::core::cmp::Eq for DDCAPS_DX3 {}
impl ::core::default::Default for DDCAPS_DX3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDCAPS_DX5 {
    pub dwSize: u32,
    pub dwCaps: u32,
    pub dwCaps2: u32,
    pub dwCKeyCaps: u32,
    pub dwFXCaps: u32,
    pub dwFXAlphaCaps: u32,
    pub dwPalCaps: u32,
    pub dwSVCaps: u32,
    pub dwAlphaBltConstBitDepths: u32,
    pub dwAlphaBltPixelBitDepths: u32,
    pub dwAlphaBltSurfaceBitDepths: u32,
    pub dwAlphaOverlayConstBitDepths: u32,
    pub dwAlphaOverlayPixelBitDepths: u32,
    pub dwAlphaOverlaySurfaceBitDepths: u32,
    pub dwZBufferBitDepths: u32,
    pub dwVidMemTotal: u32,
    pub dwVidMemFree: u32,
    pub dwMaxVisibleOverlays: u32,
    pub dwCurrVisibleOverlays: u32,
    pub dwNumFourCCCodes: u32,
    pub dwAlignBoundarySrc: u32,
    pub dwAlignSizeSrc: u32,
    pub dwAlignBoundaryDest: u32,
    pub dwAlignSizeDest: u32,
    pub dwAlignStrideAlign: u32,
    pub dwRops: [u32; 8],
    pub ddsCaps: DDSCAPS,
    pub dwMinOverlayStretch: u32,
    pub dwMaxOverlayStretch: u32,
    pub dwMinLiveVideoStretch: u32,
    pub dwMaxLiveVideoStretch: u32,
    pub dwMinHwCodecStretch: u32,
    pub dwMaxHwCodecStretch: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub dwSVBCaps: u32,
    pub dwSVBCKeyCaps: u32,
    pub dwSVBFXCaps: u32,
    pub dwSVBRops: [u32; 8],
    pub dwVSBCaps: u32,
    pub dwVSBCKeyCaps: u32,
    pub dwVSBFXCaps: u32,
    pub dwVSBRops: [u32; 8],
    pub dwSSBCaps: u32,
    pub dwSSBCKeyCaps: u32,
    pub dwSSBFXCaps: u32,
    pub dwSSBRops: [u32; 8],
    pub dwMaxVideoPorts: u32,
    pub dwCurrVideoPorts: u32,
    pub dwSVBCaps2: u32,
    pub dwNLVBCaps: u32,
    pub dwNLVBCaps2: u32,
    pub dwNLVBCKeyCaps: u32,
    pub dwNLVBFXCaps: u32,
    pub dwNLVBRops: [u32; 8],
}
impl ::core::marker::Copy for DDCAPS_DX5 {}
impl ::core::clone::Clone for DDCAPS_DX5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDCAPS_DX5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDCAPS_DX5")
            .field("dwSize", &self.dwSize)
            .field("dwCaps", &self.dwCaps)
            .field("dwCaps2", &self.dwCaps2)
            .field("dwCKeyCaps", &self.dwCKeyCaps)
            .field("dwFXCaps", &self.dwFXCaps)
            .field("dwFXAlphaCaps", &self.dwFXAlphaCaps)
            .field("dwPalCaps", &self.dwPalCaps)
            .field("dwSVCaps", &self.dwSVCaps)
            .field("dwAlphaBltConstBitDepths", &self.dwAlphaBltConstBitDepths)
            .field("dwAlphaBltPixelBitDepths", &self.dwAlphaBltPixelBitDepths)
            .field("dwAlphaBltSurfaceBitDepths", &self.dwAlphaBltSurfaceBitDepths)
            .field("dwAlphaOverlayConstBitDepths", &self.dwAlphaOverlayConstBitDepths)
            .field("dwAlphaOverlayPixelBitDepths", &self.dwAlphaOverlayPixelBitDepths)
            .field("dwAlphaOverlaySurfaceBitDepths", &self.dwAlphaOverlaySurfaceBitDepths)
            .field("dwZBufferBitDepths", &self.dwZBufferBitDepths)
            .field("dwVidMemTotal", &self.dwVidMemTotal)
            .field("dwVidMemFree", &self.dwVidMemFree)
            .field("dwMaxVisibleOverlays", &self.dwMaxVisibleOverlays)
            .field("dwCurrVisibleOverlays", &self.dwCurrVisibleOverlays)
            .field("dwNumFourCCCodes", &self.dwNumFourCCCodes)
            .field("dwAlignBoundarySrc", &self.dwAlignBoundarySrc)
            .field("dwAlignSizeSrc", &self.dwAlignSizeSrc)
            .field("dwAlignBoundaryDest", &self.dwAlignBoundaryDest)
            .field("dwAlignSizeDest", &self.dwAlignSizeDest)
            .field("dwAlignStrideAlign", &self.dwAlignStrideAlign)
            .field("dwRops", &self.dwRops)
            .field("ddsCaps", &self.ddsCaps)
            .field("dwMinOverlayStretch", &self.dwMinOverlayStretch)
            .field("dwMaxOverlayStretch", &self.dwMaxOverlayStretch)
            .field("dwMinLiveVideoStretch", &self.dwMinLiveVideoStretch)
            .field("dwMaxLiveVideoStretch", &self.dwMaxLiveVideoStretch)
            .field("dwMinHwCodecStretch", &self.dwMinHwCodecStretch)
            .field("dwMaxHwCodecStretch", &self.dwMaxHwCodecStretch)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .field("dwSVBCaps", &self.dwSVBCaps)
            .field("dwSVBCKeyCaps", &self.dwSVBCKeyCaps)
            .field("dwSVBFXCaps", &self.dwSVBFXCaps)
            .field("dwSVBRops", &self.dwSVBRops)
            .field("dwVSBCaps", &self.dwVSBCaps)
            .field("dwVSBCKeyCaps", &self.dwVSBCKeyCaps)
            .field("dwVSBFXCaps", &self.dwVSBFXCaps)
            .field("dwVSBRops", &self.dwVSBRops)
            .field("dwSSBCaps", &self.dwSSBCaps)
            .field("dwSSBCKeyCaps", &self.dwSSBCKeyCaps)
            .field("dwSSBFXCaps", &self.dwSSBFXCaps)
            .field("dwSSBRops", &self.dwSSBRops)
            .field("dwMaxVideoPorts", &self.dwMaxVideoPorts)
            .field("dwCurrVideoPorts", &self.dwCurrVideoPorts)
            .field("dwSVBCaps2", &self.dwSVBCaps2)
            .field("dwNLVBCaps", &self.dwNLVBCaps)
            .field("dwNLVBCaps2", &self.dwNLVBCaps2)
            .field("dwNLVBCKeyCaps", &self.dwNLVBCKeyCaps)
            .field("dwNLVBFXCaps", &self.dwNLVBFXCaps)
            .field("dwNLVBRops", &self.dwNLVBRops)
            .finish()
    }
}
impl ::windows::core::TypeKind for DDCAPS_DX5 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDCAPS_DX5 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwCaps == other.dwCaps
            && self.dwCaps2 == other.dwCaps2
            && self.dwCKeyCaps == other.dwCKeyCaps
            && self.dwFXCaps == other.dwFXCaps
            && self.dwFXAlphaCaps == other.dwFXAlphaCaps
            && self.dwPalCaps == other.dwPalCaps
            && self.dwSVCaps == other.dwSVCaps
            && self.dwAlphaBltConstBitDepths == other.dwAlphaBltConstBitDepths
            && self.dwAlphaBltPixelBitDepths == other.dwAlphaBltPixelBitDepths
            && self.dwAlphaBltSurfaceBitDepths == other.dwAlphaBltSurfaceBitDepths
            && self.dwAlphaOverlayConstBitDepths == other.dwAlphaOverlayConstBitDepths
            && self.dwAlphaOverlayPixelBitDepths == other.dwAlphaOverlayPixelBitDepths
            && self.dwAlphaOverlaySurfaceBitDepths == other.dwAlphaOverlaySurfaceBitDepths
            && self.dwZBufferBitDepths == other.dwZBufferBitDepths
            && self.dwVidMemTotal == other.dwVidMemTotal
            && self.dwVidMemFree == other.dwVidMemFree
            && self.dwMaxVisibleOverlays == other.dwMaxVisibleOverlays
            && self.dwCurrVisibleOverlays == other.dwCurrVisibleOverlays
            && self.dwNumFourCCCodes == other.dwNumFourCCCodes
            && self.dwAlignBoundarySrc == other.dwAlignBoundarySrc
            && self.dwAlignSizeSrc == other.dwAlignSizeSrc
            && self.dwAlignBoundaryDest == other.dwAlignBoundaryDest
            && self.dwAlignSizeDest == other.dwAlignSizeDest
            && self.dwAlignStrideAlign == other.dwAlignStrideAlign
            && self.dwRops == other.dwRops
            && self.ddsCaps == other.ddsCaps
            && self.dwMinOverlayStretch == other.dwMinOverlayStretch
            && self.dwMaxOverlayStretch == other.dwMaxOverlayStretch
            && self.dwMinLiveVideoStretch == other.dwMinLiveVideoStretch
            && self.dwMaxLiveVideoStretch == other.dwMaxLiveVideoStretch
            && self.dwMinHwCodecStretch == other.dwMinHwCodecStretch
            && self.dwMaxHwCodecStretch == other.dwMaxHwCodecStretch
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.dwReserved3 == other.dwReserved3
            && self.dwSVBCaps == other.dwSVBCaps
            && self.dwSVBCKeyCaps == other.dwSVBCKeyCaps
            && self.dwSVBFXCaps == other.dwSVBFXCaps
            && self.dwSVBRops == other.dwSVBRops
            && self.dwVSBCaps == other.dwVSBCaps
            && self.dwVSBCKeyCaps == other.dwVSBCKeyCaps
            && self.dwVSBFXCaps == other.dwVSBFXCaps
            && self.dwVSBRops == other.dwVSBRops
            && self.dwSSBCaps == other.dwSSBCaps
            && self.dwSSBCKeyCaps == other.dwSSBCKeyCaps
            && self.dwSSBFXCaps == other.dwSSBFXCaps
            && self.dwSSBRops == other.dwSSBRops
            && self.dwMaxVideoPorts == other.dwMaxVideoPorts
            && self.dwCurrVideoPorts == other.dwCurrVideoPorts
            && self.dwSVBCaps2 == other.dwSVBCaps2
            && self.dwNLVBCaps == other.dwNLVBCaps
            && self.dwNLVBCaps2 == other.dwNLVBCaps2
            && self.dwNLVBCKeyCaps == other.dwNLVBCKeyCaps
            && self.dwNLVBFXCaps == other.dwNLVBFXCaps
            && self.dwNLVBRops == other.dwNLVBRops
    }
}
impl ::core::cmp::Eq for DDCAPS_DX5 {}
impl ::core::default::Default for DDCAPS_DX5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDCAPS_DX6 {
    pub dwSize: u32,
    pub dwCaps: u32,
    pub dwCaps2: u32,
    pub dwCKeyCaps: u32,
    pub dwFXCaps: u32,
    pub dwFXAlphaCaps: u32,
    pub dwPalCaps: u32,
    pub dwSVCaps: u32,
    pub dwAlphaBltConstBitDepths: u32,
    pub dwAlphaBltPixelBitDepths: u32,
    pub dwAlphaBltSurfaceBitDepths: u32,
    pub dwAlphaOverlayConstBitDepths: u32,
    pub dwAlphaOverlayPixelBitDepths: u32,
    pub dwAlphaOverlaySurfaceBitDepths: u32,
    pub dwZBufferBitDepths: u32,
    pub dwVidMemTotal: u32,
    pub dwVidMemFree: u32,
    pub dwMaxVisibleOverlays: u32,
    pub dwCurrVisibleOverlays: u32,
    pub dwNumFourCCCodes: u32,
    pub dwAlignBoundarySrc: u32,
    pub dwAlignSizeSrc: u32,
    pub dwAlignBoundaryDest: u32,
    pub dwAlignSizeDest: u32,
    pub dwAlignStrideAlign: u32,
    pub dwRops: [u32; 8],
    pub ddsOldCaps: DDSCAPS,
    pub dwMinOverlayStretch: u32,
    pub dwMaxOverlayStretch: u32,
    pub dwMinLiveVideoStretch: u32,
    pub dwMaxLiveVideoStretch: u32,
    pub dwMinHwCodecStretch: u32,
    pub dwMaxHwCodecStretch: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub dwSVBCaps: u32,
    pub dwSVBCKeyCaps: u32,
    pub dwSVBFXCaps: u32,
    pub dwSVBRops: [u32; 8],
    pub dwVSBCaps: u32,
    pub dwVSBCKeyCaps: u32,
    pub dwVSBFXCaps: u32,
    pub dwVSBRops: [u32; 8],
    pub dwSSBCaps: u32,
    pub dwSSBCKeyCaps: u32,
    pub dwSSBFXCaps: u32,
    pub dwSSBRops: [u32; 8],
    pub dwMaxVideoPorts: u32,
    pub dwCurrVideoPorts: u32,
    pub dwSVBCaps2: u32,
    pub dwNLVBCaps: u32,
    pub dwNLVBCaps2: u32,
    pub dwNLVBCKeyCaps: u32,
    pub dwNLVBFXCaps: u32,
    pub dwNLVBRops: [u32; 8],
    pub ddsCaps: DDSCAPS2,
}
impl ::core::marker::Copy for DDCAPS_DX6 {}
impl ::core::clone::Clone for DDCAPS_DX6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDCAPS_DX6 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDCAPS_DX6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDCAPS_DX7 {
    pub dwSize: u32,
    pub dwCaps: u32,
    pub dwCaps2: u32,
    pub dwCKeyCaps: u32,
    pub dwFXCaps: u32,
    pub dwFXAlphaCaps: u32,
    pub dwPalCaps: u32,
    pub dwSVCaps: u32,
    pub dwAlphaBltConstBitDepths: u32,
    pub dwAlphaBltPixelBitDepths: u32,
    pub dwAlphaBltSurfaceBitDepths: u32,
    pub dwAlphaOverlayConstBitDepths: u32,
    pub dwAlphaOverlayPixelBitDepths: u32,
    pub dwAlphaOverlaySurfaceBitDepths: u32,
    pub dwZBufferBitDepths: u32,
    pub dwVidMemTotal: u32,
    pub dwVidMemFree: u32,
    pub dwMaxVisibleOverlays: u32,
    pub dwCurrVisibleOverlays: u32,
    pub dwNumFourCCCodes: u32,
    pub dwAlignBoundarySrc: u32,
    pub dwAlignSizeSrc: u32,
    pub dwAlignBoundaryDest: u32,
    pub dwAlignSizeDest: u32,
    pub dwAlignStrideAlign: u32,
    pub dwRops: [u32; 8],
    pub ddsOldCaps: DDSCAPS,
    pub dwMinOverlayStretch: u32,
    pub dwMaxOverlayStretch: u32,
    pub dwMinLiveVideoStretch: u32,
    pub dwMaxLiveVideoStretch: u32,
    pub dwMinHwCodecStretch: u32,
    pub dwMaxHwCodecStretch: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub dwSVBCaps: u32,
    pub dwSVBCKeyCaps: u32,
    pub dwSVBFXCaps: u32,
    pub dwSVBRops: [u32; 8],
    pub dwVSBCaps: u32,
    pub dwVSBCKeyCaps: u32,
    pub dwVSBFXCaps: u32,
    pub dwVSBRops: [u32; 8],
    pub dwSSBCaps: u32,
    pub dwSSBCKeyCaps: u32,
    pub dwSSBFXCaps: u32,
    pub dwSSBRops: [u32; 8],
    pub dwMaxVideoPorts: u32,
    pub dwCurrVideoPorts: u32,
    pub dwSVBCaps2: u32,
    pub dwNLVBCaps: u32,
    pub dwNLVBCaps2: u32,
    pub dwNLVBCKeyCaps: u32,
    pub dwNLVBFXCaps: u32,
    pub dwNLVBRops: [u32; 8],
    pub ddsCaps: DDSCAPS2,
}
impl ::core::marker::Copy for DDCAPS_DX7 {}
impl ::core::clone::Clone for DDCAPS_DX7 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDCAPS_DX7 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDCAPS_DX7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDCOLORCONTROL {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub lBrightness: i32,
    pub lContrast: i32,
    pub lHue: i32,
    pub lSaturation: i32,
    pub lSharpness: i32,
    pub lGamma: i32,
    pub lColorEnable: i32,
    pub dwReserved1: u32,
}
impl ::core::marker::Copy for DDCOLORCONTROL {}
impl ::core::clone::Clone for DDCOLORCONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDCOLORCONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDCOLORCONTROL").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("lBrightness", &self.lBrightness).field("lContrast", &self.lContrast).field("lHue", &self.lHue).field("lSaturation", &self.lSaturation).field("lSharpness", &self.lSharpness).field("lGamma", &self.lGamma).field("lColorEnable", &self.lColorEnable).field("dwReserved1", &self.dwReserved1).finish()
    }
}
impl ::windows::core::TypeKind for DDCOLORCONTROL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDCOLORCONTROL {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.lBrightness == other.lBrightness && self.lContrast == other.lContrast && self.lHue == other.lHue && self.lSaturation == other.lSaturation && self.lSharpness == other.lSharpness && self.lGamma == other.lGamma && self.lColorEnable == other.lColorEnable && self.dwReserved1 == other.dwReserved1
    }
}
impl ::core::cmp::Eq for DDCOLORCONTROL {}
impl ::core::default::Default for DDCOLORCONTROL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDCOLORKEY {
    pub dwColorSpaceLowValue: u32,
    pub dwColorSpaceHighValue: u32,
}
impl ::core::marker::Copy for DDCOLORKEY {}
impl ::core::clone::Clone for DDCOLORKEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDCOLORKEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDCOLORKEY").field("dwColorSpaceLowValue", &self.dwColorSpaceLowValue).field("dwColorSpaceHighValue", &self.dwColorSpaceHighValue).finish()
    }
}
impl ::windows::core::TypeKind for DDCOLORKEY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDCOLORKEY {
    fn eq(&self, other: &Self) -> bool {
        self.dwColorSpaceLowValue == other.dwColorSpaceLowValue && self.dwColorSpaceHighValue == other.dwColorSpaceHighValue
    }
}
impl ::core::cmp::Eq for DDCOLORKEY {}
impl ::core::default::Default for DDCOLORKEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDCOMPBUFFERINFO {
    pub dwSize: u32,
    pub dwNumCompBuffers: u32,
    pub dwWidthToCreate: u32,
    pub dwHeightToCreate: u32,
    pub dwBytesToAllocate: u32,
    pub ddCompCaps: DDSCAPS2,
    pub ddPixelFormat: DDPIXELFORMAT,
}
impl ::core::marker::Copy for DDCOMPBUFFERINFO {}
impl ::core::clone::Clone for DDCOMPBUFFERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDCOMPBUFFERINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDCOMPBUFFERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDCORECAPS {
    pub dwSize: u32,
    pub dwCaps: u32,
    pub dwCaps2: u32,
    pub dwCKeyCaps: u32,
    pub dwFXCaps: u32,
    pub dwFXAlphaCaps: u32,
    pub dwPalCaps: u32,
    pub dwSVCaps: u32,
    pub dwAlphaBltConstBitDepths: u32,
    pub dwAlphaBltPixelBitDepths: u32,
    pub dwAlphaBltSurfaceBitDepths: u32,
    pub dwAlphaOverlayConstBitDepths: u32,
    pub dwAlphaOverlayPixelBitDepths: u32,
    pub dwAlphaOverlaySurfaceBitDepths: u32,
    pub dwZBufferBitDepths: u32,
    pub dwVidMemTotal: u32,
    pub dwVidMemFree: u32,
    pub dwMaxVisibleOverlays: u32,
    pub dwCurrVisibleOverlays: u32,
    pub dwNumFourCCCodes: u32,
    pub dwAlignBoundarySrc: u32,
    pub dwAlignSizeSrc: u32,
    pub dwAlignBoundaryDest: u32,
    pub dwAlignSizeDest: u32,
    pub dwAlignStrideAlign: u32,
    pub dwRops: [u32; 8],
    pub ddsCaps: DDSCAPS,
    pub dwMinOverlayStretch: u32,
    pub dwMaxOverlayStretch: u32,
    pub dwMinLiveVideoStretch: u32,
    pub dwMaxLiveVideoStretch: u32,
    pub dwMinHwCodecStretch: u32,
    pub dwMaxHwCodecStretch: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub dwSVBCaps: u32,
    pub dwSVBCKeyCaps: u32,
    pub dwSVBFXCaps: u32,
    pub dwSVBRops: [u32; 8],
    pub dwVSBCaps: u32,
    pub dwVSBCKeyCaps: u32,
    pub dwVSBFXCaps: u32,
    pub dwVSBRops: [u32; 8],
    pub dwSSBCaps: u32,
    pub dwSSBCKeyCaps: u32,
    pub dwSSBFXCaps: u32,
    pub dwSSBRops: [u32; 8],
    pub dwMaxVideoPorts: u32,
    pub dwCurrVideoPorts: u32,
    pub dwSVBCaps2: u32,
}
impl ::core::marker::Copy for DDCORECAPS {}
impl ::core::clone::Clone for DDCORECAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDCORECAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDCORECAPS")
            .field("dwSize", &self.dwSize)
            .field("dwCaps", &self.dwCaps)
            .field("dwCaps2", &self.dwCaps2)
            .field("dwCKeyCaps", &self.dwCKeyCaps)
            .field("dwFXCaps", &self.dwFXCaps)
            .field("dwFXAlphaCaps", &self.dwFXAlphaCaps)
            .field("dwPalCaps", &self.dwPalCaps)
            .field("dwSVCaps", &self.dwSVCaps)
            .field("dwAlphaBltConstBitDepths", &self.dwAlphaBltConstBitDepths)
            .field("dwAlphaBltPixelBitDepths", &self.dwAlphaBltPixelBitDepths)
            .field("dwAlphaBltSurfaceBitDepths", &self.dwAlphaBltSurfaceBitDepths)
            .field("dwAlphaOverlayConstBitDepths", &self.dwAlphaOverlayConstBitDepths)
            .field("dwAlphaOverlayPixelBitDepths", &self.dwAlphaOverlayPixelBitDepths)
            .field("dwAlphaOverlaySurfaceBitDepths", &self.dwAlphaOverlaySurfaceBitDepths)
            .field("dwZBufferBitDepths", &self.dwZBufferBitDepths)
            .field("dwVidMemTotal", &self.dwVidMemTotal)
            .field("dwVidMemFree", &self.dwVidMemFree)
            .field("dwMaxVisibleOverlays", &self.dwMaxVisibleOverlays)
            .field("dwCurrVisibleOverlays", &self.dwCurrVisibleOverlays)
            .field("dwNumFourCCCodes", &self.dwNumFourCCCodes)
            .field("dwAlignBoundarySrc", &self.dwAlignBoundarySrc)
            .field("dwAlignSizeSrc", &self.dwAlignSizeSrc)
            .field("dwAlignBoundaryDest", &self.dwAlignBoundaryDest)
            .field("dwAlignSizeDest", &self.dwAlignSizeDest)
            .field("dwAlignStrideAlign", &self.dwAlignStrideAlign)
            .field("dwRops", &self.dwRops)
            .field("ddsCaps", &self.ddsCaps)
            .field("dwMinOverlayStretch", &self.dwMinOverlayStretch)
            .field("dwMaxOverlayStretch", &self.dwMaxOverlayStretch)
            .field("dwMinLiveVideoStretch", &self.dwMinLiveVideoStretch)
            .field("dwMaxLiveVideoStretch", &self.dwMaxLiveVideoStretch)
            .field("dwMinHwCodecStretch", &self.dwMinHwCodecStretch)
            .field("dwMaxHwCodecStretch", &self.dwMaxHwCodecStretch)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .field("dwSVBCaps", &self.dwSVBCaps)
            .field("dwSVBCKeyCaps", &self.dwSVBCKeyCaps)
            .field("dwSVBFXCaps", &self.dwSVBFXCaps)
            .field("dwSVBRops", &self.dwSVBRops)
            .field("dwVSBCaps", &self.dwVSBCaps)
            .field("dwVSBCKeyCaps", &self.dwVSBCKeyCaps)
            .field("dwVSBFXCaps", &self.dwVSBFXCaps)
            .field("dwVSBRops", &self.dwVSBRops)
            .field("dwSSBCaps", &self.dwSSBCaps)
            .field("dwSSBCKeyCaps", &self.dwSSBCKeyCaps)
            .field("dwSSBFXCaps", &self.dwSSBFXCaps)
            .field("dwSSBRops", &self.dwSSBRops)
            .field("dwMaxVideoPorts", &self.dwMaxVideoPorts)
            .field("dwCurrVideoPorts", &self.dwCurrVideoPorts)
            .field("dwSVBCaps2", &self.dwSVBCaps2)
            .finish()
    }
}
impl ::windows::core::TypeKind for DDCORECAPS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDCORECAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwCaps == other.dwCaps
            && self.dwCaps2 == other.dwCaps2
            && self.dwCKeyCaps == other.dwCKeyCaps
            && self.dwFXCaps == other.dwFXCaps
            && self.dwFXAlphaCaps == other.dwFXAlphaCaps
            && self.dwPalCaps == other.dwPalCaps
            && self.dwSVCaps == other.dwSVCaps
            && self.dwAlphaBltConstBitDepths == other.dwAlphaBltConstBitDepths
            && self.dwAlphaBltPixelBitDepths == other.dwAlphaBltPixelBitDepths
            && self.dwAlphaBltSurfaceBitDepths == other.dwAlphaBltSurfaceBitDepths
            && self.dwAlphaOverlayConstBitDepths == other.dwAlphaOverlayConstBitDepths
            && self.dwAlphaOverlayPixelBitDepths == other.dwAlphaOverlayPixelBitDepths
            && self.dwAlphaOverlaySurfaceBitDepths == other.dwAlphaOverlaySurfaceBitDepths
            && self.dwZBufferBitDepths == other.dwZBufferBitDepths
            && self.dwVidMemTotal == other.dwVidMemTotal
            && self.dwVidMemFree == other.dwVidMemFree
            && self.dwMaxVisibleOverlays == other.dwMaxVisibleOverlays
            && self.dwCurrVisibleOverlays == other.dwCurrVisibleOverlays
            && self.dwNumFourCCCodes == other.dwNumFourCCCodes
            && self.dwAlignBoundarySrc == other.dwAlignBoundarySrc
            && self.dwAlignSizeSrc == other.dwAlignSizeSrc
            && self.dwAlignBoundaryDest == other.dwAlignBoundaryDest
            && self.dwAlignSizeDest == other.dwAlignSizeDest
            && self.dwAlignStrideAlign == other.dwAlignStrideAlign
            && self.dwRops == other.dwRops
            && self.ddsCaps == other.ddsCaps
            && self.dwMinOverlayStretch == other.dwMinOverlayStretch
            && self.dwMaxOverlayStretch == other.dwMaxOverlayStretch
            && self.dwMinLiveVideoStretch == other.dwMinLiveVideoStretch
            && self.dwMaxLiveVideoStretch == other.dwMaxLiveVideoStretch
            && self.dwMinHwCodecStretch == other.dwMinHwCodecStretch
            && self.dwMaxHwCodecStretch == other.dwMaxHwCodecStretch
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.dwReserved3 == other.dwReserved3
            && self.dwSVBCaps == other.dwSVBCaps
            && self.dwSVBCKeyCaps == other.dwSVBCKeyCaps
            && self.dwSVBFXCaps == other.dwSVBFXCaps
            && self.dwSVBRops == other.dwSVBRops
            && self.dwVSBCaps == other.dwVSBCaps
            && self.dwVSBCKeyCaps == other.dwVSBCKeyCaps
            && self.dwVSBFXCaps == other.dwVSBFXCaps
            && self.dwVSBRops == other.dwVSBRops
            && self.dwSSBCaps == other.dwSSBCaps
            && self.dwSSBCKeyCaps == other.dwSSBCKeyCaps
            && self.dwSSBFXCaps == other.dwSSBFXCaps
            && self.dwSSBRops == other.dwSSBRops
            && self.dwMaxVideoPorts == other.dwMaxVideoPorts
            && self.dwCurrVideoPorts == other.dwCurrVideoPorts
            && self.dwSVBCaps2 == other.dwSVBCaps2
    }
}
impl ::core::cmp::Eq for DDCORECAPS {}
impl ::core::default::Default for DDCORECAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDDEVICEIDENTIFIER {
    pub szDriver: [u8; 512],
    pub szDescription: [u8; 512],
    pub liDriverVersion: i64,
    pub dwVendorId: u32,
    pub dwDeviceId: u32,
    pub dwSubSysId: u32,
    pub dwRevision: u32,
    pub guidDeviceIdentifier: ::windows::core::GUID,
}
impl ::core::marker::Copy for DDDEVICEIDENTIFIER {}
impl ::core::clone::Clone for DDDEVICEIDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDDEVICEIDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDDEVICEIDENTIFIER").field("szDriver", &self.szDriver).field("szDescription", &self.szDescription).field("liDriverVersion", &self.liDriverVersion).field("dwVendorId", &self.dwVendorId).field("dwDeviceId", &self.dwDeviceId).field("dwSubSysId", &self.dwSubSysId).field("dwRevision", &self.dwRevision).field("guidDeviceIdentifier", &self.guidDeviceIdentifier).finish()
    }
}
impl ::windows::core::TypeKind for DDDEVICEIDENTIFIER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDDEVICEIDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        self.szDriver == other.szDriver && self.szDescription == other.szDescription && self.liDriverVersion == other.liDriverVersion && self.dwVendorId == other.dwVendorId && self.dwDeviceId == other.dwDeviceId && self.dwSubSysId == other.dwSubSysId && self.dwRevision == other.dwRevision && self.guidDeviceIdentifier == other.guidDeviceIdentifier
    }
}
impl ::core::cmp::Eq for DDDEVICEIDENTIFIER {}
impl ::core::default::Default for DDDEVICEIDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDDEVICEIDENTIFIER2 {
    pub szDriver: [u8; 512],
    pub szDescription: [u8; 512],
    pub liDriverVersion: i64,
    pub dwVendorId: u32,
    pub dwDeviceId: u32,
    pub dwSubSysId: u32,
    pub dwRevision: u32,
    pub guidDeviceIdentifier: ::windows::core::GUID,
    pub dwWHQLLevel: u32,
}
impl ::core::marker::Copy for DDDEVICEIDENTIFIER2 {}
impl ::core::clone::Clone for DDDEVICEIDENTIFIER2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDDEVICEIDENTIFIER2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDDEVICEIDENTIFIER2").field("szDriver", &self.szDriver).field("szDescription", &self.szDescription).field("liDriverVersion", &self.liDriverVersion).field("dwVendorId", &self.dwVendorId).field("dwDeviceId", &self.dwDeviceId).field("dwSubSysId", &self.dwSubSysId).field("dwRevision", &self.dwRevision).field("guidDeviceIdentifier", &self.guidDeviceIdentifier).field("dwWHQLLevel", &self.dwWHQLLevel).finish()
    }
}
impl ::windows::core::TypeKind for DDDEVICEIDENTIFIER2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDDEVICEIDENTIFIER2 {
    fn eq(&self, other: &Self) -> bool {
        self.szDriver == other.szDriver && self.szDescription == other.szDescription && self.liDriverVersion == other.liDriverVersion && self.dwVendorId == other.dwVendorId && self.dwDeviceId == other.dwDeviceId && self.dwSubSysId == other.dwSubSysId && self.dwRevision == other.dwRevision && self.guidDeviceIdentifier == other.guidDeviceIdentifier && self.dwWHQLLevel == other.dwWHQLLevel
    }
}
impl ::core::cmp::Eq for DDDEVICEIDENTIFIER2 {}
impl ::core::default::Default for DDDEVICEIDENTIFIER2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDENABLEIRQINFO {
    pub dwIRQSources: u32,
    pub dwLine: u32,
    pub IRQCallback: PDX_IRQCALLBACK,
    pub lpIRQData: *mut DX_IRQDATA,
}
impl ::core::marker::Copy for DDENABLEIRQINFO {}
impl ::core::clone::Clone for DDENABLEIRQINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDENABLEIRQINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDENABLEIRQINFO").field("dwIRQSources", &self.dwIRQSources).field("dwLine", &self.dwLine).field("lpIRQData", &self.lpIRQData).finish()
    }
}
impl ::windows::core::TypeKind for DDENABLEIRQINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDENABLEIRQINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDFLIPOVERLAYINFO {
    pub lpCurrentSurface: *mut DDSURFACEDATA,
    pub lpTargetSurface: *mut DDSURFACEDATA,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for DDFLIPOVERLAYINFO {}
impl ::core::clone::Clone for DDFLIPOVERLAYINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDFLIPOVERLAYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDFLIPOVERLAYINFO").field("lpCurrentSurface", &self.lpCurrentSurface).field("lpTargetSurface", &self.lpTargetSurface).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::windows::core::TypeKind for DDFLIPOVERLAYINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDFLIPOVERLAYINFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpCurrentSurface == other.lpCurrentSurface && self.lpTargetSurface == other.lpTargetSurface && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for DDFLIPOVERLAYINFO {}
impl ::core::default::Default for DDFLIPOVERLAYINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDFLIPVIDEOPORTINFO {
    pub lpVideoPortData: *mut DDVIDEOPORTDATA,
    pub lpCurrentSurface: *mut DDSURFACEDATA,
    pub lpTargetSurface: *mut DDSURFACEDATA,
    pub dwFlipVPFlags: u32,
}
impl ::core::marker::Copy for DDFLIPVIDEOPORTINFO {}
impl ::core::clone::Clone for DDFLIPVIDEOPORTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDFLIPVIDEOPORTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDFLIPVIDEOPORTINFO").field("lpVideoPortData", &self.lpVideoPortData).field("lpCurrentSurface", &self.lpCurrentSurface).field("lpTargetSurface", &self.lpTargetSurface).field("dwFlipVPFlags", &self.dwFlipVPFlags).finish()
    }
}
impl ::windows::core::TypeKind for DDFLIPVIDEOPORTINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDFLIPVIDEOPORTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpVideoPortData == other.lpVideoPortData && self.lpCurrentSurface == other.lpCurrentSurface && self.lpTargetSurface == other.lpTargetSurface && self.dwFlipVPFlags == other.dwFlipVPFlags
    }
}
impl ::core::cmp::Eq for DDFLIPVIDEOPORTINFO {}
impl ::core::default::Default for DDFLIPVIDEOPORTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDGAMMARAMP {
    pub red: [u16; 256],
    pub green: [u16; 256],
    pub blue: [u16; 256],
}
impl ::core::marker::Copy for DDGAMMARAMP {}
impl ::core::clone::Clone for DDGAMMARAMP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDGAMMARAMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDGAMMARAMP").field("red", &self.red).field("green", &self.green).field("blue", &self.blue).finish()
    }
}
impl ::windows::core::TypeKind for DDGAMMARAMP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDGAMMARAMP {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}
impl ::core::cmp::Eq for DDGAMMARAMP {}
impl ::core::default::Default for DDGAMMARAMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDGETCURRENTAUTOFLIPININFO {
    pub lpVideoPortData: *mut DDVIDEOPORTDATA,
}
impl ::core::marker::Copy for DDGETCURRENTAUTOFLIPININFO {}
impl ::core::clone::Clone for DDGETCURRENTAUTOFLIPININFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDGETCURRENTAUTOFLIPININFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDGETCURRENTAUTOFLIPININFO").field("lpVideoPortData", &self.lpVideoPortData).finish()
    }
}
impl ::windows::core::TypeKind for DDGETCURRENTAUTOFLIPININFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDGETCURRENTAUTOFLIPININFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpVideoPortData == other.lpVideoPortData
    }
}
impl ::core::cmp::Eq for DDGETCURRENTAUTOFLIPININFO {}
impl ::core::default::Default for DDGETCURRENTAUTOFLIPININFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDGETCURRENTAUTOFLIPOUTINFO {
    pub dwSurfaceIndex: u32,
    pub dwVBISurfaceIndex: u32,
}
impl ::core::marker::Copy for DDGETCURRENTAUTOFLIPOUTINFO {}
impl ::core::clone::Clone for DDGETCURRENTAUTOFLIPOUTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDGETCURRENTAUTOFLIPOUTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDGETCURRENTAUTOFLIPOUTINFO").field("dwSurfaceIndex", &self.dwSurfaceIndex).field("dwVBISurfaceIndex", &self.dwVBISurfaceIndex).finish()
    }
}
impl ::windows::core::TypeKind for DDGETCURRENTAUTOFLIPOUTINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDGETCURRENTAUTOFLIPOUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSurfaceIndex == other.dwSurfaceIndex && self.dwVBISurfaceIndex == other.dwVBISurfaceIndex
    }
}
impl ::core::cmp::Eq for DDGETCURRENTAUTOFLIPOUTINFO {}
impl ::core::default::Default for DDGETCURRENTAUTOFLIPOUTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDGETIRQINFO {
    pub dwFlags: u32,
}
impl ::core::marker::Copy for DDGETIRQINFO {}
impl ::core::clone::Clone for DDGETIRQINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDGETIRQINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDGETIRQINFO").field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::windows::core::TypeKind for DDGETIRQINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDGETIRQINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for DDGETIRQINFO {}
impl ::core::default::Default for DDGETIRQINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDGETPOLARITYININFO {
    pub lpVideoPortData: *mut DDVIDEOPORTDATA,
}
impl ::core::marker::Copy for DDGETPOLARITYININFO {}
impl ::core::clone::Clone for DDGETPOLARITYININFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDGETPOLARITYININFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDGETPOLARITYININFO").field("lpVideoPortData", &self.lpVideoPortData).finish()
    }
}
impl ::windows::core::TypeKind for DDGETPOLARITYININFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDGETPOLARITYININFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpVideoPortData == other.lpVideoPortData
    }
}
impl ::core::cmp::Eq for DDGETPOLARITYININFO {}
impl ::core::default::Default for DDGETPOLARITYININFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDGETPOLARITYOUTINFO {
    pub bPolarity: u32,
}
impl ::core::marker::Copy for DDGETPOLARITYOUTINFO {}
impl ::core::clone::Clone for DDGETPOLARITYOUTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDGETPOLARITYOUTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDGETPOLARITYOUTINFO").field("bPolarity", &self.bPolarity).finish()
    }
}
impl ::windows::core::TypeKind for DDGETPOLARITYOUTINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDGETPOLARITYOUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.bPolarity == other.bPolarity
    }
}
impl ::core::cmp::Eq for DDGETPOLARITYOUTINFO {}
impl ::core::default::Default for DDGETPOLARITYOUTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDGETPREVIOUSAUTOFLIPININFO {
    pub lpVideoPortData: *mut DDVIDEOPORTDATA,
}
impl ::core::marker::Copy for DDGETPREVIOUSAUTOFLIPININFO {}
impl ::core::clone::Clone for DDGETPREVIOUSAUTOFLIPININFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDGETPREVIOUSAUTOFLIPININFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDGETPREVIOUSAUTOFLIPININFO").field("lpVideoPortData", &self.lpVideoPortData).finish()
    }
}
impl ::windows::core::TypeKind for DDGETPREVIOUSAUTOFLIPININFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDGETPREVIOUSAUTOFLIPININFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpVideoPortData == other.lpVideoPortData
    }
}
impl ::core::cmp::Eq for DDGETPREVIOUSAUTOFLIPININFO {}
impl ::core::default::Default for DDGETPREVIOUSAUTOFLIPININFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDGETPREVIOUSAUTOFLIPOUTINFO {
    pub dwSurfaceIndex: u32,
    pub dwVBISurfaceIndex: u32,
}
impl ::core::marker::Copy for DDGETPREVIOUSAUTOFLIPOUTINFO {}
impl ::core::clone::Clone for DDGETPREVIOUSAUTOFLIPOUTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDGETPREVIOUSAUTOFLIPOUTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDGETPREVIOUSAUTOFLIPOUTINFO").field("dwSurfaceIndex", &self.dwSurfaceIndex).field("dwVBISurfaceIndex", &self.dwVBISurfaceIndex).finish()
    }
}
impl ::windows::core::TypeKind for DDGETPREVIOUSAUTOFLIPOUTINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDGETPREVIOUSAUTOFLIPOUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSurfaceIndex == other.dwSurfaceIndex && self.dwVBISurfaceIndex == other.dwVBISurfaceIndex
    }
}
impl ::core::cmp::Eq for DDGETPREVIOUSAUTOFLIPOUTINFO {}
impl ::core::default::Default for DDGETPREVIOUSAUTOFLIPOUTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDGETTRANSFERSTATUSOUTINFO {
    pub dwTransferID: usize,
}
impl ::core::marker::Copy for DDGETTRANSFERSTATUSOUTINFO {}
impl ::core::clone::Clone for DDGETTRANSFERSTATUSOUTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDGETTRANSFERSTATUSOUTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDGETTRANSFERSTATUSOUTINFO").field("dwTransferID", &self.dwTransferID).finish()
    }
}
impl ::windows::core::TypeKind for DDGETTRANSFERSTATUSOUTINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDGETTRANSFERSTATUSOUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwTransferID == other.dwTransferID
    }
}
impl ::core::cmp::Eq for DDGETTRANSFERSTATUSOUTINFO {}
impl ::core::default::Default for DDGETTRANSFERSTATUSOUTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHALDDRAWFNS {
    pub dwSize: u32,
    pub lpSetInfo: LPDDHAL_SETINFO,
    pub lpVidMemAlloc: LPDDHAL_VIDMEMALLOC,
    pub lpVidMemFree: LPDDHAL_VIDMEMFREE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHALDDRAWFNS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHALDDRAWFNS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHALDDRAWFNS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHALDDRAWFNS").field("dwSize", &self.dwSize).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHALDDRAWFNS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHALDDRAWFNS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHALINFO {
    pub dwSize: u32,
    pub lpDDCallbacks: *mut DDHAL_DDCALLBACKS,
    pub lpDDSurfaceCallbacks: *mut DDHAL_DDSURFACECALLBACKS,
    pub lpDDPaletteCallbacks: *mut DDHAL_DDPALETTECALLBACKS,
    pub vmiData: VIDMEMINFO,
    pub ddCaps: DDCORECAPS,
    pub dwMonitorFrequency: u32,
    pub GetDriverInfo: LPDDHAL_GETDRIVERINFO,
    pub dwModeIndex: u32,
    pub lpdwFourCC: *mut u32,
    pub dwNumModes: u32,
    pub lpModeInfo: *mut DDHALMODEINFO,
    pub dwFlags: u32,
    pub lpPDevice: *mut ::core::ffi::c_void,
    pub hInstance: u32,
    pub lpD3DGlobalDriverData: usize,
    pub lpD3DHALCallbacks: usize,
    pub lpDDExeBufCallbacks: *mut DDHAL_DDEXEBUFCALLBACKS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHALINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHALINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHALINFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHALINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDHALMODEINFO {
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub lPitch: i32,
    pub dwBPP: u32,
    pub wFlags: u16,
    pub wRefreshRate: u16,
    pub dwRBitMask: u32,
    pub dwGBitMask: u32,
    pub dwBBitMask: u32,
    pub dwAlphaBitMask: u32,
}
impl ::core::marker::Copy for DDHALMODEINFO {}
impl ::core::clone::Clone for DDHALMODEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDHALMODEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHALMODEINFO").field("dwWidth", &self.dwWidth).field("dwHeight", &self.dwHeight).field("lPitch", &self.lPitch).field("dwBPP", &self.dwBPP).field("wFlags", &self.wFlags).field("wRefreshRate", &self.wRefreshRate).field("dwRBitMask", &self.dwRBitMask).field("dwGBitMask", &self.dwGBitMask).field("dwBBitMask", &self.dwBBitMask).field("dwAlphaBitMask", &self.dwAlphaBitMask).finish()
    }
}
impl ::windows::core::TypeKind for DDHALMODEINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDHALMODEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwWidth == other.dwWidth && self.dwHeight == other.dwHeight && self.lPitch == other.lPitch && self.dwBPP == other.dwBPP && self.wFlags == other.wFlags && self.wRefreshRate == other.wRefreshRate && self.dwRBitMask == other.dwRBitMask && self.dwGBitMask == other.dwGBitMask && self.dwBBitMask == other.dwBBitMask && self.dwAlphaBitMask == other.dwAlphaBitMask
    }
}
impl ::core::cmp::Eq for DDHALMODEINFO {}
impl ::core::default::Default for DDHALMODEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_ADDATTACHEDSURFACEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub lpSurfAttached: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub ddRVal: ::windows::core::HRESULT,
    pub AddAttachedSurface: LPDDHALSURFCB_ADDATTACHEDSURFACE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_ADDATTACHEDSURFACEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_ADDATTACHEDSURFACEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_ADDATTACHEDSURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_ADDATTACHEDSURFACEDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("lpSurfAttached", &self.lpSurfAttached).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_ADDATTACHEDSURFACEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_ADDATTACHEDSURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_BEGINMOCOMPFRAMEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpMoComp: *mut DDRAWI_DDMOTIONCOMP_LCL,
    pub lpDestSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwInputDataSize: u32,
    pub lpInputData: *mut ::core::ffi::c_void,
    pub dwOutputDataSize: u32,
    pub lpOutputData: *mut ::core::ffi::c_void,
    pub ddRVal: ::windows::core::HRESULT,
    pub BeginMoCompFrame: LPDDHALMOCOMPCB_BEGINFRAME,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_BEGINMOCOMPFRAMEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_BEGINMOCOMPFRAMEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_BEGINMOCOMPFRAMEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_BEGINMOCOMPFRAMEDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("lpDestSurface", &self.lpDestSurface).field("dwInputDataSize", &self.dwInputDataSize).field("lpInputData", &self.lpInputData).field("dwOutputDataSize", &self.dwOutputDataSize).field("lpOutputData", &self.lpOutputData).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_BEGINMOCOMPFRAMEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_BEGINMOCOMPFRAMEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_BLTDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDDestSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub rDest: super::super::Foundation::RECTL,
    pub lpDDSrcSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub rSrc: super::super::Foundation::RECTL,
    pub dwFlags: u32,
    pub dwROPFlags: u32,
    pub bltFX: DDBLTFX,
    pub ddRVal: ::windows::core::HRESULT,
    pub Blt: LPDDHALSURFCB_BLT,
    pub IsClipped: super::super::Foundation::BOOL,
    pub rOrigDest: super::super::Foundation::RECTL,
    pub rOrigSrc: super::super::Foundation::RECTL,
    pub dwRectCnt: u32,
    pub prDestRects: *mut super::super::Foundation::RECT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_BLTDATA {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_BLTDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_BLTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_CALLBACKS {
    pub cbDDCallbacks: DDHAL_DDCALLBACKS,
    pub cbDDSurfaceCallbacks: DDHAL_DDSURFACECALLBACKS,
    pub cbDDPaletteCallbacks: DDHAL_DDPALETTECALLBACKS,
    pub HALDD: DDHAL_DDCALLBACKS,
    pub HALDDSurface: DDHAL_DDSURFACECALLBACKS,
    pub HALDDPalette: DDHAL_DDPALETTECALLBACKS,
    pub HELDD: DDHAL_DDCALLBACKS,
    pub HELDDSurface: DDHAL_DDSURFACECALLBACKS,
    pub HELDDPalette: DDHAL_DDPALETTECALLBACKS,
    pub cbDDExeBufCallbacks: DDHAL_DDEXEBUFCALLBACKS,
    pub HALDDExeBuf: DDHAL_DDEXEBUFCALLBACKS,
    pub HELDDExeBuf: DDHAL_DDEXEBUFCALLBACKS,
    pub cbDDVideoPortCallbacks: DDHAL_DDVIDEOPORTCALLBACKS,
    pub HALDDVideoPort: DDHAL_DDVIDEOPORTCALLBACKS,
    pub cbDDColorControlCallbacks: DDHAL_DDCOLORCONTROLCALLBACKS,
    pub HALDDColorControl: DDHAL_DDCOLORCONTROLCALLBACKS,
    pub cbDDMiscellaneousCallbacks: DDHAL_DDMISCELLANEOUSCALLBACKS,
    pub HALDDMiscellaneous: DDHAL_DDMISCELLANEOUSCALLBACKS,
    pub cbDDKernelCallbacks: DDHAL_DDKERNELCALLBACKS,
    pub HALDDKernel: DDHAL_DDKERNELCALLBACKS,
    pub cbDDMotionCompCallbacks: DDHAL_DDMOTIONCOMPCALLBACKS,
    pub HALDDMotionComp: DDHAL_DDMOTIONCOMPCALLBACKS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_CALLBACKS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_CALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_CALLBACKS").finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_CALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_CANCREATESURFACEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurfaceDesc: *mut DDSURFACEDESC,
    pub bIsDifferentPixelFormat: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub CanCreateSurface: LPDDHAL_CANCREATESURFACE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_CANCREATESURFACEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_CANCREATESURFACEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_CANCREATESURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_CANCREATESURFACEDATA").field("lpDD", &self.lpDD).field("lpDDSurfaceDesc", &self.lpDDSurfaceDesc).field("bIsDifferentPixelFormat", &self.bIsDifferentPixelFormat).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_CANCREATESURFACEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_CANCREATESURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_CANCREATEVPORTDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpDDVideoPortDesc: *mut DDVIDEOPORTDESC,
    pub ddRVal: ::windows::core::HRESULT,
    pub CanCreateVideoPort: LPDDHALVPORTCB_CANCREATEVIDEOPORT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_CANCREATEVPORTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_CANCREATEVPORTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_CANCREATEVPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_CANCREATEVPORTDATA").field("lpDD", &self.lpDD).field("lpDDVideoPortDesc", &self.lpDDVideoPortDesc).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_CANCREATEVPORTDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_CANCREATEVPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_COLORCONTROLDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub lpColorData: *mut DDCOLORCONTROL,
    pub dwFlags: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub ColorControl: LPDDHALCOLORCB_COLORCONTROL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_COLORCONTROLDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_COLORCONTROLDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_COLORCONTROLDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_COLORCONTROLDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("lpColorData", &self.lpColorData).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_COLORCONTROLDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_COLORCONTROLDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_CREATEMOCOMPDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpMoComp: *mut DDRAWI_DDMOTIONCOMP_LCL,
    pub lpGuid: *mut ::windows::core::GUID,
    pub dwUncompWidth: u32,
    pub dwUncompHeight: u32,
    pub ddUncompPixelFormat: DDPIXELFORMAT,
    pub lpData: *mut ::core::ffi::c_void,
    pub dwDataSize: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub CreateMoComp: LPDDHALMOCOMPCB_CREATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_CREATEMOCOMPDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_CREATEMOCOMPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_CREATEMOCOMPDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_CREATEMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_CREATEPALETTEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDPalette: *mut DDRAWI_DDRAWPALETTE_GBL,
    pub lpColorTable: *mut super::Gdi::PALETTEENTRY,
    pub ddRVal: ::windows::core::HRESULT,
    pub CreatePalette: LPDDHAL_CREATEPALETTE,
    pub is_excl: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_CREATEPALETTEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_CREATEPALETTEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_CREATEPALETTEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_CREATEPALETTEDATA").field("lpDD", &self.lpDD).field("lpDDPalette", &self.lpDDPalette).field("lpColorTable", &self.lpColorTable).field("ddRVal", &self.ddRVal).field("is_excl", &self.is_excl).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_CREATEPALETTEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_CREATEPALETTEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_CREATESURFACEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurfaceDesc: *mut DDSURFACEDESC,
    pub lplpSList: *mut *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwSCnt: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub CreateSurface: LPDDHAL_CREATESURFACE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_CREATESURFACEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_CREATESURFACEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_CREATESURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_CREATESURFACEDATA").field("lpDD", &self.lpDD).field("lpDDSurfaceDesc", &self.lpDDSurfaceDesc).field("lplpSList", &self.lplpSList).field("dwSCnt", &self.dwSCnt).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_CREATESURFACEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_CREATESURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_CREATESURFACEEXDATA {
    pub dwFlags: u32,
    pub lpDDLcl: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpDDSLcl: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub ddRVal: ::windows::core::HRESULT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_CREATESURFACEEXDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_CREATESURFACEEXDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_CREATESURFACEEXDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_CREATESURFACEEXDATA").field("dwFlags", &self.dwFlags).field("lpDDLcl", &self.lpDDLcl).field("lpDDSLcl", &self.lpDDSLcl).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_CREATESURFACEEXDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDHAL_CREATESURFACEEXDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.lpDDLcl == other.lpDDLcl && self.lpDDSLcl == other.lpDDSLcl && self.ddRVal == other.ddRVal
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDHAL_CREATESURFACEEXDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_CREATESURFACEEXDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_CREATEVPORTDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpDDVideoPortDesc: *mut DDVIDEOPORTDESC,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub ddRVal: ::windows::core::HRESULT,
    pub CreateVideoPort: LPDDHALVPORTCB_CREATEVIDEOPORT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_CREATEVPORTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_CREATEVPORTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_CREATEVPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_CREATEVPORTDATA").field("lpDD", &self.lpDD).field("lpDDVideoPortDesc", &self.lpDDVideoPortDesc).field("lpVideoPort", &self.lpVideoPort).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_CREATEVPORTDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_CREATEVPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_DDCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub DestroyDriver: LPDDHAL_DESTROYDRIVER,
    pub CreateSurface: LPDDHAL_CREATESURFACE,
    pub SetColorKey: LPDDHAL_SETCOLORKEY,
    pub SetMode: LPDDHAL_SETMODE,
    pub WaitForVerticalBlank: LPDDHAL_WAITFORVERTICALBLANK,
    pub CanCreateSurface: LPDDHAL_CANCREATESURFACE,
    pub CreatePalette: LPDDHAL_CREATEPALETTE,
    pub GetScanLine: LPDDHAL_GETSCANLINE,
    pub SetExclusiveMode: LPDDHAL_SETEXCLUSIVEMODE,
    pub FlipToGDISurface: LPDDHAL_FLIPTOGDISURFACE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_DDCALLBACKS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_DDCALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_DDCALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_DDCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_DDCALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DDCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_DDCOLORCONTROLCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub ColorControl: LPDDHALCOLORCB_COLORCONTROL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_DDCOLORCONTROLCALLBACKS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_DDCOLORCONTROLCALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_DDCOLORCONTROLCALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_DDCOLORCONTROLCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_DDCOLORCONTROLCALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DDCOLORCONTROLCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_DDEXEBUFCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub CanCreateExecuteBuffer: LPDDHALEXEBUFCB_CANCREATEEXEBUF,
    pub CreateExecuteBuffer: LPDDHALEXEBUFCB_CREATEEXEBUF,
    pub DestroyExecuteBuffer: LPDDHALEXEBUFCB_DESTROYEXEBUF,
    pub LockExecuteBuffer: LPDDHALEXEBUFCB_LOCKEXEBUF,
    pub UnlockExecuteBuffer: LPDDHALEXEBUFCB_UNLOCKEXEBUF,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_DDEXEBUFCALLBACKS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_DDEXEBUFCALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_DDEXEBUFCALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_DDEXEBUFCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_DDEXEBUFCALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DDEXEBUFCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_DDKERNELCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub SyncSurfaceData: LPDDHALKERNELCB_SYNCSURFACE,
    pub SyncVideoPortData: LPDDHALKERNELCB_SYNCVIDEOPORT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_DDKERNELCALLBACKS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_DDKERNELCALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_DDKERNELCALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_DDKERNELCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_DDKERNELCALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DDKERNELCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_DDMISCELLANEOUS2CALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub Reserved: *mut ::core::ffi::c_void,
    pub CreateSurfaceEx: LPDDHAL_CREATESURFACEEX,
    pub GetDriverState: LPDDHAL_GETDRIVERSTATE,
    pub DestroyDDLocal: LPDDHAL_DESTROYDDLOCAL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_DDMISCELLANEOUS2CALLBACKS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_DDMISCELLANEOUS2CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_DDMISCELLANEOUS2CALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_DDMISCELLANEOUS2CALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_DDMISCELLANEOUS2CALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DDMISCELLANEOUS2CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_DDMISCELLANEOUSCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub GetAvailDriverMemory: LPDDHAL_GETAVAILDRIVERMEMORY,
    pub UpdateNonLocalHeap: LPDDHAL_UPDATENONLOCALHEAP,
    pub GetHeapAlignment: LPDDHAL_GETHEAPALIGNMENT,
    pub GetSysmemBltStatus: LPDDHALSURFCB_GETBLTSTATUS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_DDMISCELLANEOUSCALLBACKS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_DDMISCELLANEOUSCALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_DDMISCELLANEOUSCALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_DDMISCELLANEOUSCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_DDMISCELLANEOUSCALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DDMISCELLANEOUSCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_DDMOTIONCOMPCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub GetMoCompGuids: LPDDHALMOCOMPCB_GETGUIDS,
    pub GetMoCompFormats: LPDDHALMOCOMPCB_GETFORMATS,
    pub CreateMoComp: LPDDHALMOCOMPCB_CREATE,
    pub GetMoCompBuffInfo: LPDDHALMOCOMPCB_GETCOMPBUFFINFO,
    pub GetInternalMoCompInfo: LPDDHALMOCOMPCB_GETINTERNALINFO,
    pub BeginMoCompFrame: LPDDHALMOCOMPCB_BEGINFRAME,
    pub EndMoCompFrame: LPDDHALMOCOMPCB_ENDFRAME,
    pub RenderMoComp: LPDDHALMOCOMPCB_RENDER,
    pub QueryMoCompStatus: LPDDHALMOCOMPCB_QUERYSTATUS,
    pub DestroyMoComp: LPDDHALMOCOMPCB_DESTROY,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_DDMOTIONCOMPCALLBACKS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_DDMOTIONCOMPCALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_DDMOTIONCOMPCALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_DDMOTIONCOMPCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_DDMOTIONCOMPCALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DDMOTIONCOMPCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_DDPALETTECALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub DestroyPalette: LPDDHALPALCB_DESTROYPALETTE,
    pub SetEntries: LPDDHALPALCB_SETENTRIES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_DDPALETTECALLBACKS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_DDPALETTECALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_DDPALETTECALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_DDPALETTECALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_DDPALETTECALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DDPALETTECALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_DDSURFACECALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub DestroySurface: LPDDHALSURFCB_DESTROYSURFACE,
    pub Flip: LPDDHALSURFCB_FLIP,
    pub SetClipList: LPDDHALSURFCB_SETCLIPLIST,
    pub Lock: LPDDHALSURFCB_LOCK,
    pub Unlock: LPDDHALSURFCB_UNLOCK,
    pub Blt: LPDDHALSURFCB_BLT,
    pub SetColorKey: LPDDHALSURFCB_SETCOLORKEY,
    pub AddAttachedSurface: LPDDHALSURFCB_ADDATTACHEDSURFACE,
    pub GetBltStatus: LPDDHALSURFCB_GETBLTSTATUS,
    pub GetFlipStatus: LPDDHALSURFCB_GETFLIPSTATUS,
    pub UpdateOverlay: LPDDHALSURFCB_UPDATEOVERLAY,
    pub SetOverlayPosition: LPDDHALSURFCB_SETOVERLAYPOSITION,
    pub reserved4: *mut ::core::ffi::c_void,
    pub SetPalette: LPDDHALSURFCB_SETPALETTE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_DDSURFACECALLBACKS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_DDSURFACECALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_DDSURFACECALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_DDSURFACECALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("reserved4", &self.reserved4).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_DDSURFACECALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DDSURFACECALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_DDVIDEOPORTCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub CanCreateVideoPort: LPDDHALVPORTCB_CANCREATEVIDEOPORT,
    pub CreateVideoPort: LPDDHALVPORTCB_CREATEVIDEOPORT,
    pub FlipVideoPort: LPDDHALVPORTCB_FLIP,
    pub GetVideoPortBandwidth: LPDDHALVPORTCB_GETBANDWIDTH,
    pub GetVideoPortInputFormats: LPDDHALVPORTCB_GETINPUTFORMATS,
    pub GetVideoPortOutputFormats: LPDDHALVPORTCB_GETOUTPUTFORMATS,
    pub lpReserved1: *mut ::core::ffi::c_void,
    pub GetVideoPortField: LPDDHALVPORTCB_GETFIELD,
    pub GetVideoPortLine: LPDDHALVPORTCB_GETLINE,
    pub GetVideoPortConnectInfo: LPDDHALVPORTCB_GETVPORTCONNECT,
    pub DestroyVideoPort: LPDDHALVPORTCB_DESTROYVPORT,
    pub GetVideoPortFlipStatus: LPDDHALVPORTCB_GETFLIPSTATUS,
    pub UpdateVideoPort: LPDDHALVPORTCB_UPDATE,
    pub WaitForVideoPortSync: LPDDHALVPORTCB_WAITFORSYNC,
    pub GetVideoSignalStatus: LPDDHALVPORTCB_GETSIGNALSTATUS,
    pub ColorControl: LPDDHALVPORTCB_COLORCONTROL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_DDVIDEOPORTCALLBACKS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_DDVIDEOPORTCALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_DDVIDEOPORTCALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_DDVIDEOPORTCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("lpReserved1", &self.lpReserved1).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_DDVIDEOPORTCALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DDVIDEOPORTCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_DESTROYDDLOCALDATA {
    pub dwFlags: u32,
    pub pDDLcl: *mut DDRAWI_DIRECTDRAW_LCL,
    pub ddRVal: ::windows::core::HRESULT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_DESTROYDDLOCALDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_DESTROYDDLOCALDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_DESTROYDDLOCALDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_DESTROYDDLOCALDATA").field("dwFlags", &self.dwFlags).field("pDDLcl", &self.pDDLcl).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_DESTROYDDLOCALDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDHAL_DESTROYDDLOCALDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.pDDLcl == other.pDDLcl && self.ddRVal == other.ddRVal
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDHAL_DESTROYDDLOCALDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DESTROYDDLOCALDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_DESTROYDRIVERDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub ddRVal: ::windows::core::HRESULT,
    pub DestroyDriver: LPDDHAL_DESTROYDRIVER,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_DESTROYDRIVERDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_DESTROYDRIVERDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_DESTROYDRIVERDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_DESTROYDRIVERDATA").field("lpDD", &self.lpDD).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_DESTROYDRIVERDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DESTROYDRIVERDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_DESTROYMOCOMPDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpMoComp: *mut DDRAWI_DDMOTIONCOMP_LCL,
    pub ddRVal: ::windows::core::HRESULT,
    pub DestroyMoComp: LPDDHALMOCOMPCB_DESTROY,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_DESTROYMOCOMPDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_DESTROYMOCOMPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_DESTROYMOCOMPDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_DESTROYMOCOMPDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_DESTROYMOCOMPDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DESTROYMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_DESTROYPALETTEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDPalette: *mut DDRAWI_DDRAWPALETTE_GBL,
    pub ddRVal: ::windows::core::HRESULT,
    pub DestroyPalette: LPDDHALPALCB_DESTROYPALETTE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_DESTROYPALETTEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_DESTROYPALETTEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_DESTROYPALETTEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_DESTROYPALETTEDATA").field("lpDD", &self.lpDD).field("lpDDPalette", &self.lpDDPalette).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_DESTROYPALETTEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DESTROYPALETTEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_DESTROYSURFACEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub ddRVal: ::windows::core::HRESULT,
    pub DestroySurface: LPDDHALSURFCB_DESTROYSURFACE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_DESTROYSURFACEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_DESTROYSURFACEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_DESTROYSURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_DESTROYSURFACEDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_DESTROYSURFACEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DESTROYSURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_DESTROYVPORTDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub ddRVal: ::windows::core::HRESULT,
    pub DestroyVideoPort: LPDDHALVPORTCB_DESTROYVPORT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_DESTROYVPORTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_DESTROYVPORTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_DESTROYVPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_DESTROYVPORTDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_DESTROYVPORTDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DESTROYVPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_DRVSETCOLORKEYDATA {
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwFlags: u32,
    pub ckNew: DDCOLORKEY,
    pub ddRVal: ::windows::core::HRESULT,
    pub SetColorKey: LPDDHAL_SETCOLORKEY,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_DRVSETCOLORKEYDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_DRVSETCOLORKEYDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_DRVSETCOLORKEYDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_DRVSETCOLORKEYDATA").field("lpDDSurface", &self.lpDDSurface).field("dwFlags", &self.dwFlags).field("ckNew", &self.ckNew).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_DRVSETCOLORKEYDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_DRVSETCOLORKEYDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_ENDMOCOMPFRAMEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpMoComp: *mut DDRAWI_DDMOTIONCOMP_LCL,
    pub lpInputData: *mut ::core::ffi::c_void,
    pub dwInputDataSize: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub EndMoCompFrame: LPDDHALMOCOMPCB_ENDFRAME,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_ENDMOCOMPFRAMEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_ENDMOCOMPFRAMEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_ENDMOCOMPFRAMEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_ENDMOCOMPFRAMEDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("lpInputData", &self.lpInputData).field("dwInputDataSize", &self.dwInputDataSize).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_ENDMOCOMPFRAMEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_ENDMOCOMPFRAMEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_FLIPDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpSurfCurr: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub lpSurfTarg: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwFlags: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub Flip: LPDDHALSURFCB_FLIP,
    pub lpSurfCurrLeft: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub lpSurfTargLeft: *mut DDRAWI_DDRAWSURFACE_LCL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_FLIPDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_FLIPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_FLIPDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_FLIPDATA").field("lpDD", &self.lpDD).field("lpSurfCurr", &self.lpSurfCurr).field("lpSurfTarg", &self.lpSurfTarg).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).field("lpSurfCurrLeft", &self.lpSurfCurrLeft).field("lpSurfTargLeft", &self.lpSurfTargLeft).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_FLIPDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_FLIPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_FLIPTOGDISURFACEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub dwToGDI: u32,
    pub dwReserved: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub FlipToGDISurface: LPDDHAL_FLIPTOGDISURFACE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_FLIPTOGDISURFACEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_FLIPTOGDISURFACEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_FLIPTOGDISURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_FLIPTOGDISURFACEDATA").field("lpDD", &self.lpDD).field("dwToGDI", &self.dwToGDI).field("dwReserved", &self.dwReserved).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_FLIPTOGDISURFACEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_FLIPTOGDISURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_FLIPVPORTDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub lpSurfCurr: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub lpSurfTarg: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub ddRVal: ::windows::core::HRESULT,
    pub FlipVideoPort: LPDDHALVPORTCB_FLIP,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_FLIPVPORTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_FLIPVPORTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_FLIPVPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_FLIPVPORTDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("lpSurfCurr", &self.lpSurfCurr).field("lpSurfTarg", &self.lpSurfTarg).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_FLIPVPORTDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_FLIPVPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_GETAVAILDRIVERMEMORYDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub DDSCaps: DDSCAPS,
    pub dwTotal: u32,
    pub dwFree: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetAvailDriverMemory: LPDDHAL_GETAVAILDRIVERMEMORY,
    pub ddsCapsEx: DDSCAPSEX,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_GETAVAILDRIVERMEMORYDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_GETAVAILDRIVERMEMORYDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_GETAVAILDRIVERMEMORYDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETAVAILDRIVERMEMORYDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_GETBLTSTATUSDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwFlags: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetBltStatus: LPDDHALSURFCB_GETBLTSTATUS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_GETBLTSTATUSDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_GETBLTSTATUSDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_GETBLTSTATUSDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_GETBLTSTATUSDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_GETBLTSTATUSDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETBLTSTATUSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDHAL_GETDRIVERINFODATA {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidInfo: ::windows::core::GUID,
    pub dwExpectedSize: u32,
    pub lpvData: *mut ::core::ffi::c_void,
    pub dwActualSize: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub dwContext: usize,
}
impl ::core::marker::Copy for DDHAL_GETDRIVERINFODATA {}
impl ::core::clone::Clone for DDHAL_GETDRIVERINFODATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDHAL_GETDRIVERINFODATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_GETDRIVERINFODATA").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("guidInfo", &self.guidInfo).field("dwExpectedSize", &self.dwExpectedSize).field("lpvData", &self.lpvData).field("dwActualSize", &self.dwActualSize).field("ddRVal", &self.ddRVal).field("dwContext", &self.dwContext).finish()
    }
}
impl ::windows::core::TypeKind for DDHAL_GETDRIVERINFODATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDHAL_GETDRIVERINFODATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.guidInfo == other.guidInfo && self.dwExpectedSize == other.dwExpectedSize && self.lpvData == other.lpvData && self.dwActualSize == other.dwActualSize && self.ddRVal == other.ddRVal && self.dwContext == other.dwContext
    }
}
impl ::core::cmp::Eq for DDHAL_GETDRIVERINFODATA {}
impl ::core::default::Default for DDHAL_GETDRIVERINFODATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDHAL_GETDRIVERSTATEDATA {
    pub dwFlags: u32,
    pub Anonymous: DDHAL_GETDRIVERSTATEDATA_0,
    pub lpdwStates: *mut u32,
    pub dwLength: u32,
    pub ddRVal: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for DDHAL_GETDRIVERSTATEDATA {}
impl ::core::clone::Clone for DDHAL_GETDRIVERSTATEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDHAL_GETDRIVERSTATEDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDHAL_GETDRIVERSTATEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDHAL_GETDRIVERSTATEDATA_0 {
    pub dwhContext: usize,
}
impl ::core::marker::Copy for DDHAL_GETDRIVERSTATEDATA_0 {}
impl ::core::clone::Clone for DDHAL_GETDRIVERSTATEDATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDHAL_GETDRIVERSTATEDATA_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDHAL_GETDRIVERSTATEDATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_GETFLIPSTATUSDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwFlags: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetFlipStatus: LPDDHALSURFCB_GETFLIPSTATUS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_GETFLIPSTATUSDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_GETFLIPSTATUSDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_GETFLIPSTATUSDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_GETFLIPSTATUSDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_GETFLIPSTATUSDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETFLIPSTATUSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDHAL_GETHEAPALIGNMENTDATA {
    pub dwInstance: usize,
    pub dwHeap: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetHeapAlignment: LPDDHAL_GETHEAPALIGNMENT,
    pub Alignment: HEAPALIGNMENT,
}
impl ::core::marker::Copy for DDHAL_GETHEAPALIGNMENTDATA {}
impl ::core::clone::Clone for DDHAL_GETHEAPALIGNMENTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDHAL_GETHEAPALIGNMENTDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDHAL_GETHEAPALIGNMENTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_GETINTERNALMOCOMPDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpGuid: *mut ::windows::core::GUID,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub ddPixelFormat: DDPIXELFORMAT,
    pub dwScratchMemAlloc: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetInternalMoCompInfo: LPDDHALMOCOMPCB_GETINTERNALINFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_GETINTERNALMOCOMPDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_GETINTERNALMOCOMPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_GETINTERNALMOCOMPDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETINTERNALMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_GETMOCOMPCOMPBUFFDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpGuid: *mut ::windows::core::GUID,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub ddPixelFormat: DDPIXELFORMAT,
    pub dwNumTypesCompBuffs: u32,
    pub lpCompBuffInfo: *mut DDMCCOMPBUFFERINFO,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetMoCompBuffInfo: LPDDHALMOCOMPCB_GETCOMPBUFFINFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_GETMOCOMPCOMPBUFFDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_GETMOCOMPCOMPBUFFDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_GETMOCOMPCOMPBUFFDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETMOCOMPCOMPBUFFDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_GETMOCOMPFORMATSDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpGuid: *mut ::windows::core::GUID,
    pub dwNumFormats: u32,
    pub lpFormats: *mut DDPIXELFORMAT,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetMoCompFormats: LPDDHALMOCOMPCB_GETFORMATS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_GETMOCOMPFORMATSDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_GETMOCOMPFORMATSDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_GETMOCOMPFORMATSDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_GETMOCOMPFORMATSDATA").field("lpDD", &self.lpDD).field("lpGuid", &self.lpGuid).field("dwNumFormats", &self.dwNumFormats).field("lpFormats", &self.lpFormats).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_GETMOCOMPFORMATSDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETMOCOMPFORMATSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_GETMOCOMPGUIDSDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub dwNumGuids: u32,
    pub lpGuids: *mut ::windows::core::GUID,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetMoCompGuids: LPDDHALMOCOMPCB_GETGUIDS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_GETMOCOMPGUIDSDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_GETMOCOMPGUIDSDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_GETMOCOMPGUIDSDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_GETMOCOMPGUIDSDATA").field("lpDD", &self.lpDD).field("dwNumGuids", &self.dwNumGuids).field("lpGuids", &self.lpGuids).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_GETMOCOMPGUIDSDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETMOCOMPGUIDSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_GETSCANLINEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub dwScanLine: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetScanLine: LPDDHAL_GETSCANLINE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_GETSCANLINEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_GETSCANLINEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_GETSCANLINEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_GETSCANLINEDATA").field("lpDD", &self.lpDD).field("dwScanLine", &self.dwScanLine).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_GETSCANLINEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETSCANLINEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_GETVPORTBANDWIDTHDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub lpddpfFormat: *mut DDPIXELFORMAT,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwFlags: u32,
    pub lpBandwidth: *mut DDVIDEOPORTBANDWIDTH,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetVideoPortBandwidth: LPDDHALVPORTCB_GETBANDWIDTH,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_GETVPORTBANDWIDTHDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_GETVPORTBANDWIDTHDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_GETVPORTBANDWIDTHDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_GETVPORTBANDWIDTHDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("lpddpfFormat", &self.lpddpfFormat).field("dwWidth", &self.dwWidth).field("dwHeight", &self.dwHeight).field("dwFlags", &self.dwFlags).field("lpBandwidth", &self.lpBandwidth).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_GETVPORTBANDWIDTHDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETVPORTBANDWIDTHDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_GETVPORTCONNECTDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub dwPortId: u32,
    pub lpConnect: *mut DDVIDEOPORTCONNECT,
    pub dwNumEntries: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetVideoPortConnectInfo: LPDDHALVPORTCB_GETVPORTCONNECT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_GETVPORTCONNECTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_GETVPORTCONNECTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_GETVPORTCONNECTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_GETVPORTCONNECTDATA").field("lpDD", &self.lpDD).field("dwPortId", &self.dwPortId).field("lpConnect", &self.lpConnect).field("dwNumEntries", &self.dwNumEntries).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_GETVPORTCONNECTDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETVPORTCONNECTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_GETVPORTFIELDDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub bField: super::super::Foundation::BOOL,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetVideoPortField: LPDDHALVPORTCB_GETFIELD,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_GETVPORTFIELDDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_GETVPORTFIELDDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_GETVPORTFIELDDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_GETVPORTFIELDDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("bField", &self.bField).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_GETVPORTFIELDDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETVPORTFIELDDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_GETVPORTFLIPSTATUSDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub fpSurface: usize,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetVideoPortFlipStatus: LPDDHALVPORTCB_GETFLIPSTATUS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_GETVPORTFLIPSTATUSDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_GETVPORTFLIPSTATUSDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_GETVPORTFLIPSTATUSDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_GETVPORTFLIPSTATUSDATA").field("lpDD", &self.lpDD).field("fpSurface", &self.fpSurface).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_GETVPORTFLIPSTATUSDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETVPORTFLIPSTATUSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_GETVPORTINPUTFORMATDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub dwFlags: u32,
    pub lpddpfFormat: *mut DDPIXELFORMAT,
    pub dwNumFormats: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetVideoPortInputFormats: LPDDHALVPORTCB_GETINPUTFORMATS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_GETVPORTINPUTFORMATDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_GETVPORTINPUTFORMATDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_GETVPORTINPUTFORMATDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_GETVPORTINPUTFORMATDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwFlags", &self.dwFlags).field("lpddpfFormat", &self.lpddpfFormat).field("dwNumFormats", &self.dwNumFormats).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_GETVPORTINPUTFORMATDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETVPORTINPUTFORMATDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_GETVPORTLINEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub dwLine: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetVideoPortLine: LPDDHALVPORTCB_GETLINE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_GETVPORTLINEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_GETVPORTLINEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_GETVPORTLINEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_GETVPORTLINEDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwLine", &self.dwLine).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_GETVPORTLINEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETVPORTLINEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_GETVPORTOUTPUTFORMATDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub dwFlags: u32,
    pub lpddpfInputFormat: *mut DDPIXELFORMAT,
    pub lpddpfOutputFormats: *mut DDPIXELFORMAT,
    pub dwNumFormats: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetVideoPortOutputFormats: LPDDHALVPORTCB_GETOUTPUTFORMATS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_GETVPORTOUTPUTFORMATDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_GETVPORTOUTPUTFORMATDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_GETVPORTOUTPUTFORMATDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_GETVPORTOUTPUTFORMATDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwFlags", &self.dwFlags).field("lpddpfInputFormat", &self.lpddpfInputFormat).field("lpddpfOutputFormats", &self.lpddpfOutputFormats).field("dwNumFormats", &self.dwNumFormats).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_GETVPORTOUTPUTFORMATDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETVPORTOUTPUTFORMATDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_GETVPORTSIGNALDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub dwStatus: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetVideoSignalStatus: LPDDHALVPORTCB_GETSIGNALSTATUS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_GETVPORTSIGNALDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_GETVPORTSIGNALDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_GETVPORTSIGNALDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_GETVPORTSIGNALDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwStatus", &self.dwStatus).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_GETVPORTSIGNALDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_GETVPORTSIGNALDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_LOCKDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub bHasRect: u32,
    pub rArea: super::super::Foundation::RECTL,
    pub lpSurfData: *mut ::core::ffi::c_void,
    pub ddRVal: ::windows::core::HRESULT,
    pub Lock: LPDDHALSURFCB_LOCK,
    pub dwFlags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_LOCKDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_LOCKDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_LOCKDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_LOCKDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("bHasRect", &self.bHasRect).field("rArea", &self.rArea).field("lpSurfData", &self.lpSurfData).field("ddRVal", &self.ddRVal).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_LOCKDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_LOCKDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_QUERYMOCOMPSTATUSDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpMoComp: *mut DDRAWI_DDMOTIONCOMP_LCL,
    pub lpSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwFlags: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub QueryMoCompStatus: LPDDHALMOCOMPCB_QUERYSTATUS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_QUERYMOCOMPSTATUSDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_QUERYMOCOMPSTATUSDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_QUERYMOCOMPSTATUSDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_QUERYMOCOMPSTATUSDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("lpSurface", &self.lpSurface).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_QUERYMOCOMPSTATUSDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_QUERYMOCOMPSTATUSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_RENDERMOCOMPDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpMoComp: *mut DDRAWI_DDMOTIONCOMP_LCL,
    pub dwNumBuffers: u32,
    pub lpBufferInfo: *mut DDMCBUFFERINFO,
    pub dwFunction: u32,
    pub lpInputData: *mut ::core::ffi::c_void,
    pub dwInputDataSize: u32,
    pub lpOutputData: *mut ::core::ffi::c_void,
    pub dwOutputDataSize: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub RenderMoComp: LPDDHALMOCOMPCB_RENDER,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_RENDERMOCOMPDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_RENDERMOCOMPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_RENDERMOCOMPDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_RENDERMOCOMPDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("dwNumBuffers", &self.dwNumBuffers).field("lpBufferInfo", &self.lpBufferInfo).field("dwFunction", &self.dwFunction).field("lpInputData", &self.lpInputData).field("dwInputDataSize", &self.dwInputDataSize).field("lpOutputData", &self.lpOutputData).field("dwOutputDataSize", &self.dwOutputDataSize).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_RENDERMOCOMPDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_RENDERMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_SETCLIPLISTDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub ddRVal: ::windows::core::HRESULT,
    pub SetClipList: LPDDHALSURFCB_SETCLIPLIST,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_SETCLIPLISTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_SETCLIPLISTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_SETCLIPLISTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_SETCLIPLISTDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_SETCLIPLISTDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_SETCLIPLISTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_SETCOLORKEYDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwFlags: u32,
    pub ckNew: DDCOLORKEY,
    pub ddRVal: ::windows::core::HRESULT,
    pub SetColorKey: LPDDHALSURFCB_SETCOLORKEY,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_SETCOLORKEYDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_SETCOLORKEYDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_SETCOLORKEYDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_SETCOLORKEYDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("dwFlags", &self.dwFlags).field("ckNew", &self.ckNew).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_SETCOLORKEYDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_SETCOLORKEYDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_SETENTRIESDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDPalette: *mut DDRAWI_DDRAWPALETTE_GBL,
    pub dwBase: u32,
    pub dwNumEntries: u32,
    pub lpEntries: *mut super::Gdi::PALETTEENTRY,
    pub ddRVal: ::windows::core::HRESULT,
    pub SetEntries: LPDDHALPALCB_SETENTRIES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_SETENTRIESDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_SETENTRIESDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_SETENTRIESDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_SETENTRIESDATA").field("lpDD", &self.lpDD).field("lpDDPalette", &self.lpDDPalette).field("dwBase", &self.dwBase).field("dwNumEntries", &self.dwNumEntries).field("lpEntries", &self.lpEntries).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_SETENTRIESDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_SETENTRIESDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_SETEXCLUSIVEMODEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub dwEnterExcl: u32,
    pub dwReserved: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub SetExclusiveMode: LPDDHAL_SETEXCLUSIVEMODE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_SETEXCLUSIVEMODEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_SETEXCLUSIVEMODEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_SETEXCLUSIVEMODEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_SETEXCLUSIVEMODEDATA").field("lpDD", &self.lpDD).field("dwEnterExcl", &self.dwEnterExcl).field("dwReserved", &self.dwReserved).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_SETEXCLUSIVEMODEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_SETEXCLUSIVEMODEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_SETMODEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub dwModeIndex: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub SetMode: LPDDHAL_SETMODE,
    pub inexcl: super::super::Foundation::BOOL,
    pub useRefreshRate: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_SETMODEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_SETMODEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_SETMODEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_SETMODEDATA").field("lpDD", &self.lpDD).field("dwModeIndex", &self.dwModeIndex).field("ddRVal", &self.ddRVal).field("inexcl", &self.inexcl).field("useRefreshRate", &self.useRefreshRate).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_SETMODEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_SETMODEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_SETOVERLAYPOSITIONDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSrcSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub lpDDDestSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub lXPos: i32,
    pub lYPos: i32,
    pub ddRVal: ::windows::core::HRESULT,
    pub SetOverlayPosition: LPDDHALSURFCB_SETOVERLAYPOSITION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_SETOVERLAYPOSITIONDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_SETOVERLAYPOSITIONDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_SETOVERLAYPOSITIONDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_SETOVERLAYPOSITIONDATA").field("lpDD", &self.lpDD).field("lpDDSrcSurface", &self.lpDDSrcSurface).field("lpDDDestSurface", &self.lpDDDestSurface).field("lXPos", &self.lXPos).field("lYPos", &self.lYPos).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_SETOVERLAYPOSITIONDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_SETOVERLAYPOSITIONDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_SETPALETTEDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub lpDDPalette: *mut DDRAWI_DDRAWPALETTE_GBL,
    pub ddRVal: ::windows::core::HRESULT,
    pub SetPalette: LPDDHALSURFCB_SETPALETTE,
    pub Attach: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_SETPALETTEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_SETPALETTEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_SETPALETTEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_SETPALETTEDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("lpDDPalette", &self.lpDDPalette).field("ddRVal", &self.ddRVal).field("Attach", &self.Attach).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_SETPALETTEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_SETPALETTEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_SYNCSURFACEDATA {
    pub dwSize: u32,
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwSurfaceOffset: u32,
    pub fpLockPtr: usize,
    pub lPitch: i32,
    pub dwOverlayOffset: u32,
    pub dwOverlaySrcWidth: u32,
    pub dwOverlaySrcHeight: u32,
    pub dwOverlayDestWidth: u32,
    pub dwOverlayDestHeight: u32,
    pub dwDriverReserved1: usize,
    pub dwDriverReserved2: usize,
    pub dwDriverReserved3: usize,
    pub ddRVal: ::windows::core::HRESULT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_SYNCSURFACEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_SYNCSURFACEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_SYNCSURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_SYNCSURFACEDATA")
            .field("dwSize", &self.dwSize)
            .field("lpDD", &self.lpDD)
            .field("lpDDSurface", &self.lpDDSurface)
            .field("dwSurfaceOffset", &self.dwSurfaceOffset)
            .field("fpLockPtr", &self.fpLockPtr)
            .field("lPitch", &self.lPitch)
            .field("dwOverlayOffset", &self.dwOverlayOffset)
            .field("dwOverlaySrcWidth", &self.dwOverlaySrcWidth)
            .field("dwOverlaySrcHeight", &self.dwOverlaySrcHeight)
            .field("dwOverlayDestWidth", &self.dwOverlayDestWidth)
            .field("dwOverlayDestHeight", &self.dwOverlayDestHeight)
            .field("dwDriverReserved1", &self.dwDriverReserved1)
            .field("dwDriverReserved2", &self.dwDriverReserved2)
            .field("dwDriverReserved3", &self.dwDriverReserved3)
            .field("ddRVal", &self.ddRVal)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_SYNCSURFACEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDHAL_SYNCSURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.dwSurfaceOffset == other.dwSurfaceOffset && self.fpLockPtr == other.fpLockPtr && self.lPitch == other.lPitch && self.dwOverlayOffset == other.dwOverlayOffset && self.dwOverlaySrcWidth == other.dwOverlaySrcWidth && self.dwOverlaySrcHeight == other.dwOverlaySrcHeight && self.dwOverlayDestWidth == other.dwOverlayDestWidth && self.dwOverlayDestHeight == other.dwOverlayDestHeight && self.dwDriverReserved1 == other.dwDriverReserved1 && self.dwDriverReserved2 == other.dwDriverReserved2 && self.dwDriverReserved3 == other.dwDriverReserved3 && self.ddRVal == other.ddRVal
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDHAL_SYNCSURFACEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_SYNCSURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_SYNCVIDEOPORTDATA {
    pub dwSize: u32,
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub dwOriginOffset: u32,
    pub dwHeight: u32,
    pub dwVBIHeight: u32,
    pub dwDriverReserved1: usize,
    pub dwDriverReserved2: usize,
    pub dwDriverReserved3: usize,
    pub ddRVal: ::windows::core::HRESULT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_SYNCVIDEOPORTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_SYNCVIDEOPORTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_SYNCVIDEOPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_SYNCVIDEOPORTDATA").field("dwSize", &self.dwSize).field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwOriginOffset", &self.dwOriginOffset).field("dwHeight", &self.dwHeight).field("dwVBIHeight", &self.dwVBIHeight).field("dwDriverReserved1", &self.dwDriverReserved1).field("dwDriverReserved2", &self.dwDriverReserved2).field("dwDriverReserved3", &self.dwDriverReserved3).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_SYNCVIDEOPORTDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDHAL_SYNCVIDEOPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwOriginOffset == other.dwOriginOffset && self.dwHeight == other.dwHeight && self.dwVBIHeight == other.dwVBIHeight && self.dwDriverReserved1 == other.dwDriverReserved1 && self.dwDriverReserved2 == other.dwDriverReserved2 && self.dwDriverReserved3 == other.dwDriverReserved3 && self.ddRVal == other.ddRVal
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDHAL_SYNCVIDEOPORTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_SYNCVIDEOPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_UNLOCKDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub ddRVal: ::windows::core::HRESULT,
    pub Unlock: LPDDHALSURFCB_UNLOCK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_UNLOCKDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_UNLOCKDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_UNLOCKDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_UNLOCKDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_UNLOCKDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_UNLOCKDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_UPDATENONLOCALHEAPDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub dwHeap: u32,
    pub fpGARTLin: usize,
    pub fpGARTDev: usize,
    pub ulPolicyMaxBytes: usize,
    pub ddRVal: ::windows::core::HRESULT,
    pub UpdateNonLocalHeap: LPDDHAL_UPDATENONLOCALHEAP,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_UPDATENONLOCALHEAPDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_UPDATENONLOCALHEAPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_UPDATENONLOCALHEAPDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_UPDATENONLOCALHEAPDATA").field("lpDD", &self.lpDD).field("dwHeap", &self.dwHeap).field("fpGARTLin", &self.fpGARTLin).field("fpGARTDev", &self.fpGARTDev).field("ulPolicyMaxBytes", &self.ulPolicyMaxBytes).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_UPDATENONLOCALHEAPDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_UPDATENONLOCALHEAPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_UPDATEOVERLAYDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDDestSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub rDest: super::super::Foundation::RECTL,
    pub lpDDSrcSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub rSrc: super::super::Foundation::RECTL,
    pub dwFlags: u32,
    pub overlayFX: DDOVERLAYFX,
    pub ddRVal: ::windows::core::HRESULT,
    pub UpdateOverlay: LPDDHALSURFCB_UPDATEOVERLAY,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_UPDATEOVERLAYDATA {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_UPDATEOVERLAYDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_UPDATEOVERLAYDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_UPDATEVPORTDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub lplpDDSurface: *mut *mut DDRAWI_DDRAWSURFACE_INT,
    pub lplpDDVBISurface: *mut *mut DDRAWI_DDRAWSURFACE_INT,
    pub lpVideoInfo: *mut DDVIDEOPORTINFO,
    pub dwFlags: u32,
    pub dwNumAutoflip: u32,
    pub dwNumVBIAutoflip: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub UpdateVideoPort: LPDDHALVPORTCB_UPDATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_UPDATEVPORTDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_UPDATEVPORTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_UPDATEVPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_UPDATEVPORTDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("lplpDDSurface", &self.lplpDDSurface).field("lplpDDVBISurface", &self.lplpDDVBISurface).field("lpVideoInfo", &self.lpVideoInfo).field("dwFlags", &self.dwFlags).field("dwNumAutoflip", &self.dwNumAutoflip).field("dwNumVBIAutoflip", &self.dwNumVBIAutoflip).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_UPDATEVPORTDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_UPDATEVPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_VPORTCOLORDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub dwFlags: u32,
    pub lpColorData: *mut DDCOLORCONTROL,
    pub ddRVal: ::windows::core::HRESULT,
    pub ColorControl: LPDDHALVPORTCB_COLORCONTROL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_VPORTCOLORDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_VPORTCOLORDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_VPORTCOLORDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_VPORTCOLORDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwFlags", &self.dwFlags).field("lpColorData", &self.lpColorData).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_VPORTCOLORDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_VPORTCOLORDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_WAITFORVERTICALBLANKDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub dwFlags: u32,
    pub bIsInVB: u32,
    pub hEvent: usize,
    pub ddRVal: ::windows::core::HRESULT,
    pub WaitForVerticalBlank: LPDDHAL_WAITFORVERTICALBLANK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_WAITFORVERTICALBLANKDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_WAITFORVERTICALBLANKDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_WAITFORVERTICALBLANKDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_WAITFORVERTICALBLANKDATA").field("lpDD", &self.lpDD).field("dwFlags", &self.dwFlags).field("bIsInVB", &self.bIsInVB).field("hEvent", &self.hEvent).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_WAITFORVERTICALBLANKDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_WAITFORVERTICALBLANKDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDHAL_WAITFORVPORTSYNCDATA {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub dwFlags: u32,
    pub dwLine: u32,
    pub dwTimeOut: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub WaitForVideoPortSync: LPDDHALVPORTCB_WAITFORSYNC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDHAL_WAITFORVPORTSYNCDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDHAL_WAITFORVPORTSYNCDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDHAL_WAITFORVPORTSYNCDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDHAL_WAITFORVPORTSYNCDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwFlags", &self.dwFlags).field("dwLine", &self.dwLine).field("dwTimeOut", &self.dwTimeOut).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDHAL_WAITFORVPORTSYNCDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDHAL_WAITFORVPORTSYNCDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDKERNELCAPS {
    pub dwSize: u32,
    pub dwCaps: u32,
    pub dwIRQCaps: u32,
}
impl ::core::marker::Copy for DDKERNELCAPS {}
impl ::core::clone::Clone for DDKERNELCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDKERNELCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDKERNELCAPS").field("dwSize", &self.dwSize).field("dwCaps", &self.dwCaps).field("dwIRQCaps", &self.dwIRQCaps).finish()
    }
}
impl ::windows::core::TypeKind for DDKERNELCAPS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDKERNELCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwCaps == other.dwCaps && self.dwIRQCaps == other.dwIRQCaps
    }
}
impl ::core::cmp::Eq for DDKERNELCAPS {}
impl ::core::default::Default for DDKERNELCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDLOCKININFO {
    pub lpSurfaceData: *mut DDSURFACEDATA,
}
impl ::core::marker::Copy for DDLOCKININFO {}
impl ::core::clone::Clone for DDLOCKININFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDLOCKININFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDLOCKININFO").field("lpSurfaceData", &self.lpSurfaceData).finish()
    }
}
impl ::windows::core::TypeKind for DDLOCKININFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDLOCKININFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpSurfaceData == other.lpSurfaceData
    }
}
impl ::core::cmp::Eq for DDLOCKININFO {}
impl ::core::default::Default for DDLOCKININFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDLOCKOUTINFO {
    pub dwSurfacePtr: usize,
}
impl ::core::marker::Copy for DDLOCKOUTINFO {}
impl ::core::clone::Clone for DDLOCKOUTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDLOCKOUTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDLOCKOUTINFO").field("dwSurfacePtr", &self.dwSurfacePtr).finish()
    }
}
impl ::windows::core::TypeKind for DDLOCKOUTINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDLOCKOUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSurfacePtr == other.dwSurfacePtr
    }
}
impl ::core::cmp::Eq for DDLOCKOUTINFO {}
impl ::core::default::Default for DDLOCKOUTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDMCBUFFERINFO {
    pub dwSize: u32,
    pub lpCompSurface: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwDataOffset: u32,
    pub dwDataSize: u32,
    pub lpPrivate: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDMCBUFFERINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDMCBUFFERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDMCBUFFERINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDMCBUFFERINFO").field("dwSize", &self.dwSize).field("lpCompSurface", &self.lpCompSurface).field("dwDataOffset", &self.dwDataOffset).field("dwDataSize", &self.dwDataSize).field("lpPrivate", &self.lpPrivate).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDMCBUFFERINFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDMCBUFFERINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpCompSurface == other.lpCompSurface && self.dwDataOffset == other.dwDataOffset && self.dwDataSize == other.dwDataSize && self.lpPrivate == other.lpPrivate
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDMCBUFFERINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDMCBUFFERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDMCCOMPBUFFERINFO {
    pub dwSize: u32,
    pub dwNumCompBuffers: u32,
    pub dwWidthToCreate: u32,
    pub dwHeightToCreate: u32,
    pub dwBytesToAllocate: u32,
    pub ddCompCaps: DDSCAPS2,
    pub ddPixelFormat: DDPIXELFORMAT,
}
impl ::core::marker::Copy for DDMCCOMPBUFFERINFO {}
impl ::core::clone::Clone for DDMCCOMPBUFFERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDMCCOMPBUFFERINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDMCCOMPBUFFERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DDMOCOMPBUFFERINFO {
    pub dwSize: u32,
    pub lpCompSurface: *mut DD_SURFACE_LOCAL,
    pub dwDataOffset: u32,
    pub dwDataSize: u32,
    pub lpPrivate: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DDMOCOMPBUFFERINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DDMOCOMPBUFFERINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DDMOCOMPBUFFERINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDMOCOMPBUFFERINFO").field("dwSize", &self.dwSize).field("lpCompSurface", &self.lpCompSurface).field("dwDataOffset", &self.dwDataOffset).field("dwDataSize", &self.dwDataSize).field("lpPrivate", &self.lpPrivate).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DDMOCOMPBUFFERINFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DDMOCOMPBUFFERINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpCompSurface == other.lpCompSurface && self.dwDataOffset == other.dwDataOffset && self.dwDataSize == other.dwDataSize && self.lpPrivate == other.lpPrivate
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DDMOCOMPBUFFERINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DDMOCOMPBUFFERINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDMONITORINFO {
    pub Manufacturer: u16,
    pub Product: u16,
    pub SerialNumber: u32,
    pub DeviceIdentifier: ::windows::core::GUID,
    pub Mode640x480: i32,
    pub Mode800x600: i32,
    pub Mode1024x768: i32,
    pub Mode1280x1024: i32,
    pub Mode1600x1200: i32,
    pub ModeReserved1: i32,
    pub ModeReserved2: i32,
    pub ModeReserved3: i32,
}
impl ::core::marker::Copy for DDMONITORINFO {}
impl ::core::clone::Clone for DDMONITORINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDMONITORINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDMONITORINFO")
            .field("Manufacturer", &self.Manufacturer)
            .field("Product", &self.Product)
            .field("SerialNumber", &self.SerialNumber)
            .field("DeviceIdentifier", &self.DeviceIdentifier)
            .field("Mode640x480", &self.Mode640x480)
            .field("Mode800x600", &self.Mode800x600)
            .field("Mode1024x768", &self.Mode1024x768)
            .field("Mode1280x1024", &self.Mode1280x1024)
            .field("Mode1600x1200", &self.Mode1600x1200)
            .field("ModeReserved1", &self.ModeReserved1)
            .field("ModeReserved2", &self.ModeReserved2)
            .field("ModeReserved3", &self.ModeReserved3)
            .finish()
    }
}
impl ::windows::core::TypeKind for DDMONITORINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDMONITORINFO {
    fn eq(&self, other: &Self) -> bool {
        self.Manufacturer == other.Manufacturer && self.Product == other.Product && self.SerialNumber == other.SerialNumber && self.DeviceIdentifier == other.DeviceIdentifier && self.Mode640x480 == other.Mode640x480 && self.Mode800x600 == other.Mode800x600 && self.Mode1024x768 == other.Mode1024x768 && self.Mode1280x1024 == other.Mode1280x1024 && self.Mode1600x1200 == other.Mode1600x1200 && self.ModeReserved1 == other.ModeReserved1 && self.ModeReserved2 == other.ModeReserved2 && self.ModeReserved3 == other.ModeReserved3
    }
}
impl ::core::cmp::Eq for DDMONITORINFO {}
impl ::core::default::Default for DDMONITORINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDMORESURFACECAPS {
    pub dwSize: u32,
    pub ddsCapsMore: DDSCAPSEX,
    pub ddsExtendedHeapRestrictions: [DDMORESURFACECAPS_0; 1],
}
impl ::core::marker::Copy for DDMORESURFACECAPS {}
impl ::core::clone::Clone for DDMORESURFACECAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDMORESURFACECAPS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDMORESURFACECAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDMORESURFACECAPS_0 {
    pub ddsCapsEx: DDSCAPSEX,
    pub ddsCapsExAlt: DDSCAPSEX,
}
impl ::core::marker::Copy for DDMORESURFACECAPS_0 {}
impl ::core::clone::Clone for DDMORESURFACECAPS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDMORESURFACECAPS_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDMORESURFACECAPS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDNONLOCALVIDMEMCAPS {
    pub dwSize: u32,
    pub dwNLVBCaps: u32,
    pub dwNLVBCaps2: u32,
    pub dwNLVBCKeyCaps: u32,
    pub dwNLVBFXCaps: u32,
    pub dwNLVBRops: [u32; 8],
}
impl ::core::marker::Copy for DDNONLOCALVIDMEMCAPS {}
impl ::core::clone::Clone for DDNONLOCALVIDMEMCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDNONLOCALVIDMEMCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDNONLOCALVIDMEMCAPS").field("dwSize", &self.dwSize).field("dwNLVBCaps", &self.dwNLVBCaps).field("dwNLVBCaps2", &self.dwNLVBCaps2).field("dwNLVBCKeyCaps", &self.dwNLVBCKeyCaps).field("dwNLVBFXCaps", &self.dwNLVBFXCaps).field("dwNLVBRops", &self.dwNLVBRops).finish()
    }
}
impl ::windows::core::TypeKind for DDNONLOCALVIDMEMCAPS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDNONLOCALVIDMEMCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwNLVBCaps == other.dwNLVBCaps && self.dwNLVBCaps2 == other.dwNLVBCaps2 && self.dwNLVBCKeyCaps == other.dwNLVBCKeyCaps && self.dwNLVBFXCaps == other.dwNLVBFXCaps && self.dwNLVBRops == other.dwNLVBRops
    }
}
impl ::core::cmp::Eq for DDNONLOCALVIDMEMCAPS {}
impl ::core::default::Default for DDNONLOCALVIDMEMCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDNTCORECAPS {
    pub dwSize: u32,
    pub dwCaps: u32,
    pub dwCaps2: u32,
    pub dwCKeyCaps: u32,
    pub dwFXCaps: u32,
    pub dwFXAlphaCaps: u32,
    pub dwPalCaps: u32,
    pub dwSVCaps: u32,
    pub dwAlphaBltConstBitDepths: u32,
    pub dwAlphaBltPixelBitDepths: u32,
    pub dwAlphaBltSurfaceBitDepths: u32,
    pub dwAlphaOverlayConstBitDepths: u32,
    pub dwAlphaOverlayPixelBitDepths: u32,
    pub dwAlphaOverlaySurfaceBitDepths: u32,
    pub dwZBufferBitDepths: u32,
    pub dwVidMemTotal: u32,
    pub dwVidMemFree: u32,
    pub dwMaxVisibleOverlays: u32,
    pub dwCurrVisibleOverlays: u32,
    pub dwNumFourCCCodes: u32,
    pub dwAlignBoundarySrc: u32,
    pub dwAlignSizeSrc: u32,
    pub dwAlignBoundaryDest: u32,
    pub dwAlignSizeDest: u32,
    pub dwAlignStrideAlign: u32,
    pub dwRops: [u32; 8],
    pub ddsCaps: DDSCAPS,
    pub dwMinOverlayStretch: u32,
    pub dwMaxOverlayStretch: u32,
    pub dwMinLiveVideoStretch: u32,
    pub dwMaxLiveVideoStretch: u32,
    pub dwMinHwCodecStretch: u32,
    pub dwMaxHwCodecStretch: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwReserved3: u32,
    pub dwSVBCaps: u32,
    pub dwSVBCKeyCaps: u32,
    pub dwSVBFXCaps: u32,
    pub dwSVBRops: [u32; 8],
    pub dwVSBCaps: u32,
    pub dwVSBCKeyCaps: u32,
    pub dwVSBFXCaps: u32,
    pub dwVSBRops: [u32; 8],
    pub dwSSBCaps: u32,
    pub dwSSBCKeyCaps: u32,
    pub dwSSBFXCaps: u32,
    pub dwSSBRops: [u32; 8],
    pub dwMaxVideoPorts: u32,
    pub dwCurrVideoPorts: u32,
    pub dwSVBCaps2: u32,
}
impl ::core::marker::Copy for DDNTCORECAPS {}
impl ::core::clone::Clone for DDNTCORECAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDNTCORECAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDNTCORECAPS")
            .field("dwSize", &self.dwSize)
            .field("dwCaps", &self.dwCaps)
            .field("dwCaps2", &self.dwCaps2)
            .field("dwCKeyCaps", &self.dwCKeyCaps)
            .field("dwFXCaps", &self.dwFXCaps)
            .field("dwFXAlphaCaps", &self.dwFXAlphaCaps)
            .field("dwPalCaps", &self.dwPalCaps)
            .field("dwSVCaps", &self.dwSVCaps)
            .field("dwAlphaBltConstBitDepths", &self.dwAlphaBltConstBitDepths)
            .field("dwAlphaBltPixelBitDepths", &self.dwAlphaBltPixelBitDepths)
            .field("dwAlphaBltSurfaceBitDepths", &self.dwAlphaBltSurfaceBitDepths)
            .field("dwAlphaOverlayConstBitDepths", &self.dwAlphaOverlayConstBitDepths)
            .field("dwAlphaOverlayPixelBitDepths", &self.dwAlphaOverlayPixelBitDepths)
            .field("dwAlphaOverlaySurfaceBitDepths", &self.dwAlphaOverlaySurfaceBitDepths)
            .field("dwZBufferBitDepths", &self.dwZBufferBitDepths)
            .field("dwVidMemTotal", &self.dwVidMemTotal)
            .field("dwVidMemFree", &self.dwVidMemFree)
            .field("dwMaxVisibleOverlays", &self.dwMaxVisibleOverlays)
            .field("dwCurrVisibleOverlays", &self.dwCurrVisibleOverlays)
            .field("dwNumFourCCCodes", &self.dwNumFourCCCodes)
            .field("dwAlignBoundarySrc", &self.dwAlignBoundarySrc)
            .field("dwAlignSizeSrc", &self.dwAlignSizeSrc)
            .field("dwAlignBoundaryDest", &self.dwAlignBoundaryDest)
            .field("dwAlignSizeDest", &self.dwAlignSizeDest)
            .field("dwAlignStrideAlign", &self.dwAlignStrideAlign)
            .field("dwRops", &self.dwRops)
            .field("ddsCaps", &self.ddsCaps)
            .field("dwMinOverlayStretch", &self.dwMinOverlayStretch)
            .field("dwMaxOverlayStretch", &self.dwMaxOverlayStretch)
            .field("dwMinLiveVideoStretch", &self.dwMinLiveVideoStretch)
            .field("dwMaxLiveVideoStretch", &self.dwMaxLiveVideoStretch)
            .field("dwMinHwCodecStretch", &self.dwMinHwCodecStretch)
            .field("dwMaxHwCodecStretch", &self.dwMaxHwCodecStretch)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .field("dwSVBCaps", &self.dwSVBCaps)
            .field("dwSVBCKeyCaps", &self.dwSVBCKeyCaps)
            .field("dwSVBFXCaps", &self.dwSVBFXCaps)
            .field("dwSVBRops", &self.dwSVBRops)
            .field("dwVSBCaps", &self.dwVSBCaps)
            .field("dwVSBCKeyCaps", &self.dwVSBCKeyCaps)
            .field("dwVSBFXCaps", &self.dwVSBFXCaps)
            .field("dwVSBRops", &self.dwVSBRops)
            .field("dwSSBCaps", &self.dwSSBCaps)
            .field("dwSSBCKeyCaps", &self.dwSSBCKeyCaps)
            .field("dwSSBFXCaps", &self.dwSSBFXCaps)
            .field("dwSSBRops", &self.dwSSBRops)
            .field("dwMaxVideoPorts", &self.dwMaxVideoPorts)
            .field("dwCurrVideoPorts", &self.dwCurrVideoPorts)
            .field("dwSVBCaps2", &self.dwSVBCaps2)
            .finish()
    }
}
impl ::windows::core::TypeKind for DDNTCORECAPS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDNTCORECAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwCaps == other.dwCaps
            && self.dwCaps2 == other.dwCaps2
            && self.dwCKeyCaps == other.dwCKeyCaps
            && self.dwFXCaps == other.dwFXCaps
            && self.dwFXAlphaCaps == other.dwFXAlphaCaps
            && self.dwPalCaps == other.dwPalCaps
            && self.dwSVCaps == other.dwSVCaps
            && self.dwAlphaBltConstBitDepths == other.dwAlphaBltConstBitDepths
            && self.dwAlphaBltPixelBitDepths == other.dwAlphaBltPixelBitDepths
            && self.dwAlphaBltSurfaceBitDepths == other.dwAlphaBltSurfaceBitDepths
            && self.dwAlphaOverlayConstBitDepths == other.dwAlphaOverlayConstBitDepths
            && self.dwAlphaOverlayPixelBitDepths == other.dwAlphaOverlayPixelBitDepths
            && self.dwAlphaOverlaySurfaceBitDepths == other.dwAlphaOverlaySurfaceBitDepths
            && self.dwZBufferBitDepths == other.dwZBufferBitDepths
            && self.dwVidMemTotal == other.dwVidMemTotal
            && self.dwVidMemFree == other.dwVidMemFree
            && self.dwMaxVisibleOverlays == other.dwMaxVisibleOverlays
            && self.dwCurrVisibleOverlays == other.dwCurrVisibleOverlays
            && self.dwNumFourCCCodes == other.dwNumFourCCCodes
            && self.dwAlignBoundarySrc == other.dwAlignBoundarySrc
            && self.dwAlignSizeSrc == other.dwAlignSizeSrc
            && self.dwAlignBoundaryDest == other.dwAlignBoundaryDest
            && self.dwAlignSizeDest == other.dwAlignSizeDest
            && self.dwAlignStrideAlign == other.dwAlignStrideAlign
            && self.dwRops == other.dwRops
            && self.ddsCaps == other.ddsCaps
            && self.dwMinOverlayStretch == other.dwMinOverlayStretch
            && self.dwMaxOverlayStretch == other.dwMaxOverlayStretch
            && self.dwMinLiveVideoStretch == other.dwMinLiveVideoStretch
            && self.dwMaxLiveVideoStretch == other.dwMaxLiveVideoStretch
            && self.dwMinHwCodecStretch == other.dwMinHwCodecStretch
            && self.dwMaxHwCodecStretch == other.dwMaxHwCodecStretch
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.dwReserved3 == other.dwReserved3
            && self.dwSVBCaps == other.dwSVBCaps
            && self.dwSVBCKeyCaps == other.dwSVBCKeyCaps
            && self.dwSVBFXCaps == other.dwSVBFXCaps
            && self.dwSVBRops == other.dwSVBRops
            && self.dwVSBCaps == other.dwVSBCaps
            && self.dwVSBCKeyCaps == other.dwVSBCKeyCaps
            && self.dwVSBFXCaps == other.dwVSBFXCaps
            && self.dwVSBRops == other.dwVSBRops
            && self.dwSSBCaps == other.dwSSBCaps
            && self.dwSSBCKeyCaps == other.dwSSBCKeyCaps
            && self.dwSSBFXCaps == other.dwSSBFXCaps
            && self.dwSSBRops == other.dwSSBRops
            && self.dwMaxVideoPorts == other.dwMaxVideoPorts
            && self.dwCurrVideoPorts == other.dwCurrVideoPorts
            && self.dwSVBCaps2 == other.dwSVBCaps2
    }
}
impl ::core::cmp::Eq for DDNTCORECAPS {}
impl ::core::default::Default for DDNTCORECAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDOPTSURFACEDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub ddSCaps: DDSCAPS2,
    pub ddOSCaps: DDOSCAPS,
    pub guid: ::windows::core::GUID,
    pub dwCompressionRatio: u32,
}
impl ::core::marker::Copy for DDOPTSURFACEDESC {}
impl ::core::clone::Clone for DDOPTSURFACEDESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDOPTSURFACEDESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDOPTSURFACEDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDOSCAPS {
    pub dwCaps: u32,
}
impl ::core::marker::Copy for DDOSCAPS {}
impl ::core::clone::Clone for DDOSCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDOSCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDOSCAPS").field("dwCaps", &self.dwCaps).finish()
    }
}
impl ::windows::core::TypeKind for DDOSCAPS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDOSCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCaps == other.dwCaps
    }
}
impl ::core::cmp::Eq for DDOSCAPS {}
impl ::core::default::Default for DDOSCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDOVERLAYFX {
    pub dwSize: u32,
    pub dwAlphaEdgeBlendBitDepth: u32,
    pub dwAlphaEdgeBlend: u32,
    pub dwReserved: u32,
    pub dwAlphaDestConstBitDepth: u32,
    pub Anonymous1: DDOVERLAYFX_0,
    pub dwAlphaSrcConstBitDepth: u32,
    pub Anonymous2: DDOVERLAYFX_1,
    pub dckDestColorkey: DDCOLORKEY,
    pub dckSrcColorkey: DDCOLORKEY,
    pub dwDDFX: u32,
    pub dwFlags: u32,
}
impl ::core::clone::Clone for DDOVERLAYFX {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows::core::TypeKind for DDOVERLAYFX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDOVERLAYFX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDOVERLAYFX_0 {
    pub dwAlphaDestConst: u32,
    pub lpDDSAlphaDest: ::std::mem::ManuallyDrop<::core::option::Option<IDirectDrawSurface>>,
}
impl ::core::clone::Clone for DDOVERLAYFX_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows::core::TypeKind for DDOVERLAYFX_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDOVERLAYFX_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDOVERLAYFX_1 {
    pub dwAlphaSrcConst: u32,
    pub lpDDSAlphaSrc: ::std::mem::ManuallyDrop<::core::option::Option<IDirectDrawSurface>>,
}
impl ::core::clone::Clone for DDOVERLAYFX_1 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows::core::TypeKind for DDOVERLAYFX_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDOVERLAYFX_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDPIXELFORMAT {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwFourCC: u32,
    pub Anonymous1: DDPIXELFORMAT_0,
    pub Anonymous2: DDPIXELFORMAT_1,
    pub Anonymous3: DDPIXELFORMAT_2,
    pub Anonymous4: DDPIXELFORMAT_3,
    pub Anonymous5: DDPIXELFORMAT_4,
}
impl ::core::marker::Copy for DDPIXELFORMAT {}
impl ::core::clone::Clone for DDPIXELFORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDPIXELFORMAT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDPIXELFORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDPIXELFORMAT_0 {
    pub dwRGBBitCount: u32,
    pub dwYUVBitCount: u32,
    pub dwZBufferBitDepth: u32,
    pub dwAlphaBitDepth: u32,
    pub dwLuminanceBitCount: u32,
    pub dwBumpBitCount: u32,
    pub dwPrivateFormatBitCount: u32,
}
impl ::core::marker::Copy for DDPIXELFORMAT_0 {}
impl ::core::clone::Clone for DDPIXELFORMAT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDPIXELFORMAT_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDPIXELFORMAT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDPIXELFORMAT_1 {
    pub dwRBitMask: u32,
    pub dwYBitMask: u32,
    pub dwStencilBitDepth: u32,
    pub dwLuminanceBitMask: u32,
    pub dwBumpDuBitMask: u32,
    pub dwOperations: u32,
}
impl ::core::marker::Copy for DDPIXELFORMAT_1 {}
impl ::core::clone::Clone for DDPIXELFORMAT_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDPIXELFORMAT_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDPIXELFORMAT_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDPIXELFORMAT_2 {
    pub dwGBitMask: u32,
    pub dwUBitMask: u32,
    pub dwZBitMask: u32,
    pub dwBumpDvBitMask: u32,
    pub MultiSampleCaps: DDPIXELFORMAT_2_0,
}
impl ::core::marker::Copy for DDPIXELFORMAT_2 {}
impl ::core::clone::Clone for DDPIXELFORMAT_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDPIXELFORMAT_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDPIXELFORMAT_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDPIXELFORMAT_2_0 {
    pub wFlipMSTypes: u16,
    pub wBltMSTypes: u16,
}
impl ::core::marker::Copy for DDPIXELFORMAT_2_0 {}
impl ::core::clone::Clone for DDPIXELFORMAT_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDPIXELFORMAT_2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDPIXELFORMAT_2_0").field("wFlipMSTypes", &self.wFlipMSTypes).field("wBltMSTypes", &self.wBltMSTypes).finish()
    }
}
impl ::windows::core::TypeKind for DDPIXELFORMAT_2_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDPIXELFORMAT_2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.wFlipMSTypes == other.wFlipMSTypes && self.wBltMSTypes == other.wBltMSTypes
    }
}
impl ::core::cmp::Eq for DDPIXELFORMAT_2_0 {}
impl ::core::default::Default for DDPIXELFORMAT_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDPIXELFORMAT_3 {
    pub dwBBitMask: u32,
    pub dwVBitMask: u32,
    pub dwStencilBitMask: u32,
    pub dwBumpLuminanceBitMask: u32,
}
impl ::core::marker::Copy for DDPIXELFORMAT_3 {}
impl ::core::clone::Clone for DDPIXELFORMAT_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDPIXELFORMAT_3 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDPIXELFORMAT_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDPIXELFORMAT_4 {
    pub dwRGBAlphaBitMask: u32,
    pub dwYUVAlphaBitMask: u32,
    pub dwLuminanceAlphaBitMask: u32,
    pub dwRGBZBitMask: u32,
    pub dwYUVZBitMask: u32,
}
impl ::core::marker::Copy for DDPIXELFORMAT_4 {}
impl ::core::clone::Clone for DDPIXELFORMAT_4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDPIXELFORMAT_4 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDPIXELFORMAT_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDRAWI_DDMOTIONCOMP_INT {
    pub lpVtbl: *mut ::core::ffi::c_void,
    pub lpLcl: *mut DDRAWI_DDMOTIONCOMP_LCL,
    pub lpLink: *mut DDRAWI_DDMOTIONCOMP_INT,
    pub dwIntRefCnt: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DDMOTIONCOMP_INT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDMOTIONCOMP_INT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DDMOTIONCOMP_INT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DDMOTIONCOMP_INT").field("lpVtbl", &self.lpVtbl).field("lpLcl", &self.lpLcl).field("lpLink", &self.lpLink).field("dwIntRefCnt", &self.dwIntRefCnt).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDMOTIONCOMP_INT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDRAWI_DDMOTIONCOMP_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl && self.lpLcl == other.lpLcl && self.lpLink == other.lpLink && self.dwIntRefCnt == other.dwIntRefCnt
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDRAWI_DDMOTIONCOMP_INT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDMOTIONCOMP_INT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDRAWI_DDMOTIONCOMP_LCL {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub guid: ::windows::core::GUID,
    pub dwUncompWidth: u32,
    pub dwUncompHeight: u32,
    pub ddUncompPixelFormat: DDPIXELFORMAT,
    pub dwInternalFlags: u32,
    pub dwRefCnt: u32,
    pub dwProcessId: u32,
    pub hMoComp: super::super::Foundation::HANDLE,
    pub dwDriverReserved1: u32,
    pub dwDriverReserved2: u32,
    pub dwDriverReserved3: u32,
    pub lpDriverReserved1: *mut ::core::ffi::c_void,
    pub lpDriverReserved2: *mut ::core::ffi::c_void,
    pub lpDriverReserved3: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DDMOTIONCOMP_LCL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDMOTIONCOMP_LCL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDMOTIONCOMP_LCL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDMOTIONCOMP_LCL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDRAWI_DDRAWCLIPPER_GBL {
    pub dwRefCnt: u32,
    pub dwFlags: u32,
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub dwProcessId: u32,
    pub dwReserved1: usize,
    pub hWnd: usize,
    pub lpStaticClipList: *mut super::Gdi::RGNDATA,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DDRAWCLIPPER_GBL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDRAWCLIPPER_GBL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DDRAWCLIPPER_GBL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DDRAWCLIPPER_GBL").field("dwRefCnt", &self.dwRefCnt).field("dwFlags", &self.dwFlags).field("lpDD", &self.lpDD).field("dwProcessId", &self.dwProcessId).field("dwReserved1", &self.dwReserved1).field("hWnd", &self.hWnd).field("lpStaticClipList", &self.lpStaticClipList).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDRAWCLIPPER_GBL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDRAWI_DDRAWCLIPPER_GBL {
    fn eq(&self, other: &Self) -> bool {
        self.dwRefCnt == other.dwRefCnt && self.dwFlags == other.dwFlags && self.lpDD == other.lpDD && self.dwProcessId == other.dwProcessId && self.dwReserved1 == other.dwReserved1 && self.hWnd == other.hWnd && self.lpStaticClipList == other.lpStaticClipList
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDRAWI_DDRAWCLIPPER_GBL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWCLIPPER_GBL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDRAWI_DDRAWCLIPPER_INT {
    pub lpVtbl: *mut ::core::ffi::c_void,
    pub lpLcl: *mut DDRAWI_DDRAWCLIPPER_LCL,
    pub lpLink: *mut DDRAWI_DDRAWCLIPPER_INT,
    pub dwIntRefCnt: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DDRAWCLIPPER_INT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDRAWCLIPPER_INT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DDRAWCLIPPER_INT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DDRAWCLIPPER_INT").field("lpVtbl", &self.lpVtbl).field("lpLcl", &self.lpLcl).field("lpLink", &self.lpLink).field("dwIntRefCnt", &self.dwIntRefCnt).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDRAWCLIPPER_INT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDRAWI_DDRAWCLIPPER_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl && self.lpLcl == other.lpLcl && self.lpLink == other.lpLink && self.dwIntRefCnt == other.dwIntRefCnt
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDRAWI_DDRAWCLIPPER_INT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWCLIPPER_INT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDRAWI_DDRAWCLIPPER_LCL {
    pub lpClipMore: u32,
    pub lpGbl: *mut DDRAWI_DDRAWCLIPPER_GBL,
    pub lpDD_lcl: *mut DDRAWI_DIRECTDRAW_LCL,
    pub dwLocalRefCnt: u32,
    pub pUnkOuter: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
    pub lpDD_int: *mut DDRAWI_DIRECTDRAW_INT,
    pub dwReserved1: usize,
    pub pAddrefedThisOwner: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDRAWCLIPPER_LCL {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DDRAWCLIPPER_LCL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DDRAWCLIPPER_LCL").field("lpClipMore", &self.lpClipMore).field("lpGbl", &self.lpGbl).field("lpDD_lcl", &self.lpDD_lcl).field("dwLocalRefCnt", &self.dwLocalRefCnt).field("pUnkOuter", &self.pUnkOuter).field("lpDD_int", &self.lpDD_int).field("dwReserved1", &self.dwReserved1).field("pAddrefedThisOwner", &self.pAddrefedThisOwner).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDRAWCLIPPER_LCL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDRAWI_DDRAWCLIPPER_LCL {
    fn eq(&self, other: &Self) -> bool {
        self.lpClipMore == other.lpClipMore && self.lpGbl == other.lpGbl && self.lpDD_lcl == other.lpDD_lcl && self.dwLocalRefCnt == other.dwLocalRefCnt && self.pUnkOuter == other.pUnkOuter && self.lpDD_int == other.lpDD_int && self.dwReserved1 == other.dwReserved1 && self.pAddrefedThisOwner == other.pAddrefedThisOwner
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDRAWI_DDRAWCLIPPER_LCL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWCLIPPER_LCL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDRAWI_DDRAWPALETTE_GBL {
    pub dwRefCnt: u32,
    pub dwFlags: u32,
    pub lpDD_lcl: *mut DDRAWI_DIRECTDRAW_LCL,
    pub dwProcessId: u32,
    pub lpColorTable: *mut super::Gdi::PALETTEENTRY,
    pub Anonymous: DDRAWI_DDRAWPALETTE_GBL_0,
    pub dwDriverReserved: u32,
    pub dwContentsStamp: u32,
    pub dwSaveStamp: u32,
    pub dwHandle: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DDRAWPALETTE_GBL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDRAWPALETTE_GBL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDRAWPALETTE_GBL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWPALETTE_GBL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub union DDRAWI_DDRAWPALETTE_GBL_0 {
    pub dwReserved1: usize,
    pub hHELGDIPalette: super::Gdi::HPALETTE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DDRAWPALETTE_GBL_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDRAWPALETTE_GBL_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDRAWPALETTE_GBL_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWPALETTE_GBL_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDRAWI_DDRAWPALETTE_INT {
    pub lpVtbl: *mut ::core::ffi::c_void,
    pub lpLcl: *mut DDRAWI_DDRAWPALETTE_LCL,
    pub lpLink: *mut DDRAWI_DDRAWPALETTE_INT,
    pub dwIntRefCnt: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DDRAWPALETTE_INT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDRAWPALETTE_INT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DDRAWPALETTE_INT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DDRAWPALETTE_INT").field("lpVtbl", &self.lpVtbl).field("lpLcl", &self.lpLcl).field("lpLink", &self.lpLink).field("dwIntRefCnt", &self.dwIntRefCnt).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDRAWPALETTE_INT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDRAWI_DDRAWPALETTE_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl && self.lpLcl == other.lpLcl && self.lpLink == other.lpLink && self.dwIntRefCnt == other.dwIntRefCnt
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDRAWI_DDRAWPALETTE_INT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWPALETTE_INT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDRAWI_DDRAWPALETTE_LCL {
    pub lpPalMore: u32,
    pub lpGbl: *mut DDRAWI_DDRAWPALETTE_GBL,
    pub dwUnused0: usize,
    pub dwLocalRefCnt: u32,
    pub pUnkOuter: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
    pub lpDD_lcl: *mut DDRAWI_DIRECTDRAW_LCL,
    pub dwReserved1: usize,
    pub dwDDRAWReserved1: usize,
    pub dwDDRAWReserved2: usize,
    pub dwDDRAWReserved3: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDRAWPALETTE_LCL {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DDRAWPALETTE_LCL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DDRAWPALETTE_LCL").field("lpPalMore", &self.lpPalMore).field("lpGbl", &self.lpGbl).field("dwUnused0", &self.dwUnused0).field("dwLocalRefCnt", &self.dwLocalRefCnt).field("pUnkOuter", &self.pUnkOuter).field("lpDD_lcl", &self.lpDD_lcl).field("dwReserved1", &self.dwReserved1).field("dwDDRAWReserved1", &self.dwDDRAWReserved1).field("dwDDRAWReserved2", &self.dwDDRAWReserved2).field("dwDDRAWReserved3", &self.dwDDRAWReserved3).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDRAWPALETTE_LCL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDRAWI_DDRAWPALETTE_LCL {
    fn eq(&self, other: &Self) -> bool {
        self.lpPalMore == other.lpPalMore && self.lpGbl == other.lpGbl && self.dwUnused0 == other.dwUnused0 && self.dwLocalRefCnt == other.dwLocalRefCnt && self.pUnkOuter == other.pUnkOuter && self.lpDD_lcl == other.lpDD_lcl && self.dwReserved1 == other.dwReserved1 && self.dwDDRAWReserved1 == other.dwDDRAWReserved1 && self.dwDDRAWReserved2 == other.dwDDRAWReserved2 && self.dwDDRAWReserved3 == other.dwDDRAWReserved3
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDRAWI_DDRAWPALETTE_LCL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWPALETTE_LCL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDRAWI_DDRAWSURFACE_GBL {
    pub dwRefCnt: u32,
    pub dwGlobalFlags: u32,
    pub Anonymous1: DDRAWI_DDRAWSURFACE_GBL_0,
    pub Anonymous2: DDRAWI_DDRAWSURFACE_GBL_1,
    pub Anonymous3: DDRAWI_DDRAWSURFACE_GBL_2,
    pub fpVidMem: usize,
    pub Anonymous4: DDRAWI_DDRAWSURFACE_GBL_3,
    pub wHeight: u16,
    pub wWidth: u16,
    pub dwUsageCount: u32,
    pub dwReserved1: usize,
    pub ddpfSurface: DDPIXELFORMAT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DDRAWSURFACE_GBL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDRAWSURFACE_GBL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDRAWSURFACE_GBL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWSURFACE_GBL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub union DDRAWI_DDRAWSURFACE_GBL_0 {
    pub lpRectList: *mut ACCESSRECTLIST,
    pub dwBlockSizeY: u32,
    pub lSlicePitch: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DDRAWSURFACE_GBL_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDRAWSURFACE_GBL_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDRAWSURFACE_GBL_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWSURFACE_GBL_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub union DDRAWI_DDRAWSURFACE_GBL_1 {
    pub lpVidMemHeap: *mut VMEMHEAP,
    pub dwBlockSizeX: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DDRAWSURFACE_GBL_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDRAWSURFACE_GBL_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDRAWSURFACE_GBL_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWSURFACE_GBL_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub union DDRAWI_DDRAWSURFACE_GBL_2 {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub lpDDHandle: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DDRAWSURFACE_GBL_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDRAWSURFACE_GBL_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDRAWSURFACE_GBL_2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWSURFACE_GBL_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub union DDRAWI_DDRAWSURFACE_GBL_3 {
    pub lPitch: i32,
    pub dwLinearSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DDRAWSURFACE_GBL_3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDRAWSURFACE_GBL_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDRAWSURFACE_GBL_3 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWSURFACE_GBL_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDRAWI_DDRAWSURFACE_GBL_MORE {
    pub dwSize: u32,
    pub Anonymous: DDRAWI_DDRAWSURFACE_GBL_MORE_0,
    pub pPageTable: *mut u32,
    pub cPages: u32,
    pub dwSavedDCContext: usize,
    pub fpAliasedVidMem: usize,
    pub dwDriverReserved: usize,
    pub dwHELReserved: usize,
    pub cPageUnlocks: u32,
    pub hKernelSurface: usize,
    pub dwKernelRefCnt: u32,
    pub lpColorInfo: *mut DDCOLORCONTROL,
    pub fpNTAlias: usize,
    pub dwContentsStamp: u32,
    pub lpvUnswappedDriverReserved: *mut ::core::ffi::c_void,
    pub lpDDRAWReserved2: *mut ::core::ffi::c_void,
    pub dwDDRAWReserved1: u32,
    pub dwDDRAWReserved2: u32,
    pub fpAliasOfVidMem: usize,
}
impl ::core::marker::Copy for DDRAWI_DDRAWSURFACE_GBL_MORE {}
impl ::core::clone::Clone for DDRAWI_DDRAWSURFACE_GBL_MORE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDRAWI_DDRAWSURFACE_GBL_MORE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDRAWI_DDRAWSURFACE_GBL_MORE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDRAWI_DDRAWSURFACE_GBL_MORE_0 {
    pub dwPhysicalPageTable: u32,
    pub fpPhysicalVidMem: usize,
}
impl ::core::marker::Copy for DDRAWI_DDRAWSURFACE_GBL_MORE_0 {}
impl ::core::clone::Clone for DDRAWI_DDRAWSURFACE_GBL_MORE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDRAWI_DDRAWSURFACE_GBL_MORE_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDRAWI_DDRAWSURFACE_GBL_MORE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDRAWI_DDRAWSURFACE_INT {
    pub lpVtbl: *mut ::core::ffi::c_void,
    pub lpLcl: *mut DDRAWI_DDRAWSURFACE_LCL,
    pub lpLink: *mut DDRAWI_DDRAWSURFACE_INT,
    pub dwIntRefCnt: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DDRAWSURFACE_INT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDRAWSURFACE_INT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DDRAWSURFACE_INT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DDRAWSURFACE_INT").field("lpVtbl", &self.lpVtbl).field("lpLcl", &self.lpLcl).field("lpLink", &self.lpLink).field("dwIntRefCnt", &self.dwIntRefCnt).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDRAWSURFACE_INT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDRAWI_DDRAWSURFACE_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl && self.lpLcl == other.lpLcl && self.lpLink == other.lpLink && self.dwIntRefCnt == other.dwIntRefCnt
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDRAWI_DDRAWSURFACE_INT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWSURFACE_INT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDRAWI_DDRAWSURFACE_LCL {
    pub lpSurfMore: *mut DDRAWI_DDRAWSURFACE_MORE,
    pub lpGbl: *mut DDRAWI_DDRAWSURFACE_GBL,
    pub hDDSurface: usize,
    pub lpAttachList: *mut ATTACHLIST,
    pub lpAttachListFrom: *mut ATTACHLIST,
    pub dwLocalRefCnt: u32,
    pub dwProcessId: u32,
    pub dwFlags: u32,
    pub ddsCaps: DDSCAPS,
    pub Anonymous1: DDRAWI_DDRAWSURFACE_LCL_0,
    pub Anonymous2: DDRAWI_DDRAWSURFACE_LCL_1,
    pub dwModeCreatedIn: u32,
    pub dwBackBufferCount: u32,
    pub ddckCKDestBlt: DDCOLORKEY,
    pub ddckCKSrcBlt: DDCOLORKEY,
    pub hDC: usize,
    pub dwReserved1: usize,
    pub ddckCKSrcOverlay: DDCOLORKEY,
    pub ddckCKDestOverlay: DDCOLORKEY,
    pub lpSurfaceOverlaying: *mut DDRAWI_DDRAWSURFACE_INT,
    pub dbnOverlayNode: DBLNODE,
    pub rcOverlaySrc: super::super::Foundation::RECT,
    pub rcOverlayDest: super::super::Foundation::RECT,
    pub dwClrXparent: u32,
    pub dwAlpha: u32,
    pub lOverlayX: i32,
    pub lOverlayY: i32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DDRAWSURFACE_LCL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDRAWSURFACE_LCL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDRAWSURFACE_LCL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWSURFACE_LCL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub union DDRAWI_DDRAWSURFACE_LCL_0 {
    pub lpDDPalette: *mut DDRAWI_DDRAWPALETTE_INT,
    pub lp16DDPalette: *mut DDRAWI_DDRAWPALETTE_INT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DDRAWSURFACE_LCL_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDRAWSURFACE_LCL_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDRAWSURFACE_LCL_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWSURFACE_LCL_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub union DDRAWI_DDRAWSURFACE_LCL_1 {
    pub lpDDClipper: *mut DDRAWI_DDRAWCLIPPER_LCL,
    pub lp16DDClipper: *mut DDRAWI_DDRAWCLIPPER_INT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DDRAWSURFACE_LCL_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDRAWSURFACE_LCL_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDRAWSURFACE_LCL_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWSURFACE_LCL_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDRAWI_DDRAWSURFACE_MORE {
    pub dwSize: u32,
    pub lpIUnknowns: *mut IUNKNOWN_LIST,
    pub lpDD_lcl: *mut DDRAWI_DIRECTDRAW_LCL,
    pub dwPageLockCount: u32,
    pub dwBytesAllocated: u32,
    pub lpDD_int: *mut DDRAWI_DIRECTDRAW_INT,
    pub dwMipMapCount: u32,
    pub lpDDIClipper: *mut DDRAWI_DDRAWCLIPPER_INT,
    pub lpHeapAliasInfo: *mut HEAPALIASINFO,
    pub dwOverlayFlags: u32,
    pub rgjunc: *mut ::core::ffi::c_void,
    pub lpVideoPort: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub lpddOverlayFX: *mut DDOVERLAYFX,
    pub ddsCapsEx: DDSCAPSEX,
    pub dwTextureStage: u32,
    pub lpDDRAWReserved: *mut ::core::ffi::c_void,
    pub lpDDRAWReserved2: *mut ::core::ffi::c_void,
    pub lpDDrawReserved3: *mut ::core::ffi::c_void,
    pub dwDDrawReserved4: u32,
    pub lpDDrawReserved5: *mut ::core::ffi::c_void,
    pub lpGammaRamp: *mut u32,
    pub lpOriginalGammaRamp: *mut u32,
    pub lpDDrawReserved6: *mut ::core::ffi::c_void,
    pub dwSurfaceHandle: u32,
    pub qwDDrawReserved8: [u32; 2],
    pub lpDDrawReserved9: *mut ::core::ffi::c_void,
    pub cSurfaces: u32,
    pub pCreatedDDSurfaceDesc2: *mut DDSURFACEDESC2,
    pub slist: *mut *mut DDRAWI_DDRAWSURFACE_LCL,
    pub dwFVF: u32,
    pub lpVB: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DDRAWSURFACE_MORE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDRAWSURFACE_MORE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDRAWSURFACE_MORE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDRAWSURFACE_MORE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDRAWI_DDVIDEOPORT_INT {
    pub lpVtbl: *mut ::core::ffi::c_void,
    pub lpLcl: *mut DDRAWI_DDVIDEOPORT_LCL,
    pub lpLink: *mut DDRAWI_DDVIDEOPORT_INT,
    pub dwIntRefCnt: u32,
    pub dwFlags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DDVIDEOPORT_INT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDVIDEOPORT_INT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DDVIDEOPORT_INT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DDVIDEOPORT_INT").field("lpVtbl", &self.lpVtbl).field("lpLcl", &self.lpLcl).field("lpLink", &self.lpLink).field("dwIntRefCnt", &self.dwIntRefCnt).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDVIDEOPORT_INT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDRAWI_DDVIDEOPORT_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl && self.lpLcl == other.lpLcl && self.lpLink == other.lpLink && self.dwIntRefCnt == other.dwIntRefCnt && self.dwFlags == other.dwFlags
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDRAWI_DDVIDEOPORT_INT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDVIDEOPORT_INT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDRAWI_DDVIDEOPORT_LCL {
    pub lpDD: *mut DDRAWI_DIRECTDRAW_LCL,
    pub ddvpDesc: DDVIDEOPORTDESC,
    pub ddvpInfo: DDVIDEOPORTINFO,
    pub lpSurface: *mut DDRAWI_DDRAWSURFACE_INT,
    pub lpVBISurface: *mut DDRAWI_DDRAWSURFACE_INT,
    pub lpFlipInts: *mut *mut DDRAWI_DDRAWSURFACE_INT,
    pub dwNumAutoflip: u32,
    pub dwProcessID: u32,
    pub dwStateFlags: u32,
    pub dwFlags: u32,
    pub dwRefCnt: u32,
    pub fpLastFlip: usize,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
    pub hDDVideoPort: super::super::Foundation::HANDLE,
    pub dwNumVBIAutoflip: u32,
    pub lpVBIDesc: *mut DDVIDEOPORTDESC,
    pub lpVideoDesc: *mut DDVIDEOPORTDESC,
    pub lpVBIInfo: *mut DDVIDEOPORTINFO,
    pub lpVideoInfo: *mut DDVIDEOPORTINFO,
    pub dwVBIProcessID: u32,
    pub lpVPNotify: *mut DDRAWI_DDVIDEOPORT_INT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DDVIDEOPORT_LCL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DDVIDEOPORT_LCL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DDVIDEOPORT_LCL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DDVIDEOPORT_LCL")
            .field("lpDD", &self.lpDD)
            .field("ddvpDesc", &self.ddvpDesc)
            .field("ddvpInfo", &self.ddvpInfo)
            .field("lpSurface", &self.lpSurface)
            .field("lpVBISurface", &self.lpVBISurface)
            .field("lpFlipInts", &self.lpFlipInts)
            .field("dwNumAutoflip", &self.dwNumAutoflip)
            .field("dwProcessID", &self.dwProcessID)
            .field("dwStateFlags", &self.dwStateFlags)
            .field("dwFlags", &self.dwFlags)
            .field("dwRefCnt", &self.dwRefCnt)
            .field("fpLastFlip", &self.fpLastFlip)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .field("hDDVideoPort", &self.hDDVideoPort)
            .field("dwNumVBIAutoflip", &self.dwNumVBIAutoflip)
            .field("lpVBIDesc", &self.lpVBIDesc)
            .field("lpVideoDesc", &self.lpVideoDesc)
            .field("lpVBIInfo", &self.lpVBIInfo)
            .field("lpVideoInfo", &self.lpVideoInfo)
            .field("dwVBIProcessID", &self.dwVBIProcessID)
            .field("lpVPNotify", &self.lpVPNotify)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DDVIDEOPORT_LCL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDRAWI_DDVIDEOPORT_LCL {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD
            && self.ddvpDesc == other.ddvpDesc
            && self.ddvpInfo == other.ddvpInfo
            && self.lpSurface == other.lpSurface
            && self.lpVBISurface == other.lpVBISurface
            && self.lpFlipInts == other.lpFlipInts
            && self.dwNumAutoflip == other.dwNumAutoflip
            && self.dwProcessID == other.dwProcessID
            && self.dwStateFlags == other.dwStateFlags
            && self.dwFlags == other.dwFlags
            && self.dwRefCnt == other.dwRefCnt
            && self.fpLastFlip == other.fpLastFlip
            && self.dwReserved1 == other.dwReserved1
            && self.dwReserved2 == other.dwReserved2
            && self.hDDVideoPort == other.hDDVideoPort
            && self.dwNumVBIAutoflip == other.dwNumVBIAutoflip
            && self.lpVBIDesc == other.lpVBIDesc
            && self.lpVideoDesc == other.lpVideoDesc
            && self.lpVBIInfo == other.lpVBIInfo
            && self.lpVideoInfo == other.lpVideoInfo
            && self.dwVBIProcessID == other.dwVBIProcessID
            && self.lpVPNotify == other.lpVPNotify
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDRAWI_DDVIDEOPORT_LCL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DDVIDEOPORT_LCL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDRAWI_DIRECTDRAW_GBL {
    pub dwRefCnt: u32,
    pub dwFlags: u32,
    pub fpPrimaryOrig: usize,
    pub ddCaps: DDCORECAPS,
    pub dwInternal1: u32,
    pub dwUnused1: [u32; 9],
    pub lpDDCBtmp: *mut DDHAL_CALLBACKS,
    pub dsList: *mut DDRAWI_DDRAWSURFACE_INT,
    pub palList: *mut DDRAWI_DDRAWPALETTE_INT,
    pub clipperList: *mut DDRAWI_DDRAWCLIPPER_INT,
    pub lp16DD: *mut DDRAWI_DIRECTDRAW_GBL,
    pub dwMaxOverlays: u32,
    pub dwCurrOverlays: u32,
    pub dwMonitorFrequency: u32,
    pub ddHELCaps: DDCORECAPS,
    pub dwUnused2: [u32; 50],
    pub ddckCKDestOverlay: DDCOLORKEY,
    pub ddckCKSrcOverlay: DDCOLORKEY,
    pub vmiData: VIDMEMINFO,
    pub lpDriverHandle: *mut ::core::ffi::c_void,
    pub lpExclusiveOwner: *mut DDRAWI_DIRECTDRAW_LCL,
    pub dwModeIndex: u32,
    pub dwModeIndexOrig: u32,
    pub dwNumFourCC: u32,
    pub lpdwFourCC: *mut u32,
    pub dwNumModes: u32,
    pub lpModeInfo: *mut DDHALMODEINFO,
    pub plProcessList: PROCESS_LIST,
    pub dwSurfaceLockCount: u32,
    pub dwAliasedLockCnt: u32,
    pub dwReserved3: usize,
    pub hDD: usize,
    pub cObsolete: [u8; 12],
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dbnOverlayRoot: DBLNODE,
    pub lpwPDeviceFlags: *mut u16,
    pub dwPDevice: u32,
    pub dwWin16LockCnt: u32,
    pub dwUnused3: u32,
    pub hInstance: u32,
    pub dwEvent16: u32,
    pub dwSaveNumModes: u32,
    pub lpD3DGlobalDriverData: usize,
    pub lpD3DHALCallbacks: usize,
    pub ddBothCaps: DDCORECAPS,
    pub lpDDVideoPortCaps: *mut DDVIDEOPORTCAPS,
    pub dvpList: *mut DDRAWI_DDVIDEOPORT_INT,
    pub lpD3DHALCallbacks2: usize,
    pub rectDevice: super::super::Foundation::RECT,
    pub cMonitors: u32,
    pub gpbmiSrc: *mut ::core::ffi::c_void,
    pub gpbmiDest: *mut ::core::ffi::c_void,
    pub phaiHeapAliases: *mut HEAPALIASINFO,
    pub hKernelHandle: usize,
    pub pfnNotifyProc: usize,
    pub lpDDKernelCaps: *mut DDKERNELCAPS,
    pub lpddNLVCaps: *mut DDNONLOCALVIDMEMCAPS,
    pub lpddNLVHELCaps: *mut DDNONLOCALVIDMEMCAPS,
    pub lpddNLVBothCaps: *mut DDNONLOCALVIDMEMCAPS,
    pub lpD3DExtendedCaps: usize,
    pub dwDOSBoxEvent: u32,
    pub rectDesktop: super::super::Foundation::RECT,
    pub cDriverName: [u8; 32],
    pub lpD3DHALCallbacks3: usize,
    pub dwNumZPixelFormats: u32,
    pub lpZPixelFormats: *mut DDPIXELFORMAT,
    pub mcList: *mut DDRAWI_DDMOTIONCOMP_INT,
    pub hDDVxd: u32,
    pub ddsCapsMore: DDSCAPSEX,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DIRECTDRAW_GBL {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DIRECTDRAW_GBL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DIRECTDRAW_GBL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DIRECTDRAW_GBL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDRAWI_DIRECTDRAW_INT {
    pub lpVtbl: *mut ::core::ffi::c_void,
    pub lpLcl: *mut DDRAWI_DIRECTDRAW_LCL,
    pub lpLink: *mut DDRAWI_DIRECTDRAW_INT,
    pub dwIntRefCnt: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DDRAWI_DIRECTDRAW_INT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DIRECTDRAW_INT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DIRECTDRAW_INT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DIRECTDRAW_INT").field("lpVtbl", &self.lpVtbl).field("lpLcl", &self.lpLcl).field("lpLink", &self.lpLink).field("dwIntRefCnt", &self.dwIntRefCnt).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DIRECTDRAW_INT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DDRAWI_DIRECTDRAW_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpVtbl == other.lpVtbl && self.lpLcl == other.lpLcl && self.lpLink == other.lpLink && self.dwIntRefCnt == other.dwIntRefCnt
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DDRAWI_DIRECTDRAW_INT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DIRECTDRAW_INT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DDRAWI_DIRECTDRAW_LCL {
    pub lpDDMore: u32,
    pub lpGbl: *mut DDRAWI_DIRECTDRAW_GBL,
    pub dwUnused0: u32,
    pub dwLocalFlags: u32,
    pub dwLocalRefCnt: u32,
    pub dwProcessId: u32,
    pub pUnkOuter: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
    pub dwObsolete1: u32,
    pub hWnd: usize,
    pub hDC: usize,
    pub dwErrorMode: u32,
    pub lpPrimary: *mut DDRAWI_DDRAWSURFACE_INT,
    pub lpCB: *mut DDRAWI_DDRAWSURFACE_INT,
    pub dwPreferredMode: u32,
    pub hD3DInstance: super::super::Foundation::HMODULE,
    pub pD3DIUnknown: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
    pub lpDDCB: *mut DDHAL_CALLBACKS,
    pub hDDVxd: usize,
    pub dwAppHackFlags: u32,
    pub hFocusWnd: usize,
    pub dwHotTracking: u32,
    pub dwIMEState: u32,
    pub hWndPopup: usize,
    pub hDD: usize,
    pub hGammaCalibrator: usize,
    pub lpGammaCalibrator: LPDDGAMMACALIBRATORPROC,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DDRAWI_DIRECTDRAW_LCL {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DDRAWI_DIRECTDRAW_LCL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRAWI_DIRECTDRAW_LCL")
            .field("lpDDMore", &self.lpDDMore)
            .field("lpGbl", &self.lpGbl)
            .field("dwUnused0", &self.dwUnused0)
            .field("dwLocalFlags", &self.dwLocalFlags)
            .field("dwLocalRefCnt", &self.dwLocalRefCnt)
            .field("dwProcessId", &self.dwProcessId)
            .field("pUnkOuter", &self.pUnkOuter)
            .field("dwObsolete1", &self.dwObsolete1)
            .field("hWnd", &self.hWnd)
            .field("hDC", &self.hDC)
            .field("dwErrorMode", &self.dwErrorMode)
            .field("lpPrimary", &self.lpPrimary)
            .field("lpCB", &self.lpCB)
            .field("dwPreferredMode", &self.dwPreferredMode)
            .field("hD3DInstance", &self.hD3DInstance)
            .field("pD3DIUnknown", &self.pD3DIUnknown)
            .field("lpDDCB", &self.lpDDCB)
            .field("hDDVxd", &self.hDDVxd)
            .field("dwAppHackFlags", &self.dwAppHackFlags)
            .field("hFocusWnd", &self.hFocusWnd)
            .field("dwHotTracking", &self.dwHotTracking)
            .field("dwIMEState", &self.dwIMEState)
            .field("hWndPopup", &self.hWndPopup)
            .field("hDD", &self.hDD)
            .field("hGammaCalibrator", &self.hGammaCalibrator)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DDRAWI_DIRECTDRAW_LCL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DDRAWI_DIRECTDRAW_LCL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDRGBA {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}
impl ::core::marker::Copy for DDRGBA {}
impl ::core::clone::Clone for DDRGBA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDRGBA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDRGBA").field("red", &self.red).field("green", &self.green).field("blue", &self.blue).field("alpha", &self.alpha).finish()
    }
}
impl ::windows::core::TypeKind for DDRGBA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDRGBA {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue && self.alpha == other.alpha
    }
}
impl ::core::cmp::Eq for DDRGBA {}
impl ::core::default::Default for DDRGBA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDSCAPS {
    pub dwCaps: u32,
}
impl ::core::marker::Copy for DDSCAPS {}
impl ::core::clone::Clone for DDSCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDSCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDSCAPS").field("dwCaps", &self.dwCaps).finish()
    }
}
impl ::windows::core::TypeKind for DDSCAPS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDSCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwCaps == other.dwCaps
    }
}
impl ::core::cmp::Eq for DDSCAPS {}
impl ::core::default::Default for DDSCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDSCAPS2 {
    pub dwCaps: u32,
    pub dwCaps2: u32,
    pub dwCaps3: u32,
    pub Anonymous: DDSCAPS2_0,
}
impl ::core::marker::Copy for DDSCAPS2 {}
impl ::core::clone::Clone for DDSCAPS2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDSCAPS2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDSCAPS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDSCAPS2_0 {
    pub dwCaps4: u32,
    pub dwVolumeDepth: u32,
}
impl ::core::marker::Copy for DDSCAPS2_0 {}
impl ::core::clone::Clone for DDSCAPS2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDSCAPS2_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDSCAPS2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDSCAPSEX {
    pub dwCaps2: u32,
    pub dwCaps3: u32,
    pub Anonymous: DDSCAPSEX_0,
}
impl ::core::marker::Copy for DDSCAPSEX {}
impl ::core::clone::Clone for DDSCAPSEX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDSCAPSEX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDSCAPSEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDSCAPSEX_0 {
    pub dwCaps4: u32,
    pub dwVolumeDepth: u32,
}
impl ::core::marker::Copy for DDSCAPSEX_0 {}
impl ::core::clone::Clone for DDSCAPSEX_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDSCAPSEX_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDSCAPSEX_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDSETSTATEININFO {
    pub lpSurfaceData: *mut DDSURFACEDATA,
    pub lpVideoPortData: *mut DDVIDEOPORTDATA,
}
impl ::core::marker::Copy for DDSETSTATEININFO {}
impl ::core::clone::Clone for DDSETSTATEININFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDSETSTATEININFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDSETSTATEININFO").field("lpSurfaceData", &self.lpSurfaceData).field("lpVideoPortData", &self.lpVideoPortData).finish()
    }
}
impl ::windows::core::TypeKind for DDSETSTATEININFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDSETSTATEININFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpSurfaceData == other.lpSurfaceData && self.lpVideoPortData == other.lpVideoPortData
    }
}
impl ::core::cmp::Eq for DDSETSTATEININFO {}
impl ::core::default::Default for DDSETSTATEININFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DDSETSTATEOUTINFO {
    pub bSoftwareAutoflip: super::super::Foundation::BOOL,
    pub dwSurfaceIndex: u32,
    pub dwVBISurfaceIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DDSETSTATEOUTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DDSETSTATEOUTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DDSETSTATEOUTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDSETSTATEOUTINFO").field("bSoftwareAutoflip", &self.bSoftwareAutoflip).field("dwSurfaceIndex", &self.dwSurfaceIndex).field("dwVBISurfaceIndex", &self.dwVBISurfaceIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DDSETSTATEOUTINFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DDSETSTATEOUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.bSoftwareAutoflip == other.bSoftwareAutoflip && self.dwSurfaceIndex == other.dwSurfaceIndex && self.dwVBISurfaceIndex == other.dwVBISurfaceIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DDSETSTATEOUTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DDSETSTATEOUTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDSKIPNEXTFIELDINFO {
    pub lpVideoPortData: *mut DDVIDEOPORTDATA,
    pub dwSkipFlags: u32,
}
impl ::core::marker::Copy for DDSKIPNEXTFIELDINFO {}
impl ::core::clone::Clone for DDSKIPNEXTFIELDINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDSKIPNEXTFIELDINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDSKIPNEXTFIELDINFO").field("lpVideoPortData", &self.lpVideoPortData).field("dwSkipFlags", &self.dwSkipFlags).finish()
    }
}
impl ::windows::core::TypeKind for DDSKIPNEXTFIELDINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDSKIPNEXTFIELDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpVideoPortData == other.lpVideoPortData && self.dwSkipFlags == other.dwSkipFlags
    }
}
impl ::core::cmp::Eq for DDSKIPNEXTFIELDINFO {}
impl ::core::default::Default for DDSKIPNEXTFIELDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DDSTEREOMODE {
    pub dwSize: u32,
    pub dwHeight: u32,
    pub dwWidth: u32,
    pub dwBpp: u32,
    pub dwRefreshRate: u32,
    pub bSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DDSTEREOMODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DDSTEREOMODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DDSTEREOMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDSTEREOMODE").field("dwSize", &self.dwSize).field("dwHeight", &self.dwHeight).field("dwWidth", &self.dwWidth).field("dwBpp", &self.dwBpp).field("dwRefreshRate", &self.dwRefreshRate).field("bSupported", &self.bSupported).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DDSTEREOMODE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DDSTEREOMODE {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwHeight == other.dwHeight && self.dwWidth == other.dwWidth && self.dwBpp == other.dwBpp && self.dwRefreshRate == other.dwRefreshRate && self.bSupported == other.bSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DDSTEREOMODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DDSTEREOMODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDSURFACEDATA {
    pub ddsCaps: u32,
    pub dwSurfaceOffset: u32,
    pub fpLockPtr: usize,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub lPitch: i32,
    pub dwOverlayFlags: u32,
    pub dwOverlayOffset: u32,
    pub dwOverlaySrcWidth: u32,
    pub dwOverlaySrcHeight: u32,
    pub dwOverlayDestWidth: u32,
    pub dwOverlayDestHeight: u32,
    pub dwVideoPortId: u32,
    pub dwFormatFlags: u32,
    pub dwFormatFourCC: u32,
    pub dwFormatBitCount: u32,
    pub dwRBitMask: u32,
    pub dwGBitMask: u32,
    pub dwBBitMask: u32,
    pub dwDriverReserved1: u32,
    pub dwDriverReserved2: u32,
    pub dwDriverReserved3: u32,
    pub dwDriverReserved4: u32,
}
impl ::core::marker::Copy for DDSURFACEDATA {}
impl ::core::clone::Clone for DDSURFACEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDSURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDSURFACEDATA")
            .field("ddsCaps", &self.ddsCaps)
            .field("dwSurfaceOffset", &self.dwSurfaceOffset)
            .field("fpLockPtr", &self.fpLockPtr)
            .field("dwWidth", &self.dwWidth)
            .field("dwHeight", &self.dwHeight)
            .field("lPitch", &self.lPitch)
            .field("dwOverlayFlags", &self.dwOverlayFlags)
            .field("dwOverlayOffset", &self.dwOverlayOffset)
            .field("dwOverlaySrcWidth", &self.dwOverlaySrcWidth)
            .field("dwOverlaySrcHeight", &self.dwOverlaySrcHeight)
            .field("dwOverlayDestWidth", &self.dwOverlayDestWidth)
            .field("dwOverlayDestHeight", &self.dwOverlayDestHeight)
            .field("dwVideoPortId", &self.dwVideoPortId)
            .field("dwFormatFlags", &self.dwFormatFlags)
            .field("dwFormatFourCC", &self.dwFormatFourCC)
            .field("dwFormatBitCount", &self.dwFormatBitCount)
            .field("dwRBitMask", &self.dwRBitMask)
            .field("dwGBitMask", &self.dwGBitMask)
            .field("dwBBitMask", &self.dwBBitMask)
            .field("dwDriverReserved1", &self.dwDriverReserved1)
            .field("dwDriverReserved2", &self.dwDriverReserved2)
            .field("dwDriverReserved3", &self.dwDriverReserved3)
            .field("dwDriverReserved4", &self.dwDriverReserved4)
            .finish()
    }
}
impl ::windows::core::TypeKind for DDSURFACEDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDSURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.ddsCaps == other.ddsCaps
            && self.dwSurfaceOffset == other.dwSurfaceOffset
            && self.fpLockPtr == other.fpLockPtr
            && self.dwWidth == other.dwWidth
            && self.dwHeight == other.dwHeight
            && self.lPitch == other.lPitch
            && self.dwOverlayFlags == other.dwOverlayFlags
            && self.dwOverlayOffset == other.dwOverlayOffset
            && self.dwOverlaySrcWidth == other.dwOverlaySrcWidth
            && self.dwOverlaySrcHeight == other.dwOverlaySrcHeight
            && self.dwOverlayDestWidth == other.dwOverlayDestWidth
            && self.dwOverlayDestHeight == other.dwOverlayDestHeight
            && self.dwVideoPortId == other.dwVideoPortId
            && self.dwFormatFlags == other.dwFormatFlags
            && self.dwFormatFourCC == other.dwFormatFourCC
            && self.dwFormatBitCount == other.dwFormatBitCount
            && self.dwRBitMask == other.dwRBitMask
            && self.dwGBitMask == other.dwGBitMask
            && self.dwBBitMask == other.dwBBitMask
            && self.dwDriverReserved1 == other.dwDriverReserved1
            && self.dwDriverReserved2 == other.dwDriverReserved2
            && self.dwDriverReserved3 == other.dwDriverReserved3
            && self.dwDriverReserved4 == other.dwDriverReserved4
    }
}
impl ::core::cmp::Eq for DDSURFACEDATA {}
impl ::core::default::Default for DDSURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDSURFACEDESC {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwHeight: u32,
    pub dwWidth: u32,
    pub Anonymous1: DDSURFACEDESC_0,
    pub dwBackBufferCount: u32,
    pub Anonymous2: DDSURFACEDESC_1,
    pub dwAlphaBitDepth: u32,
    pub dwReserved: u32,
    pub lpSurface: *mut ::core::ffi::c_void,
    pub ddckCKDestOverlay: DDCOLORKEY,
    pub ddckCKDestBlt: DDCOLORKEY,
    pub ddckCKSrcOverlay: DDCOLORKEY,
    pub ddckCKSrcBlt: DDCOLORKEY,
    pub ddpfPixelFormat: DDPIXELFORMAT,
    pub ddsCaps: DDSCAPS,
}
impl ::core::marker::Copy for DDSURFACEDESC {}
impl ::core::clone::Clone for DDSURFACEDESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDSURFACEDESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDSURFACEDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDSURFACEDESC_0 {
    pub lPitch: i32,
    pub dwLinearSize: u32,
}
impl ::core::marker::Copy for DDSURFACEDESC_0 {}
impl ::core::clone::Clone for DDSURFACEDESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDSURFACEDESC_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDSURFACEDESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDSURFACEDESC_1 {
    pub dwMipMapCount: u32,
    pub dwZBufferBitDepth: u32,
    pub dwRefreshRate: u32,
}
impl ::core::marker::Copy for DDSURFACEDESC_1 {}
impl ::core::clone::Clone for DDSURFACEDESC_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDSURFACEDESC_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDSURFACEDESC_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDSURFACEDESC2 {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwHeight: u32,
    pub dwWidth: u32,
    pub Anonymous1: DDSURFACEDESC2_0,
    pub Anonymous2: DDSURFACEDESC2_1,
    pub Anonymous3: DDSURFACEDESC2_2,
    pub dwAlphaBitDepth: u32,
    pub dwReserved: u32,
    pub lpSurface: *mut ::core::ffi::c_void,
    pub Anonymous4: DDSURFACEDESC2_3,
    pub ddckCKDestBlt: DDCOLORKEY,
    pub ddckCKSrcOverlay: DDCOLORKEY,
    pub ddckCKSrcBlt: DDCOLORKEY,
    pub Anonymous5: DDSURFACEDESC2_4,
    pub ddsCaps: DDSCAPS2,
    pub dwTextureStage: u32,
}
impl ::core::marker::Copy for DDSURFACEDESC2 {}
impl ::core::clone::Clone for DDSURFACEDESC2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDSURFACEDESC2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDSURFACEDESC2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDSURFACEDESC2_0 {
    pub lPitch: i32,
    pub dwLinearSize: u32,
}
impl ::core::marker::Copy for DDSURFACEDESC2_0 {}
impl ::core::clone::Clone for DDSURFACEDESC2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDSURFACEDESC2_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDSURFACEDESC2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDSURFACEDESC2_1 {
    pub dwBackBufferCount: u32,
    pub dwDepth: u32,
}
impl ::core::marker::Copy for DDSURFACEDESC2_1 {}
impl ::core::clone::Clone for DDSURFACEDESC2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDSURFACEDESC2_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDSURFACEDESC2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDSURFACEDESC2_2 {
    pub dwMipMapCount: u32,
    pub dwRefreshRate: u32,
    pub dwSrcVBHandle: u32,
}
impl ::core::marker::Copy for DDSURFACEDESC2_2 {}
impl ::core::clone::Clone for DDSURFACEDESC2_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDSURFACEDESC2_2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDSURFACEDESC2_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDSURFACEDESC2_3 {
    pub ddckCKDestOverlay: DDCOLORKEY,
    pub dwEmptyFaceColor: u32,
}
impl ::core::marker::Copy for DDSURFACEDESC2_3 {}
impl ::core::clone::Clone for DDSURFACEDESC2_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDSURFACEDESC2_3 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDSURFACEDESC2_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DDSURFACEDESC2_4 {
    pub ddpfPixelFormat: DDPIXELFORMAT,
    pub dwFVF: u32,
}
impl ::core::marker::Copy for DDSURFACEDESC2_4 {}
impl ::core::clone::Clone for DDSURFACEDESC2_4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DDSURFACEDESC2_4 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DDSURFACEDESC2_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDTRANSFERININFO {
    pub lpSurfaceData: *mut DDSURFACEDATA,
    pub dwStartLine: u32,
    pub dwEndLine: u32,
    pub dwTransferID: usize,
    pub dwTransferFlags: u32,
    pub lpDestMDL: *mut MDL,
}
impl ::core::marker::Copy for DDTRANSFERININFO {}
impl ::core::clone::Clone for DDTRANSFERININFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDTRANSFERININFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDTRANSFERININFO").field("lpSurfaceData", &self.lpSurfaceData).field("dwStartLine", &self.dwStartLine).field("dwEndLine", &self.dwEndLine).field("dwTransferID", &self.dwTransferID).field("dwTransferFlags", &self.dwTransferFlags).field("lpDestMDL", &self.lpDestMDL).finish()
    }
}
impl ::windows::core::TypeKind for DDTRANSFERININFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDTRANSFERININFO {
    fn eq(&self, other: &Self) -> bool {
        self.lpSurfaceData == other.lpSurfaceData && self.dwStartLine == other.dwStartLine && self.dwEndLine == other.dwEndLine && self.dwTransferID == other.dwTransferID && self.dwTransferFlags == other.dwTransferFlags && self.lpDestMDL == other.lpDestMDL
    }
}
impl ::core::cmp::Eq for DDTRANSFERININFO {}
impl ::core::default::Default for DDTRANSFERININFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDTRANSFEROUTINFO {
    pub dwBufferPolarity: u32,
}
impl ::core::marker::Copy for DDTRANSFEROUTINFO {}
impl ::core::clone::Clone for DDTRANSFEROUTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDTRANSFEROUTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDTRANSFEROUTINFO").field("dwBufferPolarity", &self.dwBufferPolarity).finish()
    }
}
impl ::windows::core::TypeKind for DDTRANSFEROUTINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDTRANSFEROUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwBufferPolarity == other.dwBufferPolarity
    }
}
impl ::core::cmp::Eq for DDTRANSFEROUTINFO {}
impl ::core::default::Default for DDTRANSFEROUTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDVERSIONDATA {
    pub dwHALVersion: u32,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
}
impl ::core::marker::Copy for DDVERSIONDATA {}
impl ::core::clone::Clone for DDVERSIONDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDVERSIONDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDVERSIONDATA").field("dwHALVersion", &self.dwHALVersion).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).finish()
    }
}
impl ::windows::core::TypeKind for DDVERSIONDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDVERSIONDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwHALVersion == other.dwHALVersion && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2
    }
}
impl ::core::cmp::Eq for DDVERSIONDATA {}
impl ::core::default::Default for DDVERSIONDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDVIDEOPORTBANDWIDTH {
    pub dwSize: u32,
    pub dwCaps: u32,
    pub dwOverlay: u32,
    pub dwColorkey: u32,
    pub dwYInterpolate: u32,
    pub dwYInterpAndColorkey: u32,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
}
impl ::core::marker::Copy for DDVIDEOPORTBANDWIDTH {}
impl ::core::clone::Clone for DDVIDEOPORTBANDWIDTH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDVIDEOPORTBANDWIDTH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDVIDEOPORTBANDWIDTH").field("dwSize", &self.dwSize).field("dwCaps", &self.dwCaps).field("dwOverlay", &self.dwOverlay).field("dwColorkey", &self.dwColorkey).field("dwYInterpolate", &self.dwYInterpolate).field("dwYInterpAndColorkey", &self.dwYInterpAndColorkey).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).finish()
    }
}
impl ::windows::core::TypeKind for DDVIDEOPORTBANDWIDTH {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDVIDEOPORTBANDWIDTH {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwCaps == other.dwCaps && self.dwOverlay == other.dwOverlay && self.dwColorkey == other.dwColorkey && self.dwYInterpolate == other.dwYInterpolate && self.dwYInterpAndColorkey == other.dwYInterpAndColorkey && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2
    }
}
impl ::core::cmp::Eq for DDVIDEOPORTBANDWIDTH {}
impl ::core::default::Default for DDVIDEOPORTBANDWIDTH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDVIDEOPORTCAPS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwMaxWidth: u32,
    pub dwMaxVBIWidth: u32,
    pub dwMaxHeight: u32,
    pub dwVideoPortID: u32,
    pub dwCaps: u32,
    pub dwFX: u32,
    pub dwNumAutoFlipSurfaces: u32,
    pub dwAlignVideoPortBoundary: u32,
    pub dwAlignVideoPortPrescaleWidth: u32,
    pub dwAlignVideoPortCropBoundary: u32,
    pub dwAlignVideoPortCropWidth: u32,
    pub dwPreshrinkXStep: u32,
    pub dwPreshrinkYStep: u32,
    pub dwNumVBIAutoFlipSurfaces: u32,
    pub dwNumPreferredAutoflip: u32,
    pub wNumFilterTapsX: u16,
    pub wNumFilterTapsY: u16,
}
impl ::core::marker::Copy for DDVIDEOPORTCAPS {}
impl ::core::clone::Clone for DDVIDEOPORTCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDVIDEOPORTCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDVIDEOPORTCAPS")
            .field("dwSize", &self.dwSize)
            .field("dwFlags", &self.dwFlags)
            .field("dwMaxWidth", &self.dwMaxWidth)
            .field("dwMaxVBIWidth", &self.dwMaxVBIWidth)
            .field("dwMaxHeight", &self.dwMaxHeight)
            .field("dwVideoPortID", &self.dwVideoPortID)
            .field("dwCaps", &self.dwCaps)
            .field("dwFX", &self.dwFX)
            .field("dwNumAutoFlipSurfaces", &self.dwNumAutoFlipSurfaces)
            .field("dwAlignVideoPortBoundary", &self.dwAlignVideoPortBoundary)
            .field("dwAlignVideoPortPrescaleWidth", &self.dwAlignVideoPortPrescaleWidth)
            .field("dwAlignVideoPortCropBoundary", &self.dwAlignVideoPortCropBoundary)
            .field("dwAlignVideoPortCropWidth", &self.dwAlignVideoPortCropWidth)
            .field("dwPreshrinkXStep", &self.dwPreshrinkXStep)
            .field("dwPreshrinkYStep", &self.dwPreshrinkYStep)
            .field("dwNumVBIAutoFlipSurfaces", &self.dwNumVBIAutoFlipSurfaces)
            .field("dwNumPreferredAutoflip", &self.dwNumPreferredAutoflip)
            .field("wNumFilterTapsX", &self.wNumFilterTapsX)
            .field("wNumFilterTapsY", &self.wNumFilterTapsY)
            .finish()
    }
}
impl ::windows::core::TypeKind for DDVIDEOPORTCAPS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDVIDEOPORTCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwFlags == other.dwFlags
            && self.dwMaxWidth == other.dwMaxWidth
            && self.dwMaxVBIWidth == other.dwMaxVBIWidth
            && self.dwMaxHeight == other.dwMaxHeight
            && self.dwVideoPortID == other.dwVideoPortID
            && self.dwCaps == other.dwCaps
            && self.dwFX == other.dwFX
            && self.dwNumAutoFlipSurfaces == other.dwNumAutoFlipSurfaces
            && self.dwAlignVideoPortBoundary == other.dwAlignVideoPortBoundary
            && self.dwAlignVideoPortPrescaleWidth == other.dwAlignVideoPortPrescaleWidth
            && self.dwAlignVideoPortCropBoundary == other.dwAlignVideoPortCropBoundary
            && self.dwAlignVideoPortCropWidth == other.dwAlignVideoPortCropWidth
            && self.dwPreshrinkXStep == other.dwPreshrinkXStep
            && self.dwPreshrinkYStep == other.dwPreshrinkYStep
            && self.dwNumVBIAutoFlipSurfaces == other.dwNumVBIAutoFlipSurfaces
            && self.dwNumPreferredAutoflip == other.dwNumPreferredAutoflip
            && self.wNumFilterTapsX == other.wNumFilterTapsX
            && self.wNumFilterTapsY == other.wNumFilterTapsY
    }
}
impl ::core::cmp::Eq for DDVIDEOPORTCAPS {}
impl ::core::default::Default for DDVIDEOPORTCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDVIDEOPORTCONNECT {
    pub dwSize: u32,
    pub dwPortWidth: u32,
    pub guidTypeID: ::windows::core::GUID,
    pub dwFlags: u32,
    pub dwReserved1: usize,
}
impl ::core::marker::Copy for DDVIDEOPORTCONNECT {}
impl ::core::clone::Clone for DDVIDEOPORTCONNECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDVIDEOPORTCONNECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDVIDEOPORTCONNECT").field("dwSize", &self.dwSize).field("dwPortWidth", &self.dwPortWidth).field("guidTypeID", &self.guidTypeID).field("dwFlags", &self.dwFlags).field("dwReserved1", &self.dwReserved1).finish()
    }
}
impl ::windows::core::TypeKind for DDVIDEOPORTCONNECT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDVIDEOPORTCONNECT {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwPortWidth == other.dwPortWidth && self.guidTypeID == other.guidTypeID && self.dwFlags == other.dwFlags && self.dwReserved1 == other.dwReserved1
    }
}
impl ::core::cmp::Eq for DDVIDEOPORTCONNECT {}
impl ::core::default::Default for DDVIDEOPORTCONNECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDVIDEOPORTDATA {
    pub dwVideoPortId: u32,
    pub dwVPFlags: u32,
    pub dwOriginOffset: u32,
    pub dwHeight: u32,
    pub dwVBIHeight: u32,
    pub dwDriverReserved1: u32,
    pub dwDriverReserved2: u32,
    pub dwDriverReserved3: u32,
}
impl ::core::marker::Copy for DDVIDEOPORTDATA {}
impl ::core::clone::Clone for DDVIDEOPORTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDVIDEOPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDVIDEOPORTDATA").field("dwVideoPortId", &self.dwVideoPortId).field("dwVPFlags", &self.dwVPFlags).field("dwOriginOffset", &self.dwOriginOffset).field("dwHeight", &self.dwHeight).field("dwVBIHeight", &self.dwVBIHeight).field("dwDriverReserved1", &self.dwDriverReserved1).field("dwDriverReserved2", &self.dwDriverReserved2).field("dwDriverReserved3", &self.dwDriverReserved3).finish()
    }
}
impl ::windows::core::TypeKind for DDVIDEOPORTDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDVIDEOPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwVideoPortId == other.dwVideoPortId && self.dwVPFlags == other.dwVPFlags && self.dwOriginOffset == other.dwOriginOffset && self.dwHeight == other.dwHeight && self.dwVBIHeight == other.dwVBIHeight && self.dwDriverReserved1 == other.dwDriverReserved1 && self.dwDriverReserved2 == other.dwDriverReserved2 && self.dwDriverReserved3 == other.dwDriverReserved3
    }
}
impl ::core::cmp::Eq for DDVIDEOPORTDATA {}
impl ::core::default::Default for DDVIDEOPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDVIDEOPORTDESC {
    pub dwSize: u32,
    pub dwFieldWidth: u32,
    pub dwVBIWidth: u32,
    pub dwFieldHeight: u32,
    pub dwMicrosecondsPerField: u32,
    pub dwMaxPixelsPerSecond: u32,
    pub dwVideoPortID: u32,
    pub dwReserved1: u32,
    pub VideoPortType: DDVIDEOPORTCONNECT,
    pub dwReserved2: usize,
    pub dwReserved3: usize,
}
impl ::core::marker::Copy for DDVIDEOPORTDESC {}
impl ::core::clone::Clone for DDVIDEOPORTDESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDVIDEOPORTDESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDVIDEOPORTDESC")
            .field("dwSize", &self.dwSize)
            .field("dwFieldWidth", &self.dwFieldWidth)
            .field("dwVBIWidth", &self.dwVBIWidth)
            .field("dwFieldHeight", &self.dwFieldHeight)
            .field("dwMicrosecondsPerField", &self.dwMicrosecondsPerField)
            .field("dwMaxPixelsPerSecond", &self.dwMaxPixelsPerSecond)
            .field("dwVideoPortID", &self.dwVideoPortID)
            .field("dwReserved1", &self.dwReserved1)
            .field("VideoPortType", &self.VideoPortType)
            .field("dwReserved2", &self.dwReserved2)
            .field("dwReserved3", &self.dwReserved3)
            .finish()
    }
}
impl ::windows::core::TypeKind for DDVIDEOPORTDESC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDVIDEOPORTDESC {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFieldWidth == other.dwFieldWidth && self.dwVBIWidth == other.dwVBIWidth && self.dwFieldHeight == other.dwFieldHeight && self.dwMicrosecondsPerField == other.dwMicrosecondsPerField && self.dwMaxPixelsPerSecond == other.dwMaxPixelsPerSecond && self.dwVideoPortID == other.dwVideoPortID && self.dwReserved1 == other.dwReserved1 && self.VideoPortType == other.VideoPortType && self.dwReserved2 == other.dwReserved2 && self.dwReserved3 == other.dwReserved3
    }
}
impl ::core::cmp::Eq for DDVIDEOPORTDESC {}
impl ::core::default::Default for DDVIDEOPORTDESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DDVIDEOPORTINFO {
    pub dwSize: u32,
    pub dwOriginX: u32,
    pub dwOriginY: u32,
    pub dwVPFlags: u32,
    pub rCrop: super::super::Foundation::RECT,
    pub dwPrescaleWidth: u32,
    pub dwPrescaleHeight: u32,
    pub lpddpfInputFormat: *mut DDPIXELFORMAT,
    pub lpddpfVBIInputFormat: *mut DDPIXELFORMAT,
    pub lpddpfVBIOutputFormat: *mut DDPIXELFORMAT,
    pub dwVBIHeight: u32,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DDVIDEOPORTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DDVIDEOPORTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DDVIDEOPORTINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDVIDEOPORTINFO")
            .field("dwSize", &self.dwSize)
            .field("dwOriginX", &self.dwOriginX)
            .field("dwOriginY", &self.dwOriginY)
            .field("dwVPFlags", &self.dwVPFlags)
            .field("rCrop", &self.rCrop)
            .field("dwPrescaleWidth", &self.dwPrescaleWidth)
            .field("dwPrescaleHeight", &self.dwPrescaleHeight)
            .field("lpddpfInputFormat", &self.lpddpfInputFormat)
            .field("lpddpfVBIInputFormat", &self.lpddpfVBIInputFormat)
            .field("lpddpfVBIOutputFormat", &self.lpddpfVBIOutputFormat)
            .field("dwVBIHeight", &self.dwVBIHeight)
            .field("dwReserved1", &self.dwReserved1)
            .field("dwReserved2", &self.dwReserved2)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DDVIDEOPORTINFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DDVIDEOPORTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwOriginX == other.dwOriginX && self.dwOriginY == other.dwOriginY && self.dwVPFlags == other.dwVPFlags && self.rCrop == other.rCrop && self.dwPrescaleWidth == other.dwPrescaleWidth && self.dwPrescaleHeight == other.dwPrescaleHeight && self.lpddpfInputFormat == other.lpddpfInputFormat && self.lpddpfVBIInputFormat == other.lpddpfVBIInputFormat && self.lpddpfVBIOutputFormat == other.lpddpfVBIOutputFormat && self.dwVBIHeight == other.dwVBIHeight && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DDVIDEOPORTINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DDVIDEOPORTINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DDVIDEOPORTNOTIFY {
    pub ApproximateTimeStamp: i64,
    pub lField: i32,
    pub dwSurfaceIndex: u32,
    pub lDone: i32,
}
impl ::core::marker::Copy for DDVIDEOPORTNOTIFY {}
impl ::core::clone::Clone for DDVIDEOPORTNOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DDVIDEOPORTNOTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDVIDEOPORTNOTIFY").field("ApproximateTimeStamp", &self.ApproximateTimeStamp).field("lField", &self.lField).field("dwSurfaceIndex", &self.dwSurfaceIndex).field("lDone", &self.lDone).finish()
    }
}
impl ::windows::core::TypeKind for DDVIDEOPORTNOTIFY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DDVIDEOPORTNOTIFY {
    fn eq(&self, other: &Self) -> bool {
        self.ApproximateTimeStamp == other.ApproximateTimeStamp && self.lField == other.lField && self.dwSurfaceIndex == other.dwSurfaceIndex && self.lDone == other.lDone
    }
}
impl ::core::cmp::Eq for DDVIDEOPORTNOTIFY {}
impl ::core::default::Default for DDVIDEOPORTNOTIFY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DDVIDEOPORTSTATUS {
    pub dwSize: u32,
    pub bInUse: super::super::Foundation::BOOL,
    pub dwFlags: u32,
    pub dwReserved1: u32,
    pub VideoPortType: DDVIDEOPORTCONNECT,
    pub dwReserved2: usize,
    pub dwReserved3: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DDVIDEOPORTSTATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DDVIDEOPORTSTATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DDVIDEOPORTSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DDVIDEOPORTSTATUS").field("dwSize", &self.dwSize).field("bInUse", &self.bInUse).field("dwFlags", &self.dwFlags).field("dwReserved1", &self.dwReserved1).field("VideoPortType", &self.VideoPortType).field("dwReserved2", &self.dwReserved2).field("dwReserved3", &self.dwReserved3).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DDVIDEOPORTSTATUS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DDVIDEOPORTSTATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.bInUse == other.bInUse && self.dwFlags == other.dwFlags && self.dwReserved1 == other.dwReserved1 && self.VideoPortType == other.VideoPortType && self.dwReserved2 == other.dwReserved2 && self.dwReserved3 == other.dwReserved3
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DDVIDEOPORTSTATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DDVIDEOPORTSTATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_ADDATTACHEDSURFACEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub lpSurfAttached: *mut DD_SURFACE_LOCAL,
    pub ddRVal: ::windows::core::HRESULT,
    pub AddAttachedSurface: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_ADDATTACHEDSURFACEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_ADDATTACHEDSURFACEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_ADDATTACHEDSURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_ADDATTACHEDSURFACEDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("lpSurfAttached", &self.lpSurfAttached).field("ddRVal", &self.ddRVal).field("AddAttachedSurface", &self.AddAttachedSurface).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_ADDATTACHEDSURFACEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_ADDATTACHEDSURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.lpSurfAttached == other.lpSurfAttached && self.ddRVal == other.ddRVal && self.AddAttachedSurface == other.AddAttachedSurface
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_ADDATTACHEDSURFACEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_ADDATTACHEDSURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_ATTACHLIST {
    pub lpLink: *mut DD_ATTACHLIST,
    pub lpAttached: *mut DD_SURFACE_LOCAL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_ATTACHLIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_ATTACHLIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_ATTACHLIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_ATTACHLIST").field("lpLink", &self.lpLink).field("lpAttached", &self.lpAttached).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_ATTACHLIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_ATTACHLIST {
    fn eq(&self, other: &Self) -> bool {
        self.lpLink == other.lpLink && self.lpAttached == other.lpAttached
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_ATTACHLIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_ATTACHLIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_BEGINMOCOMPFRAMEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpMoComp: *mut DD_MOTIONCOMP_LOCAL,
    pub lpDestSurface: *mut DD_SURFACE_LOCAL,
    pub dwInputDataSize: u32,
    pub lpInputData: *mut ::core::ffi::c_void,
    pub dwOutputDataSize: u32,
    pub lpOutputData: *mut ::core::ffi::c_void,
    pub ddRVal: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_BEGINMOCOMPFRAMEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_BEGINMOCOMPFRAMEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_BEGINMOCOMPFRAMEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_BEGINMOCOMPFRAMEDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("lpDestSurface", &self.lpDestSurface).field("dwInputDataSize", &self.dwInputDataSize).field("lpInputData", &self.lpInputData).field("dwOutputDataSize", &self.dwOutputDataSize).field("lpOutputData", &self.lpOutputData).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_BEGINMOCOMPFRAMEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_BEGINMOCOMPFRAMEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpMoComp == other.lpMoComp && self.lpDestSurface == other.lpDestSurface && self.dwInputDataSize == other.dwInputDataSize && self.lpInputData == other.lpInputData && self.dwOutputDataSize == other.dwOutputDataSize && self.lpOutputData == other.lpOutputData && self.ddRVal == other.ddRVal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_BEGINMOCOMPFRAMEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_BEGINMOCOMPFRAMEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_BLTDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDDestSurface: *mut DD_SURFACE_LOCAL,
    pub rDest: super::super::Foundation::RECTL,
    pub lpDDSrcSurface: *mut DD_SURFACE_LOCAL,
    pub rSrc: super::super::Foundation::RECTL,
    pub dwFlags: u32,
    pub dwROPFlags: u32,
    pub bltFX: DDBLTFX,
    pub ddRVal: ::windows::core::HRESULT,
    pub Blt: *mut ::core::ffi::c_void,
    pub IsClipped: super::super::Foundation::BOOL,
    pub rOrigDest: super::super::Foundation::RECTL,
    pub rOrigSrc: super::super::Foundation::RECTL,
    pub dwRectCnt: u32,
    pub prDestRects: *mut super::super::Foundation::RECT,
    pub dwAFlags: u32,
    pub ddargbScaleFactors: DDARGB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_BLTDATA {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_BLTDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_BLTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DD_CALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub DestroyDriver: PDD_DESTROYDRIVER,
    pub CreateSurface: PDD_CREATESURFACE,
    pub SetColorKey: PDD_SETCOLORKEY,
    pub SetMode: PDD_SETMODE,
    pub WaitForVerticalBlank: PDD_WAITFORVERTICALBLANK,
    pub CanCreateSurface: PDD_CANCREATESURFACE,
    pub CreatePalette: PDD_CREATEPALETTE,
    pub GetScanLine: PDD_GETSCANLINE,
    pub MapMemory: PDD_MAPMEMORY,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DD_CALLBACKS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DD_CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DD_CALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_CALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DD_CALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DD_CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_CANCREATESURFACEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurfaceDesc: *mut DDSURFACEDESC,
    pub bIsDifferentPixelFormat: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub CanCreateSurface: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DD_CANCREATESURFACEDATA {}
impl ::core::clone::Clone for DD_CANCREATESURFACEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_CANCREATESURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_CANCREATESURFACEDATA").field("lpDD", &self.lpDD).field("lpDDSurfaceDesc", &self.lpDDSurfaceDesc).field("bIsDifferentPixelFormat", &self.bIsDifferentPixelFormat).field("ddRVal", &self.ddRVal).field("CanCreateSurface", &self.CanCreateSurface).finish()
    }
}
impl ::windows::core::TypeKind for DD_CANCREATESURFACEDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_CANCREATESURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurfaceDesc == other.lpDDSurfaceDesc && self.bIsDifferentPixelFormat == other.bIsDifferentPixelFormat && self.ddRVal == other.ddRVal && self.CanCreateSurface == other.CanCreateSurface
    }
}
impl ::core::cmp::Eq for DD_CANCREATESURFACEDATA {}
impl ::core::default::Default for DD_CANCREATESURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_CANCREATEVPORTDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpDDVideoPortDesc: *mut DDVIDEOPORTDESC,
    pub ddRVal: ::windows::core::HRESULT,
    pub CanCreateVideoPort: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DD_CANCREATEVPORTDATA {}
impl ::core::clone::Clone for DD_CANCREATEVPORTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_CANCREATEVPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_CANCREATEVPORTDATA").field("lpDD", &self.lpDD).field("lpDDVideoPortDesc", &self.lpDDVideoPortDesc).field("ddRVal", &self.ddRVal).field("CanCreateVideoPort", &self.CanCreateVideoPort).finish()
    }
}
impl ::windows::core::TypeKind for DD_CANCREATEVPORTDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_CANCREATEVPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDVideoPortDesc == other.lpDDVideoPortDesc && self.ddRVal == other.ddRVal && self.CanCreateVideoPort == other.CanCreateVideoPort
    }
}
impl ::core::cmp::Eq for DD_CANCREATEVPORTDATA {}
impl ::core::default::Default for DD_CANCREATEVPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_CLIPPER_GLOBAL {
    pub dwReserved1: usize,
}
impl ::core::marker::Copy for DD_CLIPPER_GLOBAL {}
impl ::core::clone::Clone for DD_CLIPPER_GLOBAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_CLIPPER_GLOBAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_CLIPPER_GLOBAL").field("dwReserved1", &self.dwReserved1).finish()
    }
}
impl ::windows::core::TypeKind for DD_CLIPPER_GLOBAL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_CLIPPER_GLOBAL {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved1 == other.dwReserved1
    }
}
impl ::core::cmp::Eq for DD_CLIPPER_GLOBAL {}
impl ::core::default::Default for DD_CLIPPER_GLOBAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_CLIPPER_LOCAL {
    pub dwReserved1: usize,
}
impl ::core::marker::Copy for DD_CLIPPER_LOCAL {}
impl ::core::clone::Clone for DD_CLIPPER_LOCAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_CLIPPER_LOCAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_CLIPPER_LOCAL").field("dwReserved1", &self.dwReserved1).finish()
    }
}
impl ::windows::core::TypeKind for DD_CLIPPER_LOCAL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_CLIPPER_LOCAL {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved1 == other.dwReserved1
    }
}
impl ::core::cmp::Eq for DD_CLIPPER_LOCAL {}
impl ::core::default::Default for DD_CLIPPER_LOCAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_COLORCONTROLCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub ColorControl: PDD_COLORCB_COLORCONTROL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_COLORCONTROLCALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_COLORCONTROLCALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_COLORCONTROLCALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_COLORCONTROLCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_COLORCONTROLCALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_COLORCONTROLCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_COLORCONTROLDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub lpColorData: *mut DDCOLORCONTROL,
    pub dwFlags: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub ColorControl: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_COLORCONTROLDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_COLORCONTROLDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_COLORCONTROLDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_COLORCONTROLDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("lpColorData", &self.lpColorData).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).field("ColorControl", &self.ColorControl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_COLORCONTROLDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_COLORCONTROLDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.lpColorData == other.lpColorData && self.dwFlags == other.dwFlags && self.ddRVal == other.ddRVal && self.ColorControl == other.ColorControl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_COLORCONTROLDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_COLORCONTROLDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_CREATEMOCOMPDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpMoComp: *mut DD_MOTIONCOMP_LOCAL,
    pub lpGuid: *mut ::windows::core::GUID,
    pub dwUncompWidth: u32,
    pub dwUncompHeight: u32,
    pub ddUncompPixelFormat: DDPIXELFORMAT,
    pub lpData: *mut ::core::ffi::c_void,
    pub dwDataSize: u32,
    pub ddRVal: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for DD_CREATEMOCOMPDATA {}
impl ::core::clone::Clone for DD_CREATEMOCOMPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DD_CREATEMOCOMPDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DD_CREATEMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct DD_CREATEPALETTEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDPalette: *mut DD_PALETTE_GLOBAL,
    pub lpColorTable: *mut super::Gdi::PALETTEENTRY,
    pub ddRVal: ::windows::core::HRESULT,
    pub CreatePalette: *mut ::core::ffi::c_void,
    pub is_excl: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for DD_CREATEPALETTEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for DD_CREATEPALETTEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DD_CREATEPALETTEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_CREATEPALETTEDATA").field("lpDD", &self.lpDD).field("lpDDPalette", &self.lpDDPalette).field("lpColorTable", &self.lpColorTable).field("ddRVal", &self.ddRVal).field("CreatePalette", &self.CreatePalette).field("is_excl", &self.is_excl).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for DD_CREATEPALETTEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DD_CREATEPALETTEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDPalette == other.lpDDPalette && self.lpColorTable == other.lpColorTable && self.ddRVal == other.ddRVal && self.CreatePalette == other.CreatePalette && self.is_excl == other.is_excl
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DD_CREATEPALETTEDATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DD_CREATEPALETTEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_CREATESURFACEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurfaceDesc: *mut DDSURFACEDESC,
    pub lplpSList: *mut *mut DD_SURFACE_LOCAL,
    pub dwSCnt: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub CreateSurface: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_CREATESURFACEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_CREATESURFACEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_CREATESURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_CREATESURFACEDATA").field("lpDD", &self.lpDD).field("lpDDSurfaceDesc", &self.lpDDSurfaceDesc).field("lplpSList", &self.lplpSList).field("dwSCnt", &self.dwSCnt).field("ddRVal", &self.ddRVal).field("CreateSurface", &self.CreateSurface).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_CREATESURFACEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_CREATESURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurfaceDesc == other.lpDDSurfaceDesc && self.lplpSList == other.lplpSList && self.dwSCnt == other.dwSCnt && self.ddRVal == other.ddRVal && self.CreateSurface == other.CreateSurface
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_CREATESURFACEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_CREATESURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_CREATESURFACEEXDATA {
    pub dwFlags: u32,
    pub lpDDLcl: *mut DD_DIRECTDRAW_LOCAL,
    pub lpDDSLcl: *mut DD_SURFACE_LOCAL,
    pub ddRVal: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_CREATESURFACEEXDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_CREATESURFACEEXDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_CREATESURFACEEXDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_CREATESURFACEEXDATA").field("dwFlags", &self.dwFlags).field("lpDDLcl", &self.lpDDLcl).field("lpDDSLcl", &self.lpDDSLcl).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_CREATESURFACEEXDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_CREATESURFACEEXDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.lpDDLcl == other.lpDDLcl && self.lpDDSLcl == other.lpDDSLcl && self.ddRVal == other.ddRVal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_CREATESURFACEEXDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_CREATESURFACEEXDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_CREATEVPORTDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpDDVideoPortDesc: *mut DDVIDEOPORTDESC,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub ddRVal: ::windows::core::HRESULT,
    pub CreateVideoPort: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_CREATEVPORTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_CREATEVPORTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_CREATEVPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_CREATEVPORTDATA").field("lpDD", &self.lpDD).field("lpDDVideoPortDesc", &self.lpDDVideoPortDesc).field("lpVideoPort", &self.lpVideoPort).field("ddRVal", &self.ddRVal).field("CreateVideoPort", &self.CreateVideoPort).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_CREATEVPORTDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_CREATEVPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDVideoPortDesc == other.lpDDVideoPortDesc && self.lpVideoPort == other.lpVideoPort && self.ddRVal == other.ddRVal && self.CreateVideoPort == other.CreateVideoPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_CREATEVPORTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_CREATEVPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_D3DBUFCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub CanCreateD3DBuffer: PDD_CANCREATESURFACE,
    pub CreateD3DBuffer: PDD_CREATESURFACE,
    pub DestroyD3DBuffer: PDD_SURFCB_DESTROYSURFACE,
    pub LockD3DBuffer: PDD_SURFCB_LOCK,
    pub UnlockD3DBuffer: PDD_SURFCB_UNLOCK,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_D3DBUFCALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_D3DBUFCALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_D3DBUFCALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_D3DBUFCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_D3DBUFCALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_D3DBUFCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_DESTROYDDLOCALDATA {
    pub dwFlags: u32,
    pub pDDLcl: *mut DD_DIRECTDRAW_LOCAL,
    pub ddRVal: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for DD_DESTROYDDLOCALDATA {}
impl ::core::clone::Clone for DD_DESTROYDDLOCALDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_DESTROYDDLOCALDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_DESTROYDDLOCALDATA").field("dwFlags", &self.dwFlags).field("pDDLcl", &self.pDDLcl).field("ddRVal", &self.ddRVal).finish()
    }
}
impl ::windows::core::TypeKind for DD_DESTROYDDLOCALDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_DESTROYDDLOCALDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.pDDLcl == other.pDDLcl && self.ddRVal == other.ddRVal
    }
}
impl ::core::cmp::Eq for DD_DESTROYDDLOCALDATA {}
impl ::core::default::Default for DD_DESTROYDDLOCALDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_DESTROYMOCOMPDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpMoComp: *mut DD_MOTIONCOMP_LOCAL,
    pub ddRVal: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for DD_DESTROYMOCOMPDATA {}
impl ::core::clone::Clone for DD_DESTROYMOCOMPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_DESTROYMOCOMPDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_DESTROYMOCOMPDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("ddRVal", &self.ddRVal).finish()
    }
}
impl ::windows::core::TypeKind for DD_DESTROYMOCOMPDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_DESTROYMOCOMPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpMoComp == other.lpMoComp && self.ddRVal == other.ddRVal
    }
}
impl ::core::cmp::Eq for DD_DESTROYMOCOMPDATA {}
impl ::core::default::Default for DD_DESTROYMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_DESTROYPALETTEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDPalette: *mut DD_PALETTE_GLOBAL,
    pub ddRVal: ::windows::core::HRESULT,
    pub DestroyPalette: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DD_DESTROYPALETTEDATA {}
impl ::core::clone::Clone for DD_DESTROYPALETTEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_DESTROYPALETTEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_DESTROYPALETTEDATA").field("lpDD", &self.lpDD).field("lpDDPalette", &self.lpDDPalette).field("ddRVal", &self.ddRVal).field("DestroyPalette", &self.DestroyPalette).finish()
    }
}
impl ::windows::core::TypeKind for DD_DESTROYPALETTEDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_DESTROYPALETTEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDPalette == other.lpDDPalette && self.ddRVal == other.ddRVal && self.DestroyPalette == other.DestroyPalette
    }
}
impl ::core::cmp::Eq for DD_DESTROYPALETTEDATA {}
impl ::core::default::Default for DD_DESTROYPALETTEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_DESTROYSURFACEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub ddRVal: ::windows::core::HRESULT,
    pub DestroySurface: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_DESTROYSURFACEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_DESTROYSURFACEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_DESTROYSURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_DESTROYSURFACEDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("ddRVal", &self.ddRVal).field("DestroySurface", &self.DestroySurface).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_DESTROYSURFACEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_DESTROYSURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.ddRVal == other.ddRVal && self.DestroySurface == other.DestroySurface
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_DESTROYSURFACEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_DESTROYSURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_DESTROYVPORTDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub ddRVal: ::windows::core::HRESULT,
    pub DestroyVideoPort: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_DESTROYVPORTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_DESTROYVPORTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_DESTROYVPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_DESTROYVPORTDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("ddRVal", &self.ddRVal).field("DestroyVideoPort", &self.DestroyVideoPort).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_DESTROYVPORTDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_DESTROYVPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.ddRVal == other.ddRVal && self.DestroyVideoPort == other.DestroyVideoPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_DESTROYVPORTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_DESTROYVPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_DIRECTDRAW_GLOBAL {
    pub dhpdev: *mut ::core::ffi::c_void,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
    pub lpDDVideoPortCaps: *mut DDVIDEOPORTCAPS,
}
impl ::core::marker::Copy for DD_DIRECTDRAW_GLOBAL {}
impl ::core::clone::Clone for DD_DIRECTDRAW_GLOBAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_DIRECTDRAW_GLOBAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_DIRECTDRAW_GLOBAL").field("dhpdev", &self.dhpdev).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).field("lpDDVideoPortCaps", &self.lpDDVideoPortCaps).finish()
    }
}
impl ::windows::core::TypeKind for DD_DIRECTDRAW_GLOBAL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_DIRECTDRAW_GLOBAL {
    fn eq(&self, other: &Self) -> bool {
        self.dhpdev == other.dhpdev && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2 && self.lpDDVideoPortCaps == other.lpDDVideoPortCaps
    }
}
impl ::core::cmp::Eq for DD_DIRECTDRAW_GLOBAL {}
impl ::core::default::Default for DD_DIRECTDRAW_GLOBAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_DIRECTDRAW_LOCAL {
    pub lpGbl: *mut DD_DIRECTDRAW_GLOBAL,
}
impl ::core::marker::Copy for DD_DIRECTDRAW_LOCAL {}
impl ::core::clone::Clone for DD_DIRECTDRAW_LOCAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_DIRECTDRAW_LOCAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_DIRECTDRAW_LOCAL").field("lpGbl", &self.lpGbl).finish()
    }
}
impl ::windows::core::TypeKind for DD_DIRECTDRAW_LOCAL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_DIRECTDRAW_LOCAL {
    fn eq(&self, other: &Self) -> bool {
        self.lpGbl == other.lpGbl
    }
}
impl ::core::cmp::Eq for DD_DIRECTDRAW_LOCAL {}
impl ::core::default::Default for DD_DIRECTDRAW_LOCAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_DRVSETCOLORKEYDATA {
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub dwFlags: u32,
    pub ckNew: DDCOLORKEY,
    pub ddRVal: ::windows::core::HRESULT,
    pub SetColorKey: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_DRVSETCOLORKEYDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_DRVSETCOLORKEYDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_DRVSETCOLORKEYDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_DRVSETCOLORKEYDATA").field("lpDDSurface", &self.lpDDSurface).field("dwFlags", &self.dwFlags).field("ckNew", &self.ckNew).field("ddRVal", &self.ddRVal).field("SetColorKey", &self.SetColorKey).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_DRVSETCOLORKEYDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_DRVSETCOLORKEYDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDDSurface == other.lpDDSurface && self.dwFlags == other.dwFlags && self.ckNew == other.ckNew && self.ddRVal == other.ddRVal && self.SetColorKey == other.SetColorKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_DRVSETCOLORKEYDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_DRVSETCOLORKEYDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_ENDMOCOMPFRAMEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpMoComp: *mut DD_MOTIONCOMP_LOCAL,
    pub lpInputData: *mut ::core::ffi::c_void,
    pub dwInputDataSize: u32,
    pub ddRVal: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for DD_ENDMOCOMPFRAMEDATA {}
impl ::core::clone::Clone for DD_ENDMOCOMPFRAMEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_ENDMOCOMPFRAMEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_ENDMOCOMPFRAMEDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("lpInputData", &self.lpInputData).field("dwInputDataSize", &self.dwInputDataSize).field("ddRVal", &self.ddRVal).finish()
    }
}
impl ::windows::core::TypeKind for DD_ENDMOCOMPFRAMEDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_ENDMOCOMPFRAMEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpMoComp == other.lpMoComp && self.lpInputData == other.lpInputData && self.dwInputDataSize == other.dwInputDataSize && self.ddRVal == other.ddRVal
    }
}
impl ::core::cmp::Eq for DD_ENDMOCOMPFRAMEDATA {}
impl ::core::default::Default for DD_ENDMOCOMPFRAMEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_FLIPDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpSurfCurr: *mut DD_SURFACE_LOCAL,
    pub lpSurfTarg: *mut DD_SURFACE_LOCAL,
    pub dwFlags: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub Flip: *mut ::core::ffi::c_void,
    pub lpSurfCurrLeft: *mut DD_SURFACE_LOCAL,
    pub lpSurfTargLeft: *mut DD_SURFACE_LOCAL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_FLIPDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_FLIPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_FLIPDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_FLIPDATA").field("lpDD", &self.lpDD).field("lpSurfCurr", &self.lpSurfCurr).field("lpSurfTarg", &self.lpSurfTarg).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).field("Flip", &self.Flip).field("lpSurfCurrLeft", &self.lpSurfCurrLeft).field("lpSurfTargLeft", &self.lpSurfTargLeft).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_FLIPDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_FLIPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpSurfCurr == other.lpSurfCurr && self.lpSurfTarg == other.lpSurfTarg && self.dwFlags == other.dwFlags && self.ddRVal == other.ddRVal && self.Flip == other.Flip && self.lpSurfCurrLeft == other.lpSurfCurrLeft && self.lpSurfTargLeft == other.lpSurfTargLeft
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_FLIPDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_FLIPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_FLIPTOGDISURFACEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub dwToGDI: u32,
    pub dwReserved: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub FlipToGDISurface: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DD_FLIPTOGDISURFACEDATA {}
impl ::core::clone::Clone for DD_FLIPTOGDISURFACEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_FLIPTOGDISURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_FLIPTOGDISURFACEDATA").field("lpDD", &self.lpDD).field("dwToGDI", &self.dwToGDI).field("dwReserved", &self.dwReserved).field("ddRVal", &self.ddRVal).field("FlipToGDISurface", &self.FlipToGDISurface).finish()
    }
}
impl ::windows::core::TypeKind for DD_FLIPTOGDISURFACEDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_FLIPTOGDISURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwToGDI == other.dwToGDI && self.dwReserved == other.dwReserved && self.ddRVal == other.ddRVal && self.FlipToGDISurface == other.FlipToGDISurface
    }
}
impl ::core::cmp::Eq for DD_FLIPTOGDISURFACEDATA {}
impl ::core::default::Default for DD_FLIPTOGDISURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_FLIPVPORTDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub lpSurfCurr: *mut DD_SURFACE_LOCAL,
    pub lpSurfTarg: *mut DD_SURFACE_LOCAL,
    pub ddRVal: ::windows::core::HRESULT,
    pub FlipVideoPort: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_FLIPVPORTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_FLIPVPORTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_FLIPVPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_FLIPVPORTDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("lpSurfCurr", &self.lpSurfCurr).field("lpSurfTarg", &self.lpSurfTarg).field("ddRVal", &self.ddRVal).field("FlipVideoPort", &self.FlipVideoPort).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_FLIPVPORTDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_FLIPVPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.lpSurfCurr == other.lpSurfCurr && self.lpSurfTarg == other.lpSurfTarg && self.ddRVal == other.ddRVal && self.FlipVideoPort == other.FlipVideoPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_FLIPVPORTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_FLIPVPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_FREEDRIVERMEMORYDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub ddRVal: ::windows::core::HRESULT,
    pub FreeDriverMemory: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_FREEDRIVERMEMORYDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_FREEDRIVERMEMORYDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_FREEDRIVERMEMORYDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_FREEDRIVERMEMORYDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("ddRVal", &self.ddRVal).field("FreeDriverMemory", &self.FreeDriverMemory).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_FREEDRIVERMEMORYDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_FREEDRIVERMEMORYDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.ddRVal == other.ddRVal && self.FreeDriverMemory == other.FreeDriverMemory
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_FREEDRIVERMEMORYDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_FREEDRIVERMEMORYDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_GETAVAILDRIVERMEMORYDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub DDSCaps: DDSCAPS,
    pub dwTotal: u32,
    pub dwFree: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetAvailDriverMemory: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DD_GETAVAILDRIVERMEMORYDATA {}
impl ::core::clone::Clone for DD_GETAVAILDRIVERMEMORYDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_GETAVAILDRIVERMEMORYDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETAVAILDRIVERMEMORYDATA").field("lpDD", &self.lpDD).field("DDSCaps", &self.DDSCaps).field("dwTotal", &self.dwTotal).field("dwFree", &self.dwFree).field("ddRVal", &self.ddRVal).field("GetAvailDriverMemory", &self.GetAvailDriverMemory).finish()
    }
}
impl ::windows::core::TypeKind for DD_GETAVAILDRIVERMEMORYDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_GETAVAILDRIVERMEMORYDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.DDSCaps == other.DDSCaps && self.dwTotal == other.dwTotal && self.dwFree == other.dwFree && self.ddRVal == other.ddRVal && self.GetAvailDriverMemory == other.GetAvailDriverMemory
    }
}
impl ::core::cmp::Eq for DD_GETAVAILDRIVERMEMORYDATA {}
impl ::core::default::Default for DD_GETAVAILDRIVERMEMORYDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_GETBLTSTATUSDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub dwFlags: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetBltStatus: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_GETBLTSTATUSDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_GETBLTSTATUSDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_GETBLTSTATUSDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETBLTSTATUSDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).field("GetBltStatus", &self.GetBltStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_GETBLTSTATUSDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_GETBLTSTATUSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.dwFlags == other.dwFlags && self.ddRVal == other.ddRVal && self.GetBltStatus == other.GetBltStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_GETBLTSTATUSDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_GETBLTSTATUSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_GETDRIVERINFODATA {
    pub dhpdev: *mut ::core::ffi::c_void,
    pub dwSize: u32,
    pub dwFlags: u32,
    pub guidInfo: ::windows::core::GUID,
    pub dwExpectedSize: u32,
    pub lpvData: *mut ::core::ffi::c_void,
    pub dwActualSize: u32,
    pub ddRVal: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for DD_GETDRIVERINFODATA {}
impl ::core::clone::Clone for DD_GETDRIVERINFODATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_GETDRIVERINFODATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETDRIVERINFODATA").field("dhpdev", &self.dhpdev).field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("guidInfo", &self.guidInfo).field("dwExpectedSize", &self.dwExpectedSize).field("lpvData", &self.lpvData).field("dwActualSize", &self.dwActualSize).field("ddRVal", &self.ddRVal).finish()
    }
}
impl ::windows::core::TypeKind for DD_GETDRIVERINFODATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_GETDRIVERINFODATA {
    fn eq(&self, other: &Self) -> bool {
        self.dhpdev == other.dhpdev && self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.guidInfo == other.guidInfo && self.dwExpectedSize == other.dwExpectedSize && self.lpvData == other.lpvData && self.dwActualSize == other.dwActualSize && self.ddRVal == other.ddRVal
    }
}
impl ::core::cmp::Eq for DD_GETDRIVERINFODATA {}
impl ::core::default::Default for DD_GETDRIVERINFODATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_GETDRIVERSTATEDATA {
    pub dwFlags: u32,
    pub Anonymous: DD_GETDRIVERSTATEDATA_0,
    pub lpdwStates: *mut u32,
    pub dwLength: u32,
    pub ddRVal: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for DD_GETDRIVERSTATEDATA {}
impl ::core::clone::Clone for DD_GETDRIVERSTATEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DD_GETDRIVERSTATEDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DD_GETDRIVERSTATEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union DD_GETDRIVERSTATEDATA_0 {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub dwhContext: usize,
}
impl ::core::marker::Copy for DD_GETDRIVERSTATEDATA_0 {}
impl ::core::clone::Clone for DD_GETDRIVERSTATEDATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DD_GETDRIVERSTATEDATA_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DD_GETDRIVERSTATEDATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_GETFLIPSTATUSDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub dwFlags: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetFlipStatus: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_GETFLIPSTATUSDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_GETFLIPSTATUSDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_GETFLIPSTATUSDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETFLIPSTATUSDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).field("GetFlipStatus", &self.GetFlipStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_GETFLIPSTATUSDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_GETFLIPSTATUSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.dwFlags == other.dwFlags && self.ddRVal == other.ddRVal && self.GetFlipStatus == other.GetFlipStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_GETFLIPSTATUSDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_GETFLIPSTATUSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_GETHEAPALIGNMENTDATA {
    pub dwInstance: usize,
    pub dwHeap: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetHeapAlignment: *mut ::core::ffi::c_void,
    pub Alignment: HEAPALIGNMENT,
}
impl ::core::marker::Copy for DD_GETHEAPALIGNMENTDATA {}
impl ::core::clone::Clone for DD_GETHEAPALIGNMENTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DD_GETHEAPALIGNMENTDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DD_GETHEAPALIGNMENTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_GETINTERNALMOCOMPDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpGuid: *mut ::windows::core::GUID,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub ddPixelFormat: DDPIXELFORMAT,
    pub dwScratchMemAlloc: u32,
    pub ddRVal: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for DD_GETINTERNALMOCOMPDATA {}
impl ::core::clone::Clone for DD_GETINTERNALMOCOMPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DD_GETINTERNALMOCOMPDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DD_GETINTERNALMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_GETMOCOMPCOMPBUFFDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpGuid: *mut ::windows::core::GUID,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub ddPixelFormat: DDPIXELFORMAT,
    pub dwNumTypesCompBuffs: u32,
    pub lpCompBuffInfo: *mut DDCOMPBUFFERINFO,
    pub ddRVal: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for DD_GETMOCOMPCOMPBUFFDATA {}
impl ::core::clone::Clone for DD_GETMOCOMPCOMPBUFFDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DD_GETMOCOMPCOMPBUFFDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DD_GETMOCOMPCOMPBUFFDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_GETMOCOMPFORMATSDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpGuid: *mut ::windows::core::GUID,
    pub dwNumFormats: u32,
    pub lpFormats: *mut DDPIXELFORMAT,
    pub ddRVal: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for DD_GETMOCOMPFORMATSDATA {}
impl ::core::clone::Clone for DD_GETMOCOMPFORMATSDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_GETMOCOMPFORMATSDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETMOCOMPFORMATSDATA").field("lpDD", &self.lpDD).field("lpGuid", &self.lpGuid).field("dwNumFormats", &self.dwNumFormats).field("lpFormats", &self.lpFormats).field("ddRVal", &self.ddRVal).finish()
    }
}
impl ::windows::core::TypeKind for DD_GETMOCOMPFORMATSDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_GETMOCOMPFORMATSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpGuid == other.lpGuid && self.dwNumFormats == other.dwNumFormats && self.lpFormats == other.lpFormats && self.ddRVal == other.ddRVal
    }
}
impl ::core::cmp::Eq for DD_GETMOCOMPFORMATSDATA {}
impl ::core::default::Default for DD_GETMOCOMPFORMATSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_GETMOCOMPGUIDSDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub dwNumGuids: u32,
    pub lpGuids: *mut ::windows::core::GUID,
    pub ddRVal: ::windows::core::HRESULT,
}
impl ::core::marker::Copy for DD_GETMOCOMPGUIDSDATA {}
impl ::core::clone::Clone for DD_GETMOCOMPGUIDSDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_GETMOCOMPGUIDSDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETMOCOMPGUIDSDATA").field("lpDD", &self.lpDD).field("dwNumGuids", &self.dwNumGuids).field("lpGuids", &self.lpGuids).field("ddRVal", &self.ddRVal).finish()
    }
}
impl ::windows::core::TypeKind for DD_GETMOCOMPGUIDSDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_GETMOCOMPGUIDSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwNumGuids == other.dwNumGuids && self.lpGuids == other.lpGuids && self.ddRVal == other.ddRVal
    }
}
impl ::core::cmp::Eq for DD_GETMOCOMPGUIDSDATA {}
impl ::core::default::Default for DD_GETMOCOMPGUIDSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_GETSCANLINEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub dwScanLine: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetScanLine: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DD_GETSCANLINEDATA {}
impl ::core::clone::Clone for DD_GETSCANLINEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_GETSCANLINEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETSCANLINEDATA").field("lpDD", &self.lpDD).field("dwScanLine", &self.dwScanLine).field("ddRVal", &self.ddRVal).field("GetScanLine", &self.GetScanLine).finish()
    }
}
impl ::windows::core::TypeKind for DD_GETSCANLINEDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_GETSCANLINEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwScanLine == other.dwScanLine && self.ddRVal == other.ddRVal && self.GetScanLine == other.GetScanLine
    }
}
impl ::core::cmp::Eq for DD_GETSCANLINEDATA {}
impl ::core::default::Default for DD_GETSCANLINEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_GETVPORTBANDWIDTHDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub lpddpfFormat: *mut DDPIXELFORMAT,
    pub dwWidth: u32,
    pub dwHeight: u32,
    pub dwFlags: u32,
    pub lpBandwidth: *mut DDVIDEOPORTBANDWIDTH,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetVideoPortBandwidth: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_GETVPORTBANDWIDTHDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_GETVPORTBANDWIDTHDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_GETVPORTBANDWIDTHDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETVPORTBANDWIDTHDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("lpddpfFormat", &self.lpddpfFormat).field("dwWidth", &self.dwWidth).field("dwHeight", &self.dwHeight).field("dwFlags", &self.dwFlags).field("lpBandwidth", &self.lpBandwidth).field("ddRVal", &self.ddRVal).field("GetVideoPortBandwidth", &self.GetVideoPortBandwidth).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_GETVPORTBANDWIDTHDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_GETVPORTBANDWIDTHDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.lpddpfFormat == other.lpddpfFormat && self.dwWidth == other.dwWidth && self.dwHeight == other.dwHeight && self.dwFlags == other.dwFlags && self.lpBandwidth == other.lpBandwidth && self.ddRVal == other.ddRVal && self.GetVideoPortBandwidth == other.GetVideoPortBandwidth
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_GETVPORTBANDWIDTHDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_GETVPORTBANDWIDTHDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_GETVPORTCONNECTDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub dwPortId: u32,
    pub lpConnect: *mut DDVIDEOPORTCONNECT,
    pub dwNumEntries: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetVideoPortConnectInfo: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DD_GETVPORTCONNECTDATA {}
impl ::core::clone::Clone for DD_GETVPORTCONNECTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_GETVPORTCONNECTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETVPORTCONNECTDATA").field("lpDD", &self.lpDD).field("dwPortId", &self.dwPortId).field("lpConnect", &self.lpConnect).field("dwNumEntries", &self.dwNumEntries).field("ddRVal", &self.ddRVal).field("GetVideoPortConnectInfo", &self.GetVideoPortConnectInfo).finish()
    }
}
impl ::windows::core::TypeKind for DD_GETVPORTCONNECTDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_GETVPORTCONNECTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwPortId == other.dwPortId && self.lpConnect == other.lpConnect && self.dwNumEntries == other.dwNumEntries && self.ddRVal == other.ddRVal && self.GetVideoPortConnectInfo == other.GetVideoPortConnectInfo
    }
}
impl ::core::cmp::Eq for DD_GETVPORTCONNECTDATA {}
impl ::core::default::Default for DD_GETVPORTCONNECTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_GETVPORTFIELDDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub bField: super::super::Foundation::BOOL,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetVideoPortField: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_GETVPORTFIELDDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_GETVPORTFIELDDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_GETVPORTFIELDDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETVPORTFIELDDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("bField", &self.bField).field("ddRVal", &self.ddRVal).field("GetVideoPortField", &self.GetVideoPortField).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_GETVPORTFIELDDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_GETVPORTFIELDDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.bField == other.bField && self.ddRVal == other.ddRVal && self.GetVideoPortField == other.GetVideoPortField
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_GETVPORTFIELDDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_GETVPORTFIELDDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_GETVPORTFLIPSTATUSDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub fpSurface: usize,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetVideoPortFlipStatus: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DD_GETVPORTFLIPSTATUSDATA {}
impl ::core::clone::Clone for DD_GETVPORTFLIPSTATUSDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_GETVPORTFLIPSTATUSDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETVPORTFLIPSTATUSDATA").field("lpDD", &self.lpDD).field("fpSurface", &self.fpSurface).field("ddRVal", &self.ddRVal).field("GetVideoPortFlipStatus", &self.GetVideoPortFlipStatus).finish()
    }
}
impl ::windows::core::TypeKind for DD_GETVPORTFLIPSTATUSDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_GETVPORTFLIPSTATUSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.fpSurface == other.fpSurface && self.ddRVal == other.ddRVal && self.GetVideoPortFlipStatus == other.GetVideoPortFlipStatus
    }
}
impl ::core::cmp::Eq for DD_GETVPORTFLIPSTATUSDATA {}
impl ::core::default::Default for DD_GETVPORTFLIPSTATUSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_GETVPORTINPUTFORMATDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub dwFlags: u32,
    pub lpddpfFormat: *mut DDPIXELFORMAT,
    pub dwNumFormats: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetVideoPortInputFormats: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_GETVPORTINPUTFORMATDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_GETVPORTINPUTFORMATDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_GETVPORTINPUTFORMATDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETVPORTINPUTFORMATDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwFlags", &self.dwFlags).field("lpddpfFormat", &self.lpddpfFormat).field("dwNumFormats", &self.dwNumFormats).field("ddRVal", &self.ddRVal).field("GetVideoPortInputFormats", &self.GetVideoPortInputFormats).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_GETVPORTINPUTFORMATDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_GETVPORTINPUTFORMATDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwFlags == other.dwFlags && self.lpddpfFormat == other.lpddpfFormat && self.dwNumFormats == other.dwNumFormats && self.ddRVal == other.ddRVal && self.GetVideoPortInputFormats == other.GetVideoPortInputFormats
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_GETVPORTINPUTFORMATDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_GETVPORTINPUTFORMATDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_GETVPORTLINEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub dwLine: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetVideoPortLine: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_GETVPORTLINEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_GETVPORTLINEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_GETVPORTLINEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETVPORTLINEDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwLine", &self.dwLine).field("ddRVal", &self.ddRVal).field("GetVideoPortLine", &self.GetVideoPortLine).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_GETVPORTLINEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_GETVPORTLINEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwLine == other.dwLine && self.ddRVal == other.ddRVal && self.GetVideoPortLine == other.GetVideoPortLine
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_GETVPORTLINEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_GETVPORTLINEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_GETVPORTOUTPUTFORMATDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub dwFlags: u32,
    pub lpddpfInputFormat: *mut DDPIXELFORMAT,
    pub lpddpfOutputFormats: *mut DDPIXELFORMAT,
    pub dwNumFormats: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetVideoPortInputFormats: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_GETVPORTOUTPUTFORMATDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_GETVPORTOUTPUTFORMATDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_GETVPORTOUTPUTFORMATDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETVPORTOUTPUTFORMATDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwFlags", &self.dwFlags).field("lpddpfInputFormat", &self.lpddpfInputFormat).field("lpddpfOutputFormats", &self.lpddpfOutputFormats).field("dwNumFormats", &self.dwNumFormats).field("ddRVal", &self.ddRVal).field("GetVideoPortInputFormats", &self.GetVideoPortInputFormats).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_GETVPORTOUTPUTFORMATDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_GETVPORTOUTPUTFORMATDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwFlags == other.dwFlags && self.lpddpfInputFormat == other.lpddpfInputFormat && self.lpddpfOutputFormats == other.lpddpfOutputFormats && self.dwNumFormats == other.dwNumFormats && self.ddRVal == other.ddRVal && self.GetVideoPortInputFormats == other.GetVideoPortInputFormats
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_GETVPORTOUTPUTFORMATDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_GETVPORTOUTPUTFORMATDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_GETVPORTSIGNALDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub dwStatus: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub GetVideoSignalStatus: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_GETVPORTSIGNALDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_GETVPORTSIGNALDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_GETVPORTSIGNALDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_GETVPORTSIGNALDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwStatus", &self.dwStatus).field("ddRVal", &self.ddRVal).field("GetVideoSignalStatus", &self.GetVideoSignalStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_GETVPORTSIGNALDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_GETVPORTSIGNALDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwStatus == other.dwStatus && self.ddRVal == other.ddRVal && self.GetVideoSignalStatus == other.GetVideoSignalStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_GETVPORTSIGNALDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_GETVPORTSIGNALDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_HALINFO {
    pub dwSize: u32,
    pub vmiData: VIDEOMEMORYINFO,
    pub ddCaps: DDNTCORECAPS,
    pub GetDriverInfo: PDD_GETDRIVERINFO,
    pub dwFlags: u32,
    pub lpD3DGlobalDriverData: *mut ::core::ffi::c_void,
    pub lpD3DHALCallbacks: *mut ::core::ffi::c_void,
    pub lpD3DBufCallbacks: *mut DD_D3DBUFCALLBACKS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_HALINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_HALINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_HALINFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_HALINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_HALINFO_V4 {
    pub dwSize: u32,
    pub vmiData: VIDEOMEMORYINFO,
    pub ddCaps: DDNTCORECAPS,
    pub GetDriverInfo: PDD_GETDRIVERINFO,
    pub dwFlags: u32,
}
impl ::core::marker::Copy for DD_HALINFO_V4 {}
impl ::core::clone::Clone for DD_HALINFO_V4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DD_HALINFO_V4 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DD_HALINFO_V4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_KERNELCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub SyncSurfaceData: PDD_KERNELCB_SYNCSURFACE,
    pub SyncVideoPortData: PDD_KERNELCB_SYNCVIDEOPORT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_KERNELCALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_KERNELCALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_KERNELCALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_KERNELCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_KERNELCALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_KERNELCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_LOCKDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub bHasRect: u32,
    pub rArea: super::super::Foundation::RECTL,
    pub lpSurfData: *mut ::core::ffi::c_void,
    pub ddRVal: ::windows::core::HRESULT,
    pub Lock: *mut ::core::ffi::c_void,
    pub dwFlags: u32,
    pub fpProcess: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_LOCKDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_LOCKDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_LOCKDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_LOCKDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("bHasRect", &self.bHasRect).field("rArea", &self.rArea).field("lpSurfData", &self.lpSurfData).field("ddRVal", &self.ddRVal).field("Lock", &self.Lock).field("dwFlags", &self.dwFlags).field("fpProcess", &self.fpProcess).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_LOCKDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_LOCKDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.bHasRect == other.bHasRect && self.rArea == other.rArea && self.lpSurfData == other.lpSurfData && self.ddRVal == other.ddRVal && self.Lock == other.Lock && self.dwFlags == other.dwFlags && self.fpProcess == other.fpProcess
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_LOCKDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_LOCKDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_MAPMEMORYDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub bMap: super::super::Foundation::BOOL,
    pub hProcess: super::super::Foundation::HANDLE,
    pub fpProcess: usize,
    pub ddRVal: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_MAPMEMORYDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_MAPMEMORYDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_MAPMEMORYDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_MAPMEMORYDATA").field("lpDD", &self.lpDD).field("bMap", &self.bMap).field("hProcess", &self.hProcess).field("fpProcess", &self.fpProcess).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_MAPMEMORYDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_MAPMEMORYDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.bMap == other.bMap && self.hProcess == other.hProcess && self.fpProcess == other.fpProcess && self.ddRVal == other.ddRVal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_MAPMEMORYDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_MAPMEMORYDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_MISCELLANEOUS2CALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub AlphaBlt: PDD_ALPHABLT,
    pub CreateSurfaceEx: PDD_CREATESURFACEEX,
    pub GetDriverState: PDD_GETDRIVERSTATE,
    pub DestroyDDLocal: PDD_DESTROYDDLOCAL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_MISCELLANEOUS2CALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_MISCELLANEOUS2CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_MISCELLANEOUS2CALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_MISCELLANEOUS2CALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_MISCELLANEOUS2CALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_MISCELLANEOUS2CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_MISCELLANEOUSCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub GetAvailDriverMemory: PDD_GETAVAILDRIVERMEMORY,
}
impl ::core::marker::Copy for DD_MISCELLANEOUSCALLBACKS {}
impl ::core::clone::Clone for DD_MISCELLANEOUSCALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_MISCELLANEOUSCALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_MISCELLANEOUSCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::windows::core::TypeKind for DD_MISCELLANEOUSCALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DD_MISCELLANEOUSCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_MORECAPS {
    pub dwSize: u32,
    pub dwAlphaCaps: u32,
    pub dwSVBAlphaCaps: u32,
    pub dwVSBAlphaCaps: u32,
    pub dwSSBAlphaCaps: u32,
    pub dwFilterCaps: u32,
    pub dwSVBFilterCaps: u32,
    pub dwVSBFilterCaps: u32,
    pub dwSSBFilterCaps: u32,
}
impl ::core::marker::Copy for DD_MORECAPS {}
impl ::core::clone::Clone for DD_MORECAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_MORECAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_MORECAPS").field("dwSize", &self.dwSize).field("dwAlphaCaps", &self.dwAlphaCaps).field("dwSVBAlphaCaps", &self.dwSVBAlphaCaps).field("dwVSBAlphaCaps", &self.dwVSBAlphaCaps).field("dwSSBAlphaCaps", &self.dwSSBAlphaCaps).field("dwFilterCaps", &self.dwFilterCaps).field("dwSVBFilterCaps", &self.dwSVBFilterCaps).field("dwVSBFilterCaps", &self.dwVSBFilterCaps).field("dwSSBFilterCaps", &self.dwSSBFilterCaps).finish()
    }
}
impl ::windows::core::TypeKind for DD_MORECAPS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_MORECAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwAlphaCaps == other.dwAlphaCaps && self.dwSVBAlphaCaps == other.dwSVBAlphaCaps && self.dwVSBAlphaCaps == other.dwVSBAlphaCaps && self.dwSSBAlphaCaps == other.dwSSBAlphaCaps && self.dwFilterCaps == other.dwFilterCaps && self.dwSVBFilterCaps == other.dwSVBFilterCaps && self.dwVSBFilterCaps == other.dwVSBFilterCaps && self.dwSSBFilterCaps == other.dwSSBFilterCaps
    }
}
impl ::core::cmp::Eq for DD_MORECAPS {}
impl ::core::default::Default for DD_MORECAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_MORESURFACECAPS {
    pub dwSize: u32,
    pub ddsCapsMore: DDSCAPSEX,
    pub ddsExtendedHeapRestrictions: [DD_MORESURFACECAPS_0; 1],
}
impl ::core::marker::Copy for DD_MORESURFACECAPS {}
impl ::core::clone::Clone for DD_MORESURFACECAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DD_MORESURFACECAPS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DD_MORESURFACECAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_MORESURFACECAPS_0 {
    pub ddsCapsEx: DDSCAPSEX,
    pub ddsCapsExAlt: DDSCAPSEX,
}
impl ::core::marker::Copy for DD_MORESURFACECAPS_0 {}
impl ::core::clone::Clone for DD_MORESURFACECAPS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DD_MORESURFACECAPS_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DD_MORESURFACECAPS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_MOTIONCOMPCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub GetMoCompGuids: PDD_MOCOMPCB_GETGUIDS,
    pub GetMoCompFormats: PDD_MOCOMPCB_GETFORMATS,
    pub CreateMoComp: PDD_MOCOMPCB_CREATE,
    pub GetMoCompBuffInfo: PDD_MOCOMPCB_GETCOMPBUFFINFO,
    pub GetInternalMoCompInfo: PDD_MOCOMPCB_GETINTERNALINFO,
    pub BeginMoCompFrame: PDD_MOCOMPCB_BEGINFRAME,
    pub EndMoCompFrame: PDD_MOCOMPCB_ENDFRAME,
    pub RenderMoComp: PDD_MOCOMPCB_RENDER,
    pub QueryMoCompStatus: PDD_MOCOMPCB_QUERYSTATUS,
    pub DestroyMoComp: PDD_MOCOMPCB_DESTROY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_MOTIONCOMPCALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_MOTIONCOMPCALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_MOTIONCOMPCALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_MOTIONCOMPCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_MOTIONCOMPCALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_MOTIONCOMPCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_MOTIONCOMP_LOCAL {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub guid: ::windows::core::GUID,
    pub dwUncompWidth: u32,
    pub dwUncompHeight: u32,
    pub ddUncompPixelFormat: DDPIXELFORMAT,
    pub dwDriverReserved1: u32,
    pub dwDriverReserved2: u32,
    pub dwDriverReserved3: u32,
    pub lpDriverReserved1: *mut ::core::ffi::c_void,
    pub lpDriverReserved2: *mut ::core::ffi::c_void,
    pub lpDriverReserved3: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DD_MOTIONCOMP_LOCAL {}
impl ::core::clone::Clone for DD_MOTIONCOMP_LOCAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DD_MOTIONCOMP_LOCAL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DD_MOTIONCOMP_LOCAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_NONLOCALVIDMEMCAPS {
    pub dwSize: u32,
    pub dwNLVBCaps: u32,
    pub dwNLVBCaps2: u32,
    pub dwNLVBCKeyCaps: u32,
    pub dwNLVBFXCaps: u32,
    pub dwNLVBRops: [u32; 8],
}
impl ::core::marker::Copy for DD_NONLOCALVIDMEMCAPS {}
impl ::core::clone::Clone for DD_NONLOCALVIDMEMCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_NONLOCALVIDMEMCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_NONLOCALVIDMEMCAPS").field("dwSize", &self.dwSize).field("dwNLVBCaps", &self.dwNLVBCaps).field("dwNLVBCaps2", &self.dwNLVBCaps2).field("dwNLVBCKeyCaps", &self.dwNLVBCKeyCaps).field("dwNLVBFXCaps", &self.dwNLVBFXCaps).field("dwNLVBRops", &self.dwNLVBRops).finish()
    }
}
impl ::windows::core::TypeKind for DD_NONLOCALVIDMEMCAPS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_NONLOCALVIDMEMCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwNLVBCaps == other.dwNLVBCaps && self.dwNLVBCaps2 == other.dwNLVBCaps2 && self.dwNLVBCKeyCaps == other.dwNLVBCKeyCaps && self.dwNLVBFXCaps == other.dwNLVBFXCaps && self.dwNLVBRops == other.dwNLVBRops
    }
}
impl ::core::cmp::Eq for DD_NONLOCALVIDMEMCAPS {}
impl ::core::default::Default for DD_NONLOCALVIDMEMCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_NTCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub FreeDriverMemory: PDD_FREEDRIVERMEMORY,
    pub SetExclusiveMode: PDD_SETEXCLUSIVEMODE,
    pub FlipToGDISurface: PDD_FLIPTOGDISURFACE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_NTCALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_NTCALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_NTCALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_NTCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_NTCALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_NTCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_NTPRIVATEDRIVERCAPS {
    pub dwSize: u32,
    pub dwPrivateCaps: u32,
}
impl ::core::marker::Copy for DD_NTPRIVATEDRIVERCAPS {}
impl ::core::clone::Clone for DD_NTPRIVATEDRIVERCAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_NTPRIVATEDRIVERCAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_NTPRIVATEDRIVERCAPS").field("dwSize", &self.dwSize).field("dwPrivateCaps", &self.dwPrivateCaps).finish()
    }
}
impl ::windows::core::TypeKind for DD_NTPRIVATEDRIVERCAPS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_NTPRIVATEDRIVERCAPS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwPrivateCaps == other.dwPrivateCaps
    }
}
impl ::core::cmp::Eq for DD_NTPRIVATEDRIVERCAPS {}
impl ::core::default::Default for DD_NTPRIVATEDRIVERCAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct DD_PALETTECALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub DestroyPalette: PDD_PALCB_DESTROYPALETTE,
    pub SetEntries: PDD_PALCB_SETENTRIES,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for DD_PALETTECALLBACKS {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for DD_PALETTECALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for DD_PALETTECALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_PALETTECALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows::core::TypeKind for DD_PALETTECALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for DD_PALETTECALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_PALETTE_GLOBAL {
    pub dwReserved1: usize,
}
impl ::core::marker::Copy for DD_PALETTE_GLOBAL {}
impl ::core::clone::Clone for DD_PALETTE_GLOBAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_PALETTE_GLOBAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_PALETTE_GLOBAL").field("dwReserved1", &self.dwReserved1).finish()
    }
}
impl ::windows::core::TypeKind for DD_PALETTE_GLOBAL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_PALETTE_GLOBAL {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved1 == other.dwReserved1
    }
}
impl ::core::cmp::Eq for DD_PALETTE_GLOBAL {}
impl ::core::default::Default for DD_PALETTE_GLOBAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_PALETTE_LOCAL {
    pub dwReserved0: u32,
    pub dwReserved1: usize,
}
impl ::core::marker::Copy for DD_PALETTE_LOCAL {}
impl ::core::clone::Clone for DD_PALETTE_LOCAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_PALETTE_LOCAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_PALETTE_LOCAL").field("dwReserved0", &self.dwReserved0).field("dwReserved1", &self.dwReserved1).finish()
    }
}
impl ::windows::core::TypeKind for DD_PALETTE_LOCAL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_PALETTE_LOCAL {
    fn eq(&self, other: &Self) -> bool {
        self.dwReserved0 == other.dwReserved0 && self.dwReserved1 == other.dwReserved1
    }
}
impl ::core::cmp::Eq for DD_PALETTE_LOCAL {}
impl ::core::default::Default for DD_PALETTE_LOCAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_QUERYMOCOMPSTATUSDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpMoComp: *mut DD_MOTIONCOMP_LOCAL,
    pub lpSurface: *mut DD_SURFACE_LOCAL,
    pub dwFlags: u32,
    pub ddRVal: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_QUERYMOCOMPSTATUSDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_QUERYMOCOMPSTATUSDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_QUERYMOCOMPSTATUSDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_QUERYMOCOMPSTATUSDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("lpSurface", &self.lpSurface).field("dwFlags", &self.dwFlags).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_QUERYMOCOMPSTATUSDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_QUERYMOCOMPSTATUSDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpMoComp == other.lpMoComp && self.lpSurface == other.lpSurface && self.dwFlags == other.dwFlags && self.ddRVal == other.ddRVal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_QUERYMOCOMPSTATUSDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_QUERYMOCOMPSTATUSDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_RENDERMOCOMPDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpMoComp: *mut DD_MOTIONCOMP_LOCAL,
    pub dwNumBuffers: u32,
    pub lpBufferInfo: *mut DDMOCOMPBUFFERINFO,
    pub dwFunction: u32,
    pub lpInputData: *mut ::core::ffi::c_void,
    pub dwInputDataSize: u32,
    pub lpOutputData: *mut ::core::ffi::c_void,
    pub dwOutputDataSize: u32,
    pub ddRVal: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_RENDERMOCOMPDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_RENDERMOCOMPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_RENDERMOCOMPDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_RENDERMOCOMPDATA").field("lpDD", &self.lpDD).field("lpMoComp", &self.lpMoComp).field("dwNumBuffers", &self.dwNumBuffers).field("lpBufferInfo", &self.lpBufferInfo).field("dwFunction", &self.dwFunction).field("lpInputData", &self.lpInputData).field("dwInputDataSize", &self.dwInputDataSize).field("lpOutputData", &self.lpOutputData).field("dwOutputDataSize", &self.dwOutputDataSize).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_RENDERMOCOMPDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_RENDERMOCOMPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpMoComp == other.lpMoComp && self.dwNumBuffers == other.dwNumBuffers && self.lpBufferInfo == other.lpBufferInfo && self.dwFunction == other.dwFunction && self.lpInputData == other.lpInputData && self.dwInputDataSize == other.dwInputDataSize && self.lpOutputData == other.lpOutputData && self.dwOutputDataSize == other.dwOutputDataSize && self.ddRVal == other.ddRVal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_RENDERMOCOMPDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_RENDERMOCOMPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_SETCLIPLISTDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub ddRVal: ::windows::core::HRESULT,
    pub SetClipList: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_SETCLIPLISTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_SETCLIPLISTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_SETCLIPLISTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_SETCLIPLISTDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("ddRVal", &self.ddRVal).field("SetClipList", &self.SetClipList).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_SETCLIPLISTDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_SETCLIPLISTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.ddRVal == other.ddRVal && self.SetClipList == other.SetClipList
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_SETCLIPLISTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SETCLIPLISTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_SETCOLORKEYDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub dwFlags: u32,
    pub ckNew: DDCOLORKEY,
    pub ddRVal: ::windows::core::HRESULT,
    pub SetColorKey: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_SETCOLORKEYDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_SETCOLORKEYDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_SETCOLORKEYDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_SETCOLORKEYDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("dwFlags", &self.dwFlags).field("ckNew", &self.ckNew).field("ddRVal", &self.ddRVal).field("SetColorKey", &self.SetColorKey).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_SETCOLORKEYDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_SETCOLORKEYDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.dwFlags == other.dwFlags && self.ckNew == other.ckNew && self.ddRVal == other.ddRVal && self.SetColorKey == other.SetColorKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_SETCOLORKEYDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SETCOLORKEYDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub struct DD_SETENTRIESDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDPalette: *mut DD_PALETTE_GLOBAL,
    pub dwBase: u32,
    pub dwNumEntries: u32,
    pub lpEntries: *mut super::Gdi::PALETTEENTRY,
    pub ddRVal: ::windows::core::HRESULT,
    pub SetEntries: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::marker::Copy for DD_SETENTRIESDATA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::clone::Clone for DD_SETENTRIESDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::fmt::Debug for DD_SETENTRIESDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_SETENTRIESDATA").field("lpDD", &self.lpDD).field("lpDDPalette", &self.lpDDPalette).field("dwBase", &self.dwBase).field("dwNumEntries", &self.dwNumEntries).field("lpEntries", &self.lpEntries).field("ddRVal", &self.ddRVal).field("SetEntries", &self.SetEntries).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows::core::TypeKind for DD_SETENTRIESDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::PartialEq for DD_SETENTRIESDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDPalette == other.lpDDPalette && self.dwBase == other.dwBase && self.dwNumEntries == other.dwNumEntries && self.lpEntries == other.lpEntries && self.ddRVal == other.ddRVal && self.SetEntries == other.SetEntries
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::cmp::Eq for DD_SETENTRIESDATA {}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::core::default::Default for DD_SETENTRIESDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_SETEXCLUSIVEMODEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub dwEnterExcl: u32,
    pub dwReserved: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub SetExclusiveMode: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DD_SETEXCLUSIVEMODEDATA {}
impl ::core::clone::Clone for DD_SETEXCLUSIVEMODEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_SETEXCLUSIVEMODEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_SETEXCLUSIVEMODEDATA").field("lpDD", &self.lpDD).field("dwEnterExcl", &self.dwEnterExcl).field("dwReserved", &self.dwReserved).field("ddRVal", &self.ddRVal).field("SetExclusiveMode", &self.SetExclusiveMode).finish()
    }
}
impl ::windows::core::TypeKind for DD_SETEXCLUSIVEMODEDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_SETEXCLUSIVEMODEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwEnterExcl == other.dwEnterExcl && self.dwReserved == other.dwReserved && self.ddRVal == other.ddRVal && self.SetExclusiveMode == other.SetExclusiveMode
    }
}
impl ::core::cmp::Eq for DD_SETEXCLUSIVEMODEDATA {}
impl ::core::default::Default for DD_SETEXCLUSIVEMODEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_SETOVERLAYPOSITIONDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSrcSurface: *mut DD_SURFACE_LOCAL,
    pub lpDDDestSurface: *mut DD_SURFACE_LOCAL,
    pub lXPos: i32,
    pub lYPos: i32,
    pub ddRVal: ::windows::core::HRESULT,
    pub SetOverlayPosition: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_SETOVERLAYPOSITIONDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_SETOVERLAYPOSITIONDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_SETOVERLAYPOSITIONDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_SETOVERLAYPOSITIONDATA").field("lpDD", &self.lpDD).field("lpDDSrcSurface", &self.lpDDSrcSurface).field("lpDDDestSurface", &self.lpDDDestSurface).field("lXPos", &self.lXPos).field("lYPos", &self.lYPos).field("ddRVal", &self.ddRVal).field("SetOverlayPosition", &self.SetOverlayPosition).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_SETOVERLAYPOSITIONDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_SETOVERLAYPOSITIONDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSrcSurface == other.lpDDSrcSurface && self.lpDDDestSurface == other.lpDDDestSurface && self.lXPos == other.lXPos && self.lYPos == other.lYPos && self.ddRVal == other.ddRVal && self.SetOverlayPosition == other.SetOverlayPosition
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_SETOVERLAYPOSITIONDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SETOVERLAYPOSITIONDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_SETPALETTEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub lpDDPalette: *mut DD_PALETTE_GLOBAL,
    pub ddRVal: ::windows::core::HRESULT,
    pub SetPalette: *mut ::core::ffi::c_void,
    pub Attach: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_SETPALETTEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_SETPALETTEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_SETPALETTEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_SETPALETTEDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("lpDDPalette", &self.lpDDPalette).field("ddRVal", &self.ddRVal).field("SetPalette", &self.SetPalette).field("Attach", &self.Attach).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_SETPALETTEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_SETPALETTEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.lpDDPalette == other.lpDDPalette && self.ddRVal == other.ddRVal && self.SetPalette == other.SetPalette && self.Attach == other.Attach
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_SETPALETTEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SETPALETTEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_STEREOMODE {
    pub dwSize: u32,
    pub dwHeight: u32,
    pub dwWidth: u32,
    pub dwBpp: u32,
    pub dwRefreshRate: u32,
    pub bSupported: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_STEREOMODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_STEREOMODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_STEREOMODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_STEREOMODE").field("dwSize", &self.dwSize).field("dwHeight", &self.dwHeight).field("dwWidth", &self.dwWidth).field("dwBpp", &self.dwBpp).field("dwRefreshRate", &self.dwRefreshRate).field("bSupported", &self.bSupported).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_STEREOMODE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_STEREOMODE {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwHeight == other.dwHeight && self.dwWidth == other.dwWidth && self.dwBpp == other.dwBpp && self.dwRefreshRate == other.dwRefreshRate && self.bSupported == other.bSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_STEREOMODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_STEREOMODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_SURFACECALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub DestroySurface: PDD_SURFCB_DESTROYSURFACE,
    pub Flip: PDD_SURFCB_FLIP,
    pub SetClipList: PDD_SURFCB_SETCLIPLIST,
    pub Lock: PDD_SURFCB_LOCK,
    pub Unlock: PDD_SURFCB_UNLOCK,
    pub Blt: PDD_SURFCB_BLT,
    pub SetColorKey: PDD_SURFCB_SETCOLORKEY,
    pub AddAttachedSurface: PDD_SURFCB_ADDATTACHEDSURFACE,
    pub GetBltStatus: PDD_SURFCB_GETBLTSTATUS,
    pub GetFlipStatus: PDD_SURFCB_GETFLIPSTATUS,
    pub UpdateOverlay: PDD_SURFCB_UPDATEOVERLAY,
    pub SetOverlayPosition: PDD_SURFCB_SETOVERLAYPOSITION,
    pub reserved4: *mut ::core::ffi::c_void,
    pub SetPalette: PDD_SURFCB_SETPALETTE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_SURFACECALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_SURFACECALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_SURFACECALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_SURFACECALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("reserved4", &self.reserved4).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_SURFACECALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SURFACECALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_SURFACE_GLOBAL {
    pub Anonymous1: DD_SURFACE_GLOBAL_0,
    pub Anonymous2: DD_SURFACE_GLOBAL_1,
    pub fpVidMem: usize,
    pub Anonymous3: DD_SURFACE_GLOBAL_2,
    pub yHint: i32,
    pub xHint: i32,
    pub wHeight: u32,
    pub wWidth: u32,
    pub dwReserved1: usize,
    pub ddpfSurface: DDPIXELFORMAT,
    pub fpHeapOffset: usize,
    pub hCreatorProcess: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_SURFACE_GLOBAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_SURFACE_GLOBAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_SURFACE_GLOBAL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SURFACE_GLOBAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DD_SURFACE_GLOBAL_0 {
    pub dwBlockSizeY: u32,
    pub lSlicePitch: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_SURFACE_GLOBAL_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_SURFACE_GLOBAL_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_SURFACE_GLOBAL_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SURFACE_GLOBAL_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DD_SURFACE_GLOBAL_1 {
    pub lpVidMemHeap: *mut VIDEOMEMORY,
    pub dwBlockSizeX: u32,
    pub dwUserMemSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_SURFACE_GLOBAL_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_SURFACE_GLOBAL_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_SURFACE_GLOBAL_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SURFACE_GLOBAL_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DD_SURFACE_GLOBAL_2 {
    pub lPitch: i32,
    pub dwLinearSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_SURFACE_GLOBAL_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_SURFACE_GLOBAL_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_SURFACE_GLOBAL_2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SURFACE_GLOBAL_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_SURFACE_INT {
    pub lpLcl: *mut DD_SURFACE_LOCAL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_SURFACE_INT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_SURFACE_INT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_SURFACE_INT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_SURFACE_INT").field("lpLcl", &self.lpLcl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_SURFACE_INT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_SURFACE_INT {
    fn eq(&self, other: &Self) -> bool {
        self.lpLcl == other.lpLcl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_SURFACE_INT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SURFACE_INT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_SURFACE_LOCAL {
    pub lpGbl: *mut DD_SURFACE_GLOBAL,
    pub dwFlags: u32,
    pub ddsCaps: DDSCAPS,
    pub dwReserved1: usize,
    pub Anonymous1: DD_SURFACE_LOCAL_0,
    pub Anonymous2: DD_SURFACE_LOCAL_1,
    pub lpSurfMore: *mut DD_SURFACE_MORE,
    pub lpAttachList: *mut DD_ATTACHLIST,
    pub lpAttachListFrom: *mut DD_ATTACHLIST,
    pub rcOverlaySrc: super::super::Foundation::RECT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_SURFACE_LOCAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_SURFACE_LOCAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_SURFACE_LOCAL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SURFACE_LOCAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DD_SURFACE_LOCAL_0 {
    pub ddckCKSrcOverlay: DDCOLORKEY,
    pub ddckCKSrcBlt: DDCOLORKEY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_SURFACE_LOCAL_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_SURFACE_LOCAL_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_SURFACE_LOCAL_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SURFACE_LOCAL_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union DD_SURFACE_LOCAL_1 {
    pub ddckCKDestOverlay: DDCOLORKEY,
    pub ddckCKDestBlt: DDCOLORKEY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_SURFACE_LOCAL_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_SURFACE_LOCAL_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_SURFACE_LOCAL_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SURFACE_LOCAL_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_SURFACE_MORE {
    pub dwMipMapCount: u32,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub dwOverlayFlags: u32,
    pub ddsCapsEx: DDSCAPSEX,
    pub dwSurfaceHandle: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_SURFACE_MORE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_SURFACE_MORE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_SURFACE_MORE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SURFACE_MORE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_SYNCSURFACEDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub dwSurfaceOffset: u32,
    pub fpLockPtr: usize,
    pub lPitch: i32,
    pub dwOverlayOffset: u32,
    pub dwDriverReserved1: u32,
    pub dwDriverReserved2: u32,
    pub dwDriverReserved3: u32,
    pub dwDriverReserved4: u32,
    pub ddRVal: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_SYNCSURFACEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_SYNCSURFACEDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_SYNCSURFACEDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_SYNCSURFACEDATA")
            .field("lpDD", &self.lpDD)
            .field("lpDDSurface", &self.lpDDSurface)
            .field("dwSurfaceOffset", &self.dwSurfaceOffset)
            .field("fpLockPtr", &self.fpLockPtr)
            .field("lPitch", &self.lPitch)
            .field("dwOverlayOffset", &self.dwOverlayOffset)
            .field("dwDriverReserved1", &self.dwDriverReserved1)
            .field("dwDriverReserved2", &self.dwDriverReserved2)
            .field("dwDriverReserved3", &self.dwDriverReserved3)
            .field("dwDriverReserved4", &self.dwDriverReserved4)
            .field("ddRVal", &self.ddRVal)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_SYNCSURFACEDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_SYNCSURFACEDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.dwSurfaceOffset == other.dwSurfaceOffset && self.fpLockPtr == other.fpLockPtr && self.lPitch == other.lPitch && self.dwOverlayOffset == other.dwOverlayOffset && self.dwDriverReserved1 == other.dwDriverReserved1 && self.dwDriverReserved2 == other.dwDriverReserved2 && self.dwDriverReserved3 == other.dwDriverReserved3 && self.dwDriverReserved4 == other.dwDriverReserved4 && self.ddRVal == other.ddRVal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_SYNCSURFACEDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SYNCSURFACEDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_SYNCVIDEOPORTDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub dwOriginOffset: u32,
    pub dwHeight: u32,
    pub dwVBIHeight: u32,
    pub dwDriverReserved1: u32,
    pub dwDriverReserved2: u32,
    pub dwDriverReserved3: u32,
    pub ddRVal: ::windows::core::HRESULT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_SYNCVIDEOPORTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_SYNCVIDEOPORTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_SYNCVIDEOPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_SYNCVIDEOPORTDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwOriginOffset", &self.dwOriginOffset).field("dwHeight", &self.dwHeight).field("dwVBIHeight", &self.dwVBIHeight).field("dwDriverReserved1", &self.dwDriverReserved1).field("dwDriverReserved2", &self.dwDriverReserved2).field("dwDriverReserved3", &self.dwDriverReserved3).field("ddRVal", &self.ddRVal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_SYNCVIDEOPORTDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_SYNCVIDEOPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwOriginOffset == other.dwOriginOffset && self.dwHeight == other.dwHeight && self.dwVBIHeight == other.dwVBIHeight && self.dwDriverReserved1 == other.dwDriverReserved1 && self.dwDriverReserved2 == other.dwDriverReserved2 && self.dwDriverReserved3 == other.dwDriverReserved3 && self.ddRVal == other.ddRVal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_SYNCVIDEOPORTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_SYNCVIDEOPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_UNLOCKDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDSurface: *mut DD_SURFACE_LOCAL,
    pub ddRVal: ::windows::core::HRESULT,
    pub Unlock: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_UNLOCKDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_UNLOCKDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_UNLOCKDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_UNLOCKDATA").field("lpDD", &self.lpDD).field("lpDDSurface", &self.lpDDSurface).field("ddRVal", &self.ddRVal).field("Unlock", &self.Unlock).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_UNLOCKDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_UNLOCKDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpDDSurface == other.lpDDSurface && self.ddRVal == other.ddRVal && self.Unlock == other.Unlock
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_UNLOCKDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_UNLOCKDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_UPDATENONLOCALHEAPDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub dwHeap: u32,
    pub fpGARTLin: usize,
    pub fpGARTDev: usize,
    pub ulPolicyMaxBytes: usize,
    pub ddRVal: ::windows::core::HRESULT,
    pub UpdateNonLocalHeap: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DD_UPDATENONLOCALHEAPDATA {}
impl ::core::clone::Clone for DD_UPDATENONLOCALHEAPDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_UPDATENONLOCALHEAPDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_UPDATENONLOCALHEAPDATA").field("lpDD", &self.lpDD).field("dwHeap", &self.dwHeap).field("fpGARTLin", &self.fpGARTLin).field("fpGARTDev", &self.fpGARTDev).field("ulPolicyMaxBytes", &self.ulPolicyMaxBytes).field("ddRVal", &self.ddRVal).field("UpdateNonLocalHeap", &self.UpdateNonLocalHeap).finish()
    }
}
impl ::windows::core::TypeKind for DD_UPDATENONLOCALHEAPDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_UPDATENONLOCALHEAPDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwHeap == other.dwHeap && self.fpGARTLin == other.fpGARTLin && self.fpGARTDev == other.fpGARTDev && self.ulPolicyMaxBytes == other.ulPolicyMaxBytes && self.ddRVal == other.ddRVal && self.UpdateNonLocalHeap == other.UpdateNonLocalHeap
    }
}
impl ::core::cmp::Eq for DD_UPDATENONLOCALHEAPDATA {}
impl ::core::default::Default for DD_UPDATENONLOCALHEAPDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_UPDATEOVERLAYDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub lpDDDestSurface: *mut DD_SURFACE_LOCAL,
    pub rDest: super::super::Foundation::RECTL,
    pub lpDDSrcSurface: *mut DD_SURFACE_LOCAL,
    pub rSrc: super::super::Foundation::RECTL,
    pub dwFlags: u32,
    pub overlayFX: DDOVERLAYFX,
    pub ddRVal: ::windows::core::HRESULT,
    pub UpdateOverlay: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_UPDATEOVERLAYDATA {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_UPDATEOVERLAYDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_UPDATEOVERLAYDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_UPDATEVPORTDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub lplpDDSurface: *mut *mut DD_SURFACE_INT,
    pub lplpDDVBISurface: *mut *mut DD_SURFACE_INT,
    pub lpVideoInfo: *mut DDVIDEOPORTINFO,
    pub dwFlags: u32,
    pub dwNumAutoflip: u32,
    pub dwNumVBIAutoflip: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub UpdateVideoPort: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_UPDATEVPORTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_UPDATEVPORTDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_UPDATEVPORTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_UPDATEVPORTDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("lplpDDSurface", &self.lplpDDSurface).field("lplpDDVBISurface", &self.lplpDDVBISurface).field("lpVideoInfo", &self.lpVideoInfo).field("dwFlags", &self.dwFlags).field("dwNumAutoflip", &self.dwNumAutoflip).field("dwNumVBIAutoflip", &self.dwNumVBIAutoflip).field("ddRVal", &self.ddRVal).field("UpdateVideoPort", &self.UpdateVideoPort).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_UPDATEVPORTDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_UPDATEVPORTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.lplpDDSurface == other.lplpDDSurface && self.lplpDDVBISurface == other.lplpDDVBISurface && self.lpVideoInfo == other.lpVideoInfo && self.dwFlags == other.dwFlags && self.dwNumAutoflip == other.dwNumAutoflip && self.dwNumVBIAutoflip == other.dwNumVBIAutoflip && self.ddRVal == other.ddRVal && self.UpdateVideoPort == other.UpdateVideoPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_UPDATEVPORTDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_UPDATEVPORTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_VIDEOPORTCALLBACKS {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub CanCreateVideoPort: PDD_VPORTCB_CANCREATEVIDEOPORT,
    pub CreateVideoPort: PDD_VPORTCB_CREATEVIDEOPORT,
    pub FlipVideoPort: PDD_VPORTCB_FLIP,
    pub GetVideoPortBandwidth: PDD_VPORTCB_GETBANDWIDTH,
    pub GetVideoPortInputFormats: PDD_VPORTCB_GETINPUTFORMATS,
    pub GetVideoPortOutputFormats: PDD_VPORTCB_GETOUTPUTFORMATS,
    pub lpReserved1: *mut ::core::ffi::c_void,
    pub GetVideoPortField: PDD_VPORTCB_GETFIELD,
    pub GetVideoPortLine: PDD_VPORTCB_GETLINE,
    pub GetVideoPortConnectInfo: PDD_VPORTCB_GETVPORTCONNECT,
    pub DestroyVideoPort: PDD_VPORTCB_DESTROYVPORT,
    pub GetVideoPortFlipStatus: PDD_VPORTCB_GETFLIPSTATUS,
    pub UpdateVideoPort: PDD_VPORTCB_UPDATE,
    pub WaitForVideoPortSync: PDD_VPORTCB_WAITFORSYNC,
    pub GetVideoSignalStatus: PDD_VPORTCB_GETSIGNALSTATUS,
    pub ColorControl: PDD_VPORTCB_COLORCONTROL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_VIDEOPORTCALLBACKS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_VIDEOPORTCALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_VIDEOPORTCALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_VIDEOPORTCALLBACKS").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("lpReserved1", &self.lpReserved1).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_VIDEOPORTCALLBACKS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_VIDEOPORTCALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_VIDEOPORT_LOCAL {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub ddvpDesc: DDVIDEOPORTDESC,
    pub ddvpInfo: DDVIDEOPORTINFO,
    pub lpSurface: *mut DD_SURFACE_INT,
    pub lpVBISurface: *mut DD_SURFACE_INT,
    pub dwNumAutoflip: u32,
    pub dwNumVBIAutoflip: u32,
    pub dwReserved1: usize,
    pub dwReserved2: usize,
    pub dwReserved3: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_VIDEOPORT_LOCAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_VIDEOPORT_LOCAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_VIDEOPORT_LOCAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_VIDEOPORT_LOCAL").field("lpDD", &self.lpDD).field("ddvpDesc", &self.ddvpDesc).field("ddvpInfo", &self.ddvpInfo).field("lpSurface", &self.lpSurface).field("lpVBISurface", &self.lpVBISurface).field("dwNumAutoflip", &self.dwNumAutoflip).field("dwNumVBIAutoflip", &self.dwNumVBIAutoflip).field("dwReserved1", &self.dwReserved1).field("dwReserved2", &self.dwReserved2).field("dwReserved3", &self.dwReserved3).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_VIDEOPORT_LOCAL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_VIDEOPORT_LOCAL {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.ddvpDesc == other.ddvpDesc && self.ddvpInfo == other.ddvpInfo && self.lpSurface == other.lpSurface && self.lpVBISurface == other.lpVBISurface && self.dwNumAutoflip == other.dwNumAutoflip && self.dwNumVBIAutoflip == other.dwNumVBIAutoflip && self.dwReserved1 == other.dwReserved1 && self.dwReserved2 == other.dwReserved2 && self.dwReserved3 == other.dwReserved3
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_VIDEOPORT_LOCAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_VIDEOPORT_LOCAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_VPORTCOLORDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub dwFlags: u32,
    pub lpColorData: *mut DDCOLORCONTROL,
    pub ddRVal: ::windows::core::HRESULT,
    pub ColorControl: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_VPORTCOLORDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_VPORTCOLORDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_VPORTCOLORDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_VPORTCOLORDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwFlags", &self.dwFlags).field("lpColorData", &self.lpColorData).field("ddRVal", &self.ddRVal).field("ColorControl", &self.ColorControl).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_VPORTCOLORDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_VPORTCOLORDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwFlags == other.dwFlags && self.lpColorData == other.lpColorData && self.ddRVal == other.ddRVal && self.ColorControl == other.ColorControl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_VPORTCOLORDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_VPORTCOLORDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DD_WAITFORVERTICALBLANKDATA {
    pub lpDD: *mut DD_DIRECTDRAW_GLOBAL,
    pub dwFlags: u32,
    pub bIsInVB: u32,
    pub hEvent: usize,
    pub ddRVal: ::windows::core::HRESULT,
    pub WaitForVerticalBlank: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DD_WAITFORVERTICALBLANKDATA {}
impl ::core::clone::Clone for DD_WAITFORVERTICALBLANKDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DD_WAITFORVERTICALBLANKDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_WAITFORVERTICALBLANKDATA").field("lpDD", &self.lpDD).field("dwFlags", &self.dwFlags).field("bIsInVB", &self.bIsInVB).field("hEvent", &self.hEvent).field("ddRVal", &self.ddRVal).field("WaitForVerticalBlank", &self.WaitForVerticalBlank).finish()
    }
}
impl ::windows::core::TypeKind for DD_WAITFORVERTICALBLANKDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DD_WAITFORVERTICALBLANKDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.dwFlags == other.dwFlags && self.bIsInVB == other.bIsInVB && self.hEvent == other.hEvent && self.ddRVal == other.ddRVal && self.WaitForVerticalBlank == other.WaitForVerticalBlank
    }
}
impl ::core::cmp::Eq for DD_WAITFORVERTICALBLANKDATA {}
impl ::core::default::Default for DD_WAITFORVERTICALBLANKDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DD_WAITFORVPORTSYNCDATA {
    pub lpDD: *mut DD_DIRECTDRAW_LOCAL,
    pub lpVideoPort: *mut DD_VIDEOPORT_LOCAL,
    pub dwFlags: u32,
    pub dwLine: u32,
    pub dwTimeOut: u32,
    pub ddRVal: ::windows::core::HRESULT,
    pub UpdateVideoPort: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DD_WAITFORVPORTSYNCDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DD_WAITFORVPORTSYNCDATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DD_WAITFORVPORTSYNCDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DD_WAITFORVPORTSYNCDATA").field("lpDD", &self.lpDD).field("lpVideoPort", &self.lpVideoPort).field("dwFlags", &self.dwFlags).field("dwLine", &self.dwLine).field("dwTimeOut", &self.dwTimeOut).field("ddRVal", &self.ddRVal).field("UpdateVideoPort", &self.UpdateVideoPort).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DD_WAITFORVPORTSYNCDATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DD_WAITFORVPORTSYNCDATA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDD == other.lpDD && self.lpVideoPort == other.lpVideoPort && self.dwFlags == other.dwFlags && self.dwLine == other.dwLine && self.dwTimeOut == other.dwTimeOut && self.ddRVal == other.ddRVal && self.UpdateVideoPort == other.UpdateVideoPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DD_WAITFORVPORTSYNCDATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DD_WAITFORVPORTSYNCDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DXAPI_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut ::core::ffi::c_void,
    pub InterfaceReference: *mut ::core::ffi::c_void,
    pub InterfaceDereference: *mut ::core::ffi::c_void,
    pub DxGetIrqInfo: PDX_GETIRQINFO,
    pub DxEnableIrq: PDX_ENABLEIRQ,
    pub DxSkipNextField: PDX_SKIPNEXTFIELD,
    pub DxBobNextField: PDX_BOBNEXTFIELD,
    pub DxSetState: PDX_SETSTATE,
    pub DxLock: PDX_LOCK,
    pub DxFlipOverlay: PDX_FLIPOVERLAY,
    pub DxFlipVideoPort: PDX_FLIPVIDEOPORT,
    pub DxGetPolarity: PDX_GETPOLARITY,
    pub DxGetCurrentAutoflip: PDX_GETCURRENTAUTOFLIP,
    pub DxGetPreviousAutoflip: PDX_GETPREVIOUSAUTOFLIP,
    pub DxTransfer: PDX_TRANSFER,
    pub DxGetTransferStatus: PDX_GETTRANSFERSTATUS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DXAPI_INTERFACE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DXAPI_INTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXAPI_INTERFACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXAPI_INTERFACE").field("Size", &self.Size).field("Version", &self.Version).field("Context", &self.Context).field("InterfaceReference", &self.InterfaceReference).field("InterfaceDereference", &self.InterfaceDereference).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DXAPI_INTERFACE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXAPI_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct DX_IRQDATA {
    pub dwIrqFlags: u32,
}
impl ::core::marker::Copy for DX_IRQDATA {}
impl ::core::clone::Clone for DX_IRQDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DX_IRQDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DX_IRQDATA").field("dwIrqFlags", &self.dwIrqFlags).finish()
    }
}
impl ::windows::core::TypeKind for DX_IRQDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DX_IRQDATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwIrqFlags == other.dwIrqFlags
    }
}
impl ::core::cmp::Eq for DX_IRQDATA {}
impl ::core::default::Default for DX_IRQDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct HEAPALIAS {
    pub fpVidMem: usize,
    pub lpAlias: *mut ::core::ffi::c_void,
    pub dwAliasSize: u32,
}
impl ::core::marker::Copy for HEAPALIAS {}
impl ::core::clone::Clone for HEAPALIAS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HEAPALIAS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HEAPALIAS").field("fpVidMem", &self.fpVidMem).field("lpAlias", &self.lpAlias).field("dwAliasSize", &self.dwAliasSize).finish()
    }
}
impl ::windows::core::TypeKind for HEAPALIAS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HEAPALIAS {
    fn eq(&self, other: &Self) -> bool {
        self.fpVidMem == other.fpVidMem && self.lpAlias == other.lpAlias && self.dwAliasSize == other.dwAliasSize
    }
}
impl ::core::cmp::Eq for HEAPALIAS {}
impl ::core::default::Default for HEAPALIAS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct HEAPALIASINFO {
    pub dwRefCnt: u32,
    pub dwFlags: u32,
    pub dwNumHeaps: u32,
    pub lpAliases: *mut HEAPALIAS,
}
impl ::core::marker::Copy for HEAPALIASINFO {}
impl ::core::clone::Clone for HEAPALIASINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HEAPALIASINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HEAPALIASINFO").field("dwRefCnt", &self.dwRefCnt).field("dwFlags", &self.dwFlags).field("dwNumHeaps", &self.dwNumHeaps).field("lpAliases", &self.lpAliases).finish()
    }
}
impl ::windows::core::TypeKind for HEAPALIASINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HEAPALIASINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwRefCnt == other.dwRefCnt && self.dwFlags == other.dwFlags && self.dwNumHeaps == other.dwNumHeaps && self.lpAliases == other.lpAliases
    }
}
impl ::core::cmp::Eq for HEAPALIASINFO {}
impl ::core::default::Default for HEAPALIASINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct HEAPALIGNMENT {
    pub dwSize: u32,
    pub ddsCaps: DDSCAPS,
    pub dwReserved: u32,
    pub ExecuteBuffer: SURFACEALIGNMENT,
    pub Overlay: SURFACEALIGNMENT,
    pub Texture: SURFACEALIGNMENT,
    pub ZBuffer: SURFACEALIGNMENT,
    pub AlphaBuffer: SURFACEALIGNMENT,
    pub Offscreen: SURFACEALIGNMENT,
    pub FlipTarget: SURFACEALIGNMENT,
}
impl ::core::marker::Copy for HEAPALIGNMENT {}
impl ::core::clone::Clone for HEAPALIGNMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for HEAPALIGNMENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for HEAPALIGNMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct IUNKNOWN_LIST {
    pub lpLink: *mut IUNKNOWN_LIST,
    pub lpGuid: *mut ::windows::core::GUID,
    pub lpIUnknown: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
}
impl ::core::clone::Clone for IUNKNOWN_LIST {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for IUNKNOWN_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IUNKNOWN_LIST").field("lpLink", &self.lpLink).field("lpGuid", &self.lpGuid).field("lpIUnknown", &self.lpIUnknown).finish()
    }
}
impl ::windows::core::TypeKind for IUNKNOWN_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for IUNKNOWN_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.lpLink == other.lpLink && self.lpGuid == other.lpGuid && self.lpIUnknown == other.lpIUnknown
    }
}
impl ::core::cmp::Eq for IUNKNOWN_LIST {}
impl ::core::default::Default for IUNKNOWN_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct MDL {
    pub MdlNext: *mut MDL,
    pub MdlSize: i16,
    pub MdlFlags: i16,
    pub Process: *mut MDL_0,
    pub lpMappedSystemVa: *mut u32,
    pub lpStartVa: *mut u32,
    pub ByteCount: u32,
    pub ByteOffset: u32,
}
impl ::core::marker::Copy for MDL {}
impl ::core::clone::Clone for MDL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MDL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MDL").field("MdlNext", &self.MdlNext).field("MdlSize", &self.MdlSize).field("MdlFlags", &self.MdlFlags).field("Process", &self.Process).field("lpMappedSystemVa", &self.lpMappedSystemVa).field("lpStartVa", &self.lpStartVa).field("ByteCount", &self.ByteCount).field("ByteOffset", &self.ByteOffset).finish()
    }
}
impl ::windows::core::TypeKind for MDL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MDL {
    fn eq(&self, other: &Self) -> bool {
        self.MdlNext == other.MdlNext && self.MdlSize == other.MdlSize && self.MdlFlags == other.MdlFlags && self.Process == other.Process && self.lpMappedSystemVa == other.lpMappedSystemVa && self.lpStartVa == other.lpStartVa && self.ByteCount == other.ByteCount && self.ByteOffset == other.ByteOffset
    }
}
impl ::core::cmp::Eq for MDL {}
impl ::core::default::Default for MDL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MDL_0(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct PROCESS_LIST {
    pub lpLink: *mut PROCESS_LIST,
    pub dwProcessId: u32,
    pub dwRefCnt: u32,
    pub dwAlphaDepth: u32,
    pub dwZDepth: u32,
}
impl ::core::marker::Copy for PROCESS_LIST {}
impl ::core::clone::Clone for PROCESS_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_LIST").field("lpLink", &self.lpLink).field("dwProcessId", &self.dwProcessId).field("dwRefCnt", &self.dwRefCnt).field("dwAlphaDepth", &self.dwAlphaDepth).field("dwZDepth", &self.dwZDepth).finish()
    }
}
impl ::windows::core::TypeKind for PROCESS_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for PROCESS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.lpLink == other.lpLink && self.dwProcessId == other.dwProcessId && self.dwRefCnt == other.dwRefCnt && self.dwAlphaDepth == other.dwAlphaDepth && self.dwZDepth == other.dwZDepth
    }
}
impl ::core::cmp::Eq for PROCESS_LIST {}
impl ::core::default::Default for PROCESS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct SURFACEALIGNMENT {
    pub Anonymous: SURFACEALIGNMENT_0,
}
impl ::core::marker::Copy for SURFACEALIGNMENT {}
impl ::core::clone::Clone for SURFACEALIGNMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for SURFACEALIGNMENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SURFACEALIGNMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub union SURFACEALIGNMENT_0 {
    pub Linear: SURFACEALIGNMENT_0_0,
    pub Rectangular: SURFACEALIGNMENT_0_1,
}
impl ::core::marker::Copy for SURFACEALIGNMENT_0 {}
impl ::core::clone::Clone for SURFACEALIGNMENT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for SURFACEALIGNMENT_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for SURFACEALIGNMENT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct SURFACEALIGNMENT_0_0 {
    pub dwStartAlignment: u32,
    pub dwPitchAlignment: u32,
    pub dwFlags: u32,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for SURFACEALIGNMENT_0_0 {}
impl ::core::clone::Clone for SURFACEALIGNMENT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SURFACEALIGNMENT_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SURFACEALIGNMENT_0_0").field("dwStartAlignment", &self.dwStartAlignment).field("dwPitchAlignment", &self.dwPitchAlignment).field("dwFlags", &self.dwFlags).field("dwReserved2", &self.dwReserved2).finish()
    }
}
impl ::windows::core::TypeKind for SURFACEALIGNMENT_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SURFACEALIGNMENT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwStartAlignment == other.dwStartAlignment && self.dwPitchAlignment == other.dwPitchAlignment && self.dwFlags == other.dwFlags && self.dwReserved2 == other.dwReserved2
    }
}
impl ::core::cmp::Eq for SURFACEALIGNMENT_0_0 {}
impl ::core::default::Default for SURFACEALIGNMENT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct SURFACEALIGNMENT_0_1 {
    pub dwXAlignment: u32,
    pub dwYAlignment: u32,
    pub dwFlags: u32,
    pub dwReserved2: u32,
}
impl ::core::marker::Copy for SURFACEALIGNMENT_0_1 {}
impl ::core::clone::Clone for SURFACEALIGNMENT_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SURFACEALIGNMENT_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SURFACEALIGNMENT_0_1").field("dwXAlignment", &self.dwXAlignment).field("dwYAlignment", &self.dwYAlignment).field("dwFlags", &self.dwFlags).field("dwReserved2", &self.dwReserved2).finish()
    }
}
impl ::windows::core::TypeKind for SURFACEALIGNMENT_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SURFACEALIGNMENT_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwXAlignment == other.dwXAlignment && self.dwYAlignment == other.dwYAlignment && self.dwFlags == other.dwFlags && self.dwReserved2 == other.dwReserved2
    }
}
impl ::core::cmp::Eq for SURFACEALIGNMENT_0_1 {}
impl ::core::default::Default for SURFACEALIGNMENT_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VIDEOMEMORY {
    pub dwFlags: u32,
    pub fpStart: usize,
    pub Anonymous1: VIDEOMEMORY_0,
    pub ddsCaps: DDSCAPS,
    pub ddsCapsAlt: DDSCAPS,
    pub Anonymous2: VIDEOMEMORY_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VIDEOMEMORY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VIDEOMEMORY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VIDEOMEMORY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VIDEOMEMORY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union VIDEOMEMORY_0 {
    pub fpEnd: usize,
    pub dwWidth: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VIDEOMEMORY_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VIDEOMEMORY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VIDEOMEMORY_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VIDEOMEMORY_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union VIDEOMEMORY_1 {
    pub lpHeap: *mut VMEMHEAP,
    pub dwHeight: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VIDEOMEMORY_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VIDEOMEMORY_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VIDEOMEMORY_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VIDEOMEMORY_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub struct VIDEOMEMORYINFO {
    pub fpPrimary: usize,
    pub dwFlags: u32,
    pub dwDisplayWidth: u32,
    pub dwDisplayHeight: u32,
    pub lDisplayPitch: i32,
    pub ddpfDisplay: DDPIXELFORMAT,
    pub dwOffscreenAlign: u32,
    pub dwOverlayAlign: u32,
    pub dwTextureAlign: u32,
    pub dwZBufferAlign: u32,
    pub dwAlphaAlign: u32,
    pub pvPrimary: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for VIDEOMEMORYINFO {}
impl ::core::clone::Clone for VIDEOMEMORYINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for VIDEOMEMORYINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for VIDEOMEMORYINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VIDMEM {
    pub dwFlags: u32,
    pub fpStart: usize,
    pub Anonymous1: VIDMEM_0,
    pub ddsCaps: DDSCAPS,
    pub ddsCapsAlt: DDSCAPS,
    pub Anonymous2: VIDMEM_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VIDMEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VIDMEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VIDMEM {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VIDMEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union VIDMEM_0 {
    pub fpEnd: usize,
    pub dwWidth: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VIDMEM_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VIDMEM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VIDMEM_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VIDMEM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union VIDMEM_1 {
    pub lpHeap: *mut VMEMHEAP,
    pub dwHeight: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VIDMEM_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VIDMEM_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VIDMEM_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VIDMEM_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VIDMEMINFO {
    pub fpPrimary: usize,
    pub dwFlags: u32,
    pub dwDisplayWidth: u32,
    pub dwDisplayHeight: u32,
    pub lDisplayPitch: i32,
    pub ddpfDisplay: DDPIXELFORMAT,
    pub dwOffscreenAlign: u32,
    pub dwOverlayAlign: u32,
    pub dwTextureAlign: u32,
    pub dwZBufferAlign: u32,
    pub dwAlphaAlign: u32,
    pub dwNumHeaps: u32,
    pub pvmList: *mut VIDMEM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VIDMEMINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VIDMEMINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VIDMEMINFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VIDMEMINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VMEMHEAP {
    pub dwFlags: u32,
    pub stride: u32,
    pub freeList: *mut ::core::ffi::c_void,
    pub allocList: *mut ::core::ffi::c_void,
    pub dwTotalSize: u32,
    pub fpGARTLin: usize,
    pub fpGARTDev: usize,
    pub dwCommitedSize: u32,
    pub dwCoalesceCount: u32,
    pub Alignment: HEAPALIGNMENT,
    pub ddsCapsEx: DDSCAPSEX,
    pub ddsCapsExAlt: DDSCAPSEX,
    pub liPhysAGPBase: i64,
    pub hdevAGP: super::super::Foundation::HANDLE,
    pub pvPhysRsrv: *mut ::core::ffi::c_void,
    pub pAgpCommitMask: *mut u8,
    pub dwAgpCommitMaskSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VMEMHEAP {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VMEMHEAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VMEMHEAP {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VMEMHEAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VMEML {
    pub next: *mut VMEML,
    pub ptr: usize,
    pub size: u32,
    pub bDiscardable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VMEML {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VMEML {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VMEML {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VMEML").field("next", &self.next).field("ptr", &self.ptr).field("size", &self.size).field("bDiscardable", &self.bDiscardable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VMEML {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VMEML {
    fn eq(&self, other: &Self) -> bool {
        self.next == other.next && self.ptr == other.ptr && self.size == other.size && self.bDiscardable == other.bDiscardable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VMEML {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VMEML {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VMEMR {
    pub next: *mut VMEMR,
    pub prev: *mut VMEMR,
    pub pUp: *mut VMEMR,
    pub pDown: *mut VMEMR,
    pub pLeft: *mut VMEMR,
    pub pRight: *mut VMEMR,
    pub ptr: usize,
    pub size: u32,
    pub x: u32,
    pub y: u32,
    pub cx: u32,
    pub cy: u32,
    pub flags: u32,
    pub pBits: usize,
    pub bDiscardable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for VMEMR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VMEMR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VMEMR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VMEMR").field("next", &self.next).field("prev", &self.prev).field("pUp", &self.pUp).field("pDown", &self.pDown).field("pLeft", &self.pLeft).field("pRight", &self.pRight).field("ptr", &self.ptr).field("size", &self.size).field("x", &self.x).field("y", &self.y).field("cx", &self.cx).field("cy", &self.cy).field("flags", &self.flags).field("pBits", &self.pBits).field("bDiscardable", &self.bDiscardable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for VMEMR {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VMEMR {
    fn eq(&self, other: &Self) -> bool {
        self.next == other.next && self.prev == other.prev && self.pUp == other.pUp && self.pDown == other.pDown && self.pLeft == other.pLeft && self.pRight == other.pRight && self.ptr == other.ptr && self.size == other.size && self.x == other.x && self.y == other.y && self.cx == other.cx && self.cy == other.cy && self.flags == other.flags && self.pBits == other.pBits && self.bDiscardable == other.bDiscardable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VMEMR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VMEMR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct _DDFXROP(pub u8);
#[repr(C)]
pub struct _DD_DESTROYDRIVERDATA(pub u8);
#[repr(C)]
pub struct _DD_GETVPORTAUTOFLIPSURFACEDATA(pub u8);
#[repr(C)]
pub struct _DD_SETMODEDATA(pub u8);
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPCLIPPERCALLBACK = ::core::option::Option<unsafe extern "system" fn(lpddclipper: ::core::option::Option<IDirectDrawClipper>, hwnd: super::super::Foundation::HWND, code: u32, lpcontext: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type LPDD32BITDRIVERINIT = ::core::option::Option<unsafe extern "system" fn(dwcontext: u32) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDDENUMCALLBACKA = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::windows::core::GUID, param1: ::windows::core::PCSTR, param2: ::windows::core::PCSTR, param3: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDENUMCALLBACKEXA = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::windows::core::GUID, param1: ::windows::core::PCSTR, param2: ::windows::core::PCSTR, param3: *mut ::core::ffi::c_void, param4: super::Gdi::HMONITOR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDENUMCALLBACKEXW = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::windows::core::GUID, param1: ::windows::core::PCWSTR, param2: ::windows::core::PCWSTR, param3: *mut ::core::ffi::c_void, param4: super::Gdi::HMONITOR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPDDENUMCALLBACKW = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::windows::core::GUID, param1: ::windows::core::PCWSTR, param2: ::windows::core::PCWSTR, param3: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type LPDDENUMMODESCALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDSURFACEDESC, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type LPDDENUMMODESCALLBACK2 = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDSURFACEDESC2, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type LPDDENUMSURFACESCALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: ::core::option::Option<IDirectDrawSurface>, param1: *mut DDSURFACEDESC, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type LPDDENUMSURFACESCALLBACK2 = ::core::option::Option<unsafe extern "system" fn(param0: ::core::option::Option<IDirectDrawSurface4>, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type LPDDENUMSURFACESCALLBACK7 = ::core::option::Option<unsafe extern "system" fn(param0: ::core::option::Option<IDirectDrawSurface7>, param1: *mut DDSURFACEDESC2, param2: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type LPDDENUMVIDEOCALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDVIDEOPORTCAPS, param1: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type LPDDGAMMACALIBRATORPROC = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDGAMMARAMP, param1: *mut u8) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALCOLORCB_COLORCONTROL = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_COLORCONTROLDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALEXEBUFCB_CANCREATEEXEBUF = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_CANCREATESURFACEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALEXEBUFCB_CREATEEXEBUF = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_CREATESURFACEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALEXEBUFCB_DESTROYEXEBUF = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_DESTROYSURFACEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALEXEBUFCB_LOCKEXEBUF = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_LOCKDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALEXEBUFCB_UNLOCKEXEBUF = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_UNLOCKDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALKERNELCB_SYNCSURFACE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_SYNCSURFACEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALKERNELCB_SYNCVIDEOPORT = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_SYNCVIDEOPORTDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALMOCOMPCB_BEGINFRAME = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_BEGINMOCOMPFRAMEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALMOCOMPCB_CREATE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_CREATEMOCOMPDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALMOCOMPCB_DESTROY = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_DESTROYMOCOMPDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALMOCOMPCB_ENDFRAME = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_ENDMOCOMPFRAMEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALMOCOMPCB_GETCOMPBUFFINFO = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_GETMOCOMPCOMPBUFFDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALMOCOMPCB_GETFORMATS = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_GETMOCOMPFORMATSDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALMOCOMPCB_GETGUIDS = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_GETMOCOMPGUIDSDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALMOCOMPCB_GETINTERNALINFO = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_GETINTERNALMOCOMPDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALMOCOMPCB_QUERYSTATUS = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_QUERYMOCOMPSTATUSDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALMOCOMPCB_RENDER = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_RENDERMOCOMPDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALPALCB_DESTROYPALETTE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_DESTROYPALETTEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALPALCB_SETENTRIES = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_SETENTRIESDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALSURFCB_ADDATTACHEDSURFACE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_ADDATTACHEDSURFACEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALSURFCB_BLT = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_BLTDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALSURFCB_DESTROYSURFACE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_DESTROYSURFACEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALSURFCB_FLIP = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_FLIPDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALSURFCB_GETBLTSTATUS = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_GETBLTSTATUSDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALSURFCB_GETFLIPSTATUS = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_GETFLIPSTATUSDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALSURFCB_LOCK = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_LOCKDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALSURFCB_SETCLIPLIST = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_SETCLIPLISTDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALSURFCB_SETCOLORKEY = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_SETCOLORKEYDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALSURFCB_SETOVERLAYPOSITION = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_SETOVERLAYPOSITIONDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALSURFCB_SETPALETTE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_SETPALETTEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALSURFCB_UNLOCK = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_UNLOCKDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALSURFCB_UPDATEOVERLAY = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_UPDATEOVERLAYDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALVPORTCB_CANCREATEVIDEOPORT = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_CANCREATEVPORTDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALVPORTCB_COLORCONTROL = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_VPORTCOLORDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALVPORTCB_CREATEVIDEOPORT = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_CREATEVPORTDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALVPORTCB_DESTROYVPORT = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_DESTROYVPORTDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALVPORTCB_FLIP = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_FLIPVPORTDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALVPORTCB_GETBANDWIDTH = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_GETVPORTBANDWIDTHDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALVPORTCB_GETFIELD = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_GETVPORTFIELDDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALVPORTCB_GETFLIPSTATUS = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_GETVPORTFLIPSTATUSDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALVPORTCB_GETINPUTFORMATS = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_GETVPORTINPUTFORMATDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALVPORTCB_GETLINE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_GETVPORTLINEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALVPORTCB_GETOUTPUTFORMATS = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_GETVPORTOUTPUTFORMATDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALVPORTCB_GETSIGNALSTATUS = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_GETVPORTSIGNALDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALVPORTCB_GETVPORTCONNECT = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_GETVPORTCONNECTDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALVPORTCB_UPDATE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_UPDATEVPORTDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHALVPORTCB_WAITFORSYNC = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_WAITFORVPORTSYNCDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHAL_CANCREATESURFACE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_CANCREATESURFACEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHAL_CREATEPALETTE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_CREATEPALETTEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHAL_CREATESURFACE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_CREATESURFACEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHAL_CREATESURFACEEX = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_CREATESURFACEEXDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHAL_DESTROYDDLOCAL = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_DESTROYDDLOCALDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHAL_DESTROYDRIVER = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_DESTROYDRIVERDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHAL_FLIPTOGDISURFACE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_FLIPTOGDISURFACEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHAL_GETAVAILDRIVERMEMORY = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_GETAVAILDRIVERMEMORYDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type LPDDHAL_GETDRIVERINFO = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_GETDRIVERINFODATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type LPDDHAL_GETDRIVERSTATE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_GETDRIVERSTATEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type LPDDHAL_GETHEAPALIGNMENT = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_GETHEAPALIGNMENTDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHAL_GETSCANLINE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_GETSCANLINEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHAL_SETCOLORKEY = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_DRVSETCOLORKEYDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHAL_SETEXCLUSIVEMODE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_SETEXCLUSIVEMODEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHAL_SETINFO = ::core::option::Option<unsafe extern "system" fn(lpddhalinfo: *mut DDHALINFO, reset: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHAL_SETMODE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_SETMODEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHAL_UPDATENONLOCALHEAP = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_UPDATENONLOCALHEAPDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHAL_VIDMEMALLOC = ::core::option::Option<unsafe extern "system" fn(lpdd: *mut DDRAWI_DIRECTDRAW_GBL, heap: i32, dwwidth: u32, dwheight: u32) -> usize>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHAL_VIDMEMFREE = ::core::option::Option<unsafe extern "system" fn(lpdd: *mut DDRAWI_DIRECTDRAW_GBL, heap: i32, fpmem: usize) -> ()>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHAL_WAITFORVERTICALBLANK = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDHAL_WAITFORVERTICALBLANKDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDDHEL_INIT = ::core::option::Option<unsafe extern "system" fn(param0: *mut DDRAWI_DIRECTDRAW_GBL, param1: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDIRECTDRAWENUMERATEEXA = ::core::option::Option<unsafe extern "system" fn(lpcallback: LPDDENUMCALLBACKEXA, lpcontext: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type LPDIRECTDRAWENUMERATEEXW = ::core::option::Option<unsafe extern "system" fn(lpcallback: LPDDENUMCALLBACKEXW, lpcontext: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_ALPHABLT = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_BLTDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_CANCREATESURFACE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_CANCREATESURFACEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_COLORCB_COLORCONTROL = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_COLORCONTROLDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub type PDD_CREATEPALETTE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_CREATEPALETTEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_CREATESURFACE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_CREATESURFACEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_CREATESURFACEEX = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_CREATESURFACEEXDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_DESTROYDDLOCAL = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_DESTROYDDLOCALDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_DESTROYDRIVER = ::core::option::Option<unsafe extern "system" fn(param0: *mut _DD_DESTROYDRIVERDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_FLIPTOGDISURFACE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_FLIPTOGDISURFACEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_FREEDRIVERMEMORY = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_FREEDRIVERMEMORYDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_GETAVAILDRIVERMEMORY = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_GETAVAILDRIVERMEMORYDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_GETDRIVERINFO = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_GETDRIVERINFODATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_GETDRIVERSTATE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_GETDRIVERSTATEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_GETSCANLINE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_GETSCANLINEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_KERNELCB_SYNCSURFACE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_SYNCSURFACEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_KERNELCB_SYNCVIDEOPORT = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_SYNCVIDEOPORTDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_MAPMEMORY = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_MAPMEMORYDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_MOCOMPCB_BEGINFRAME = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_BEGINMOCOMPFRAMEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_MOCOMPCB_CREATE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_CREATEMOCOMPDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_MOCOMPCB_DESTROY = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_DESTROYMOCOMPDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_MOCOMPCB_ENDFRAME = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_ENDMOCOMPFRAMEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_MOCOMPCB_GETCOMPBUFFINFO = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_GETMOCOMPCOMPBUFFDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_MOCOMPCB_GETFORMATS = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_GETMOCOMPFORMATSDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_MOCOMPCB_GETGUIDS = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_GETMOCOMPGUIDSDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_MOCOMPCB_GETINTERNALINFO = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_GETINTERNALMOCOMPDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_MOCOMPCB_QUERYSTATUS = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_QUERYMOCOMPSTATUSDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_MOCOMPCB_RENDER = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_RENDERMOCOMPDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_PALCB_DESTROYPALETTE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_DESTROYPALETTEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub type PDD_PALCB_SETENTRIES = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_SETENTRIESDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_SETCOLORKEY = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_DRVSETCOLORKEYDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_SETEXCLUSIVEMODE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_SETEXCLUSIVEMODEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_SETMODE = ::core::option::Option<unsafe extern "system" fn(param0: *mut _DD_SETMODEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_SURFCB_ADDATTACHEDSURFACE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_ADDATTACHEDSURFACEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_SURFCB_BLT = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_BLTDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_SURFCB_DESTROYSURFACE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_DESTROYSURFACEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_SURFCB_FLIP = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_FLIPDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_SURFCB_GETBLTSTATUS = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_GETBLTSTATUSDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_SURFCB_GETFLIPSTATUS = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_GETFLIPSTATUSDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_SURFCB_LOCK = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_LOCKDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_SURFCB_SETCLIPLIST = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_SETCLIPLISTDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_SURFCB_SETCOLORKEY = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_SETCOLORKEYDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_SURFCB_SETOVERLAYPOSITION = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_SETOVERLAYPOSITIONDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_SURFCB_SETPALETTE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_SETPALETTEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_SURFCB_UNLOCK = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_UNLOCKDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_SURFCB_UPDATEOVERLAY = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_UPDATEOVERLAYDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_VPORTCB_CANCREATEVIDEOPORT = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_CANCREATEVPORTDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_VPORTCB_COLORCONTROL = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_VPORTCOLORDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_VPORTCB_CREATEVIDEOPORT = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_CREATEVPORTDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_VPORTCB_DESTROYVPORT = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_DESTROYVPORTDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_VPORTCB_FLIP = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_FLIPVPORTDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_VPORTCB_GETAUTOFLIPSURF = ::core::option::Option<unsafe extern "system" fn(param0: *mut _DD_GETVPORTAUTOFLIPSURFACEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_VPORTCB_GETBANDWIDTH = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_GETVPORTBANDWIDTHDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_VPORTCB_GETFIELD = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_GETVPORTFIELDDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_VPORTCB_GETFLIPSTATUS = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_GETVPORTFLIPSTATUSDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_VPORTCB_GETINPUTFORMATS = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_GETVPORTINPUTFORMATDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_VPORTCB_GETLINE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_GETVPORTLINEDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_VPORTCB_GETOUTPUTFORMATS = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_GETVPORTOUTPUTFORMATDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_VPORTCB_GETSIGNALSTATUS = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_GETVPORTSIGNALDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_VPORTCB_GETVPORTCONNECT = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_GETVPORTCONNECTDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_VPORTCB_UPDATE = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_UPDATEVPORTDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDD_VPORTCB_WAITFORSYNC = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_WAITFORVPORTSYNCDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDD_WAITFORVERTICALBLANK = ::core::option::Option<unsafe extern "system" fn(param0: *mut DD_WAITFORVERTICALBLANKDATA) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDX_BOBNEXTFIELD = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: *mut DDBOBNEXTFIELDINFO, param2: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDX_ENABLEIRQ = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: *mut DDENABLEIRQINFO, param2: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDX_FLIPOVERLAY = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: *mut DDFLIPOVERLAYINFO, param2: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDX_FLIPVIDEOPORT = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: *mut DDFLIPVIDEOPORTINFO, param2: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDX_GETCURRENTAUTOFLIP = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: *mut DDGETCURRENTAUTOFLIPININFO, param2: *mut DDGETCURRENTAUTOFLIPOUTINFO) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDX_GETIRQINFO = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: *mut ::core::ffi::c_void, param2: *mut DDGETIRQINFO) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDX_GETPOLARITY = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: *mut DDGETPOLARITYININFO, param2: *mut DDGETPOLARITYOUTINFO) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDX_GETPREVIOUSAUTOFLIP = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: *mut DDGETPREVIOUSAUTOFLIPININFO, param2: *mut DDGETPREVIOUSAUTOFLIPOUTINFO) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDX_GETTRANSFERSTATUS = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: *mut ::core::ffi::c_void, param2: *mut DDGETTRANSFERSTATUSOUTINFO) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDX_IRQCALLBACK = ::core::option::Option<unsafe extern "system" fn(pirqdata: *mut DX_IRQDATA) -> ()>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDX_LOCK = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: *mut DDLOCKININFO, param2: *mut DDLOCKOUTINFO) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PDX_SETSTATE = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: *mut DDSETSTATEININFO, param2: *mut DDSETSTATEOUTINFO) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDX_SKIPNEXTFIELD = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: *mut DDSKIPNEXTFIELDINFO, param2: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_Graphics_DirectDraw\"`*"]
pub type PDX_TRANSFER = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: *mut DDTRANSFERININFO, param2: *mut DDTRANSFEROUTINFO) -> u32>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
