#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn CreateDirect3D11DeviceFromDXGIDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGIDevice>>(dxgidevice: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDirect3D11DeviceFromDXGIDevice(dxgidevice: ::windows::core::RawPtr, graphicsdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        CreateDirect3D11DeviceFromDXGIDevice(dxgidevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IInspectable>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn CreateDirect3D11SurfaceFromDXGISurface<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGISurface>>(dgxisurface: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDirect3D11SurfaceFromDXGISurface(dgxisurface: ::windows::core::RawPtr, graphicssurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        CreateDirect3D11SurfaceFromDXGISurface(dgxisurface.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IInspectable>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
pub struct IDirect3DDxgiInterfaceAccess(::windows::core::IUnknown);
impl IDirect3DDxgiInterfaceAccess {
    pub unsafe fn GetInterface<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IDirect3DDxgiInterfaceAccess> for ::windows::core::IUnknown {
    fn from(value: IDirect3DDxgiInterfaceAccess) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DDxgiInterfaceAccess> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DDxgiInterfaceAccess) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DDxgiInterfaceAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDirect3DDxgiInterfaceAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
unsafe impl ::windows::core::Interface for IDirect3DDxgiInterfaceAccess {
    type Vtable = IDirect3DDxgiInterfaceAccessVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9b3d012_3df2_4ee3_b8d1_8695f457d3c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDxgiInterfaceAccessVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, p: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT);
