#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_System_WinRT_Display'*"]
#[repr(transparent)]
pub struct IDisplayDeviceInterop(::windows::core::IUnknown);
impl IDisplayDeviceInterop {
    #[doc = "*Required features: 'Win32_System_WinRT_Display', 'Win32_Foundation', 'Win32_Security'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn CreateSharedHandle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, pobject: Param0, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: Param3) -> ::windows::core::Result<super::super::super::Foundation::HANDLE> {
        let mut result__: super::super::super::Foundation::HANDLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pobject.into_param().abi(), ::core::mem::transmute(psecurityattributes), ::core::mem::transmute(access), name.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::HANDLE>(result__)
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Display', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, nthandle: Param0, riid: Param1, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), nthandle.into_param().abi(), riid.into_param().abi(), ::core::mem::transmute(ppvobj)).ok()
    }
}
impl ::core::convert::From<IDisplayDeviceInterop> for ::windows::core::IUnknown {
    fn from(value: IDisplayDeviceInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDisplayDeviceInterop> for ::windows::core::IUnknown {
    fn from(value: &IDisplayDeviceInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDisplayDeviceInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDisplayDeviceInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDisplayDeviceInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDisplayDeviceInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDisplayDeviceInterop {}
unsafe impl ::windows::core::Interface for IDisplayDeviceInterop {
    type Vtable = IDisplayDeviceInteropVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64338358_366a_471b_bd56_dd8ef48e439b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayDeviceInteropVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, psecurityattributes: *const super::super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, phandle: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nthandle: super::super::super::Foundation::HANDLE, riid: ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_System_WinRT_Display'*"]
#[repr(transparent)]
pub struct IDisplayPathInterop(::windows::core::IUnknown);
impl IDisplayPathInterop {
    #[doc = "*Required features: 'Win32_System_WinRT_Display', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSourcePresentationHandle(&self) -> ::windows::core::Result<super::super::super::Foundation::HANDLE> {
        let mut result__: super::super::super::Foundation::HANDLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::HANDLE>(result__)
    }
    #[doc = "*Required features: 'Win32_System_WinRT_Display'*"]
    pub unsafe fn GetSourceId(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
}
impl ::core::convert::From<IDisplayPathInterop> for ::windows::core::IUnknown {
    fn from(value: IDisplayPathInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDisplayPathInterop> for ::windows::core::IUnknown {
    fn from(value: &IDisplayPathInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDisplayPathInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDisplayPathInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDisplayPathInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDisplayPathInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDisplayPathInterop {}
unsafe impl ::windows::core::Interface for IDisplayPathInterop {
    type Vtable = IDisplayPathInteropVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6ba4205_e59e_4e71_b25b_4e436d21ee3d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPathInteropVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pvalue: *mut super::super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psourceid: *mut u32) -> ::windows::core::HRESULT,
);
