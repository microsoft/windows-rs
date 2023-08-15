#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct IGeometrySource2DInterop(::windows_core::IUnknown);
impl IGeometrySource2DInterop {
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn GetGeometry(&self) -> ::windows_core::Result<super::super::super::super::Graphics::Direct2D::ID2D1Geometry> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetGeometry)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct2D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn TryGetGeometryUsingFactory<P0>(&self, factory: P0) -> ::windows_core::Result<super::super::super::super::Graphics::Direct2D::ID2D1Geometry>
    where
        P0: ::windows_core::IntoParam<super::super::super::super::Graphics::Direct2D::ID2D1Factory>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).TryGetGeometryUsingFactory)(::windows_core::Interface::as_raw(self), factory.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IGeometrySource2DInterop, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IGeometrySource2DInterop {
    type Vtable = IGeometrySource2DInterop_Vtbl;
}
impl ::core::clone::Clone for IGeometrySource2DInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGeometrySource2DInterop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0657af73_53fd_47cf_84ff_c8492d2a80a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometrySource2DInterop_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub GetGeometry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))]
    GetGeometry: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub TryGetGeometryUsingFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))]
    TryGetGeometryUsingFactory: usize,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
#[repr(transparent)]
pub struct IGraphicsEffectD2D1Interop(::windows_core::IUnknown);
impl IGraphicsEffectD2D1Interop {
    pub unsafe fn GetEffectId(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetEffectId)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNamedPropertyMapping<P0>(&self, name: P0, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).GetNamedPropertyMapping)(::windows_core::Interface::as_raw(self), name.into_param().abi(), index, mapping).ok()
    }
    pub unsafe fn GetPropertyCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPropertyCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub unsafe fn GetProperty(&self, index: u32) -> ::windows_core::Result<super::super::super::super::super::Foundation::IPropertyValue> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetProperty)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Graphics_Effects\"`*"]
    #[cfg(feature = "Graphics_Effects")]
    pub unsafe fn GetSource(&self, index: u32) -> ::windows_core::Result<super::super::super::super::super::Graphics::Effects::IGraphicsEffectSource> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSource)(::windows_core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSourceCount(&self) -> ::windows_core::Result<u32> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetSourceCount)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IGraphicsEffectD2D1Interop, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IGraphicsEffectD2D1Interop {
    type Vtable = IGraphicsEffectD2D1Interop_Vtbl;
}
impl ::core::clone::Clone for IGraphicsEffectD2D1Interop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IGraphicsEffectD2D1Interop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2fc57384_a068_44d7_a331_30982fcf7177);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsEffectD2D1Interop_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetEffectId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub GetNamedPropertyMapping: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> ::windows_core::HRESULT,
    pub GetPropertyCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetProperty: usize,
    #[cfg(feature = "Graphics_Effects")]
    pub GetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, source: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))]
    GetSource: usize,
    pub GetSourceCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLORMATRIX_ALPHA_MODE: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(8i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLOR_TO_VECTOR3: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(9i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLOR_TO_VECTOR4: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(10i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_DIRECT: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(1i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_RADIANS_TO_DEGREES: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(7i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_RECT_TO_VECTOR4: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(6i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_UNKNOWN: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(0i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORW: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(5i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORX: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(2i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORY: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(3i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORZ: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(4i32);
#[doc = "*Required features: `\"Win32_System_WinRT_Graphics_Direct2D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct GRAPHICS_EFFECT_PROPERTY_MAPPING(pub i32);
impl ::core::marker::Copy for GRAPHICS_EFFECT_PROPERTY_MAPPING {}
impl ::core::clone::Clone for GRAPHICS_EFFECT_PROPERTY_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GRAPHICS_EFFECT_PROPERTY_MAPPING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for GRAPHICS_EFFECT_PROPERTY_MAPPING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for GRAPHICS_EFFECT_PROPERTY_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GRAPHICS_EFFECT_PROPERTY_MAPPING").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
