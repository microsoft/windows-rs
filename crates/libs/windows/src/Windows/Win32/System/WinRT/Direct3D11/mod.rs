#[doc = "*Required features: `\"Win32_System_WinRT_Direct3D11\"`, `\"Win32_Graphics_Dxgi\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn CreateDirect3D11DeviceFromDXGIDevice<'a, P0>(dxgidevice: P0) -> ::windows::core::Result<::windows::core::IInspectable>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Dxgi::IDXGIDevice>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDirect3D11DeviceFromDXGIDevice(dxgidevice: *mut ::core::ffi::c_void, graphicsdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateDirect3D11DeviceFromDXGIDevice(dxgidevice.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IInspectable>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT_Direct3D11\"`, `\"Win32_Graphics_Dxgi\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn CreateDirect3D11SurfaceFromDXGISurface<'a, P0>(dgxisurface: P0) -> ::windows::core::Result<::windows::core::IInspectable>
where
    P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Graphics::Dxgi::IDXGISurface>>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateDirect3D11SurfaceFromDXGISurface(dgxisurface: *mut ::core::ffi::c_void, graphicssurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateDirect3D11SurfaceFromDXGISurface(dgxisurface.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IInspectable>(result__)
}
#[doc = "*Required features: `\"Win32_System_WinRT_Direct3D11\"`*"]
#[repr(transparent)]
pub struct IDirect3DDxgiInterfaceAccess(::windows::core::IUnknown);
impl IDirect3DDxgiInterfaceAccess {
    pub unsafe fn GetInterface<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetInterface)(::windows::core::Interface::as_raw(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IDirect3DDxgiInterfaceAccess> for ::windows::core::IUnknown {
    fn from(value: IDirect3DDxgiInterfaceAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDirect3DDxgiInterfaceAccess> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDirect3DDxgiInterfaceAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DDxgiInterfaceAccess> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DDxgiInterfaceAccess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDirect3DDxgiInterfaceAccess {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DDxgiInterfaceAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DDxgiInterfaceAccess {}
impl ::core::fmt::Debug for IDirect3DDxgiInterfaceAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DDxgiInterfaceAccess").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDirect3DDxgiInterfaceAccess {
    type Vtable = IDirect3DDxgiInterfaceAccess_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9b3d012_3df2_4ee3_b8d1_8695f457d3c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDxgiInterfaceAccess_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, p: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
