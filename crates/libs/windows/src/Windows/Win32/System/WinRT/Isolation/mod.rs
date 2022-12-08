#[doc = "*Required features: `\"Win32_System_WinRT_Isolation\"`*"]
#[repr(transparent)]
pub struct IIsolatedEnvironmentInterop(::windows::core::IUnknown);
impl IIsolatedEnvironmentInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHostHwndInterop<P0>(&self, containerhwnd: P0) -> ::windows::core::Result<super::super::super::Foundation::HWND>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHostHwndInterop)(::windows::core::Vtable::as_raw(self), containerhwnd.into(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IIsolatedEnvironmentInterop, ::windows::core::IUnknown);
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
unsafe impl ::windows::core::Vtable for IIsolatedEnvironmentInterop {
    type Vtable = IIsolatedEnvironmentInterop_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedEnvironmentInterop {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85713c2e_8e62_46c5_8de2_c647e1d54636);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedEnvironmentInterop_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHostHwndInterop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, containerhwnd: super::super::super::Foundation::HWND, hosthwnd: *mut super::super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHostHwndInterop: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
