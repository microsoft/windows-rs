#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D'*"]
pub type GRAPHICS_EFFECT_PROPERTY_MAPPING = i32;
#[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D'*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_UNKNOWN: GRAPHICS_EFFECT_PROPERTY_MAPPING = 0i32;
#[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D'*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_DIRECT: GRAPHICS_EFFECT_PROPERTY_MAPPING = 1i32;
#[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D'*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORX: GRAPHICS_EFFECT_PROPERTY_MAPPING = 2i32;
#[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D'*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORY: GRAPHICS_EFFECT_PROPERTY_MAPPING = 3i32;
#[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D'*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORZ: GRAPHICS_EFFECT_PROPERTY_MAPPING = 4i32;
#[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D'*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORW: GRAPHICS_EFFECT_PROPERTY_MAPPING = 5i32;
#[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D'*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_RECT_TO_VECTOR4: GRAPHICS_EFFECT_PROPERTY_MAPPING = 6i32;
#[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D'*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_RADIANS_TO_DEGREES: GRAPHICS_EFFECT_PROPERTY_MAPPING = 7i32;
#[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D'*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLORMATRIX_ALPHA_MODE: GRAPHICS_EFFECT_PROPERTY_MAPPING = 8i32;
#[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D'*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLOR_TO_VECTOR3: GRAPHICS_EFFECT_PROPERTY_MAPPING = 9i32;
#[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D'*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLOR_TO_VECTOR4: GRAPHICS_EFFECT_PROPERTY_MAPPING = 10i32;
#[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D'*"]
#[repr(transparent)]
pub struct IGeometrySource2DInterop(::windows::core::IUnknown);
impl IGeometrySource2DInterop {
    #[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D', 'Win32_Graphics_Direct2D'*"]
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn GetGeometry(&self) -> ::windows::core::Result<super::super::super::super::Graphics::Direct2D::ID2D1Geometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::Graphics::Direct2D::ID2D1Geometry>(result__)
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D', 'Win32_Graphics_Direct2D'*"]
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn TryGetGeometryUsingFactory<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Graphics::Direct2D::ID2D1Factory>>(&self, factory: Param0) -> ::windows::core::Result<super::super::super::super::Graphics::Direct2D::ID2D1Geometry> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), factory.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::Graphics::Direct2D::ID2D1Geometry>(result__)
    }
}
impl ::core::convert::From<IGeometrySource2DInterop> for ::windows::core::IUnknown {
    fn from(value: IGeometrySource2DInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGeometrySource2DInterop> for ::windows::core::IUnknown {
    fn from(value: &IGeometrySource2DInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGeometrySource2DInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IGeometrySource2DInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGeometrySource2DInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGeometrySource2DInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGeometrySource2DInterop {}
impl ::core::fmt::Debug for IGeometrySource2DInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGeometrySource2DInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGeometrySource2DInterop {
    type Vtable = IGeometrySource2DInteropVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0657af73_53fd_47cf_84ff_c8492d2a80a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometrySource2DInteropVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct2D")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct2D")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
);
#[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D'*"]
#[repr(transparent)]
pub struct IGraphicsEffectD2D1Interop(::windows::core::IUnknown);
impl IGraphicsEffectD2D1Interop {
    #[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D'*"]
    pub unsafe fn GetEffectId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::GUID>(result__)
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNamedPropertyMapping<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::PWSTR>>(&self, name: Param0, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), name.into_param().abi(), ::core::mem::transmute(index), ::core::mem::transmute(mapping)).ok()
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D'*"]
    pub unsafe fn GetPropertyCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub unsafe fn GetProperty(&self, index: u32) -> ::windows::core::Result<super::super::super::super::super::Foundation::IPropertyValue> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::super::Foundation::IPropertyValue>(result__)
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D', 'Graphics_Effects'*"]
    #[cfg(feature = "Graphics_Effects")]
    pub unsafe fn GetSource(&self, index: u32) -> ::windows::core::Result<super::super::super::super::super::Graphics::Effects::IGraphicsEffectSource> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::super::super::Graphics::Effects::IGraphicsEffectSource>(result__)
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Graphics_Direct2D'*"]
    pub unsafe fn GetSourceCount(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IGraphicsEffectD2D1Interop> for ::windows::core::IUnknown {
    fn from(value: IGraphicsEffectD2D1Interop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGraphicsEffectD2D1Interop> for ::windows::core::IUnknown {
    fn from(value: &IGraphicsEffectD2D1Interop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGraphicsEffectD2D1Interop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IGraphicsEffectD2D1Interop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGraphicsEffectD2D1Interop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGraphicsEffectD2D1Interop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGraphicsEffectD2D1Interop {}
impl ::core::fmt::Debug for IGraphicsEffectD2D1Interop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGraphicsEffectD2D1Interop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IGraphicsEffectD2D1Interop {
    type Vtable = IGraphicsEffectD2D1InteropVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2fc57384_a068_44d7_a331_30982fcf7177);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsEffectD2D1InteropVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::super::super::Foundation::PWSTR, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Graphics_Effects")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, source: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT,
);
