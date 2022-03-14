#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_System_WinRT_Isolation\"`*"]
#[repr(transparent)]
pub struct IIsolatedEnvironmentInterop(::windows::core::IUnknown);
impl IIsolatedEnvironmentInterop {
    #[doc = "*Required features: `\"Win32_System_WinRT_Isolation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHostHwndInterop<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::HWND>>(&self, containerhwnd: Param0) -> ::windows::core::Result<super::super::super::Foundation::HWND> {
        let mut result__: super::super::super::Foundation::HWND = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetHostHwndInterop)(::core::mem::transmute_copy(self), containerhwnd.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::HWND>(result__)
    }
}
impl ::core::convert::From<IIsolatedEnvironmentInterop> for ::windows::core::IUnknown {
    fn from(value: IIsolatedEnvironmentInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IIsolatedEnvironmentInterop> for ::windows::core::IUnknown {
    fn from(value: &IIsolatedEnvironmentInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IIsolatedEnvironmentInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IIsolatedEnvironmentInterop {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IIsolatedEnvironmentInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IIsolatedEnvironmentInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIsolatedEnvironmentInterop {}
impl ::core::fmt::Debug for IIsolatedEnvironmentInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIsolatedEnvironmentInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IIsolatedEnvironmentInterop {
    type Vtable = IIsolatedEnvironmentInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85713c2e_8e62_46c5_8de2_c647e1d54636);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedEnvironmentInterop_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHostHwndInterop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, containerhwnd: super::super::super::Foundation::HWND, hosthwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHostHwndInterop: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
