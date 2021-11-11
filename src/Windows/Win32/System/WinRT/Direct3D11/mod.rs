#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_WinRT_Direct3D11`, `Win32_Graphics_Dxgi`*"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn CreateDirect3D11DeviceFromDXGIDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGIDevice>>(dxgidevice: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDirect3D11DeviceFromDXGIDevice(dxgidevice: ::windows::runtime::RawPtr, graphicsdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateDirect3D11DeviceFromDXGIDevice(dxgidevice.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT_Direct3D11`, `Win32_Graphics_Dxgi`*"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn CreateDirect3D11SurfaceFromDXGISurface<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGISurface>>(dgxisurface: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDirect3D11SurfaceFromDXGISurface(dgxisurface: ::windows::runtime::RawPtr, graphicssurface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        CreateDirect3D11SurfaceFromDXGISurface(dgxisurface.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT_Direct3D11`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirect3DDxgiInterfaceAccess(pub ::windows::runtime::IUnknown);
impl IDirect3DDxgiInterfaceAccess {
    #[doc = "*Required features: `Win32_System_WinRT_Direct3D11`*"]
    pub unsafe fn GetInterface<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirect3DDxgiInterfaceAccess {
    type Vtable = IDirect3DDxgiInterfaceAccess_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa9b3d012_3df2_4ee3_b8d1_8695f457d3c1);
}
impl ::core::convert::From<IDirect3DDxgiInterfaceAccess> for ::windows::runtime::IUnknown {
    fn from(value: IDirect3DDxgiInterfaceAccess) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirect3DDxgiInterfaceAccess> for ::windows::runtime::IUnknown {
    fn from(value: &IDirect3DDxgiInterfaceAccess) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirect3DDxgiInterfaceAccess {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirect3DDxgiInterfaceAccess {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDxgiInterfaceAccess_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: *const ::windows::runtime::GUID, p: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
);
