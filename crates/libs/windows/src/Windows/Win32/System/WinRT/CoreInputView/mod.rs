#[doc = "*Required features: `\"Win32_System_WinRT_CoreInputView\"`*"]
#[repr(transparent)]
pub struct ICoreFrameworkInputViewInterop(::windows::core::IUnknown);
impl ICoreFrameworkInputViewInterop {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<'a, P0, T>(&self, appwindow: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::super::Foundation::HWND>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).GetForWindow)(::windows::core::Interface::as_raw(self), appwindow.into(), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
impl ::core::convert::From<ICoreFrameworkInputViewInterop> for ::windows::core::IUnknown {
    fn from(value: ICoreFrameworkInputViewInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICoreFrameworkInputViewInterop> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICoreFrameworkInputViewInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreFrameworkInputViewInterop> for ::windows::core::IUnknown {
    fn from(value: &ICoreFrameworkInputViewInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ICoreFrameworkInputViewInterop> for ::windows::core::IInspectable {
    fn from(value: ICoreFrameworkInputViewInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICoreFrameworkInputViewInterop> for &'a ::windows::core::IInspectable {
    fn from(value: &'a ICoreFrameworkInputViewInterop) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreFrameworkInputViewInterop> for ::windows::core::IInspectable {
    fn from(value: &ICoreFrameworkInputViewInterop) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICoreFrameworkInputViewInterop {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreFrameworkInputViewInterop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreFrameworkInputViewInterop {}
impl ::core::fmt::Debug for ICoreFrameworkInputViewInterop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreFrameworkInputViewInterop").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICoreFrameworkInputViewInterop {
    type Vtable = ICoreFrameworkInputViewInterop_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e3da342_b11c_484b_9c1c_be0d61c2f6c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewInterop_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetForWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appwindow: super::super::super::Foundation::HWND, riid: *const ::windows::core::GUID, coreframeworkinputview: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetForWindow: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
