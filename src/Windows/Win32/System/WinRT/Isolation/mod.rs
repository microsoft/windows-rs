#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_WinRT_Isolation`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IIsolatedEnvironmentInterop(pub ::windows::runtime::IUnknown);
impl IIsolatedEnvironmentInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Isolation`, `Win32_Foundation`*"]
    pub unsafe fn GetHostHwndInterop<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, containerhwnd: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::HWND> {
        let mut result__: <super::super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), containerhwnd.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IIsolatedEnvironmentInterop {
    type Vtable = IIsolatedEnvironmentInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x85713c2e_8e62_46c5_8de2_c647e1d54636);
}
impl ::core::convert::From<IIsolatedEnvironmentInterop> for ::windows::runtime::IUnknown {
    fn from(value: IIsolatedEnvironmentInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IIsolatedEnvironmentInterop> for ::windows::runtime::IUnknown {
    fn from(value: &IIsolatedEnvironmentInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IIsolatedEnvironmentInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IIsolatedEnvironmentInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedEnvironmentInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, containerhwnd: super::super::super::Foundation::HWND, hosthwnd: *mut super::super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
