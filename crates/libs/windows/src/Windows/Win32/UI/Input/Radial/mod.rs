#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRadialControllerConfigurationInterop(::windows_core::IUnknown);
impl IRadialControllerConfigurationInterop {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<P0, T>(&self, hwnd: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetForWindow)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IRadialControllerConfigurationInterop, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::windows_core::Interface for IRadialControllerConfigurationInterop {
    type Vtable = IRadialControllerConfigurationInterop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRadialControllerConfigurationInterop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x787cdaac_3186_476d_87e4_b9374a7b9970);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerConfigurationInterop_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRadialControllerIndependentInputSourceInterop(::windows_core::IUnknown);
impl IRadialControllerIndependentInputSourceInterop {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateForWindow<P0, T>(&self, hwnd: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).CreateForWindow)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IRadialControllerIndependentInputSourceInterop, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::windows_core::Interface for IRadialControllerIndependentInputSourceInterop {
    type Vtable = IRadialControllerIndependentInputSourceInterop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRadialControllerIndependentInputSourceInterop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3d577eff_4cee_11e6_b535_001bdc06ab3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerIndependentInputSourceInterop_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateForWindow: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IRadialControllerInterop(::windows_core::IUnknown);
impl IRadialControllerInterop {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateForWindow<P0, T>(&self, hwnd: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).CreateForWindow)(::windows_core::Interface::as_raw(self), hwnd.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IRadialControllerInterop, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::windows_core::Interface for IRadialControllerInterop {
    type Vtable = IRadialControllerInterop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IRadialControllerInterop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1b0535c9_57ad_45c1_9d79_ad5c34360513);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerInterop_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateForWindow: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
