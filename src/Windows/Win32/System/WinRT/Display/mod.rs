#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_WinRT_Display`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDisplayDeviceInterop(pub ::windows::runtime::IUnknown);
impl IDisplayDeviceInterop {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    #[doc = "*Required features: `Win32_System_WinRT_Display`, `Win32_Foundation`, `Win32_Security`*"]
    pub unsafe fn CreateSharedHandle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, pobject: Param0, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: Param3) -> ::windows::runtime::Result<super::super::super::Foundation::HANDLE> {
        let mut result__: <super::super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pobject.into_param().abi(), ::core::mem::transmute(psecurityattributes), ::core::mem::transmute(access), name.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Display`, `Win32_Foundation`*"]
    pub unsafe fn OpenSharedHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, nthandle: Param0, riid: Param1, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), nthandle.into_param().abi(), riid.into_param().abi(), ::core::mem::transmute(ppvobj)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDisplayDeviceInterop {
    type Vtable = IDisplayDeviceInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1681097560, 13930, 18203, [189, 86, 221, 142, 244, 142, 67, 155]);
}
impl ::core::convert::From<IDisplayDeviceInterop> for ::windows::runtime::IUnknown {
    fn from(value: IDisplayDeviceInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDisplayDeviceInterop> for ::windows::runtime::IUnknown {
    fn from(value: &IDisplayDeviceInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDisplayDeviceInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDisplayDeviceInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayDeviceInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pobject: ::windows::runtime::RawPtr, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, phandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nthandle: super::super::super::Foundation::HANDLE, riid: ::windows::runtime::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT_Display`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDisplayPathInterop(pub ::windows::runtime::IUnknown);
impl IDisplayPathInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT_Display`, `Win32_Foundation`*"]
    pub unsafe fn CreateSourcePresentationHandle(&self) -> ::windows::runtime::Result<super::super::super::Foundation::HANDLE> {
        let mut result__: <super::super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::HANDLE>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT_Display`*"]
    pub unsafe fn GetSourceId(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDisplayPathInterop {
    type Vtable = IDisplayPathInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2797224453, 58782, 20081, [178, 91, 78, 67, 109, 33, 238, 61]);
}
impl ::core::convert::From<IDisplayPathInterop> for ::windows::runtime::IUnknown {
    fn from(value: IDisplayPathInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDisplayPathInterop> for ::windows::runtime::IUnknown {
    fn from(value: &IDisplayPathInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDisplayPathInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDisplayPathInterop {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPathInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut super::super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psourceid: *mut u32) -> ::windows::runtime::HRESULT,
);
