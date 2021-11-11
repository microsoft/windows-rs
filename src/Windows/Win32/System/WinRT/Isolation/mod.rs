#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_WinRT_Isolation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IIsolatedEnvironmentInterop(pub ::windows::core::IUnknown);
impl IIsolatedEnvironmentInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Isolation`, `Win32_Foundation`*"]
    pub unsafe fn GetHostHwndInterop<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, containerhwnd: Param0) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__: <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), containerhwnd.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
}
unsafe impl ::windows::core::Interface for IIsolatedEnvironmentInterop {
    type Vtable = IIsolatedEnvironmentInterop_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85713c2e_8e62_46c5_8de2_c647e1d54636);
}
impl ::core::convert::From<IIsolatedEnvironmentInterop> for ::windows::core::IUnknown {
    fn from(value: IIsolatedEnvironmentInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IIsolatedEnvironmentInterop> for ::windows::core::IUnknown {
    fn from(value: &IIsolatedEnvironmentInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IIsolatedEnvironmentInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IIsolatedEnvironmentInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedEnvironmentInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, containerhwnd: super::super::super::Foundation::HWND, hosthwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
