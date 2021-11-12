#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDisplayDeviceInterop(pub ::windows::core::IUnknown);
impl IDisplayDeviceInterop {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn CreateSharedHandle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, pobject: Param0, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: Param3) -> ::windows::core::Result<super::super::super::Foundation::HANDLE> {
        let mut result__: <super::super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pobject.into_param().abi(), ::core::mem::transmute(psecurityattributes), ::core::mem::transmute(access), name.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, nthandle: Param0, riid: Param1, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), nthandle.into_param().abi(), riid.into_param().abi(), ::core::mem::transmute(ppvobj)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDisplayDeviceInterop {
    type Vtable = IDisplayDeviceInterop_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64338358_366a_471b_bd56_dd8ef48e439b);
}
impl ::core::convert::From<IDisplayDeviceInterop> for ::windows::core::IUnknown {
    fn from(value: IDisplayDeviceInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDisplayDeviceInterop> for ::windows::core::IUnknown {
    fn from(value: &IDisplayDeviceInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDisplayDeviceInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDisplayDeviceInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayDeviceInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pobject: ::windows::core::RawPtr, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, phandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nthandle: super::super::super::Foundation::HANDLE, riid: ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDisplayPathInterop(pub ::windows::core::IUnknown);
impl IDisplayPathInterop {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSourcePresentationHandle(&self) -> ::windows::core::Result<super::super::super::Foundation::HANDLE> {
        let mut result__: <super::super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::super::Foundation::HANDLE>(result__)
    }
    pub unsafe fn GetSourceId(&self) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDisplayPathInterop {
    type Vtable = IDisplayPathInterop_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6ba4205_e59e_4e71_b25b_4e436d21ee3d);
}
impl ::core::convert::From<IDisplayPathInterop> for ::windows::core::IUnknown {
    fn from(value: IDisplayPathInterop) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDisplayPathInterop> for ::windows::core::IUnknown {
    fn from(value: &IDisplayPathInterop) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDisplayPathInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDisplayPathInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPathInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pvalue: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, psourceid: *mut u32) -> ::windows::core::HRESULT,
);
