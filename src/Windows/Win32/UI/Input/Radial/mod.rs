#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_UI_Input_Radial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRadialControllerConfigurationInterop(pub ::windows::runtime::IUnknown);
impl IRadialControllerConfigurationInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Input_Radial`, `Win32_Foundation`*"]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, hwnd: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), hwnd.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IRadialControllerConfigurationInterop {
    type Vtable = IRadialControllerConfigurationInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2021448364, 12678, 18285, [135, 228, 185, 55, 74, 123, 153, 112]);
}
impl ::std::convert::From<IRadialControllerConfigurationInterop> for ::windows::runtime::IUnknown {
    fn from(value: IRadialControllerConfigurationInterop) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRadialControllerConfigurationInterop> for ::windows::runtime::IUnknown {
    fn from(value: &IRadialControllerConfigurationInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRadialControllerConfigurationInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRadialControllerConfigurationInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerConfigurationInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_Input_Radial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRadialControllerIndependentInputSourceInterop(pub ::windows::runtime::IUnknown);
impl IRadialControllerIndependentInputSourceInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Input_Radial`, `Win32_Foundation`*"]
    pub unsafe fn CreateForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, hwnd: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), hwnd.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IRadialControllerIndependentInputSourceInterop {
    type Vtable = IRadialControllerIndependentInputSourceInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1029144319, 19694, 4582, [181, 53, 0, 27, 220, 6, 171, 59]);
}
impl ::std::convert::From<IRadialControllerIndependentInputSourceInterop> for ::windows::runtime::IUnknown {
    fn from(value: IRadialControllerIndependentInputSourceInterop) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRadialControllerIndependentInputSourceInterop> for ::windows::runtime::IUnknown {
    fn from(value: &IRadialControllerIndependentInputSourceInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRadialControllerIndependentInputSourceInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRadialControllerIndependentInputSourceInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerIndependentInputSourceInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_Input_Radial`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRadialControllerInterop(pub ::windows::runtime::IUnknown);
impl IRadialControllerInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_Input_Radial`, `Win32_Foundation`*"]
    pub unsafe fn CreateForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, hwnd: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), hwnd.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IRadialControllerInterop {
    type Vtable = IRadialControllerInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(453326281, 22445, 17857, [157, 121, 173, 92, 52, 54, 5, 19]);
}
impl ::std::convert::From<IRadialControllerInterop> for ::windows::runtime::IUnknown {
    fn from(value: IRadialControllerInterop) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRadialControllerInterop> for ::windows::runtime::IUnknown {
    fn from(value: &IRadialControllerInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRadialControllerInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRadialControllerInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
