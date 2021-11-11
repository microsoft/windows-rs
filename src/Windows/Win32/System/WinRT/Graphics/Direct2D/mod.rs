#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_WinRT_Graphics_Direct2D`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct GRAPHICS_EFFECT_PROPERTY_MAPPING(pub i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_UNKNOWN: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(0i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_DIRECT: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(1i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORX: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(2i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORY: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(3i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORZ: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(4i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORW: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(5i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_RECT_TO_VECTOR4: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(6i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_RADIANS_TO_DEGREES: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(7i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLORMATRIX_ALPHA_MODE: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(8i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLOR_TO_VECTOR3: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(9i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLOR_TO_VECTOR4: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(10i32);
impl ::core::convert::From<i32> for GRAPHICS_EFFECT_PROPERTY_MAPPING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for GRAPHICS_EFFECT_PROPERTY_MAPPING {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WinRT_Graphics_Direct2D`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGeometrySource2DInterop(pub ::windows::core::IUnknown);
impl IGeometrySource2DInterop {
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    #[doc = "*Required features: `Win32_System_WinRT_Graphics_Direct2D`, `Win32_Graphics_Direct2D`*"]
    pub unsafe fn GetGeometry(&self) -> ::windows::core::Result<super::super::super::super::Graphics::Direct2D::ID2D1Geometry> {
        let mut result__: <super::super::super::super::Graphics::Direct2D::ID2D1Geometry as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::super::Graphics::Direct2D::ID2D1Geometry>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    #[doc = "*Required features: `Win32_System_WinRT_Graphics_Direct2D`, `Win32_Graphics_Direct2D`*"]
    pub unsafe fn TryGetGeometryUsingFactory<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Graphics::Direct2D::ID2D1Factory>>(&self, factory: Param0) -> ::windows::core::Result<super::super::super::super::Graphics::Direct2D::ID2D1Geometry> {
        let mut result__: <super::super::super::super::Graphics::Direct2D::ID2D1Geometry as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), factory.into_param().abi(), &mut result__).from_abi::<super::super::super::super::Graphics::Direct2D::ID2D1Geometry>(result__)
    }
}
unsafe impl ::windows::core::Interface for IGeometrySource2DInterop {
    type Vtable = IGeometrySource2DInterop_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0657af73_53fd_47cf_84ff_c8492d2a80a3);
}
impl ::core::convert::From<IGeometrySource2DInterop> for ::windows::core::IUnknown {
    fn from(value: IGeometrySource2DInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGeometrySource2DInterop> for ::windows::core::IUnknown {
    fn from(value: &IGeometrySource2DInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGeometrySource2DInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGeometrySource2DInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometrySource2DInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct2D")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct2D")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, factory: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Graphics_Direct2D`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IGraphicsEffectD2D1Interop(pub ::windows::core::IUnknown);
impl IGraphicsEffectD2D1Interop {
    #[doc = "*Required features: `Win32_System_WinRT_Graphics_Direct2D`*"]
    pub unsafe fn GetEffectId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: <::windows::core::GUID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::core::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Graphics_Direct2D`, `Win32_Foundation`*"]
    pub unsafe fn GetNamedPropertyMapping<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, name: Param0, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(index), ::core::mem::transmute(mapping)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT_Graphics_Direct2D`*"]
    pub unsafe fn GetPropertyCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Graphics_Direct2D`, `Foundation`*"]
    pub unsafe fn GetProperty(&self, index: u32) -> ::windows::core::Result<super::super::super::super::super::Foundation::IPropertyValue> {
        let mut result__: <super::super::super::super::super::Foundation::IPropertyValue as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<super::super::super::super::super::Foundation::IPropertyValue>(result__)
    }
    #[cfg(feature = "Graphics_Effects")]
    #[doc = "*Required features: `Win32_System_WinRT_Graphics_Direct2D`, `Graphics_Effects`*"]
    pub unsafe fn GetSource(&self, index: u32) -> ::windows::core::Result<super::super::super::super::super::Graphics::Effects::IGraphicsEffectSource> {
        let mut result__: <super::super::super::super::super::Graphics::Effects::IGraphicsEffectSource as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), &mut result__).from_abi::<super::super::super::super::super::Graphics::Effects::IGraphicsEffectSource>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT_Graphics_Direct2D`*"]
    pub unsafe fn GetSourceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IGraphicsEffectD2D1Interop {
    type Vtable = IGraphicsEffectD2D1Interop_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2fc57384_a068_44d7_a331_30982fcf7177);
}
impl ::core::convert::From<IGraphicsEffectD2D1Interop> for ::windows::core::IUnknown {
    fn from(value: IGraphicsEffectD2D1Interop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IGraphicsEffectD2D1Interop> for ::windows::core::IUnknown {
    fn from(value: &IGraphicsEffectD2D1Interop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGraphicsEffectD2D1Interop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGraphicsEffectD2D1Interop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsEffectD2D1Interop_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, name: super::super::super::super::Foundation::PWSTR, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Graphics_Effects")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, source: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32) -> ::windows::core::HRESULT,
);
