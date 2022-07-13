#[doc = "*Required features: `\"Win32_UI_Input_Radial\"`*"]
#[repr(transparent)]
pub struct IRadialControllerConfigurationInterop(::windows::core::IUnknown);
impl IRadialControllerConfigurationInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<'a, P0, T>(&self, hwnd: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetForWindow)(::windows::core::Interface::as_raw(self), hwnd.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IRadialControllerConfigurationInterop> for ::windows::core::IUnknown {
    fn from(value: IRadialControllerConfigurationInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRadialControllerConfigurationInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRadialControllerConfigurationInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRadialControllerConfigurationInterop> for ::windows::core::IUnknown {
    fn from(value: &IRadialControllerConfigurationInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IRadialControllerConfigurationInterop> for ::windows::core::IInspectable {
    fn from(value: IRadialControllerConfigurationInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRadialControllerConfigurationInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IRadialControllerConfigurationInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRadialControllerConfigurationInterop> for ::windows::core::IInspectable {
    fn from(value: &IRadialControllerConfigurationInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRadialControllerConfigurationInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRadialControllerConfigurationInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRadialControllerConfigurationInterop {}
impl ::core::fmt::Debug for IRadialControllerConfigurationInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRadialControllerConfigurationInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRadialControllerConfigurationInterop {
    type Vtable = IRadialControllerConfigurationInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x787cdaac_3186_476d_87e4_b9374a7b9970);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerConfigurationInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[doc = "*Required features: `\"Win32_UI_Input_Radial\"`*"]
#[repr(transparent)]
pub struct IRadialControllerIndependentInputSourceInterop(::windows::core::IUnknown);
impl IRadialControllerIndependentInputSourceInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateForWindow<'a, P0, T>(&self, hwnd: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateForWindow)(::windows::core::Interface::as_raw(self), hwnd.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IRadialControllerIndependentInputSourceInterop> for ::windows::core::IUnknown {
    fn from(value: IRadialControllerIndependentInputSourceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRadialControllerIndependentInputSourceInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRadialControllerIndependentInputSourceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRadialControllerIndependentInputSourceInterop> for ::windows::core::IUnknown {
    fn from(value: &IRadialControllerIndependentInputSourceInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IRadialControllerIndependentInputSourceInterop> for ::windows::core::IInspectable {
    fn from(value: IRadialControllerIndependentInputSourceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRadialControllerIndependentInputSourceInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IRadialControllerIndependentInputSourceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRadialControllerIndependentInputSourceInterop> for ::windows::core::IInspectable {
    fn from(value: &IRadialControllerIndependentInputSourceInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRadialControllerIndependentInputSourceInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRadialControllerIndependentInputSourceInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRadialControllerIndependentInputSourceInterop {}
impl ::core::fmt::Debug for IRadialControllerIndependentInputSourceInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRadialControllerIndependentInputSourceInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRadialControllerIndependentInputSourceInterop {
    type Vtable = IRadialControllerIndependentInputSourceInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d577eff_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerIndependentInputSourceInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateForWindow: usize,
}
#[doc = "*Required features: `\"Win32_UI_Input_Radial\"`*"]
#[repr(transparent)]
pub struct IRadialControllerInterop(::windows::core::IUnknown);
impl IRadialControllerInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateForWindow<'a, P0, T>(&self, hwnd: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).CreateForWindow)(::windows::core::Interface::as_raw(self), hwnd.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<IRadialControllerInterop> for ::windows::core::IUnknown {
    fn from(value: IRadialControllerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRadialControllerInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRadialControllerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRadialControllerInterop> for ::windows::core::IUnknown {
    fn from(value: &IRadialControllerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IRadialControllerInterop> for ::windows::core::IInspectable {
    fn from(value: IRadialControllerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRadialControllerInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IRadialControllerInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRadialControllerInterop> for ::windows::core::IInspectable {
    fn from(value: &IRadialControllerInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRadialControllerInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRadialControllerInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRadialControllerInterop {}
impl ::core::fmt::Debug for IRadialControllerInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRadialControllerInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRadialControllerInterop {
    type Vtable = IRadialControllerInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1b0535c9_57ad_45c1_9d79_ad5c34360513);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateForWindow: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
