#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowManagementPreview(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowManagementPreview {
    type Vtable = IWindowManagementPreview_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ef55b0d_561d_513c_a67c_2c02b69cef41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowManagementPreview_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWindowManagementPreviewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWindowManagementPreviewStatics {
    type Vtable = IWindowManagementPreviewStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f9725c6_c004_5a23_8fd2_8d092ce2704a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowManagementPreviewStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub SetPreferredMinSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: ::windows::core::RawPtr, preferredframeminsize: super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetPreferredMinSize: usize,
}
#[doc = "*Required features: `\"UI_WindowManagement_Preview\"`*"]
#[repr(transparent)]
pub struct WindowManagementPreview(::windows::core::IUnknown);
impl WindowManagementPreview {
    #[doc = "*Required features: `\"UI_WindowManagement_Preview\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPreferredMinSize<'a, Param0: ::windows::core::IntoParam<'a, super::AppWindow>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>>(window: Param0, preferredframeminsize: Param1) -> ::windows::core::Result<()> {
        Self::IWindowManagementPreviewStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetPreferredMinSize)(::core::mem::transmute_copy(this), window.into_param().abi(), preferredframeminsize.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IWindowManagementPreviewStatics<R, F: FnOnce(&IWindowManagementPreviewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WindowManagementPreview, IWindowManagementPreviewStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for WindowManagementPreview {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WindowManagementPreview {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WindowManagementPreview {}
impl ::core::fmt::Debug for WindowManagementPreview {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WindowManagementPreview").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WindowManagementPreview {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WindowManagement.Preview.WindowManagementPreview;{4ef55b0d-561d-513c-a67c-2c02b69cef41})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WindowManagementPreview {
    type Vtable = IWindowManagementPreview_Vtbl;
    const IID: ::windows::core::GUID = <IWindowManagementPreview as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WindowManagementPreview {
    const NAME: &'static str = "Windows.UI.WindowManagement.Preview.WindowManagementPreview";
}
impl ::core::convert::From<WindowManagementPreview> for ::windows::core::IUnknown {
    fn from(value: WindowManagementPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowManagementPreview> for ::windows::core::IUnknown {
    fn from(value: &WindowManagementPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WindowManagementPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WindowManagementPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WindowManagementPreview> for ::windows::core::IInspectable {
    fn from(value: WindowManagementPreview) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WindowManagementPreview> for ::windows::core::IInspectable {
    fn from(value: &WindowManagementPreview) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WindowManagementPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WindowManagementPreview {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WindowManagementPreview {}
unsafe impl ::core::marker::Sync for WindowManagementPreview {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
