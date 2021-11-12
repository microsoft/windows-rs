#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn CreateDirect3D11DeviceFromDXGIDevice<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Graphics::Dxgi::IDXGIDevice>>(dxgidevice: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDirect3D11DeviceFromDXGIDevice(dxgidevice: ::windows::core::RawPtr, graphicsdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::IInspectable as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateDirect3D11DeviceFromDXGIDevice(dxgidevice.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
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
            fn CreateDirect3D11SurfaceFromDXGISurface(dgxisurface: ::windows::core::RawPtr, graphicssurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        let mut result__: <::windows::core::IInspectable as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateDirect3D11SurfaceFromDXGISurface(dgxisurface.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDirect3DDxgiInterfaceAccess(pub ::windows::core::IUnknown);
impl IDirect3DDxgiInterfaceAccess {
    pub unsafe fn GetInterface<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for IDirect3DDxgiInterfaceAccess {
    type Vtable = IDirect3DDxgiInterfaceAccess_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa9b3d012_3df2_4ee3_b8d1_8695f457d3c1);
}
impl ::core::convert::From<IDirect3DDxgiInterfaceAccess> for ::windows::core::IUnknown {
    fn from(value: IDirect3DDxgiInterfaceAccess) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirect3DDxgiInterfaceAccess> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DDxgiInterfaceAccess) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DDxgiInterfaceAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DDxgiInterfaceAccess {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDxgiInterfaceAccess_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: *const ::windows::core::GUID, p: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
