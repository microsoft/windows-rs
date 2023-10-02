#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ICoreFrameworkInputViewInterop(::windows_core::IUnknown);
impl ICoreFrameworkInputViewInterop {
    #[doc = "Required features: `\"Win32_Foundation\"`"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<P0, T>(&self, appwindow: P0) -> ::windows_core::Result<T>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::HWND>,
        T: ::windows_core::ComInterface,
    {
        let mut result__ = ::std::ptr::null_mut();
        (::windows_core::Interface::vtable(self).GetForWindow)(::windows_core::Interface::as_raw(self), appwindow.into_param().abi(), &<T as ::windows_core::ComInterface>::IID, &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(ICoreFrameworkInputViewInterop, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::windows_core::Interface for ICoreFrameworkInputViewInterop {
    type Vtable = ICoreFrameworkInputViewInterop_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ICoreFrameworkInputViewInterop {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x0e3da342_b11c_484b_9c1c_be0d61c2f6c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewInterop_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows_core::GUID, coreframeworkinputview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
